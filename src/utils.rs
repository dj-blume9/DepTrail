
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
pub struct DependencyReport {
    pub name: String,
    pub current_version: String,
    pub latest_version: String,
}

pub fn detect_file_type(path: &str) -> Option<&'static str> {
    if path.ends_with("Cargo.toml") {
        Some("Cargo.toml")
    } else {
        None
    }
}

pub async fn parse_cargo_toml(path: &str) -> Vec<DependencyReport> {
    let metadata = MetadataCommand::new()
        .manifest_path(path)
        .exec()
        .expect("Failed to parse Cargo.toml");

    // Collect names of direct dependencies
    let direct_dependencies: Vec<_> = metadata
        .resolve
        .expect("No resolve information found")
        .nodes
        .iter()
        .find(|node| node.id == metadata.workspace_members[0]) // Find the root package
        .expect("Root package not found")
        .dependencies
        .clone();

    let mut report = Vec::new();

    for package in metadata.packages.iter().filter(|pkg| direct_dependencies.contains(&pkg.id)) {
        let latest_version = fetch_latest_version(&package.name).await;
        report.push(DependencyReport {
            name: package.name.clone(),
            current_version: package.version.to_string(),
            latest_version: latest_version.unwrap_or_else(|| "unknown".to_string()),
        });
    }

    report
}


pub async fn fetch_latest_version(crate_name: &str) -> Option<String> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let client = reqwest::Client::new();
    let response = client.get(&url)
        .header("User-Agent", "dep_tracker/1.0 (https://github.com/dj-blume9/DepTrail.git)")
        .send()
        .await
        .expect("Failed to fetch data");
    let status = response.status();
    if status.is_success() {
        let crate_info: CrateInfo = response
            .json()
            .await
            .expect("Failed to parse json");

        if let Some(crate_details) = crate_info.crate_details {
            return Some(crate_details.max_version);
        }
    }
    
    println!("Status: {}", &status.as_str());
    None
}


pub fn generate_report(report: Vec<DependencyReport>) {
    let json = serde_json::to_string_pretty(&report).unwrap();
    println!("{}", json);
}

