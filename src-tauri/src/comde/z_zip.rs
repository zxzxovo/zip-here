use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::{ZipArchive, ZipWriter};
use zip::write::{FileOptions, SimpleFileOptions};
use zip::CompressionMethod;

use super::{ComdeAble, CompressionOptions, DecompressionOptions};

#[derive(Clone)]
pub struct ZipCompressor;

impl ZipCompressor {
    pub fn new() -> Self {
        ZipCompressor
    }

    // 将目录添加到zip中
    fn add_directory_to_zip<W: Write + std::io::Seek>(
        &self,
        zip: &mut ZipWriter<W>,
        dir_path: &Path,
        options: SimpleFileOptions,
    ) -> Result<(), String> {
        for entry in std::fs::read_dir(dir_path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            
            if path.is_file() {
                let mut file = File::open(&path).map_err(|e| e.to_string())?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
                
                let name = path.strip_prefix(dir_path)
                    .map_err(|e| e.to_string())?
                    .to_string_lossy();
                
                // 不使用clone方法，因为FileOptions可能不实现Clone
                zip.start_file(name, options).map_err(|e| e.to_string())?;
                zip.write_all(&buffer).map_err(|e| e.to_string())?;
            } else if path.is_dir() {
                // 递归处理子目录
                self.add_directory_to_zip(zip, &path, options)?;
            }
        }
        
        Ok(())
    }
}

impl ComdeAble for ZipCompressor {
    async fn compress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        options: Option<CompressionOptions>,
    ) -> Result<(), String> {
        // 获取压缩选项
        let (compression_level, _password) = match options {
            Some(CompressionOptions::Zip { level, password }) => (level, password),
            _ => (6, None), // 默认压缩级别为6，无密码
        };

        // 创建输出文件
        let file = File::create(output_path).map_err(|e| e.to_string())?;
        let mut zip = ZipWriter::new(file);

        // 根据压缩级别配置选项，添加明确的类型标注
        let options: FileOptions<'_, _> = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .compression_level(Some(compression_level as i64));  // 使用i32类型而不是i64

        // 遍历所有输入路径并添加到压缩文件中
        for input_path in input_paths {
            let path = Path::new(input_path);
            if path.is_dir() {
                // 压缩文件夹
                self.add_directory_to_zip(&mut zip, path, options)?;
            } else {
                // 压缩单个文件
                let file_name = path.file_name().ok_or("无效的文件名")?;
                let mut file = File::open(path).map_err(|e| e.to_string())?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
                
                zip.start_file(file_name.to_string_lossy(), options).map_err(|e| e.to_string())?;
                zip.write_all(&buffer).map_err(|e| e.to_string())?;
            }
        }

        // 完成压缩
        zip.finish().map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn decompress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        options: Option<DecompressionOptions>,
    ) -> Result<(), String> {
        // 处理密码选项
        let _password = match options {
            Some(DecompressionOptions::Zip { password }) => password,
            _ => None,
        };

        // 确保输出目录存在
        let output_dir = Path::new(output_path);
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
        }

        // 处理所有输入文件
        for input_path in input_paths {
            let file = File::open(input_path).map_err(|e| e.to_string())?;
            let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

            // 解压每个文件
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
                let outpath = output_dir.join(file.name());

                if file.name().ends_with('/') {
                    // 创建目录
                    std::fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
                } else {
                    // 确保父目录存在
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            std::fs::create_dir_all(p).map_err(|e| e.to_string())?;
                        }
                    }

                    // 解压文件
                    let mut outfile = File::create(&outpath).map_err(|e| e.to_string())?;
                    std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
                }
            }
        }

        Ok(())
    }
}