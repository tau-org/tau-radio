use std::{
    net::{SocketAddr, TcpStream}, 
    path::Path,
    sync::{
      atomic::{AtomicBool, Ordering},
      Arc
    },
    thread::{sleep, spawn},
    time::{Duration, Instant}
};

use tungstenite::{
  connect,
  http::Uri,
  stream::MaybeTlsStream,
  ClientRequestBuilder,
  Message,
  WebSocket
};

use crossbeam::channel::{Receiver, bounded};
use ringbuf::traits::Consumer;

use crate::{Credentials, DEFAULT_CH};
use crate::audio::{
  audio_capture_loop,
  encode_audio,
  record_audio
};

const LOG_TIME: Duration = Duration::from_secs(10);

pub fn thread(
    mut rx: impl Consumer<Item = f32> + Send + 'static,
    url: SocketAddr,
    filename: Arc<String>,
    credentials: Credentials,
    shutdown: Arc<AtomicBool>
) {
  let framesize = 960 * DEFAULT_CH;
  let (opus_tx, opus_rx) = bounded::<Vec<u8>>(4096 * 32);
  let (audio_tx, audio_rx) = bounded::<f32>(4096 * 32);

  let shutdown_clone = shutdown.clone();
  let audio_capture_thread = spawn(move || {
    audio_capture_loop(shutdown_clone, &mut rx, &[audio_tx]);
  });

  let shutdown_clone = shutdown.clone();
  // Encoding thread
  let encoder_thread = spawn(move || {
    encode_audio(shutdown_clone, filename, &audio_rx, &opus_tx, framesize);
  });

  websocket_connect_loop(shutdown, &opus_rx, &url, &credentials);

  if let Err(e) = audio_capture_thread.join() { eprintln!("Audio capture join thread error: {e:?}") };
  if let Err(e) = encoder_thread.join() { eprintln!("Encoder thread join error: {e:?}") };
}

pub fn rec_thread(
    mut rx: impl Consumer<Item = f32> + Send + 'static,
    url: SocketAddr,
    path: &Path,
    filename: Arc<String>,
    credentials: Credentials,
    shutdown: Arc<AtomicBool>
) {
  let framesize = 960 * DEFAULT_CH;
  let (opus_tx, opus_rx) = bounded::<Vec<u8>>(4096 * 32);
  let (encode_tx, encode_rx) = bounded::<f32>(4096 * 32);
  let (record_tx, record_rx) = bounded::<f32>(4096 * 32);
  
  let shutdown_clone = shutdown.clone();
  let audio_capture_thread = spawn(move || {
    audio_capture_loop(shutdown_clone, &mut rx, &[encode_tx, record_tx]);
  });

  let shutdown_clone = shutdown.clone();
  let filename_clone = filename.clone();
  // Encoding thread
  let encoder_thread = spawn(move || {
    encode_audio(shutdown_clone, filename_clone, &encode_rx, &opus_tx, framesize);
  });

  let filename_clone = filename.clone();
  let shutdown_clone = shutdown.clone();
  let out_path = path.join(filename.clone().to_string());
  // Recording thread
  let recorder_thread = spawn(move || {
    record_audio(shutdown_clone, filename_clone, &record_rx, &out_path, framesize);
  });

  websocket_connect_loop(shutdown, &opus_rx, &url, &credentials);

  if let Err(e) = audio_capture_thread.join() { eprintln!("Audio capture join thread error: {e:?}") };
  if let Err(e) = recorder_thread.join() { eprintln!("Recorder thread join error: {e:?}") };
  if let Err(e) = encoder_thread.join() { eprintln!("Encoder thread join error: {e:?}") };
}

fn handle_websocket(shutdown: Arc<AtomicBool>, ws: &mut WebSocket<MaybeTlsStream<TcpStream>>, rx: &Receiver<Vec<u8>>) {
  'outer: loop {
    while let Ok(page) = rx.recv() {
      if shutdown.load(Ordering::SeqCst) { break 'outer; }
      if let Err(e) = ws.send(Message::Binary(page.into())) {
        eprintln!("Websocket send error: {e}");
        return;
      }
    }
  }
}


fn websocket_connect_loop(shutdown: Arc<AtomicBool>, opus_rx: &Receiver<Vec<u8>>, url: &SocketAddr, credentials: &Credentials) {
  let connected = Arc::new(AtomicBool::new(false));
  let uri = Uri::builder()
    .scheme("ws")
    .authority(format!("{}:{}", url.ip(), url.port()))
    .path_and_query("/")
    .build()
    .unwrap();

  let request = ClientRequestBuilder::new(uri.clone())
    .with_header("password", credentials.password.clone())
    .with_header("username", credentials.username.clone())
    .with_header("port", credentials.broadcast_port.to_string());

  let mut last_log = Instant::now();

  loop {
    if shutdown.load(Ordering::SeqCst) { break; }
    if !connected.load(Ordering::SeqCst) {
      match connect(request.clone()) {
        Ok((mut ws, _)) => {
          connected.store(true, Ordering::SeqCst);
          let connected_inner = connected.clone();
          let opus_rx_receiver = opus_rx.clone();
          let shutdown_clone = shutdown.clone();
          spawn(move || {
            handle_websocket(shutdown_clone, &mut ws, &opus_rx_receiver);
            connected_inner.store(false, Ordering::SeqCst);
          });
        }
        Err(e) => {
          if last_log.elapsed() > LOG_TIME {
            eprintln!("HandshakeError: {e}");
            last_log = Instant::now();
          }
          sleep(std::time::Duration::from_millis(50));
        }
      }
    } else {
      sleep(std::time::Duration::from_millis(50));
    }
  }

}
