#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            // 启动 monitor_clipboard 函数
            thread::spawn(|| {
                monitor_clipboard();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use copypasta::{ClipboardContext, ClipboardProvider};
use std::thread;
use std::time::Duration;

fn monitor_clipboard() {
    let mut ctx = ClipboardContext::new().unwrap();
    let mut last_content = String::new();
    loop {
        let content = ctx.get_contents().unwrap_or_default();
        if content != last_content {
            last_content = content.clone();
            // 触发同步逻辑（发送到其他设备）
            println!("Clipboard content: {}", content);
        }
        thread::sleep(Duration::from_millis(500));
    }
}
