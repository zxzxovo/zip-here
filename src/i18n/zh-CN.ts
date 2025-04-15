import { I18nKeyType } from "./type"

const zhCN: I18nKeyType = {
    // General
    "appName": "ZipHere",
    "pageCompress": "压缩",
    "pageDecompress": "解压缩",
    "pageOptions": "设置",
    
    // Actions
    "start": "开始",
    "cancel": "取消",
    "save": "保存",
    "reset": "重置",
    "loading": "加载中...",
    "processing": "处理中...",
    "success": "成功",
    "error": "错误",
    "warning": "警告",
    "info": "信息",
    "close": "关闭",
    
    // Compress
    "compressTitle": "文件压缩",
    "selectFilesToCompress": "选择要压缩的文件",
    "dragFilesHere": "拖拽文件到此处，或点击选择文件",
    "multipleFilesSupported": "可选择多个文件或文件夹进行压缩",
    "selectedFiles": "已选择 {0} 个文件",
    "clearSelection": "清除选择",
    "compressFormat": "压缩格式：",
    "outputPath": "输出路径：",
    "selectOutputPath": "选择输出路径",
    "useSourcePath": "使用源文件路径",
    "outputToDesktop": "输出到桌面",
    "fileName": "文件名：",
    "enterArchiveName": "输入压缩包名称",
    "resultPath": "输出路径: {0}",
    "compressionLevel": "压缩级别:",
    "passwordProtection": "使用密码保护",
    "password": "密码:",
    "enterPassword": "输入密码",
    "showAdvanced": "显示高级选项 ▼",
    "hideAdvanced": "隐藏高级选项 ▲",
    "compressButtonText": "开始压缩",
    "compressing": "压缩中...",
    "compressSuccess": "文件压缩成功!",
    "compressError": "压缩失败: {0}",
    "pleaseSelectFiles": "请选择要压缩的文件",
    "pleaseSelectOutputPath": "请选择输出路径",
    
    // Decompress
    "decompressTitle": "文件解压",
    "selectArchiveToDecompress": "选择压缩文件",
    "dragArchiveHere": "拖拽文件到此处，或点击选择文件",
    "supportedFormatsAll": "支持 .zip、.rar、.7z、.tar.gz 等格式",
    "decompressMethod": "解压方式：",
    "createFolderForEach": "为每个压缩包创建文件夹",
    "directExtract": "直接解压到选择的目录",
    "extractToOneFolder": "解压到一个文件夹",
    "usePasswordToDecompress": "使用密码",
    "decompressButtonText": "开始解压",
    "decompressing": "解压中...",
    "decompressSuccess": "文件解压成功!",
    "decompressError": "解压失败: {0}",
    "pleaseSelectArchive": "请选择要解压的文件",
    
    // Options - General
    "generalSettings": "常规设置",
    "autoExtractBehavior": "自动解压行为:",
    "askBeforeExtracting": "解压前询问",
    "alwaysExtract": "始终自动解压",
    "neverExtract": "从不自动解压",
    "defaultCompressFormat": "默认压缩格式:",
    "defaultCompressionLevel": "默认压缩级别: {0}",
    "showNotification": "操作完成时显示通知",
    "clearFilesAfterOp": "操作后清除文件选择",
    "darkMode": "深色模式",
    "settingsSaved": "设置已保存",
    "failedToSaveSettings": "保存设置失败: {0}",
    "saveSettings": "保存设置",
    "saving": "保存中...",
    "resetToDefault": "重置为默认值",
    
    // Options - Formats
    "supportedFormats": "支持的格式",
    "loadingFormats": "加载格式信息...",
    "formatsIntro": "ZipHere 支持以下压缩和解压缩格式:",
    "formatCapabilities": {
        "compression": "压缩",
        "decompression": "解压缩",
        "passwordProtection": "密码保护",
        "compressionLevels": "压缩级别"
    },
    "compressionLevels": "压缩级别: {0} - {1}",
    "defaultLevel": "(默认: {0})",
    "formatsNote": "注意: 部分格式的支持可能取决于系统库和配置。",
    
    // Options - About
    "aboutTitle": "关于 ZipHere",
    "loadingAppInfo": "加载应用信息...",
    "version": "版本: {0}",
    "buildTime": "构建时间: {0}",
    "createdBy": "作者: {0}",
    "builtWith": "技术栈:",
    "tauriDesc": "Tauri - 使用网页前端的原生桌面应用程序",
    "vueDesc": "Vue.js - 渐进式 JavaScript 框架",
    "rustDesc": "Rust - 赋能每个人构建可靠高效软件的语言",
    "copyright": "© 2025 ZipHere. 保留所有权利。",
    "disclaimer": "本软件按原样提供，不提供任何形式的保证。"
}

export default zhCN
