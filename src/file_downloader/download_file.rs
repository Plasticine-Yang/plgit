use std::{
    cmp::min,
    fmt::Write,
    fs::File,
    io::{copy, Cursor},
};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use crate::error::PlGitError;

pub struct FileDownloaderConfig {
    url: String,
    output_file_path: String,
}

pub async fn download_file(config: FileDownloaderConfig) -> Result<(), PlGitError> {
    let FileDownloaderConfig {
        url,
        output_file_path,
    } = config;

    // 创建文件，文件创建失败时直接抛出异常
    let mut file = File::create(&output_file_path).map_err(|error| {
        PlGitError::new(
            format!("Failed to create file: {}", output_file_path),
            Some(Box::new(error)),
        )
    })?;

    // 请求文件，获取二进制流
    let mut response = reqwest::get(&url).await.map_err(|error| {
        PlGitError::new(
            format!("Failed to request url: {}", url),
            Some(Box::new(error)),
        )
    })?;

    // 统计已下载的字节数 - 用于更新 progress bar
    let mut downloaded_bytes = 0;

    // 获取文件总大小
    let total_file_size = response.content_length().unwrap_or_else(|| {
        println!("Failed to get total file size, the progress bar may not work.");
        0
    });

    // 创建 ProgressCalculator 实例用于计算下载进度
    let option_pb = {
        if total_file_size > 0 {
            let _pb = ProgressBar::new(total_file_size);
            _pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap().with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64())
                .unwrap())
                .progress_chars("#>-"));

            Some(_pb)
        } else {
            None
        }
    };

    // 分块下载二进制流到文件中
    while let Some(chunk) = response.chunk().await.unwrap() {
        let mut cursor = Cursor::new(&chunk);

        copy(&mut cursor, &mut file).map_err(|error| {
            return PlGitError::new(
                String::from("Failed to download chunk."),
                Some(Box::new(error)),
            );
        })?;

        if let Some(ref pb) = option_pb {
            let next_downloaded_bytes =
                min(downloaded_bytes + chunk.len(), total_file_size as usize);

            downloaded_bytes = next_downloaded_bytes;
            pb.set_position(next_downloaded_bytes.try_into().unwrap());
        }
    }

    if let Some(ref pb) = option_pb {
        pb.finish_with_message("download file success!");
    }

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
        };

        let _ = download_file(config).await.is_err_and(|error| {
            println!("{}", error);

            false
        });
    }
}
