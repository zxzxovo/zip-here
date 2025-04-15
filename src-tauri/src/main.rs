// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 尝试处理命令行参数
    if std::env::args().len() > 1 {
        // 有命令行参数，尝试以CLI模式运行
        if let Ok(true) = ziphere_lib::cli::handle_cli().await {
            // 命令行处理成功，退出程序
            return Ok(());
        }
    }

    // 无命令行参数或CLI模式失败，以GUI模式运行
    ziphere_lib::run();
    Ok(())
}
