// filepath: f:\TauriProjects\ZipHere\src-tauri\src\cli.rs
use crate::comde::{CompressionFormat, CompressionOptions, Compressor, DecompressionOptions};
use clap::{Args, Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 压缩文件
    #[command(name = "c", alias = "compress")]
    Compress(CompressArgs),

    /// 解压文件
    #[command(name = "d", alias = "decompress")]
    Decompress(DecompressArgs),

    /// Windows系统配置选项
    #[command(name = "config")]
    Config(ConfigArgs),

    /// 查看压缩文件内容（文件查看器模式）
    #[command(name = "view")]
    View(ViewArgs),
}

#[derive(Args)]
pub struct CompressArgs {
    /// 输入文件或目录的路径
    #[arg(required = true)]
    pub input: Vec<PathBuf>,

    /// 输出文件路径
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// 压缩格式，可选: zip, tar, gz, bz2, xz, zst, 7z
    #[arg(short, long, default_value = "zip")]
    pub format: String,

    /// 压缩级别 (1-9，对于zstd是1-22)
    #[arg(short, long)]
    pub level: Option<u32>,

    /// 用于加密的密码 (仅支持zip和7z格式)
    #[arg(short, long)]
    pub password: Option<String>,
}

#[derive(Args)]
pub struct DecompressArgs {
    /// 要解压的文件路径
    #[arg(required = true)]
    pub input: Vec<PathBuf>,

    /// 解压的目标目录
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// 用于解密的密码 (仅在需要时使用)
    #[arg(short, long)]
    pub password: Option<String>,
}

#[derive(Args)]
pub struct ConfigArgs {
    /// 设置文件关联 (仅Windows)
    #[arg(long)]
    pub set_association: bool,

    /// 移除文件关联 (仅Windows)
    #[arg(long)]
    pub remove_association: bool,

    /// 要关联的文件格式，可选: all, zip, tar, gz, bz2, xz, zst, 7z
    #[arg(long, default_value = "all")]
    pub formats: String,

    /// 添加到Windows右键菜单
    #[arg(long)]
    pub add_context_menu: bool,

    /// 从Windows右键菜单移除
    #[arg(long)]
    pub remove_context_menu: bool,

    /// 右键菜单的打开模式 (cli/gui)
    #[arg(long, default_value = "gui")]
    pub context_menu_mode: String,

    /// 文件关联的打开模式 (gui/viewer)
    #[arg(long, default_value = "gui")]
    pub association_mode: String,
}

#[derive(Args)]
pub struct ViewArgs {
    /// 要查看的压缩文件路径
    #[arg(required = true)]
    pub input: PathBuf,
}

/// 处理CLI命令执行
pub async fn handle_cli() -> anyhow::Result<bool> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Compress(args) => {
            compress_command(args).await?;
            Ok(true)
        },
        Commands::Decompress(args) => {
            decompress_command(args).await?;
            Ok(true)
        },
        Commands::Config(args) => {
            #[cfg(windows)]
            {
                config_command(args)?;
            }
            #[cfg(not(windows))]
            {
                println!("配置选项仅在Windows系统上可用");
            }
            Ok(true)
        },
        Commands::View(args) => {
            view_command(args).await?;
            Ok(true)
        },
    }
}

async fn compress_command(args: CompressArgs) -> anyhow::Result<()> {
    // 验证输入路径
    for path in &args.input {
        if !path.exists() {
            return Err(anyhow::anyhow!("输入路径不存在: {}", path.display()));
        }
    }

    // 确定输出路径
    let output_path = if let Some(path) = args.output {
        path
    } else if args.input.len() == 1 {
        // 如果只有一个输入，使用其名称 + 格式后缀
        let input = &args.input[0];
        let stem = input.file_stem().unwrap_or_default();
        let mut output = PathBuf::from(stem);
        output.set_extension(&args.format);
        output
    } else {
        // 多个输入时创建一个新的压缩文件
        let mut output = PathBuf::from("compressed");
        output.set_extension(&args.format);
        output
    };

    // 创建输出目录
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    // 解析压缩格式
    let format = CompressionFormat::from_extension(&args.format)
        .ok_or_else(|| anyhow::anyhow!("不支持的压缩格式: {}", args.format))?;

    // 创建压缩器
    let compressor = Compressor::new(format);

    // 创建压缩选项
    let options = match format {
        CompressionFormat::Zip => Some(CompressionOptions::Zip {
            level: args.level.unwrap_or(6),
            password: args.password,
        }),
        CompressionFormat::Tar => Some(CompressionOptions::Tar {
            level: args.level.unwrap_or(6),
        }),
        CompressionFormat::Gzip => Some(CompressionOptions::Gzip {
            level: args.level.unwrap_or(6),
        }),
        CompressionFormat::Bzip2 => Some(CompressionOptions::Bzip2 {
            level: args.level.unwrap_or(6),
        }),
        CompressionFormat::Xz => Some(CompressionOptions::Xz {
            level: args.level.unwrap_or(6),
        }),
        CompressionFormat::Zstd => Some(CompressionOptions::Zstd {
            level: args.level.unwrap_or(3),
        }),
        CompressionFormat::SevenZip => Some(CompressionOptions::SevenZip {
            level: args.level.unwrap_or(6),
            password: args.password,
        }),
    };

    // 转换路径为字符串
    let input_paths: Vec<&str> = args.input.iter().filter_map(|p| p.to_str()).collect();

    if input_paths.is_empty() {
        return Err(anyhow::anyhow!("无法处理输入路径"));
    }

    // 执行压缩
    compressor
        .compress(
            input_paths,
            output_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("无法处理输出路径"))?,
            options,
        )
        .await
        .map_err(|e: String| anyhow::anyhow!(e))?;

    println!("压缩完成: {}", output_path.display());
    Ok(())
}

async fn decompress_command(args: DecompressArgs) -> anyhow::Result<()> {
    // 验证输入路径
    for path in &args.input {
        if !path.exists() {
            return Err(anyhow::anyhow!("输入文件不存在: {}", path.display()));
        }
    }

    // 确定输出路径
    let output_path = if let Some(path) = args.output {
        path
    } else if args.input.len() == 1 {
        // 如果只有一个输入，使用其名称作为目录
        let input = &args.input[0];
        if let Some(stem) = input.file_stem() {
            PathBuf::from(stem)
        } else {
            PathBuf::from("extracted")
        }
    } else {
        // 多个输入时创建一个新的目录
        PathBuf::from("extracted")
    };

    // 创建输出目录
    if !output_path.exists() {
        std::fs::create_dir_all(&output_path)?;
    }

    // 转换路径为字符串
    let input_paths: Vec<&str> = args.input.iter().filter_map(|p| p.to_str()).collect();

    if input_paths.is_empty() {
        return Err(anyhow::anyhow!("无法处理输入路径"));
    }

    // 自动检测格式
    let first_path = Path::new(input_paths[0]);
    let format = if let Some(ext) = first_path.extension() {
        if let Some(ext_str) = ext.to_str() {
            CompressionFormat::from_extension(ext_str)
                .ok_or_else(|| anyhow::anyhow!("无法从文件扩展名识别格式: {}", ext_str))?
        } else {
            return Err(anyhow::anyhow!("无效的文件扩展名"));
        }
    } else {
        return Err(anyhow::anyhow!("文件没有扩展名，无法确定格式"));
    };

    // 创建解压器
    let decompressor = Compressor::new(format);

    // 创建解压选项
    let options = match format {
        CompressionFormat::Zip => Some(DecompressionOptions::Zip {
            password: args.password,
        }),
        CompressionFormat::Tar => Some(DecompressionOptions::Tar {}),
        CompressionFormat::Gzip => Some(DecompressionOptions::Gzip {}),
        CompressionFormat::Bzip2 => Some(DecompressionOptions::Bzip2 {}),
        CompressionFormat::Xz => Some(DecompressionOptions::Xz {}),
        CompressionFormat::Zstd => Some(DecompressionOptions::Zstd {}),
        CompressionFormat::SevenZip => Some(DecompressionOptions::SevenZip {
            password: args.password,
        }),
    };

    // 执行解压
    decompressor
        .decompress(
            input_paths,
            output_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("无法处理输出路径"))?,
            options,
        )
        .await
        .map_err(|e: String| anyhow::anyhow!(e))?;

    println!("解压完成: {}", output_path.display());
    Ok(())
}

async fn view_command(args: ViewArgs) -> anyhow::Result<()> {
    // 验证输入文件是否存在
    if !args.input.exists() {
        return Err(anyhow::anyhow!("输入文件不存在: {}", args.input.display()));
    }
    
    // 获取文件扩展名
    let format = if let Some(ext) = args.input.extension() {
        if let Some(ext_str) = ext.to_str() {
            CompressionFormat::from_extension(ext_str)
                .ok_or_else(|| anyhow::anyhow!("无法从文件扩展名识别格式: {}", ext_str))?
        } else {
            return Err(anyhow::anyhow!("无效的文件扩展名"));
        }
    } else {
        return Err(anyhow::anyhow!("文件没有扩展名，无法确定格式"));
    };
    
    println!("TODO: 文件查看器模式尚未实现");
    println!("将打开文件: {}", args.input.display());
    println!("文件格式: {}", format.name());
    
    // 这里我们将来会实现实际的文件查看器逻辑
    // 可能包括：
    // 1. 列出压缩文件中的条目
    // 2. 提供预览选定条目的功能
    // 3. 提供从压缩文件中提取单个条目的功能
    
    Ok(())
}

#[cfg(windows)]
fn config_command(args: ConfigArgs) -> anyhow::Result<()> {
    use std::process::Command;

    // 获取程序路径
    let exe_path = std::env::current_exe()?;
    let exe_path_str = exe_path.to_string_lossy();

    // 处理文件关联
    if args.set_association {
        let formats = if args.formats == "all" {
            CompressionFormat::all_formats()
        } else {
            let mut formats = Vec::new();
            for format_str in args.formats.split(',') {
                if let Some(format) = CompressionFormat::from_extension(format_str.trim()) {
                    formats.push(format);
                }
            }
            formats
        };

        for format in formats {
            set_file_association(format, &exe_path_str, &args.association_mode)?;
            println!(
                "已设置.{} 文件关联，打开模式: {}",
                format.extension(),
                args.association_mode
            );
        }
    }

    if args.remove_association {
        let formats = if args.formats == "all" {
            CompressionFormat::all_formats()
        } else {
            let mut formats = Vec::new();
            for format_str in args.formats.split(',') {
                if let Some(format) = CompressionFormat::from_extension(format_str.trim()) {
                    formats.push(format);
                }
            }
            formats
        };

        for format in formats {
            remove_file_association(format)?;
            println!("已移除.{} 文件关联", format.extension());
        }
    }

    // 处理右键菜单
    if args.add_context_menu {
        add_context_menu(&exe_path_str, &args.context_menu_mode)?;
        println!("已添加右键菜单项，打开模式: {}", args.context_menu_mode);
    }

    if args.remove_context_menu {
        remove_context_menu()?;
        println!("已移除右键菜单项");
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
        .output()?;

    Ok(())
}

#[cfg(windows)]
fn set_file_association(
    format: CompressionFormat,
    exe_path: &str,
    mode: &str,
) -> anyhow::Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

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
    let file_type_key = hkcu.create_subkey(&format!("Software\\Classes\\.{}", extension))?;
    file_type_key.0.set_value("", &name)?;

    // 创建应用注册表项
    let app_key = hkcu.create_subkey(&format!("Software\\Classes\\{}", name))?;
    app_key.0.set_value("", &description)?;

    // 设置默认图标
    let icon_key = app_key.0.create_subkey("DefaultIcon")?;
    icon_key.0.set_value("", &format!("{},0", exe_path))?;

    // 添加打开命令
    let command_key = app_key.0.create_subkey("shell\\open\\command")?;
    
    // 根据打开模式选择命令行参数
    let command_str = if mode == "viewer" {
        // 文件查看器模式
        format!("\"{}\" view \"%1\"", exe_path)
    } else {
        // GUI 模式 (默认)
        format!("\"{}\" d \"%1\"", exe_path)
    };
    
    command_key.0.set_value("", &command_str)?;

    Ok(())
}

#[cfg(windows)]
fn remove_file_association(format: CompressionFormat) -> anyhow::Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let extension = format.extension();
    let name = format!("ZipHere.{}", format.name());

    // 删除注册表项
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    hkcu.delete_subkey_all(&format!("Software\\Classes\\.{}", extension))?;
    hkcu.delete_subkey_all(&format!("Software\\Classes\\{}", name))?;

    Ok(())
}

#[cfg(windows)]
fn add_context_menu(exe_path: &str, mode: &str) -> anyhow::Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    // 为目录添加右键菜单
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // 目录右键菜单 - 压缩到...
    {
        let dir_key = hkcu.create_subkey("Software\\Classes\\Directory\\shell\\ZipHere")?;
        dir_key.0.set_value("", &"使用 ZipHere 压缩")?;
        dir_key.0.set_value("Icon", &format!("{},0", exe_path))?;

        let command_key = dir_key.0.create_subkey("command")?;
        
        // 根据打开模式选择命令行参数
        let command_str = if mode == "cli" {
            format!("\"{}\" c \"%1\"", exe_path)
        } else {
            // GUI模式 (默认)
            format!("\"{}\"", exe_path)
        };
        
        command_key.0.set_value("", &command_str)?;
    }

    // 多选文件右键菜单 - 压缩选中项...
    {
        let multi_key = hkcu.create_subkey("Software\\Classes\\*\\shell\\ZipHere")?;
        multi_key.0.set_value("", &"使用 ZipHere 压缩")?;
        multi_key.0.set_value("Icon", &format!("{},0", exe_path))?;
        multi_key.0.set_value("MultiSelectModel", &"Player")?;

        let command_key = multi_key.0.create_subkey("command")?;
        
        // 根据打开模式选择命令行参数
        let command_str = if mode == "cli" {
            format!("\"{}\" c \"%1\"", exe_path)
        } else {
            // GUI模式 (默认)
            format!("\"{}\"", exe_path)
        };
        
        command_key.0.set_value("", &command_str)?;
    }

    Ok(())
}

#[cfg(windows)]
fn remove_context_menu() -> anyhow::Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // 删除目录右键菜单
    hkcu.delete_subkey_all("Software\\Classes\\Directory\\shell\\ZipHere")?;

    // 删除文件右键菜单
    hkcu.delete_subkey_all("Software\\Classes\\*\\shell\\ZipHere")?;

    Ok(())
}
