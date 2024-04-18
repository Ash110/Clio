use chrono::Local;
use colored::Colorize;

pub fn init(directory: Option<String>, verbose: bool) {
    macro_rules! log {
        ($($arg: expr),*) => {
            if verbose {
                let now = Local::now();
                println!(
                    "{} - {}",
                    now.format("%H:%M:%S"),
                    format!($($arg),*)
                );
            }
        };
    }

    let directory_string: String;
    if directory.as_ref().is_some() {
        if directory.as_ref().unwrap().starts_with("/") {
            directory_string = directory.unwrap();
        } else {
            directory_string = format!("./{}", directory.unwrap());
        }
    } else {
        directory_string = "./".to_string();
    }

    log!("Initializing in directory: {}", directory_string);

    // Check if the folder already has a .clio folder
    let clio_folder = format!("{}/.clio", directory_string);
    if std::path::Path::new(&clio_folder).exists() {
        println!("{}", "Clio is already initialized in this directory".red());
    } else {
        log!("Creating .clio folder");
        // Create the .clio folder and if it fails, print an error message
        match std::fs::create_dir(&clio_folder) {
            Ok(_) => {
                let message = format!("Clio initialized successfully in {}", directory_string);
                println!("{}", message.green());
            }
            Err(e) => {
                println!("{}", format!("Error initializing Clio: {}", e).red());
            }
        }
    }
}
