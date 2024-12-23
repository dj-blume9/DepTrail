
use cargo_metadata::MetadataCommand;
use reqwest;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct CrateInfo {
    #[serde(rename = "crate")]
    crate_details: Option<CrateDetails>,
}

#[derive(Deserialize)]
struct CrateDetails {
    max_version: String,
}

#[derive(Serialize)]
struct DependencyReport {
    name: String,
    current_version: String,
    latest_version: String,
}

pub fn detect_file_type(path: &str) -> Option<&'static str> {
    if path.ends_with("Cargo.toml") {
        Some("Cargo.toml")
    } else {
        None
    }
}

pub async fn parse_cargo_toml(path: &str) {
    let metadata = MetadataCommand::new()
        .manifest_path(path)
        .exec()
        .expect("Failed to parse Cargo.toml");

    let first_package = metadata.packages.first().unwrap();
    fetch_latest_version(&first_package.name).await;
}

pub async fn fetch_latest_version(crate_name: &str) {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let client = reqwest::Client::new();
    let response:CrateInfo = client.get(&url)
        .header("User-Agent", "dep_tracker/1.0 (https://github.com/dj-blume9/DepTrail.git)")
        .send()
        .await
        .expect("Failed to fetch data")
        .json()
        .await
        .expect("Failed to parse JSON");
    
       if let Some(crate_details) = response.crate_details {
        println!(
            "Latest version of {}: {}",
            crate_name, crate_details.max_version
        );
    } else {
        println!("No data available for crate: {}", crate_name);
    }
}


pub fn generate_report(report: Vec<DependencyReport>) {
    let json = serde_json::to_string_pretty(&report).unwrap();
    println!("{}", json);
}

