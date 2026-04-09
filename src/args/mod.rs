use clap::Parser;

// use crate::StreamType;
use crate::{config::TauConfigError, util::{IP_RE, URL_RE}};

#[derive(Parser)]
#[command(name = "tau-radio")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command( about = "Webradio client, Hijacks chosen audio device, encodes into Ogg Opus and streams to tau-tower radio server")]
pub(crate) struct Args {
    /// Webradio server username
    #[arg(long)]
    pub username: Option<String>,

    /// Webradio server password
    #[arg(long)]
    pub password: Option<String>,

    /// Tau-tower server ip
    #[arg(short, long, value_parser=|s: &str| validate_url_or_ip(s.to_string()))]
    pub url: Option<String>,

    /// Tau-tower server port
    #[arg(short='p', long, value_parser=|p: &str| validate_port(parse_port(p).unwrap()))]
    pub upstream_port: Option<u16>,

    /// Optional custom filename of local copy
    #[arg(short, long)]
    pub file: Option<String>,

    /// Disables the local recording of stream
    #[arg(long)]
    pub no_recording: bool,

    /// Output directory [default: $HOME/tau/recordings/]
    #[arg(short, long)]
    pub output: Option<String>,

    /// Resets config.toml 
    #[arg(long)]
    pub reset_config: bool,
}


pub fn validate_url_or_ip(ip_or_url: String) -> Result<String, TauConfigError> {
  if IP_RE.is_match(&ip_or_url) || URL_RE.is_match(&ip_or_url) { return Ok(ip_or_url) } 
  Err(TauConfigError::InvalidUrl(ip_or_url))
}

fn parse_port(p: &str) -> Result<u16, TauConfigError> {
  p.parse::<u16>()
    .map_err(|e| TauConfigError::Input(format!("Unable to parse as number: {e}")))
}

pub fn validate_port(port: u16) -> Result<u16, TauConfigError> {
  if !(1..=0xFFFF).contains(&port) {
    return Err(TauConfigError::InvalidPort(port.to_string()));
  }
  Ok(port)
}

// pub fn validate_stream_type(t: &str) -> Result<StreamType, TauConfigError> {
//   match t {
//     "icecast" => Ok(StreamType::IceCast),
//     "websocket" => Ok(StreamType::WebSocket),
//     _ => Err(TauConfigError::Input("Invalid streaming type".to_string())),
//   }
// }
