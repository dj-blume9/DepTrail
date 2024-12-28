mod utils;

use clap::{Arg, Command, ArgAction};
use utils::{detect_file_type, parse_cargo_toml};

#[tokio::main]
async fn main() {
    let matches = Command::new("dep_tracker")
        .version("1.0")
        .author("D.J. Blume")
        .about("Tracks dependencies in your project")
        .arg(Arg::new("path")
            .short('p')
            .long("path")
            .value_name("FILE")
            .help("Path to the dependency file")
            .required(false))
        .arg(Arg::new("outdated")
            .short('o')
            .long("outdated")
            .help("Check for outdated dependencies")
            .action(ArgAction::SetTrue)
            .required(false))
        .get_matches();

    if let Some(file_path) = matches.get_one::<String>("path") {
        println!("Dependency File: {}", file_path);

        match detect_file_type(file_path) {
            Some("Cargo.toml") => {
                let cargo_result = parse_cargo_toml(file_path).await;
                for result in cargo_result {
                    if result.current_version != result.latest_version {
                        println!("Name: {}", result.name);
                        println!("Current Version: {}", result.current_version);
                        println!("Latest Version: {}", result.latest_version);
                    } 
                }
            },
            _ => println!("Unsupported file type."),
        }
    }

    if matches.get_flag("outdated") {
        println!("Checking for outdated dependencies...");
        // Add your logic for handling outdated dependencies here
    }
}
