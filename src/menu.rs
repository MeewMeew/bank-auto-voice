use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;

use super::config::Config;

static mut CONFIG: Config = Config::new();

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Menu {}

impl Menu {
  pub fn new() -> Self {
    unsafe {
      CONFIG = Config::read().unwrap();
    }
    Menu {}
  }

  fn clear_screen(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    std::io::stdout().flush().unwrap();
    print!("{esc}[?30l", esc = 27 as char);
  }

  fn print_credit(&self) {
    println!(" [+] Welcome to the program");
    println!(" [+] This program will monitor your MBBank account and notify you when a new transaction is detected");
    println!();
    println!(" [CREDITS] Made by @mewthedev");
    println!();
    println!("{}", "-".repeat(30));
  }

  fn print_menu(&self) {
    self.clear_screen();
    self.print_credit();
    println!(" [+] Choose an option:");
    println!("{}", "-".repeat(30));
    println!(" [1] Start monitoring");
    println!(" [2] Configure");
    println!("{}", "-".repeat(30));
    println!(" [0] Exit program");
    println!("{}", "-".repeat(30));
    print!(" [+] Your choice: ");
  }

  fn print_configure(&self) {
    self.clear_screen();
    self.print_credit();
    println!(" [+] Choose an option:");
    println!("{}", "-".repeat(30));
    println!(" [1] Set API key ({})", unsafe {
      if CONFIG.apikey.is_empty() {
        "not set"
      } else {
        "set"
      }
    });
    println!(" [2] Set refresh interval ({} seconds)", unsafe {
      CONFIG.refresh_interval
    });
    println!(" [3] Toggle debug mode ({})", unsafe {
      if CONFIG.debug {
        "enabled"
      } else {
        "disabled"
      }
    });
    println!("{}", "-".repeat(30));
    println!(" [0] Back to main menu");
    println!("{}", "-".repeat(30));
    print!(" [+] Your choice: ");
  }

  fn set_api_key(&self) -> Result<()> {
    self.clear_screen();
    self.print_credit();
    loop {
      print!(" [+] Please enter your API key: ");
      std::io::stdout().flush().unwrap();
      let mut apikey_string = String::new();
      std::io::stdin().read_line(&mut apikey_string)?;
      let apikey_string = apikey_string.trim();
      if apikey_string.is_empty() {
        println!(" [!] API key cannot be empty");
      } else {
        unsafe {
          CONFIG.set_apikey(apikey_string);
          CONFIG.write()?;
        }
        println!(" [+] API key saved");
        break;
      }
    }
    Ok(())
  }

  fn set_refresh_interval(&self) -> Result<()> {
    self.clear_screen();
    self.print_credit();
    let mut interval: u16;
    loop {
      print!(" [+] Please enter the refresh interval (in seconds): ");
      std::io::stdout().flush().unwrap();
      let mut interval_string = String::new();
      std::io::stdin().read_line(&mut interval_string)?;
      let interval_string = interval_string.trim();
      interval = interval_string
        .parse::<u16>()
        .expect(" [!] Invalid interval");
      if interval < 1 {
        println!(" [!] Interval must be at least 1 second");
      } else {
        break;
      }
    }

    unsafe {
      CONFIG.set_refresh_interval(interval);
      CONFIG.write()?;
    }
    println!(" [+] Refresh interval saved");
    Ok(())
  }

  fn toggle_debug(&self) -> Result<()> {
    self.clear_screen();
    self.print_credit();
    unsafe {
      CONFIG.toggle_debug();
      CONFIG.write()?;
      println!(
        " [+] Debug mode {}",
        if CONFIG.debug { "enabled" } else { "disabled" }
      );
    }
    Ok(())
  }

  fn wait_for_enter(&self) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
  }

  pub fn init(&mut self) {
    loop {
      self.print_menu();
      std::io::stdout().flush().unwrap();

      let mut option = String::new();
      std::io::stdin().read_line(&mut option).unwrap();
      let option = option.trim();

      match option {
        "0" => {
          self.clear_screen();
          println!(" [-] Exiting program");
          std::process::exit(0);
        }
        "1" => {
          println!(" [+] Starting monitoring");
          break;
        }
        "2" => loop {
          self.clear_screen();
          self.print_configure();
          std::io::stdout().flush().unwrap();
          let mut setting_option = String::new();
          std::io::stdin().read_line(&mut setting_option).unwrap();
          let setting_option = setting_option.trim();
          match setting_option {
            "0" => {
              self.clear_screen();
              self.print_menu();
              break;
            }
            "1" => {
              self.clear_screen();
              self.set_api_key().unwrap();
              println!(" [!] Enter to back to main menu");
              self.wait_for_enter();
              self.print_configure();
            }
            "2" => {
              self.clear_screen();
              self.set_refresh_interval().unwrap();
              println!(" [!] Enter to back to main menu");
              self.wait_for_enter();
              self.print_configure();
            }
            "3" => {
              self.clear_screen();
              self.toggle_debug().unwrap();
              println!(" [!] Enter to back to main menu");
              self.wait_for_enter();
              self.print_configure();
            }
            _ => {
              self.clear_screen();
              self.print_credit();
              println!(" [!] Invalid option");
              println!(" [!] Press enter to retry");
              self.wait_for_enter();
            }
          }
        },
        _ => {
          self.clear_screen();
          self.print_credit();
          println!(" [!] Invalid option");
          println!(" [!] Press enter to retry");
          self.wait_for_enter();
        }
      }
    }
  }
}
