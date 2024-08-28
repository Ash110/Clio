use crate::helpers::Logger::Logger;

pub fn init(directory: Option<String>, verbose: bool) {
    let logger = Logger::new(true);
    let verbose_logger = Logger::new(verbose);

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

    verbose_logger.log(format!("Attempting to initialise in directory: {}", directory_string));

    // Check if the folder already has a .clio folder
    let clio_folder = format!("{}/.clio", directory_string);
    if std::path::Path::new(&clio_folder).exists() {
        logger.warn("Clio is already initialized in this directory".to_string());
    } else {
        verbose_logger.log("Creating .clio folder".to_string());
        // Create the .clio folder and if it fails, print an error message
        match std::fs::create_dir(&clio_folder) {
            Ok(_) => {
                let message = format!("Clio initialized successfully in {}", directory_string);
                logger.success(message);
            }
            Err(e) => {
                let message = format!("Failed to initialize Clio in {}: {}", directory_string, e);
                logger.error(message);
            }
        }
    }
}
