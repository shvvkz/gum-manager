use tauri_plugin_global_shortcut::{Builder as ShortcutBuilder, ShortcutState};

mod commands;
mod models;
mod services;
mod shortcuts;
mod window;

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
