use tauri_plugin_global_shortcut::{Builder as ShortcutBuilder, ShortcutState};

mod commands;
mod init;
mod models;
mod services;
mod shortcuts;
mod update;
mod window;

use shortcuts::handler::handle_shortcut;

pub fn restart() {
    let exe = std::env::current_exe().expect("Failed to get current exe");

    std::process::Command::new(exe)
        .spawn()
        .expect("Failed to restart app");

    std::process::exit(0);
}

pub fn run() {
    tauri::Builder::default()
        .plugin(
            ShortcutBuilder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }

                    handle_shortcut(app, shortcut);
                })
                .build(),
        )
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let mut need_restart = false;
                let remote_gums = init::get_remote_gum().await;
                let local_gums = init::get_local_gum(&app_handle);
                let remote_version = init::get_gum_version(remote_gums.clone());
                let local_version = init::get_gum_version(local_gums.clone());
                if !init::pack_file_exists(&app_handle) {
                    println!("Pack file does not exist, need to create it");
                    init::create_pack_file(&app_handle).expect("Failed to create pack file");
                    need_restart = true;
                }

                if local_version == "unknown" {
                    println!("local version unknown, file must be missing or corrupted, updating gums...");
                    init::write_gums(&app_handle, remote_gums).expect("Failed to write gums");
                    need_restart = true;
                } else {
                    let local_version_split: Vec<&str> = local_version.split('.').collect();
                    let remote_version_split: Vec<&str> = remote_version.split('.').collect();
                    if remote_version_split[0] > local_version_split[0]
                        || remote_version_split[1] > local_version_split[1]
                        || remote_version_split[2] > local_version_split[2]
                    {
                        println!("New version available, updating gums...");
                        init::write_gums(&app_handle, remote_gums).expect("Failed to write gums");
                        need_restart = true;
                    }
                }
                if need_restart {
                    restart();
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::gum_commands::get_gums,
            commands::pack_commands::get_pack,
            commands::pack_commands::set_gum_in_pack,
            commands::window_commands::set_overlay,
            commands::window_commands::start_drag,
            commands::shortcut_commands::register_shortcut,
            commands::shortcut_commands::unregister_shortcut,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
