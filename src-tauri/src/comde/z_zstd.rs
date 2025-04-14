use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zstd::stream::{decode_all, encode_all};

use super::{ComdeAble, CompressionOptions, DecompressionOptions};

#[derive(Clone)]
pub struct ZstdCompressor;

impl ZstdCompressor {
    pub fn new() -> Self {
        ZstdCompressor
    }
}

impl ComdeAble for ZstdCompressor {
    async fn compress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        options: Option<CompressionOptions>,
    ) -> Result<(), String> {
        // 处理压缩选项
        let compression_level = match options {
            Some(CompressionOptions::Tar { level, .. }) => level,
            _ => 3, // 默认压缩级别为3
        };
        
        // ZSTD只能压缩单个文件，不能打包多个文件
        if input_paths.len() > 1 {
            return Err("ZSTD只能压缩单个文件，无法打包多个文件。请使用TAR+ZSTD组合".to_string());
        }
        
        let input_path = input_paths[0];
        let path = Path::new(input_path);
        
        if path.is_dir() {
            return Err("ZSTD不能直接压缩目录，请先使用TAR打包后再用ZSTD压缩".to_string());
        }
        
        // 打开输入文件
        let mut input_file = File::open(path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        input_file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        
        // 创建输出文件
        let mut output_file = File::create(output_path).map_err(|e| e.to_string())?;
        
        // ZSTD压缩级别范围是-7到22 (越大压缩率越高，越慢)
        let level = compression_level as i32;
        let compression_level = match level {
            i32::MIN..=-1 => -1, // 最低压缩级别(最快)
            0..=3 => 3,         // 默认压缩级别
            4..=21 => level,    // 常规压缩级别
            _ => 22,            // 最高压缩级别(最慢)
        };
        
        // 压缩数据
        let compressed = encode_all(&buffer[..], compression_level).map_err(|e| e.to_string())?;
        output_file.write_all(&compressed).map_err(|e| e.to_string())?;
        
        Ok(())
    }

    async fn decompress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        _options: Option<DecompressionOptions>,
    ) -> Result<(), String> {
        // ZSTD只能处理单个文件
        for input_path in input_paths {
            let mut input_file = File::open(input_path).map_err(|e| e.to_string())?;
            
            // 获取输出文件名
            let output_file_path = if Path::new(output_path).is_dir() {
                let input_filename = Path::new(input_path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or("无法获取输入文件名")?;
                
                // 移除.zst扩展名
                let original_name = if input_filename.ends_with(".zst") {
                    &input_filename[0..input_filename.len() - 4]
                } else {
                    input_filename
                };
                
                Path::new(output_path).join(original_name)
            } else {
                Path::new(output_path).to_path_buf()
            };
            
            // 读取压缩数据
            let mut buffer = Vec::new();
            input_file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
            
            // 解压数据
            let decompressed = decode_all(&buffer[..]).map_err(|e| e.to_string())?;
            
            // 写入解压后的数据
            let mut output_file = File::create(&output_file_path).map_err(|e| e.to_string())?;
            output_file.write_all(&decompressed).map_err(|e| e.to_string())?;
        }
        
        Ok(())
    }
}