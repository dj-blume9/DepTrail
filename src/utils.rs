
use cargo_metadata::MetadataCommand;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct CrateInfo {
    #[serde(rename = "crate")]
    crate_details: Option<CrateDetails>,
}

#[derive(Deserialize)]
struct CrateDetails {
    max_version: String,
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

    for package in metadata.packages {
        println!("Parsing package: {}", &package.name);
        fetch_latest_version(&package.name).await;
    }
}

pub async fn fetch_latest_version(crate_name: &str) {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let response = reqwest::get(&url)
        .await
        .expect("Failed to fetch data")
        .text()
        .await
        .expect("Failed to parse JSON");
    
    println!("Raw JSON data from {}: {}", &crate_name, response);

   // if let Some(crate_details) = response.crate_details {
     //   println!(
       //     "Latest version of {}: {}",
         //   crate_name, crate_details.max_version
       // );
  //  } else {
  //      println!("No data available for crate: {}", crate_name);
  //  }
}

