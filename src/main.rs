#![allow(dead_code, unused_imports)]

mod mods;
mod panic_catching;

use anyhow::{Ok, Result};
use mods::{config::Config, logger::Logger, mb::MB, menu};
use std::os::windows::process::CommandExt;
use std::path::Path;

static mut CONFIG: Config = Config::new();
static LOGGER: Logger = Logger::new();

fn build_query(text: &str, lang: &str) -> Result<String> {
  let url = format!(
    "http://translate.google.com/translate_tts?ie=UTF-8&q={}&tl={}&client=tw-ob",
    urlencoding::encode(text).to_string().as_str(),
    lang
  );

  Ok(url)
}

fn play_mp3(query: &str) -> Result<()> {
  let current_dir = std::env::current_dir()?;

  let cmd = current_dir.join("bin\\mpg123.exe");

  if !Path::new(&cmd).exists() {
    unsafe {
      if CONFIG.debug {
        LOGGER.error("mpg123 not found");
        return Err(anyhow::anyhow!("mpg123 not found"));
      }
    }
  }

  let output = std::process::Command::new(cmd.display().to_string())
    .creation_flags(0x08000000)
    .arg(query)
    .output()
    .expect("Failed to play MP3");

  if output.status.success() {
    Ok(())
  } else {
    unsafe {
      if CONFIG.debug {
        LOGGER.error("Failed to play MP3");
      }
    }
    Err(anyhow::anyhow!("Failed to play MP3"))
  }
}

fn main() {
  panic_catching::init();
  panic_catching::trace(|| {
    unsafe {
      CONFIG = Config::read().expect("Failed to read config");
    }

    let mut menu = menu::Menu::new();
    menu.init();

    let mut mb = MB::new();
    loop {
      mb.fetch_transaction().expect("Failed to fetch transaction");
      if mb
        .compare_transaction()
        .expect("Failed to compare transaction")
      {
        let latest_transaction = mb
          .get_latest_transaction()
          .expect("Failed to get latest transaction");
        if latest_transaction.amount > 0 {
          unsafe {
            if CONFIG.debug {
              println!("{}", "-".repeat(30));
              LOGGER.info("Transaction found");
            }
          }
          let text = format!("Đã nhận được số tiền {} đồng.", latest_transaction.amount);
          let lang = "vi";
          let query = build_query(&text, lang).expect("Failed to build query");
          play_mp3(&query).expect("Failed to play MP3");
          unsafe {
            if CONFIG.debug {
              LOGGER.done("Notification processed");
              println!("{}", "-".repeat(30));
            }
          }
        };
      }
      std::thread::sleep(std::time::Duration::from_secs(unsafe {
        CONFIG.refresh_interval as u64
      }));
    }
  });
}
