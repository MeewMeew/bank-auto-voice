pub struct Logger {}

impl Logger {
  pub const fn new() -> Self {
    Logger {}
  }

  pub fn red(&self, message: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", message)
  }

  pub fn green(&self, message: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", message)
  }

  pub fn yellow(&self, message: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", message)
  }

  pub fn blue(&self, message: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", message)
  }

  pub fn cyan(&self, message: &str) -> String {
    format!("\x1b[36m{}\x1b[0m", message)
  }

  pub fn magenta(&self, message: &str) -> String {
    format!("\x1b[35m{}\x1b[0m", message)
  }

  pub fn bold(&self, message: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", message)
  }

  pub fn log(&self, message: &str) {
    println!("[{}] {}", self.blue("LOG"), message);
  }

  pub fn error(&self, message: &str) {
    println!(
      "[{}] {}",
      self.bold(self.red("ERROR").as_str()),
      self.bold(message)
    );
  }

  pub fn warn(&self, message: &str) {
    println!("[{}] {}", self.bold(self.yellow("WARN").as_str()), message);
  }

  pub fn info(&self, message: &str) {
    println!("[{}] {}", self.bold(self.green("INFO").as_str()), message);
  }

  pub fn done(&self, message: &str) {
    println!(
      "[{}] {}",
      self.bold(self.green("DONE").as_str()),
      self.bold(message)
    );
  }

  pub fn prompt(&self, message: &str) {
    print!(
      "[{}] {}",
      self.bold(self.magenta("PROMPT").as_str()),
      message
    );
  }

  pub fn option(&self, option: &str, message: &str) {
    println!(
      "[{}] {}",
      self.bold(self.magenta(option.to_uppercase().as_str()).as_str()),
      message
    );
  }
}
