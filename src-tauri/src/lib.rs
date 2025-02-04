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
            tauri::async_runtime::block_on(mdns_discover_devices());
            tauri::async_runtime::block_on(mdns_publish_service());
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

use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;

async fn mdns_discover_devices() {
    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Browse for a service type.
    let service_type = "_mdns-sd-my-test._udp.local.";
    let receiver = mdns.browse(service_type).expect("Failed to browse");

    // Receive the browse events in sync or async. Here is
    // an example of using a thread. Users can call `receiver.recv_async().await`
    // if running in async environment.
    thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    println!(
                        "Resolved a new service: {}, {}",
                        info.get_fullname(),
                        info.get_properties()
                    );
                }
                other_event => {
                    println!("Received other event: {:?}", &other_event);
                }
            }
        }
    });
}

async fn mdns_publish_service() {
    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Create a service info.
    let service_type = "_mdns-sd-my-test._udp.local.";
    let instance_name = "my_instance";
    let ip = "192.168.3.125";
    let host_name = "sunshow.local.";
    let port = 5200;
    let properties = [("property_1", "test"), ("property_2", "1234")];

    let my_service = ServiceInfo::new(
        service_type,
        instance_name,
        host_name,
        ip,
        port,
        &properties[..],
    )
    .unwrap();

    // Register with the daemon, which publishes the service.
    mdns.register(my_service)
        .expect("Failed to register our service");

    println!("Service registered.");

    // Gracefully shutdown the daemon
    // thread::sleep(Duration::from_secs(1));
    // mdns.shutdown().unwrap();
}
