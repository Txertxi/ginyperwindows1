// Evita la finestra de consola a Windows en la versió final.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_window_state::{AppHandleExt, Builder as WindowState, StateFlags};

fn main() {
    tauri::Builder::default()
        // Només recorda la POSICIÓ: la mida l'ajusta la pàgina al contingut.
        .plugin(
            WindowState::default()
                .with_state_flags(StateFlags::POSITION)
                .build(),
        )
        // Desa la posició cada cop que es mou la finestra, així no es perd
        // si Windows tanca el procés en apagar l'equip.
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Moved(_) = event {
                let _ = window.app_handle().save_window_state(StateFlags::POSITION);
            }
        })
        .run(tauri::generate_context!())
        .expect("error en executar el widget");
}
