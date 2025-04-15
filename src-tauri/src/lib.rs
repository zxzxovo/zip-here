// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// Learn more about Tauri plugins at https://tauri.app/develop/plugins/

pub mod cli;
pub mod comde;
pub mod commands;

use commands::{
    compress_files, decompress_files, get_format_options, get_supported_formats, get_version_info,
};

// 添加 Windows 集成相关的命令
#[cfg(windows)]
use commands::{
    add_context_menu, remove_context_menu, remove_file_association, set_file_association,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            compress_files,
            decompress_files,
            get_supported_formats,
            get_format_options,
            get_version_info,
            // 添加 Windows 集成相关的命令
            #[cfg(windows)]
            add_context_menu,
            #[cfg(windows)]
            remove_context_menu,
            #[cfg(windows)]
            set_file_association,
            #[cfg(windows)]
            remove_file_association
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
