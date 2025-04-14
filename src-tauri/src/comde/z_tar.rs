use std::fs::{self, File};
use std::path::Path;
use tar::{Builder, Archive};

use super::{ComdeAble, CompressionOptions, DecompressionOptions};

#[derive(Clone)]
pub struct TarCompressor;

impl TarCompressor {
    pub fn new() -> Self {
        TarCompressor
    }
}

impl ComdeAble for TarCompressor {
    async fn compress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        _options: Option<CompressionOptions>,
    ) -> Result<(), String> {
        // 创建输出文件
        let file = File::create(output_path).map_err(|e| e.to_string())?;
        let mut builder = Builder::new(file);
        
        // 将每个输入路径添加到tar归档中
        for input_path in input_paths {
            let path = Path::new(input_path);
            
            if path.is_dir() {
                // 添加目录及其所有内容
                builder.append_dir_all(
                    path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                    path
                ).map_err(|e| e.to_string())?;
            } else if path.is_file() {
                // 添加单个文件
                let mut file = File::open(path).map_err(|e| e.to_string())?;
                let file_name = path.file_name().unwrap_or_default();
                builder.append_file(file_name, &mut file).map_err(|e| e.to_string())?;
            }
        }
        
        // 完成归档
        builder.finish().map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn decompress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        _options: Option<DecompressionOptions>,
    ) -> Result<(), String> {
        // 确保输出目录存在
        let output_dir = Path::new(output_path);
        if !output_dir.exists() {
            fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
        }
        
        // 解压每个输入文件
        for input_path in input_paths {
            let file = File::open(input_path).map_err(|e| e.to_string())?;
            let mut archive = Archive::new(file);
            
            // 解压所有文件到输出目录
            archive.unpack(output_dir).map_err(|e| e.to_string())?;
        }
        
        Ok(())
    }
}