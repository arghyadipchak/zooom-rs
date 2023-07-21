use std::{
  fs, io,
  path::Path,
  process::{Child, Command},
};

use chrono::{Datelike, Duration, Local, NaiveDate, NaiveTime, Weekday};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum MFrequency {
  Once(NaiveDate),
  Daily,
  Weekly(Vec<Weekday>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Meeting {
  name: String,
  freq: MFrequency,
  start: NaiveTime,
  end: NaiveTime,
  metno: String,
  paswd: Option<String>,
}

#[derive(Debug, Error)]
pub enum MeetingReadError {
  #[error(transparent)]
  IOError(#[from] io::Error),

  #[cfg(feature = "json")]
  #[error(transparent)]
  JsonError(#[from] serde_json::Error),

  #[cfg(feature = "toml")]
  #[error(transparent)]
  TomlError(#[from] toml::de::Error),

  #[cfg(feature = "yaml")]
  #[error(transparent)]
  YamlError(#[from] serde_yaml::Error),

  #[error("Format not supported!")]
  FormatNotSupported,
}

impl Meeting {
  pub fn read_meetings<P: AsRef<Path>>(
    path: P,
  ) -> Result<Vec<Meeting>, MeetingReadError> {
    let s = fs::read_to_string(&path)?;

    Ok(
      match path
        .as_ref()
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
      {
        #[cfg(feature = "json")]
        "json" => serde_json::from_str(&s)?,

        #[cfg(feature = "toml")]
        "toml" => toml::from_str(&s)?,

        #[cfg(feature = "yaml")]
        "yaml" | "yml" => serde_yaml::from_str(&s)?,

        _ => return Err(MeetingReadError::FormatNotSupported),
      },
    )
  }

  pub fn is_now(&self, buffer_start: i64, buffer_end: i64) -> bool {
    let now = Local::now().naive_local();

    if let MFrequency::Once(date) = self.freq {
      if date != now.date() {
        return false;
      }
    }

    if let MFrequency::Weekly(days) = &self.freq {
      if !days.contains(&now.weekday()) {
        return false;
      }
    }

    self.start - now.time() <= Duration::seconds(buffer_start)
      && self.end - now.time() >= Duration::seconds(buffer_end)
  }

  fn get_url(&self) -> String {
    format!(
      "zoommtg://zoom.us/join?confno={}{}",
      self.metno,
      self.paswd.as_ref().map_or("".to_string(), |p| format!(
        "{}&pwd={}",
        if cfg!(windows) { "^" } else { "" },
        p
      ))
    )
  }

  pub fn join(&self) -> Result<Child, io::Error> {
    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(self.get_url()).spawn()
  }

  #[cfg(target_os = "macos")]
  pub fn join(&self) -> Result<Child, io::Error> {
    Command::new("open").arg(self.get_zoom_url()).spawn()
  }

  #[cfg(target_os = "windows")]
  pub fn join(&self) -> Result<Child, io::Error> {
    Command::new("cmd")
      .args(["/c", "start", "/wait"])
      .arg(self.get_zoom_url())
      .spawn()
  }
}
