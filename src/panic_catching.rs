use super::mods::{config::Config, logger::Logger};

use std::{backtrace::Backtrace, cell::Cell, panic, process};

static mut CONFIG: Config = Config::new();
static LOGGER: Logger = Logger::new();

thread_local! {
  pub static BACKTRACE: Cell<Option<Backtrace>> = const {Cell::new(None)};
}

pub fn init() {
  panic::set_hook(Box::new(|_| {
    let trace = Backtrace::force_capture();
    BACKTRACE.with(move |b| b.set(Some(trace)));
  }));
}

pub fn trace<F, T>(f: F)
where
  F: FnOnce() -> T + Send + 'static + panic::UnwindSafe,
  T: Send + 'static,
{
  if let Err(error) = panic::catch_unwind(f) {
    unsafe {
      CONFIG = Config::read().expect("Failed to read config");
    }
    let trace = BACKTRACE
      .with(|b| b.take())
      .expect("Cannot get trace")
      .to_string();

    let mut trimmed_trace = trace
      .split("\n")
      .filter(|e| e.contains("at ") && !e.contains("/rustc/") && e.contains(".rs"))
      .map(|e| format!("{:>4}", "") + e.trim())
      .collect::<Vec<_>>()
      .join("\n");

    if trimmed_trace.is_empty() {
      trimmed_trace = "- Trace:\n".to_string() + "No backtrace available";
    } else {
      trimmed_trace = "- Trace:\n".to_string() + &trimmed_trace;
    }

    let error = if let Some(error) = error.downcast_ref::<String>() {
      format!("- Error: {}", error)
    } else if let Some(error) = error.downcast_ref::<&str>() {
      format!("- Error: {}", error)
    } else {
      "- Error: Unknown".to_string()
    };

    LOGGER.error(&error.as_str());
    unsafe {
      if CONFIG.debug {
        LOGGER.error(trimmed_trace.as_str());
      }
    }
    LOGGER.error("The program will now exit");
    process::exit(0);
  }
}
