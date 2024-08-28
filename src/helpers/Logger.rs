use chrono::Local;
use colored::Colorize;

pub struct Logger {
    verbose: bool,
}

impl Logger {
    pub fn new(verbose: bool) -> Self {
        Logger { verbose }
    }

    pub fn log(&self, message: String) {
        if self.verbose {
            let now = Local::now();
            println!("{} - {}", now.format("%H:%M:%S"), message);
        }
    }

    pub fn success(&self, message: String) {
        if self.verbose {
            let now = Local::now();
            println!("{} - {}", now.format("%H:%M:%S"), message.green());
        }
    }

    pub fn warn(&self, message: String) {
        if self.verbose {
            let now = Local::now();
            println!("{} - {}", now.format("%H:%M:%S"), message.yellow());
        }
    }

    pub fn error(&self, message: String) {
        if self.verbose {
            let now = Local::now();
            println!("{} - {}", now.format("%H:%M:%S"), message.red());
        }
    }
}