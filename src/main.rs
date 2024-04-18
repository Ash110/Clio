use clap::{Parser, Subcommand};
mod functions;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, default_value = "false")]
    verbose: bool, // New verbose flag
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get {
        key: Option<String>,
    },

    #[clap(name = "set")]
    Set {
        #[clap(short, long)]
        key: String,
        #[clap(short, long)]
        value: String,
        #[clap(short, long)]
        istrue: bool,
    },

    Init {
        directory: Option<String>,
    },
}

fn main() {
    let verbose;
    let args = Args::parse();
    match args.verbose {
        true => verbose = true,
        false => verbose = false,
    }
    if verbose {
        println!("Verbose mode enabled");
    }
    match args.cmd {
        Commands::Get { key } => {
            println!("Getting value: {}", key.unwrap_or("default".to_string()))
        }
        Commands::Set { key, value, istrue } => {
            println!(
                "Setting key: {} with value: {} and istrue: {}",
                key, value, istrue
            );
        }
        Commands::Init { directory } => functions::init::init(directory, verbose),
    }
}
