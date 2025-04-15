use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::{FileOptions, SimpleFileOptions};
use zip::CompressionMethod;
use zip::{ZipArchive, ZipWriter};

use super::{ComdeAble, CompressionOptions, DecompressionOptions};

#[derive(Clone)]
pub struct ZipCompressor;

impl ZipCompressor {
    pub fn new() -> Self {
        ZipCompressor
    }

    // Add directory contents to the zip file
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

                let name = path
                    .strip_prefix(dir_path)
                    .map_err(|e| e.to_string())?
                    .to_string_lossy();

                // Start a new file entry in the zip
                zip.start_file(name, options).map_err(|e| e.to_string())?;
                zip.write_all(&buffer).map_err(|e| e.to_string())?;
            } else if path.is_dir() {
                // Recursively process subdirectories
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
        // Extract compression options
        let (compression_level, password) = match options {
            Some(CompressionOptions::Zip { level, password }) => (level, password),
            _ => (6, None), // Default compression level is 6, no password
        };

        // Create output file
        let file = File::create(output_path).map_err(|e| e.to_string())?;
        let mut zip = ZipWriter::new(file);

        // Configure compression options
        let mut options = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .compression_level(Some(compression_level as i64));

        // Set password if provided (implementation depends on the zip library's capabilities)
        if let Some(pwd) = password {
            // Note: Standard zip crate doesn't support encryption
            // This would need a different crate or implementation
            return Err("Password protection for ZIP files not yet implemented".to_string());
        }

        // Process all input paths
        for input_path in input_paths {
            let path = Path::new(input_path);
            if path.is_dir() {
                // Compress directory
                self.add_directory_to_zip(&mut zip, path, options)?;
            } else {
                // Compress single file
                let file_name = path.file_name().ok_or("Invalid filename")?;
                let mut file = File::open(path).map_err(|e| e.to_string())?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

                zip.start_file(file_name.to_string_lossy(), options)
                    .map_err(|e| e.to_string())?;
                zip.write_all(&buffer).map_err(|e| e.to_string())?;
            }
        }

        // Finalize the zip file
        zip.finish().map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn decompress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        options: Option<DecompressionOptions>,
    ) -> Result<(), String> {
        // Process password option
        let password = match options {
            Some(DecompressionOptions::Zip { password }) => password,
            _ => None,
        };

        // Ensure output directory exists
        let output_dir = Path::new(output_path);
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
        }

        // Process all input archives
        for input_path in input_paths {
            let file = File::open(input_path).map_err(|e| e.to_string())?;
            let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

            // If password is provided, try to set it (implementation depends on the zip library)
            if let Some(pwd) = &password {
                // Note: Standard zip crate doesn't support decryption
                // This would need a different crate or implementation
                return Err("Password handling for ZIP files not yet implemented".to_string());
            }

            // Extract each file in the archive
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
                let outpath = output_dir.join(file.name());

                if file.name().ends_with('/') {
                    // Create directory
                    std::fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
                } else {
                    // Ensure parent directory exists
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            std::fs::create_dir_all(p).map_err(|e| e.to_string())?;
                        }
                    }

                    // Extract file
                    let mut outfile = File::create(&outpath).map_err(|e| e.to_string())?;
                    std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
                }
            }
        }

        Ok(())
    }
}
