use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Config {
  pub apikey: String,
  pub refresh_interval: u16,
  pub debug: bool,
}

impl Config {
  pub const fn new() -> Self {
    Config {
      apikey: String::new(),
      refresh_interval: 5,
      debug: false,
    }
  }

  fn get_path() -> Result<std::path::PathBuf> {
    let exe_path = std::env::current_exe()?;
    let config_path = std::path::Path::new(exe_path.parent().unwrap()).join("config.json");
    Ok(config_path)
  }

  pub fn toggle_debug(&mut self) {
    self.debug = !self.debug;
  }

  pub fn set_apikey(&mut self, apikey: &str) {
    self.apikey = apikey.to_string();
  }

  pub fn set_refresh_interval(&mut self, interval: u16) {
    self.refresh_interval = if interval < 1 { 1 } else { interval };
  }

  pub fn write(&self) -> Result<Self> {
    std::fs::write(Config::get_path()?, self.stringify()?)?;
    Ok(self.clone())
  }

  pub fn read() -> Result<Self> {
    let path = Config::get_path()?;
    if path.exists() {
      Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
    } else {
      let config = Config::new();
      config.write()?;
      Ok(config)
    }
  }

  pub fn stringify(&self) -> Result<String> {
    Ok(serde_json::to_string_pretty(self)?)
  }
}
