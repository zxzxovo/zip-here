use sevenz_rust2 as sevenz;
use std::path::{Path, PathBuf};

use super::{ComdeAble, CompressionOptions, DecompressionOptions};

#[derive(Clone)]
pub struct SevenZipCompressor;

impl SevenZipCompressor {
    pub fn new() -> Self {
        SevenZipCompressor
    }
}

impl ComdeAble for SevenZipCompressor {
    async fn compress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        options: Option<CompressionOptions>,
    ) -> Result<(), String> {
        // 处理压缩选项
        let (_level, _password) = match options {
            Some(CompressionOptions::SevenZip { level, password }) => (level, password),
            _ => (6, None), // 默认压缩级别为6，无密码
        };

        // 将所有输入路径转换为PathBuf
        let paths: Vec<PathBuf> = input_paths
            .iter()
            .map(|p| Path::new(*p).to_path_buf())
            .collect();

        // 使用7z压缩文件或目录
        // 注：sevenz-rust2库目前不支持密码和压缩级别设置，需要等待库的更新
        // 对单个文件或目录特殊处理
        if paths.len() == 1 {
            // 如果只有一个路径，直接使用它
            let path = &paths[0];
            return match sevenz::compress_to_path(path, output_path) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("压缩失败: {}", e)),
            };
        } else {
            // 如果有多个文件，需要单独处理每个文件
            for path in &paths {
                // 获取文件名作为输出文件的一部分
                let file_name = path.file_name().ok_or("无效的文件名")?.to_string_lossy();

                // 创建临时输出路径
                let temp_output = format!("{}.tmp", output_path);

                // 压缩单个文件
                match sevenz::compress_to_path(path, &temp_output) {
                    Ok(_) => {
                        // 成功压缩，将临时文件移动到最终位置
                        std::fs::rename(temp_output, output_path)
                            .map_err(|e| format!("重命名文件失败: {}", e))?;
                    }
                    Err(e) => return Err(format!("压缩文件 {} 失败: {}", file_name, e)),
                }
            }

            // 如果代码能运行到这里，表示所有文件都已成功压缩
            Ok(())
        }
    }

    async fn decompress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        options: Option<DecompressionOptions>,
    ) -> Result<(), String> {
        // 处理解压选项
        let _password = match options {
            Some(DecompressionOptions::SevenZip { password }) => password,
            _ => None,
        };

        // 确保输出目录存在
        let output_dir = Path::new(output_path);
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
        }

        // 解压所有输入文件
        for input_path in input_paths {
            match sevenz::decompress_file(input_path, output_path) {
                Ok(_) => (),
                Err(e) => return Err(format!("解压失败: {}", e)),
            }
        }

        Ok(())
    }
}
