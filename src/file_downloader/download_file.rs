use std::{
    fs::File,
    io::{copy, Cursor},
};

use reqwest::{redirect::Policy, Client};

use crate::error::PlGitError;

pub struct FileDownloaderConfig {
    url: String,
    output_file_path: String,
    proxy: Option<String>,
}

pub async fn download_file(config: FileDownloaderConfig) -> Result<(), PlGitError> {
    let FileDownloaderConfig {
        url,
        output_file_path,
        proxy,
    } = config;

    // 创建文件，文件创建失败时直接抛出异常
    let mut file = File::create(&output_file_path).map_err(|error| {
        PlGitError::new(
            format!("Failed to create file: {}", output_file_path),
            Some(Box::new(error)),
        )
    })?;

    // 请求文件，获取二进制流
    let client = Client::builder()
        .redirect(Policy::default())
        .build()
        .unwrap();

    let mut response = client.get(&url).send().await.map_err(|error| {
        PlGitError::new(
            format!("Failed to request url: {}", url),
            Some(Box::new(error)),
        )
    })?;

    // 获取文件总大小
    // let total_file_size = response.content_length().unwrap_or_else(|| {
    //     println!("Failed to get total file size, the progress bar may not work.");
    //     0
    // });

    // TODO: 创建 ProgressCalculator 实例用于计算下载进度

    // 分块下载二进制流到文件中
    while let Some(chunk) = response.chunk().await.unwrap() {
        let mut cursor = Cursor::new(&chunk);

        copy(&mut cursor, &mut file).map_err(|error| {
            return PlGitError::new(
                String::from("Failed to download chunk."),
                Some(Box::new(error)),
            );
        })?;

        println!("download chunk of file success!");
    }

    println!("download file success!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[tokio::test]
    async fn test_download_file() {
        let output_file_path = String::from(
            env::current_dir()
                .unwrap()
                .join("fixtures")
                .join("downloaded_test_file.tar.gz")
                .to_str()
                .unwrap(),
        );

        let config = FileDownloaderConfig {
            url: String::from("https://github.com/Plasticine-Yang/templates/archive/602aaeb736af0fddcf9f2afc7ba7c5eac44f7c64.tar.gz"),
            output_file_path,
            proxy: None,
        };

        let _ = download_file(config).await.is_err_and(|error| {
            println!("{}", error);

            false
        });
    }
}
