// Export compression format related modules
pub mod z_7zip;
pub mod z_bzip2;
pub mod z_gzip;
pub mod z_tar;
pub mod z_xz;
pub mod z_zip;
pub mod z_zstd;

// Import compressor implementations
use z_7zip::SevenZipCompressor;
use z_bzip2::Bzip2Compressor;
use z_gzip::GzipCompressor;
use z_tar::TarCompressor;
use z_xz::XzCompressor;
use z_zip::ZipCompressor;
use z_zstd::ZstdCompressor;

// Compression options
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

// Decompression options
pub enum DecompressionOptions {
    Zip { password: Option<String> },
    Tar {},
    Gzip {},
    Bzip2 {},
    Xz {},
    Zstd {},
    SevenZip { password: Option<String> },
}

// Compression/decompression interface
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

// Supported compression formats
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
    // Get the file extension for the format
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

    // Get format from file extension
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

    // Get all supported formats
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

    // Get format display name
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

// Compressor, using enum instead of trait objects
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
    // Create a compressor for the specified format
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

    // Compression method
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

    // Decompression method
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

// Simplified compressor factory without trait objects
pub struct CompressorFactory;

impl CompressorFactory {
    // Create a new compressor factory
    pub fn new() -> Self {
        Self {}
    }

    // Get a compressor for the specified format
    pub fn get(&self, format: CompressionFormat) -> Option<Compressor> {
        Some(Compressor::new(format))
    }

    // Get all supported formats
    pub fn supported_formats(&self) -> Vec<CompressionFormat> {
        CompressionFormat::all_formats()
    }
}
