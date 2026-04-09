use cpal::BuildStreamError;
#[allow(unused)]
use inline_colorization::{
  color_bright_cyan,
  color_red,
  color_reset
};
use std::error::Error;

#[cfg(target_os = "macos")]
use crate::DEFAULT_INPUT;

#[cfg(target_os = "macos")]
pub fn default_not_found() -> String {
  format!(
    "\n{}Error:{} {} driver not found.\n\
    \nInstall with:\n {}$ brew install --cask blackhole-2ch{}\n\
    or:\n{} $ port install BlackHole{}\n",
    color_red,
    color_reset,
    DEFAULT_INPUT,
    color_bright_cyan,
    color_reset,
    color_bright_cyan,
    color_reset,
  )
}

#[cfg(target_os = "linux")]
use crate::DEFAULT_INPUT;

#[cfg(target_os = "linux")]
pub fn default_not_found() -> String {
    format!(
        "\n{}Error:{} {} driver not found.\n\
        \nInstall with:\n {}$ pacman -Syuy pipewire pipewire-alsa pipewire-jack{}\n\
        or:\n{} $ apt-get install pipewire pipewire-alsa pipewire-jack{}\n",
        color_red,
        color_reset,
        DEFAULT_INPUT,
        color_bright_cyan,
        color_reset,
        color_bright_cyan,
        color_reset,
    )
}

pub const AUDIO_INTERFACE_NOT_FOUND: &str = "\nThe audio_interface you have chosen does not match anything on your system. \n\
                                            Check spelling in config.toml, and make sure it is installed and connected correctly.";

pub fn handle_input_build_error(err: BuildStreamError) -> anyhow::Error {
  match err {
    BuildStreamError::StreamConfigNotSupported => anyhow::anyhow!(
      "StreamConfigNotSupported: \n\
      \tSome requirements for running Tau is not met by your audio source. \n\
      \tCheck samplerate, it should be 48kHz.\
      \n\tPlease adjust your system audio settings and try again."
    ),
    BuildStreamError::InvalidArgument => {
      anyhow::anyhow!("Argument to underlying C-functions were not understood.")
    }
    BuildStreamError::StreamIdOverflow => {
      anyhow::anyhow!("ID of stream caused an integer overflow.")
    }
    BuildStreamError::DeviceNotAvailable => {
      anyhow::anyhow!("Audio Device is no longer available.")
    }
    BuildStreamError::BackendSpecific { err } => anyhow::anyhow!(
      "BackendSpecificError: {}\n\n{:?}",
      err.description,
      err.source()
    ),
  }
}
