use std::{
    fs::{self, File},
    io::{copy, Cursor},
};

use reqwest::Error;

use super::github_repository::GithubRepositoryInfo;

pub fn fetch_github_repository(_info: GithubRepositoryInfo) {}

/// https://github.com/Plasticine-Yang/templates/archive/602aaeb736af0fddcf9f2afc7ba7c5eac44f7c64.tar.gz
async fn http_get() -> Result<(), Error> {
    let mut file = File::create("./downloaded_target").unwrap();
    let mut response = reqwest::get("https://github.com/Plasticine-Yang/templates/archive/602aaeb736af0fddcf9f2afc7ba7c5eac44f7c64.tar.gz")
        .await?;

    println!("size: {:?}", response.content_length().unwrap_or(0));

    // let content = response.bytes().await?;
    // while let Some(chunk) = response.chunk().await? {
    //     println!("chunk length: {}", chunk.len());
    // }

    // let mut cursor = Cursor::new(content);

    // copy(&mut cursor, &mut file);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_get() {
        http_get().await;
    }
}
