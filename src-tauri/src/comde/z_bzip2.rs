use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use bzip2::Compression;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use super::{ComdeAble, CompressionOptions, DecompressionOptions};

#[derive(Clone)]
pub struct Bzip2Compressor;

impl Bzip2Compressor {
    pub fn new() -> Self {
        Bzip2Compressor
    }
}

impl ComdeAble for Bzip2Compressor {
    async fn compress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        options: Option<CompressionOptions>,
    ) -> Result<(), String> {
        // 处理压缩选项
        let compression_level = match options {
            Some(CompressionOptions::Bzip2 { level, .. }) => level,
            _ => 6, // 默认压缩级别为6
        };

        // BZIP2只能压缩单个文件，不能打包多个文件
        if input_paths.len() > 1 {
            return Err("BZIP2只能压缩单个文件，无法打包多个文件。请使用TAR+BZIP2组合".to_string());
        }

        let input_path = input_paths[0];
        let path = Path::new(input_path);

        if path.is_dir() {
            return Err("BZIP2不能直接压缩目录，请先使用TAR打包后再用BZIP2压缩".to_string());
        }

        // 打开输入文件
        let mut input_file = File::open(path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        input_file
            .read_to_end(&mut buffer)
            .map_err(|e| e.to_string())?;

        // 创建输出文件
        let output_file = File::create(output_path).map_err(|e| e.to_string())?;

        // 选择压缩级别 (1-9，9为最高压缩率)
        // 修复: Compression::Fastest和Compression::Best不存在，使用函数替代
        let compression_level = match compression_level {
            0..=1 => Compression::new(1), // 最低级别
            2..=8 => Compression::new(compression_level as u32),
            _ => Compression::new(9), // 最高级别
        };

        // 压缩数据
        let mut encoder = BzEncoder::new(output_file, compression_level);
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
        // BZIP2只能处理单个文件
        for input_path in input_paths {
            let input_file = File::open(input_path).map_err(|e| e.to_string())?;

            // 获取输出文件名
            let output_file_path = if Path::new(output_path).is_dir() {
                let input_filename = Path::new(input_path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or("无法获取输入文件名")?;

                // 移除.bz2扩展名
                let original_name = if input_filename.ends_with(".bz2") {
                    &input_filename[0..input_filename.len() - 4]
                } else {
                    input_filename
                };

                Path::new(output_path).join(original_name)
            } else {
                Path::new(output_path).to_path_buf()
            };

            // 解压数据
            let mut decoder = BzDecoder::new(input_file);
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
