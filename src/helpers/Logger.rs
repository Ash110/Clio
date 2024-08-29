use chrono::Local;
use colored::Colorize;

pub struct Logger {
    verbose: bool,
}

impl Logger {
    pub fn new(verbose: bool) -> Self {
        Logger { verbose }
    }

    pub fn log<S: AsRef<str>>(&self, message: S) {
        if self.verbose {
            let now = Local::now();
            println!("{} - {}", now.format("%H:%M:%S"), message.as_ref());
        }
    }

    pub fn success<S: AsRef<str>>(&self, message: S) {
        if self.verbose {
            let now = Local::now();
            println!("{} - {}", now.format("%H:%M:%S"), message.as_ref().green());
        }
    }

    pub fn warn<S: AsRef<str>>(&self, message: S) {
        if self.verbose {
            let now = Local::now();
            println!("{} - {}", now.format("%H:%M:%S"), message.as_ref().yellow());
        }
    }

    pub fn error<S: AsRef<str>>(&self, message: S) {
        if self.verbose {
            let now = Local::now();
            println!("{} - {}", now.format("%H:%M:%S"), message.as_ref().red());
        }
    }
}