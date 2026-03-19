use tauri_plugin_global_shortcut::{Builder as ShortcutBuilder, ShortcutState};

mod commands;
mod models;
mod services;
mod shortcuts;
mod window;
mod init;
mod update;

use shortcuts::handler::handle_shortcut;

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
                let remote_version = init::get_remote_gum_version().await;
                let local_version = init::get_local_gum_version(&app_handle);
                if local_version == "unknown" {
                    // update::update_gums(&app_handle, &remote_version).await;
                }
                println!("Remote version: {}, Local version: {}", remote_version, local_version);
                let local_version_split: Vec<&str> = local_version.split('.').collect();
                let remote_version_split: Vec<&str> = remote_version.split('.').collect();
                // if remote_version != local_version {
                //     update::update_gums(&app_handle, &remote_version).await;
                // }
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
