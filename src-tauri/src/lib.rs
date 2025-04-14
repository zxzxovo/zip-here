// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// Learn more about Tauri plugins at https://tauri.app/develop/plugins/

mod comde;
mod commands;

use commands::{
    compress_files, decompress_files, get_supported_formats,
    get_format_options, get_version_info
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            compress_files,
            decompress_files,
            get_supported_formats,
            get_format_options,
            get_version_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
