use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Menu {}

impl Menu {
  pub fn new() -> Self {
    Menu {}
  }

  fn clear_screen(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    std::io::stdout().flush().unwrap();
    print!("{esc}[?25l", esc = 27 as char);
  }

  fn print_menu(&self) {
    self.clear_screen();
    println!(" [+] Welcome to the program");
    println!(" [+] This program will monitor your MBBank account and notify you when a new transaction is detected");
    println!();
    println!(" [CREDITS] Made by @mewthedev");
    println!();
    println!("{}", "-".repeat(30));
    println!(" [+] Choose an option:");
    println!(" [1] Start monitoring");
    println!(" [2] Set API key");
    println!(" [3] Exit");
    println!("{}", "-".repeat(30));
    print!(" [+] Your choice: ");
  }

  fn set_api_key(&self) -> Result<()> {
    print!(" [+] Please enter your API key: ");
    std::io::stdout().flush().unwrap();
    let mut file = fs::File::create(".env")?;
    let mut apikey_string = String::new();
    std::io::stdin().read_line(&mut apikey_string)?;
    let apikey_string = apikey_string.trim();
    file.write_all(format!("API_KEY={}", apikey_string).as_bytes())?;

    println!(" [+] API key saved");
    println!(" [+] Restart the program to start monitoring");
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
        "1" => {
          println!(" [+] Starting monitoring");
          break;
        }
        "2" => {
          self.clear_screen();
          self.set_api_key().unwrap();
          println!(" [+] Press enter to close the program");
          self.wait_for_enter();
          std::process::exit(0);
        }
        "3" => {
          self.clear_screen();
          println!(" [-] Exiting program");
          std::process::exit(0);
        }
        _ => {
          self.clear_screen();
          println!(" [!] Invalid option");
          println!(" [!] Press enter to retry");
          self.wait_for_enter();
        }
      }
    }
  }
}
