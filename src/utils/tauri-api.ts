import { invoke } from '@tauri-apps/api/core';
import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog';

/**
 * 压缩选项接口
 */
export interface CompressOptions {
  format: string;
  level?: number;
  password?: string;
}

/**
 * 解压选项接口
 */
export interface DecompressOptions {
  password?: string;
}

/**
 * 格式选项接口
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
 * 版本信息接口
 */
export interface VersionInfo {
  version: string;
  build_time: string;
  author: string;
  description: string;
}

/**
 * 压缩文件
 * @param inputPaths 输入文件/目录路径列表
 * @param outputPath 输出文件路径
 * @param options 压缩选项
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
 * 解压文件
 * @param inputPaths 输入文件路径列表
 * @param outputPath 输出目录路径
 * @param format 文件格式（可选，如果未提供则自动检测）
 * @param options 解压选项
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
 * 打开文件选择对话框
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