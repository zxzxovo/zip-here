use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use xz2::read::XzDecoder;
use xz2::write::XzEncoder;

use super::{ComdeAble, CompressionOptions, DecompressionOptions};

#[derive(Clone)]
pub struct XzCompressor;

impl XzCompressor {
    pub fn new() -> Self {
        XzCompressor
    }
}

impl ComdeAble for XzCompressor {
    async fn compress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        options: Option<CompressionOptions>,
    ) -> Result<(), String> {
        // 处理压缩选项
        let compression_level = match options {
            Some(CompressionOptions::Tar { level, .. }) => level,
            _ => 6, // 默认压缩级别为6
        };

        // XZ只能压缩单个文件，不能打包多个文件
        if input_paths.len() > 1 {
            return Err("XZ只能压缩单个文件，无法打包多个文件。请使用TAR+XZ组合".to_string());
        }

        let input_path = input_paths[0];
        let path = Path::new(input_path);

        if path.is_dir() {
            return Err("XZ不能直接压缩目录，请先使用TAR打包后再用XZ压缩".to_string());
        }

        // 打开输入文件
        let mut input_file = File::open(path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        input_file
            .read_to_end(&mut buffer)
            .map_err(|e| e.to_string())?;

        // 创建输出文件
        let output_file = File::create(output_path).map_err(|e| e.to_string())?;

        // 设置压缩级别 (0-9，9为最高压缩率)
        let compression_level = if compression_level > 9 {
            9
        } else {
            compression_level
        };

        // 压缩数据
        let mut encoder = XzEncoder::new(output_file, compression_level);
        encoder.write_all(&buffer).map_err(|e| e.to_string())?;
        encoder.finish().map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn decompress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        _options: Option<DecompressionOptions>,
    ) -> Result<(), String> {
        // XZ只能处理单个文件
        for input_path in input_paths {
            let input_file = File::open(input_path).map_err(|e| e.to_string())?;

            // 获取输出文件名
            let output_file_path = if Path::new(output_path).is_dir() {
                let input_filename = Path::new(input_path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or("无法获取输入文件名")?;

                // 移除.xz扩展名
                let original_name = if input_filename.ends_with(".xz") {
                    &input_filename[0..input_filename.len() - 3]
                } else {
                    input_filename
                };

                Path::new(output_path).join(original_name)
            } else {
                Path::new(output_path).to_path_buf()
            };

            // 解压数据
            let mut decoder = XzDecoder::new(input_file);
            let mut output_file = File::create(&output_file_path).map_err(|e| e.to_string())?;
            let mut buffer = Vec::new();

            decoder
                .read_to_end(&mut buffer)
                .map_err(|e| e.to_string())?;
            output_file.write_all(&buffer).map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}
