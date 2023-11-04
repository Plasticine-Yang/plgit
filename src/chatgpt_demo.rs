use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use tokio::fs as tokio_fs;
use tokio::runtime::Runtime;

#[derive(Debug, Deserialize)]
struct GithubRepo {
    default_branch: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CachedRepo {
    default_branch: String,
    etag: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: plgit <user/repo> <destination> [-b branch]");
        process::exit(1);
    }

    let repo = &args[1];
    let destination = &args[2];
    let branch = if args.len() > 3 && args[3] == "-b" {
        Some(args[4].clone())
    } else {
        None
    };

    let cache_path = format!(".plgit_cache/{}.json", repo.replace("/", "_"));
    let cache_exists = Path::new(&cache_path).exists();

    let client = Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    if cache_exists {
        let cache_file = fs::read_to_string(&cache_path).unwrap();
        let cached_repo: CachedRepo = serde_json::from_str(&cache_file).unwrap();
        headers.insert(
            reqwest::header::IF_NONE_MATCH,
            cached_repo.etag.parse().unwrap(),
        );
    }

    let url = format!("https://api.github.com/repos/{}", repo);
    let request = client.get(&url).headers(headers);
    let response = Runtime::new().unwrap().block_on(request.send());

    match response {
        Ok(response) => {
            if response.status().is_success() {
                let etag = response
                    .headers()
                    .get(reqwest::header::ETAG)
                    .and_then(|header| header.to_str().ok())
                    .unwrap_or("");

                let repo: GithubRepo = response.json().unwrap();
                let default_branch = branch
                    .as_ref()
                    .map_or_else(|| repo.default_branch.clone(), |b| b.clone());
                let clone_url = format!("https://github.com/{}/{}", repo, default_branch);

                if cache_exists {
                    let cache_file = fs::read_to_string(&cache_path).unwrap();
                    let cached_repo: CachedRepo = serde_json::from_str(&cache_file).unwrap();
                    if cached_repo.default_branch == default_branch && cached_repo.etag == etag {
                        println!("Using cached result");
                        process::exit(0);
                    }
                }

                let temp_dir = format!(".plgit_temp/{}", repo.replace("/", "_"));
                if Path::new(&temp_dir).exists() {
                    fs::remove_dir_all(&temp_dir).unwrap();
                }

                let clone_dest = format!("{}/{}", destination, repo.split('/').last().unwrap());
                let clone_command = process::Command::new("git")
                    .args(&[
                        "clone",
                        "--depth",
                        "1",
                        "--single-branch",
                        "--branch",
                        &default_branch,
                        &clone_url,
                        &clone_dest,
                    ])
                    .output();

                match clone_command {
                    Ok(output) => {
                        if output.status.success() {
                            fs::create_dir_all(Path::new(&clone_dest).join(".git")).unwrap();
                            fs::remove_dir_all(&clone_dest).unwrap();
                            fs::create_dir_all(destination).unwrap();
                            fs::rename(&temp_dir, &clone_dest).unwrap();

                            let cached_repo = CachedRepo {
                                default_branch,
                                etag: etag.to_string(),
                            };
                            let cache_file = fs::File::create(&cache_path).unwrap();
                            serde_json::to_writer(cache_file, &cached_repo).unwrap();

                            println!("Cloned repository to {}", clone_dest);
                        } else {
                            eprintln!(
                                "Failed to clone repository: {}",
                                String::from_utf8_lossy(&output.stderr)
                            );
                            process::exit(1);
                        }
                    }
                }
            }
        }
    }
}
