import { invoke } from '@tauri-apps/api/core';
import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog';
import { desktopDir } from '@tauri-apps/api/path';
import { platform } from '@tauri-apps/plugin-os';

/**
 * Compression options interface
 */
export interface CompressOptions {
  format: string;
  level?: number;
  password?: string;
}

/**
 * Decompression options interface
 */
export interface DecompressOptions {
  password?: string;
}

/**
 * Format option interface
 */
export interface FormatOption {
  id: string;
  name: string;
  extension: string;
  can_compress: boolean;
  can_decompress: boolean;
  supports_password: boolean;
  supports_level: boolean;
  min_level?: number;
  max_level?: number;
  default_level?: number;
}

/**
 * Version information interface
 */
export interface VersionInfo {
  version: string;
  build_time: string;
  author: string;
  description: string;
}

/**
 * Compress files
 * @param inputPaths List of input file/directory paths
 * @param outputPath Output file path
 * @param options Compression options
 */
export async function compressFiles(
  inputPaths: string[], 
  outputPath: string, 
  options: CompressOptions
): Promise<void> {
  return await invoke<void>('compress_files', {
    inputPaths,
    outputPath,
    options
  });
}

/**
 * Decompress files
 * @param inputPaths List of input archive file paths
 * @param outputPath Output directory path
 * @param format File format (optional, auto-detected if not provided)
 * @param options Decompression options
 */
export async function decompressFiles(
  inputPaths: string[], 
  outputPath: string, 
  format: string = '', 
  options?: DecompressOptions
): Promise<void> {
  return await invoke<void>('decompress_files', {
    inputPaths,
    outputPath,
    format,
    options
  });
}

/**
 * 获取支持的压缩/解压缩格式
 */
export async function getSupportedFormats(): Promise<string[]> {
  return await invoke<string[]>('get_supported_formats');
}

/**
 * 获取指定格式的选项信息
 * @param format 格式名称
 */
export async function getFormatOptions(format: string): Promise<FormatOption> {
  return await invoke<FormatOption>('get_format_options', { format });
}

/**
 * 获取应用版本信息
 */
export async function getVersionInfo(): Promise<VersionInfo> {
  return await invoke<VersionInfo>('get_version_info');
}

/**
 * 检查是否为 Windows 系统
 */
export async function isWindowsOS(): Promise<boolean> {
  return (await platform() as string) === 'win32';
}

/**
 * 添加到 Windows 右键菜单
 * @param openMode 打开模式 ('cli' 或 'gui')
 */
export async function addContextMenu(openMode: 'cli' | 'gui'): Promise<void> {
  return await invoke<void>('add_context_menu', { openMode });
}

/**
 * 从 Windows 右键菜单中移除
 */
export async function removeContextMenu(): Promise<void> {
  return await invoke<void>('remove_context_menu');
}

/**
 * 设置文件关联
 * @param formats 要关联的文件格式（逗号分隔的字符串或 'all'）
 * @param openMode 打开模式 ('gui' 或 'viewer')
 */
export async function setFileAssociation(
  formats: string,
  openMode: 'gui' | 'viewer'
): Promise<void> {
  return await invoke<void>('set_file_association', { formats, openMode });
}

/**
 * 移除文件关联
 * @param formats 要移除关联的文件格式（逗号分隔的字符串或 'all'）
 */
export async function removeFileAssociation(formats: string): Promise<void> {
  return await invoke<void>('remove_file_association', { formats });
}

/**
 * @param multiple 是否允许多选
 * @param filters 文件过滤器
 */
export async function selectFiles(
  multiple: boolean = false,
  filters?: { name: string, extensions: string[] }[]
): Promise<string | string[] | null> {
  return await openDialog({
    multiple,
    filters
  });
}

/**
 * 打开目录选择对话框
 * @param multiple 是否允许多选
 */
export async function selectDirectory(
  multiple: boolean = false
): Promise<string | string[] | null> {
  return await openDialog({
    directory: true,
    multiple
  });
}

/**
 * 打开文件保存对话框
 * @param defaultPath 默认路径
 * @param filters 文件过滤器
 */
export async function saveFile(
  defaultPath?: string,
  filters?: { name: string, extensions: string[] }[]
): Promise<string | null> {
  return await saveDialog({
    defaultPath,
    filters
  });
}

/**
 * 根据文件扩展名获取合适的文件过滤器
 * @param format 格式名称
 */
export function getFileFilters(format: string): { name: string, extensions: string[] }[] {
  switch (format.toLowerCase()) {
    case 'zip':
      return [{ name: 'ZIP文件', extensions: ['zip'] }];
    case 'tar':
      return [{ name: 'TAR文件', extensions: ['tar'] }];
    case 'gz':
    case 'gzip':
      return [{ name: 'GZIP文件', extensions: ['gz', 'gzip'] }];
    case 'bz2':
    case 'bzip2':
      return [{ name: 'BZIP2文件', extensions: ['bz2', 'bzip2'] }];
    case 'xz':
      return [{ name: 'XZ文件', extensions: ['xz'] }];
    case 'zst':
    case 'zstd':
      return [{ name: 'ZSTD文件', extensions: ['zst', 'zstd'] }];
    case '7z':
      return [{ name: '7-Zip文件', extensions: ['7z'] }];
    default:
      return [
        { 
          name: '压缩文件', 
          extensions: ['zip', 'tar', 'gz', 'bz2', 'xz', 'zst', '7z'] 
        }
      ];
  }
}

/**
 * Get desktop directory path
 */
export async function getDesktopPath(): Promise<string> {
  return await desktopDir();
}