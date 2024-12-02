use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;

use super::{config::Config, logger::Logger};

static mut CONFIG: Config = Config::new();
static LOGGER: Logger = Logger::new();

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
    self.clear_screen();
    println!("{}", "-".repeat(30));
    println!();
    LOGGER.info("MBBank Transaction Monitor");
    LOGGER.info(
      r#"This program will monitor
     your MBBank account and 
     notify you when a new 
     transaction is detected    
    "#,
    );
    LOGGER.info(format!("Author: {}", LOGGER.cyan("@mewthedev")).as_str());
    println!();
    println!("{}", "-".repeat(30));
  }

  fn print_menu(&self) {
    self.print_credit();
    LOGGER.info("Choose an option:");
    println!("{}", "-".repeat(30));
    LOGGER.option("1", "Start monitoring");
    LOGGER.option("2", "Configure");
    LOGGER.option("0", "Exit program");
    println!("{}", "-".repeat(30));
    LOGGER.prompt("Your choice: ");
  }

  fn print_configure(&self) {
    self.print_credit();
    LOGGER.info("Choose an option:");
    println!("{}", "-".repeat(30));
    LOGGER.option(
      "1",
      format!("Set API key ({})", unsafe {
        if CONFIG.apikey.is_empty() {
          "not set"
        } else {
          "set"
        }
      })
      .as_str(),
    );
    LOGGER.option(
      "2",
      format!("Set refresh interval ({})", unsafe {
        CONFIG.refresh_interval
      })
      .as_str(),
    );
    LOGGER.option(
      "3",
      format!("Toggle debug mode ({})", unsafe {
        if CONFIG.debug {
          "enabled"
        } else {
          "disabled"
        }
      })
      .as_str(),
    );
    LOGGER.option("0", "Back to main menu");
    println!("{}", "-".repeat(30));
    LOGGER.prompt("Your choice: ");
  }

  fn set_api_key(&self) -> Result<()> {
    self.print_credit();
    loop {
      LOGGER.prompt("Enter your API key: ");
      std::io::stdout().flush().unwrap();
      let mut apikey_string = String::new();
      std::io::stdin().read_line(&mut apikey_string)?;
      let apikey_string = apikey_string.trim();
      if apikey_string.is_empty() {
        LOGGER.warn("API key cannot be empty");
      } else {
        unsafe {
          CONFIG.set_apikey(apikey_string);
          CONFIG.write()?;
        }
        LOGGER.done("API key saved");
        break;
      }
    }
    Ok(())
  }

  fn set_refresh_interval(&self) -> Result<()> {
    self.print_credit();
    let mut interval: u16;
    loop {
      LOGGER.prompt("Enter refresh interval (in seconds): ");
      std::io::stdout().flush().unwrap();
      let mut interval_string = String::new();
      std::io::stdin().read_line(&mut interval_string)?;
      let interval_string = interval_string.trim();
      interval = interval_string.parse::<u16>().expect(
        format!(
          "[{}] Invalid interval",
          LOGGER.blue(LOGGER.red("ERROR").as_str())
        )
        .as_str(),
      );
      if interval < 1 {
        LOGGER.warn("Interval must be at least 1 second");
      } else {
        break;
      }
    }

    unsafe {
      CONFIG.set_refresh_interval(interval);
      CONFIG.write()?;
    }
    LOGGER.done("Refresh interval saved");
    Ok(())
  }

  fn toggle_debug(&self) -> Result<()> {
    self.print_credit();
    unsafe {
      CONFIG.toggle_debug();
      CONFIG.write()?;
      LOGGER.done(
        format!(
          "Debug mode {}",
          if CONFIG.debug { "enabled" } else { "disabled" }
        )
        .as_str(),
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
          self.print_credit();
          LOGGER.info("Exiting program");
          LOGGER.info("Goodbye!");
          std::process::exit(0);
        }
        "1" => {
          self.print_credit();
          LOGGER.info("Monitoring started");
          LOGGER.info("Press Ctrl+C to stop monitoring");
          break;
        }
        "2" => loop {
          self.print_configure();
          std::io::stdout().flush().unwrap();
          let mut setting_option = String::new();
          std::io::stdin().read_line(&mut setting_option).unwrap();
          let setting_option = setting_option.trim();
          match setting_option {
            "0" => {
              self.print_menu();
              break;
            }
            "1" => {
              self.print_credit();
              self.set_api_key().unwrap();
              LOGGER.info("Enter to back to main menu");
              self.wait_for_enter();
              self.print_configure();
            }
            "2" => {
              self.print_credit();
              self.set_refresh_interval().unwrap();
              LOGGER.info("Enter to back to main menu");
              self.wait_for_enter();
              self.print_configure();
            }
            "3" => {
              self.print_credit();
              self.toggle_debug().unwrap();
              LOGGER.info("Enter to back to main menu");
              self.wait_for_enter();
              self.print_configure();
            }
            _ => {
              self.print_credit();
              LOGGER.error("Invalid option");
              LOGGER.info("Press enter to retry");
              self.wait_for_enter();
            }
          }
        },
        _ => {
          self.print_credit();
          LOGGER.error("Invalid option");
          LOGGER.info("Press enter to retry");
          self.wait_for_enter();
        }
      }
    }
  }
}
