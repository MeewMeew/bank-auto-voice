#![allow(dead_code)]

mod config;
mod mb;
mod menu;

use anyhow::{Ok, Result};
use config::Config;
use std::os::windows::process::CommandExt;
use std::path::Path;

static mut CONFIG: Config = Config::new();

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
        eprintln!(" [!] mpg123 not found");
        return Err(anyhow::anyhow!("mpg123 not found"));
      }
    }
  }

  let output = std::process::Command::new(cmd.display().to_string())
    .creation_flags(0x08000000)
    .arg(query)
    .output()?;

  if output.status.success() {
    Ok(())
  } else {
    unsafe {
      if CONFIG.debug {
        eprintln!(" [!] Failed to play MP3");
      }
    }
    Err(anyhow::anyhow!("Failed to play MP3"))
  }
}

fn main() -> Result<()> {
  unsafe {
    CONFIG = Config::read()?;
  }

  let mut menu = menu::Menu::new();
  menu.init();

  println!(" [+] Program started");
  let mut mb = mb::MB::new();
  loop {
    mb.fetch_transaction().expect("Failed to fetch transaction");
    if mb
      .compare_transaction()
      .expect("Failed to compare transaction")
    {
      let latest_transaction = mb.get_latest_transaction()?;
      if latest_transaction.amount > 0 {
        println!();
        println!("  [-] New transaction detected");
        let text = format!("Đã nhận được số tiền {} đồng.", latest_transaction.amount);
        let lang = "vi";
        let query = build_query(&text, lang)?;
        play_mp3(&query)?;
        println!("  [-] Notification played");
        println!();
      };
    }
    std::thread::sleep(std::time::Duration::from_secs(5));
  }
}
