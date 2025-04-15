// Imports for file and path handling
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Imports from local modules
use crate::comde::{CompressionFormat, CompressionOptions, Compressor, DecompressionOptions};

// 新增导入用于Windows注册表操作
#[cfg(windows)]
use std::process::Command;

// Format option struct for frontend-backend communication
#[derive(Debug, Serialize)]
pub struct FormatOption {
    pub id: String,                 // Unique identifier for format
    pub name: String,               // Display name
    pub extension: String,          // File extension
    pub can_compress: bool,         // Whether format supports compression
    pub can_decompress: bool,       // Whether format supports decompression
    pub supports_password: bool,    // Whether format supports password protection
    pub supports_level: bool,       // Whether format supports compression levels
    pub min_level: Option<u32>,     // Minimum compression level if applicable
    pub max_level: Option<u32>,     // Maximum compression level if applicable
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
    // Validate input paths
    if input_paths.is_empty() {
        return Err("No input files provided".to_string());
    }

    // Validate output path
    if output_path.is_empty() {
        return Err("No output path provided".to_string());
    }

    // Check if all input paths exist
    for path in &input_paths {
        if !std::path::Path::new(path).exists() {
            return Err(format!("Input path does not exist: {}", path));
        }
    }

    // Create output directory if it doesn't exist
    let output_dir = std::path::Path::new(&output_path)
        .parent()
        .ok_or_else(|| "Invalid output path".to_string())?;

    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

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
    compressor
        .compress(input_paths_refs, &output_path, compress_options)
        .await
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
    // Validate input paths
    if input_paths.is_empty() {
        return Err("No input archives provided".to_string());
    }

    // Validate output path
    if output_path.is_empty() {
        return Err("No output path provided".to_string());
    }

    // Check if all input paths exist
    for path in &input_paths {
        if !std::path::Path::new(path).exists() {
            return Err(format!("Archive does not exist: {}", path));
        }
    }

    // Create output directory if it doesn't exist
    if !std::path::Path::new(&output_path).exists() {
        std::fs::create_dir_all(&output_path)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    // Parse or auto-detect format
    let format = if !format.is_empty() {
        match CompressionFormat::from_extension(&format) {
            Some(format) => format,
            None => return Err(format!("Unsupported format: {}", format)),
        }
    } else {
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
                Some(DecompressionOptions::Zip { password: None })
            }
        }
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
                Some(DecompressionOptions::SevenZip { password: None })
            }
        }
    };

    // Convert string paths to &str references
    let input_paths_refs: Vec<&str> = input_paths.iter().map(|s| s.as_str()).collect();

    // Execute decompression
    decompressor
        .decompress(input_paths_refs, &output_path, decompress_options)
        .await
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

    let (
        can_compress,
        can_decompress,
        supports_password,
        supports_level,
        min_level,
        max_level,
        default_level,
    ) = match format {
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

/// Add ZipHere to Windows right-click context menu
///
/// # Arguments
/// * `open_mode` - The mode to open the app in when using context menu ("cli" or "gui")
///
/// # Returns
/// * `Result<(), String>` - Ok if successful, Err with error message otherwise
#[cfg(windows)]
#[tauri::command]
pub fn add_context_menu(open_mode: String) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    // 获取程序路径
    let exe_path =
        std::env::current_exe().map_err(|e| format!("Failed to get executable path: {}", e))?;
    let exe_path_str = exe_path.to_string_lossy().to_string();

    // 为目录添加右键菜单
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // 目录右键菜单 - 压缩到...
    {
        let dir_key = hkcu
            .create_subkey("Software\\Classes\\Directory\\shell\\ZipHere")
            .map_err(|e| format!("Failed to create directory menu registry key: {}", e))?;
        dir_key
            .0
            .set_value("", &"使用 ZipHere 压缩")
            .map_err(|e| format!("Failed to set menu text: {}", e))?;
        dir_key
            .0
            .set_value("Icon", &format!("{},0", exe_path_str))
            .map_err(|e| format!("Failed to set menu icon: {}", e))?;

        let command_key = dir_key
            .0
            .create_subkey("command")
            .map_err(|e| format!("Failed to create command key: {}", e))?;

        // 根据打开模式选择命令行参数
        let command_str = if open_mode == "cli" {
            format!("\"{}\" c \"%1\"", exe_path_str)
        } else {
            // GUI模式，需要调用前端
            format!("\"{}\"", exe_path_str)
        };

        command_key
            .0
            .set_value("", &command_str)
            .map_err(|e| format!("Failed to set command value: {}", e))?;
    }

    // 多选文件右键菜单 - 压缩选中项...
    {
        let multi_key = hkcu
            .create_subkey("Software\\Classes\\*\\shell\\ZipHere")
            .map_err(|e| format!("Failed to create file menu registry key: {}", e))?;
        multi_key
            .0
            .set_value("", &"使用 ZipHere 压缩")
            .map_err(|e| format!("Failed to set menu text: {}", e))?;
        multi_key
            .0
            .set_value("Icon", &format!("{},0", exe_path_str))
            .map_err(|e| format!("Failed to set menu icon: {}", e))?;
        multi_key
            .0
            .set_value("MultiSelectModel", &"Player")
            .map_err(|e| format!("Failed to set MultiSelectModel: {}", e))?;

        let command_key = multi_key
            .0
            .create_subkey("command")
            .map_err(|e| format!("Failed to create command key: {}", e))?;

        // 根据打开模式选择命令行参数
        let command_str = if open_mode == "cli" {
            format!("\"{}\" c \"%1\"", exe_path_str)
        } else {
            // GUI模式，需要调用前端
            format!("\"{}\"", exe_path_str)
        };

        command_key
            .0
            .set_value("", &command_str)
            .map_err(|e| format!("Failed to set command value: {}", e))?;
    }

    // 刷新Windows资源管理器
    Command::new("cmd")
        .args([
            "/C",
            "taskkill",
            "/f",
            "/im",
            "explorer.exe",
            "&&",
            "start",
            "explorer.exe",
        ])
        .output()
        .map_err(|e| format!("Failed to refresh Explorer: {}", e))?;

    Ok(())
}

/// Remove ZipHere from Windows right-click context menu
///
/// # Returns
/// * `Result<(), String>` - Ok if successful, Err with error message otherwise
#[cfg(windows)]
#[tauri::command]
pub fn remove_context_menu() -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // 删除目录右键菜单
    match hkcu.delete_subkey_all("Software\\Classes\\Directory\\shell\\ZipHere") {
        Ok(_) => (),
        Err(e) => {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(format!("Failed to remove directory menu: {}", e));
            }
        }
    }

    // 删除文件右键菜单
    match hkcu.delete_subkey_all("Software\\Classes\\*\\shell\\ZipHere") {
        Ok(_) => (),
        Err(e) => {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(format!("Failed to remove file menu: {}", e));
            }
        }
    }

    // 刷新Windows资源管理器
    Command::new("cmd")
        .args([
            "/C",
            "taskkill",
            "/f",
            "/im",
            "explorer.exe",
            "&&",
            "start",
            "explorer.exe",
        ])
        .output()
        .map_err(|e| format!("Failed to refresh Explorer: {}", e))?;

    Ok(())
}

/// Set file associations for ZipHere
///
/// # Arguments
/// * `formats` - Comma-separated list of formats to associate, or "all"
/// * `open_mode` - The mode to open when associated files are clicked ("gui" or "viewer")
///
/// # Returns
/// * `Result<(), String>` - Ok if successful, Err with error message otherwise
#[cfg(windows)]
#[tauri::command]
pub fn set_file_association(formats: String, open_mode: String) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    // 获取程序路径
    let exe_path =
        std::env::current_exe().map_err(|e| format!("Failed to get executable path: {}", e))?;
    let exe_path_str = exe_path.to_string_lossy().to_string();

    // 确定要关联的格式
    let format_list = if formats == "all" {
        CompressionFormat::all_formats()
    } else {
        let mut formats_vec = Vec::new();
        for format_str in formats.split(',') {
            if let Some(format) = CompressionFormat::from_extension(format_str.trim()) {
                formats_vec.push(format);
            }
        }
        formats_vec
    };

    if format_list.is_empty() {
        return Err("No valid formats specified".to_string());
    }

    // 为每种格式设置文件关联
    for format in format_list {
        let extension = format.extension();
        let name = format!("ZipHere.{}", format.name());
        let description = match format {
            CompressionFormat::Zip => "ZIP 压缩文件",
            CompressionFormat::Tar => "TAR 归档文件",
            CompressionFormat::Gzip => "GZIP 压缩文件",
            CompressionFormat::Bzip2 => "BZIP2 压缩文件",
            CompressionFormat::Xz => "XZ 压缩文件",
            CompressionFormat::Zstd => "Zstandard 压缩文件",
            CompressionFormat::SevenZip => "7-Zip 压缩文件",
        };

        // 注册文件类型
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let file_type_key = hkcu
            .create_subkey(&format!("Software\\Classes\\.{}", extension))
            .map_err(|e| format!("Failed to create file type key: {}", e))?;
        file_type_key
            .0
            .set_value("", &name)
            .map_err(|e| format!("Failed to set file type: {}", e))?;

        // 创建应用注册表项
        let app_key = hkcu
            .create_subkey(&format!("Software\\Classes\\{}", name))
            .map_err(|e| format!("Failed to create app key: {}", e))?;
        app_key
            .0
            .set_value("", &description)
            .map_err(|e| format!("Failed to set description: {}", e))?;

        // 设置默认图标
        let icon_key = app_key
            .0
            .create_subkey("DefaultIcon")
            .map_err(|e| format!("Failed to create icon key: {}", e))?;
        icon_key
            .0
            .set_value("", &format!("{},0", exe_path_str))
            .map_err(|e| format!("Failed to set icon: {}", e))?;

        // 添加打开命令
        let command_key = app_key
            .0
            .create_subkey("shell\\open\\command")
            .map_err(|e| format!("Failed to create command key: {}", e))?;

        // 根据打开模式选择命令行参数
        let command_str = if open_mode == "viewer" {
            // 文件查看器模式
            println!("TODO: 实现文件查看器模式");
            format!("\"{}\" view \"%1\"", exe_path_str)
        } else {
            // GUI模式
            format!("\"{}\" d \"%1\"", exe_path_str)
        };

        command_key
            .0
            .set_value("", &command_str)
            .map_err(|e| format!("Failed to set command: {}", e))?;
    }

    // 刷新Windows资源管理器
    Command::new("cmd")
        .args([
            "/C",
            "taskkill",
            "/f",
            "/im",
            "explorer.exe",
            "&&",
            "start",
            "explorer.exe",
        ])
        .output()
        .map_err(|e| format!("Failed to refresh Explorer: {}", e))?;

    Ok(())
}

/// Remove file associations for ZipHere
///
/// # Arguments
/// * `formats` - Comma-separated list of formats to remove associations for, or "all"
///
/// # Returns
/// * `Result<(), String>` - Ok if successful, Err with error message otherwise
#[cfg(windows)]
#[tauri::command]
pub fn remove_file_association(formats: String) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    // 确定要删除关联的格式
    let format_list = if formats == "all" {
        CompressionFormat::all_formats()
    } else {
        let mut formats_vec = Vec::new();
        for format_str in formats.split(',') {
            if let Some(format) = CompressionFormat::from_extension(format_str.trim()) {
                formats_vec.push(format);
            }
        }
        formats_vec
    };

    if format_list.is_empty() {
        return Err("No valid formats specified".to_string());
    }

    // 为每种格式删除文件关联
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for format in format_list {
        let extension = format.extension();
        let name = format!("ZipHere.{}", format.name());

        // 删除注册表项
        match hkcu.delete_subkey_all(&format!("Software\\Classes\\.{}", extension)) {
            Ok(_) => (),
            Err(e) => {
                if e.kind() != std::io::ErrorKind::NotFound {
                    return Err(format!("Failed to remove extension association: {}", e));
                }
            }
        }

        match hkcu.delete_subkey_all(&format!("Software\\Classes\\{}", name)) {
            Ok(_) => (),
            Err(e) => {
                if e.kind() != std::io::ErrorKind::NotFound {
                    return Err(format!("Failed to remove application association: {}", e));
                }
            }
        }
    }

    // 刷新Windows资源管理器
    Command::new("cmd")
        .args([
            "/C",
            "taskkill",
            "/f",
            "/im",
            "explorer.exe",
            "&&",
            "start",
            "explorer.exe",
        ])
        .output()
        .map_err(|e| format!("Failed to refresh Explorer: {}", e))?;

    Ok(())
}
