use chrono::Local;
use std::sync::Arc;
use std::path::Path;
use inline_colorization::*;
use regex::Regex;
use std::sync::LazyLock;

/// Formats the file name of output file, using current local datetime.
/// If no filename is given in cli arguments. default = `tau_[datetime].ogg`
pub fn format_filename(filename: Option<String>) -> Arc<String> {
  let now = Local::now().format("%d-%m-%Y_%H_%M_%S").to_string();
  Arc::new(
    filename
      .map(|f| format!("{f}.ogg"))
      .unwrap_or_else(|| format!("tau_{}.ogg", now)),
  )
}

pub fn create_recordings_dir(path: &Path) -> Result<(), std::io::Error> {
  std::fs::create_dir_all(path)?;
  println!(
    "{}First time run:\t\t{}Created directory for saving recorded sessions{}", 
    color_bright_yellow,
    color_bright_cyan, 
    color_reset
  );
  Ok(())
}

pub mod consts {
  /// Ogg opus fixed samplerate
  pub const DEFAULT_SR: i32 = 48000;
  /// Ogg opus fixed channel size
  // TODO: Handle multichannel stream based on user config
  pub const DEFAULT_CH: usize = 2;
  #[cfg(target_os = "macos")]
  pub const DEFAULT_INPUT: &str = "BlackHole 2ch";
  #[cfg(target_os = "linux")]
  pub const DEFAULT_INPUT: &str = "pipewire";


}

#[allow(clippy::expect_used)]
pub static URL_RE: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"(^$|((http(s)?)(://))?([\w-]+\.)+[\w-]+([\w\- ;,./?%&=]*))")
    .expect("regex is malformed and could not be built")
});


#[allow(clippy::expect_used)]
pub static IP_RE: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$")
    .expect("regex is malformed and could not be built")
});
