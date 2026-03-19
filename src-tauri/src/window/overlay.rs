use tauri::{PhysicalPosition, PhysicalSize, Position, Size, Window};

pub fn set_overlay(window: Window, enabled: bool) {
    if enabled {
        let _ = window.set_decorations(false);
        let _ = window.set_always_on_top(true);
        let _ = window.set_resizable(false);

        let _ = window.set_size(Size::Physical(PhysicalSize {
            width: 440,
            height: 120,
        }));
    } else {
        let _ = window.set_always_on_top(false);
        let _ = window.set_decorations(true);

        let _ = window.set_size(Size::Physical(PhysicalSize {
            width: 950,
            height: 650,
        }));

        if let Ok(Some(monitor)) = window.current_monitor() {
            let size = monitor.size();
            let win_size = window.outer_size().unwrap();

            let x = ((size.width - win_size.width) / 2) as i32;
            let y = ((size.height - win_size.height) / 2) as i32;

            let _ = window.set_position(Position::Physical(PhysicalPosition { x, y }));
        }
    }

    let _ = window.set_maximizable(false);
    let _ = window.set_minimizable(false);
}
