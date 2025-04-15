import { I18nKeyType } from "./type"

const en: I18nKeyType = {
    // General
    "appName": "ZipHere",
    "pageCompress": "Compress",
    "pageDecompress": "Decompress",
    "pageOptions": "Options",
    
    // Actions
    "start": "Start",
    "cancel": "Cancel",
    "save": "Save",
    "reset": "Reset",
    "loading": "Loading...",
    "processing": "Processing...",
    "success": "Success",
    "error": "Error",
    "warning": "Warning",
    "info": "Information",
    "close": "Close",
    
    // Compress
    "compressTitle": "File Compression",
    "selectFilesToCompress": "Select files to compress",
    "dragFilesHere": "Drag files here, or click to select files",
    "multipleFilesSupported": "Multiple files or folders can be selected",
    "selectedFiles": "{0} files selected",
    "clearSelection": "Clear selection",
    "compressFormat": "Compression format:",
    "outputPath": "Output path:",
    "selectOutputPath": "Select output path",
    "useSourcePath": "Use source file path",
    "outputToDesktop": "Output to desktop",
    "fileName": "File name:",
    "enterArchiveName": "Enter archive name",
    "resultPath": "Output path: {0}",
    "compressionLevel": "Compression level:",
    "passwordProtection": "Use password protection",
    "password": "Password:",
    "enterPassword": "Enter password",
    "showAdvanced": "Show advanced options ▼",
    "hideAdvanced": "Hide advanced options ▲",
    "compressButtonText": "Start Compression",
    "compressing": "Compressing...",
    "compressSuccess": "Files compressed successfully!",
    "compressError": "Compression failed: {0}",
    "pleaseSelectFiles": "Please select files to compress",
    "pleaseSelectOutputPath": "Please select an output path",
    
    // Decompress
    "decompressTitle": "File Extraction",
    "selectArchiveToDecompress": "Select archive file",
    "dragArchiveHere": "Drag archive here, or click to select file",
    "supportedFormatsAll": "Supports .zip, .rar, .7z, .tar.gz and other formats",
    "decompressMethod": "Extraction method:",
    "createFolderForEach": "Create folder for each archive",
    "directExtract": "Extract directly to selected directory",
    "extractToOneFolder": "Extract to one folder",
    "usePasswordToDecompress": "Use password",
    "decompressButtonText": "Start Extraction",
    "decompressing": "Extracting...",
    "decompressSuccess": "Files extracted successfully!",
    "decompressError": "Extraction failed: {0}",
    "pleaseSelectArchive": "Please select an archive to extract",
    
    // Options - General
    "generalSettings": "General Settings",
    "autoExtractBehavior": "Auto-extract behavior:",
    "askBeforeExtracting": "Ask before extracting",
    "alwaysExtract": "Always extract automatically",
    "neverExtract": "Never extract automatically",
    "defaultCompressFormat": "Default compression format:",
    "defaultCompressionLevel": "Default compression level: {0}",
    "showNotification": "Show notification when operation completes",
    "clearFilesAfterOp": "Clear file selection after operation",
    "darkMode": "Dark Mode",
    "settingsSaved": "Settings saved successfully",
    "failedToSaveSettings": "Failed to save settings: {0}",
    "saveSettings": "Save Settings",
    "saving": "Saving...",
    "resetToDefault": "Reset to Default",
    
    // Options - Formats
    "supportedFormats": "Supported Formats",
    "loadingFormats": "Loading format information...",
    "formatsIntro": "ZipHere supports the following compression and decompression formats:",
    "formatCapabilities": {
        "compression": "Compression",
        "decompression": "Decompression",
        "passwordProtection": "Password Protection",
        "compressionLevels": "Compression Levels"
    },
    "compressionLevels": "Compression Levels: {0} - {1}",
    "defaultLevel": "(Default: {0})",
    "formatsNote": "Note: Support for some formats may depend on system libraries and configurations.",
    
    // Options - About
    "aboutTitle": "About ZipHere",
    "loadingAppInfo": "Loading application information...",
    "version": "Version: {0}",
    "buildTime": "Build Time: {0}",
    "createdBy": "Created by: {0}",
    "builtWith": "Built with:",
    "tauriDesc": "Tauri - Native desktop applications with a web frontend",
    "vueDesc": "Vue.js - Progressive JavaScript framework",
    "rustDesc": "Rust - A language empowering everyone to build reliable and efficient software",
    "copyright": "© 2025 ZipHere. All rights reserved.",
    "disclaimer": "This software is provided as-is without warranty of any kind."
}

export default en