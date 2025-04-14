// Imports for file and path handling
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

// Imports from local modules
use crate::comde::{
    Compressor,
    CompressionFormat,
    CompressionOptions,
    DecompressionOptions,
};

// Format option struct for frontend-backend communication
#[derive(Debug, Serialize)]
pub struct FormatOption {
    pub id: String,           // Unique identifier for format
    pub name: String,         // Display name
    pub extension: String,    // File extension
    pub can_compress: bool,   // Whether format supports compression
    pub can_decompress: bool, // Whether format supports decompression
    pub supports_password: bool, // Whether format supports password protection
    pub supports_level: bool, // Whether format supports compression levels
    pub min_level: Option<u32>, // Minimum compression level if applicable
    pub max_level: Option<u32>, // Maximum compression level if applicable
    pub default_level: Option<u32>, // Default compression level if applicable
}

// Compression options passed from frontend
#[derive(Debug, Deserialize)]
pub struct CompressOptions {
    pub format: String,
    pub level: Option<u32>,
    pub password: Option<String>,
}

// Decompression options passed from frontend
#[derive(Debug, Deserialize)]
pub struct DecompressOptions {
    pub password: Option<String>,
}

// Application version information
#[derive(Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub build_time: String,
    pub author: String,
    pub description: String,
}

/// Compress files or directories
/// 
/// # Arguments
/// * `input_paths` - A list of file or directory paths to compress
/// * `output_path` - The output file path for the compressed archive
/// * `options` - Compression options including format, level, and password
/// 
/// # Returns
/// * `Result<(), String>` - Ok if successful, Err with error message otherwise
#[tauri::command]
pub async fn compress_files(
    input_paths: Vec<String>,
    output_path: String,
    options: CompressOptions,
) -> Result<(), String> {
    // Parse compression format
    let format = match CompressionFormat::from_extension(&options.format) {
        Some(format) => format,
        None => return Err(format!("Unsupported format: {}", options.format)),
    };
    
    // Create compressor instance
    let compressor = Compressor::new(format);
    
    // Create compression options based on format
    let compress_options = match format {
        CompressionFormat::Zip => Some(CompressionOptions::Zip {
            level: options.level.unwrap_or(6),
            password: options.password,
        }),
        CompressionFormat::Tar => Some(CompressionOptions::Tar {
            level: options.level.unwrap_or(6),
        }),
        CompressionFormat::Gzip => Some(CompressionOptions::Gzip {
            level: options.level.unwrap_or(6),
        }),
        CompressionFormat::Bzip2 => Some(CompressionOptions::Bzip2 {
            level: options.level.unwrap_or(6),
        }),
        CompressionFormat::Xz => Some(CompressionOptions::Xz {
            level: options.level.unwrap_or(6),
        }),
        CompressionFormat::Zstd => Some(CompressionOptions::Zstd {
            level: options.level.unwrap_or(3),
        }),
        CompressionFormat::SevenZip => Some(CompressionOptions::SevenZip {
            level: options.level.unwrap_or(6),
            password: options.password,
        }),
    };
    
    // Convert string paths to &str references
    let input_paths_refs: Vec<&str> = input_paths.iter().map(|s| s.as_str()).collect();
    
    // Execute compression
    compressor.compress(input_paths_refs, &output_path, compress_options).await
}

/// Decompress archive files
/// 
/// # Arguments
/// * `input_paths` - A list of archive file paths to decompress
/// * `output_path` - The output directory path for decompressed files
/// * `format` - The format identifier (can be auto-detected from file extension)
/// * `options` - Decompression options including password
/// 
/// # Returns
/// * `Result<(), String>` - Ok if successful, Err with error message otherwise
#[tauri::command]
pub async fn decompress_files(
    input_paths: Vec<String>,
    output_path: String,
    format: String,
    options: Option<DecompressOptions>,
) -> Result<(), String> {
    // Parse or auto-detect format
    let format = match CompressionFormat::from_extension(&format) {
        Some(format) => format,
        None => {
            // Try to auto-detect format from file extension
            if let Some(path) = input_paths.first() {
                let path = PathBuf::from(path);
                if let Some(ext) = path.extension() {
                    if let Some(ext_str) = ext.to_str() {
                        if let Some(detected_format) = CompressionFormat::from_extension(ext_str) {
                            detected_format
                        } else {
                            return Err(format!("Cannot detect format from extension: {}", ext_str));
                        }
                    } else {
                        return Err("Invalid file extension".to_string());
                    }
                } else {
                    return Err("File has no extension".to_string());
                }
            } else {
                return Err("No input file provided".to_string());
            }
        }
    };
    
    // Create decompressor instance
    let decompressor = Compressor::new(format);
    
    // Create decompression options based on format
    let decompress_options = match format {
        CompressionFormat::Zip => {
            if let Some(opts) = options {
                Some(DecompressionOptions::Zip {
                    password: opts.password,
                })
            } else {
                Some(DecompressionOptions::Zip {
                    password: None,
                })
            }
        },
        CompressionFormat::Tar => Some(DecompressionOptions::Tar {}),
        CompressionFormat::Gzip => Some(DecompressionOptions::Gzip {}),
        CompressionFormat::Bzip2 => Some(DecompressionOptions::Bzip2 {}),
        CompressionFormat::Xz => Some(DecompressionOptions::Xz {}),
        CompressionFormat::Zstd => Some(DecompressionOptions::Zstd {}),
        CompressionFormat::SevenZip => {
            if let Some(opts) = options {
                Some(DecompressionOptions::SevenZip {
                    password: opts.password,
                })
            } else {
                Some(DecompressionOptions::SevenZip {
                    password: None,
                })
            }
        },
    };
    
    // Convert string paths to &str references
    let input_paths_refs: Vec<&str> = input_paths.iter().map(|s| s.as_str()).collect();
    
    // Execute decompression
    decompressor.decompress(input_paths_refs, &output_path, decompress_options).await
}

/// Get list of supported compression formats
/// 
/// # Returns
/// * `Vec<String>` - List of supported format identifiers
#[tauri::command]
pub fn get_supported_formats() -> Vec<String> {
    CompressionFormat::all_formats()
        .iter()
        .map(|format| format.extension().to_string())
        .collect()
}

/// Get detailed options for a specific format
/// 
/// # Arguments
/// * `format` - Format identifier string
/// 
/// # Returns
/// * `Result<FormatOption, String>` - Format options if supported, error otherwise
#[tauri::command]
pub fn get_format_options(format: String) -> Result<FormatOption, String> {
    let format = match CompressionFormat::from_extension(&format) {
        Some(format) => format,
        None => return Err(format!("Unsupported format: {}", format)),
    };
    
    let (can_compress, can_decompress, supports_password, supports_level, min_level, max_level, default_level) = 
        match format {
            CompressionFormat::Zip => (true, true, true, true, Some(1), Some(9), Some(6)),
            CompressionFormat::Tar => (true, true, false, true, Some(1), Some(9), Some(6)),
            CompressionFormat::Gzip => (true, true, false, true, Some(1), Some(9), Some(6)),
            CompressionFormat::Bzip2 => (true, true, false, true, Some(1), Some(9), Some(6)),
            CompressionFormat::Xz => (true, true, false, true, Some(1), Some(9), Some(6)),
            CompressionFormat::Zstd => (true, true, false, true, Some(1), Some(22), Some(3)),
            CompressionFormat::SevenZip => (true, true, true, true, Some(1), Some(9), Some(6)),
        };
    
    Ok(FormatOption {
        id: format.extension().to_string(),
        name: format.name().to_string(),
        extension: format.extension().to_string(),
        can_compress,
        can_decompress,
        supports_password,
        supports_level,
        min_level,
        max_level,
        default_level,
    })
}

/// Get application version information
/// 
/// # Returns
/// * `VersionInfo` - Application version details
#[tauri::command]
pub fn get_version_info() -> VersionInfo {
    VersionInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        build_time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        author: env!("CARGO_PKG_AUTHORS").to_string(),
        description: env!("CARGO_PKG_DESCRIPTION").to_string(),
    }
}