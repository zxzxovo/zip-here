// 向外部导出压缩格式相关的模块
pub mod z_zip;
pub mod z_bzip2;
pub mod z_zstd;
pub mod z_tar;
pub mod z_gzip;
pub mod z_xz;
pub mod z_7zip;

// 导入压缩器实现
use z_zip::ZipCompressor;
use z_bzip2::Bzip2Compressor;
use z_zstd::ZstdCompressor;
use z_tar::TarCompressor;
use z_gzip::GzipCompressor;
use z_xz::XzCompressor;
use z_7zip::SevenZipCompressor;

// 压缩选项
pub enum CompressionOptions {
    Zip {
        level: u32,
        password: Option<String>,
    },
    Tar {
        level: u32,
    },
    Gzip {
        level: u32,
    },
    Bzip2 {
        level: u32,
    },
    Xz {
        level: u32,
    },
    Zstd {
        level: u32,
    },
    SevenZip {
        level: u32,
        password: Option<String>,
    },
}

// 解压选项
pub enum DecompressionOptions {
    Zip {
        password: Option<String>,
    },
    Tar {},
    Gzip {},
    Bzip2 {},
    Xz {},
    Zstd {},
    SevenZip {
        password: Option<String>,
    },
}

// 压缩/解压缩接口
pub trait ComdeAble {
    async fn compress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        options: Option<CompressionOptions>,
    ) -> Result<(), String>;

    async fn decompress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        options: Option<DecompressionOptions>,
    ) -> Result<(), String>;
}

// 支持的压缩格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompressionFormat {
    Zip,
    Tar,
    Gzip,
    Bzip2,
    Xz,
    Zstd,
    SevenZip,
}

impl CompressionFormat {
    // 获取格式对应的文件扩展名
    pub fn extension(&self) -> &'static str {
        match self {
            CompressionFormat::Zip => "zip",
            CompressionFormat::Tar => "tar",
            CompressionFormat::Gzip => "gz",
            CompressionFormat::Bzip2 => "bz2",
            CompressionFormat::Xz => "xz",
            CompressionFormat::Zstd => "zst",
            CompressionFormat::SevenZip => "7z",
        }
    }

    // 从扩展名获取格式
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "zip" => Some(Self::Zip),
            "tar" => Some(Self::Tar),
            "gz" | "gzip" => Some(Self::Gzip),
            "bz2" | "bzip2" => Some(Self::Bzip2),
            "xz" => Some(Self::Xz),
            "zst" | "zstd" => Some(Self::Zstd),
            "7z" => Some(Self::SevenZip),
            _ => None,
        }
    }

    // 获取所有支持的格式
    pub fn all_formats() -> Vec<Self> {
        vec![
            Self::Zip,
            Self::Tar,
            Self::Gzip,
            Self::Bzip2,
            Self::Xz,
            Self::Zstd,
            Self::SevenZip,
        ]
    }

    // 获取格式名称
    pub fn name(&self) -> &'static str {
        match self {
            CompressionFormat::Zip => "ZIP",
            CompressionFormat::Tar => "TAR",
            CompressionFormat::Gzip => "GZIP",
            CompressionFormat::Bzip2 => "BZIP2",
            CompressionFormat::Xz => "XZ",
            CompressionFormat::Zstd => "ZSTD",
            CompressionFormat::SevenZip => "7ZIP",
        }
    }
}

// 压缩器，使用枚举代替trait对象
#[derive(Clone)]
pub enum Compressor {
    Zip(ZipCompressor),
    Tar(TarCompressor),
    Gzip(GzipCompressor),
    Bzip2(Bzip2Compressor),
    Xz(XzCompressor),
    Zstd(ZstdCompressor),
    SevenZip(SevenZipCompressor),
}

impl Compressor {
    // 根据格式创建对应的压缩器
    pub fn new(format: CompressionFormat) -> Self {
        match format {
            CompressionFormat::Zip => Self::Zip(ZipCompressor::new()),
            CompressionFormat::Tar => Self::Tar(TarCompressor::new()),
            CompressionFormat::Gzip => Self::Gzip(GzipCompressor::new()),
            CompressionFormat::Bzip2 => Self::Bzip2(Bzip2Compressor::new()),
            CompressionFormat::Xz => Self::Xz(XzCompressor::new()),
            CompressionFormat::Zstd => Self::Zstd(ZstdCompressor::new()),
            CompressionFormat::SevenZip => Self::SevenZip(SevenZipCompressor::new()),
        }
    }

    // 压缩方法
    pub async fn compress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        options: Option<CompressionOptions>,
    ) -> Result<(), String> {
        match self {
            Self::Zip(c) => c.compress(input_paths, output_path, options).await,
            Self::Tar(c) => c.compress(input_paths, output_path, options).await,
            Self::Gzip(c) => c.compress(input_paths, output_path, options).await,
            Self::Bzip2(c) => c.compress(input_paths, output_path, options).await,
            Self::Xz(c) => c.compress(input_paths, output_path, options).await,
            Self::Zstd(c) => c.compress(input_paths, output_path, options).await,
            Self::SevenZip(c) => c.compress(input_paths, output_path, options).await,
        }
    }

    // 解压方法
    pub async fn decompress(
        &self,
        input_paths: Vec<&str>,
        output_path: &str,
        options: Option<DecompressionOptions>,
    ) -> Result<(), String> {
        match self {
            Self::Zip(c) => c.decompress(input_paths, output_path, options).await,
            Self::Tar(c) => c.decompress(input_paths, output_path, options).await,
            Self::Gzip(c) => c.decompress(input_paths, output_path, options).await,
            Self::Bzip2(c) => c.decompress(input_paths, output_path, options).await,
            Self::Xz(c) => c.decompress(input_paths, output_path, options).await,
            Self::Zstd(c) => c.decompress(input_paths, output_path, options).await,
            Self::SevenZip(c) => c.decompress(input_paths, output_path, options).await,
        }
    }
}

// 压缩器工厂简化版本，不再使用trait对象
pub struct CompressorFactory;

impl CompressorFactory {
    // 创建压缩器工厂
    pub fn new() -> Self {
        Self {}
    }

    // 获取指定格式的压缩器
    pub fn get(&self, format: CompressionFormat) -> Option<Compressor> {
        Some(Compressor::new(format))
    }

    // 获取所有支持的格式
    pub fn supported_formats(&self) -> Vec<CompressionFormat> {
        CompressionFormat::all_formats()
    }
}