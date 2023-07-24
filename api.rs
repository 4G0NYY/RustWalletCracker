use reqwest::StatusCode;
use std::fs;

fn main() {
    let api_file = "api.txt";
    let api_list = read_api_file(api_file);

    let rpc: Vec<String> = api_list
        .iter()
        .filter_map(|api| {
            if is_responsive(api) {
                Some(api.to_string())
            } else {
                None
            }
        })
        .collect();

    println!("RPC: {:?}", rpc);
}

fn read_api_file(file_path: &str) -> Vec<String> {
    match fs::read_to_string(file_path) {
        Ok(contents) => contents.lines().map(|line| line.trim().to_string()).collect(),
        Err(_) => {
            eprintln!("Error reading the file {}", file_path);
            Vec::new()
        }
    }
}

fn is_responsive(api_url: &str) -> bool {
    match reqwest::blocking::get(api_url) {
        Ok(response) => response.status() == StatusCode::OK,
        Err(_) => false,
    }
}
