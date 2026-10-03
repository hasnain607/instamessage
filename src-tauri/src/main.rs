#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Url, WebviewUrl, WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_opener::OpenerExt;

const INBOX_URL: &str = "https://www.instagram.com/direct/inbox/";
/// Page zoom. Lower it to shrink the UI further, raise it to enlarge.
const ZOOM: f64 = 0.85;
const ALLOWED_DOMAINS: [&str; 4] = [
    "instagram.com",
    "facebook.com",
    "cdninstagram.com",
    "fbcdn.net",
];

fn is_allowed(url: &Url) -> bool {
    if !matches!(url.scheme(), "http" | "https") {
        return true;
    }
    let host = url.host_str().unwrap_or_default();
    ALLOWED_DOMAINS
        .iter()
        .any(|d| host == *d || host.ends_with(&format!(".{d}")))
}

fn show_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

fn toggle_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let visible = w.is_visible().unwrap_or(false);
        let minimized = w.is_minimized().unwrap_or(false);
        if visible && !minimized {
            let _ = w.hide();
        } else {
            show_main(app);
        }
    }
}

fn main() {
    tauri::Builder::default()
        // must be registered first
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main(app);
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--hidden"]),
        ))
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let _ = app.autolaunch().enable();

            let hidden = std::env::args().any(|a| a == "--hidden");
            let handle = app.handle().clone();

            let window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External(INBOX_URL.parse::<Url>()?),
            )
            .title("Instagram DMs")
            .inner_size(1100.0, 750.0)
            .visible(!hidden)
            .initialization_script(include_str!("inject.js"))
            .on_navigation(move |url| {
                if is_allowed(url) {
                    true
                } else {
                    // external links open in the default browser
                    let _ = handle.opener().open_url(url.as_str(), None::<&str>);
                    false
                }
            })
            .build()?;
            let _ = window.set_zoom(ZOOM);

            let toggle = MenuItem::with_id(app, "toggle", "Show / Hide", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&toggle, &quit])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "toggle" => toggle_main(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        toggle_main(tray.app_handle());
                    }
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building app")
        .run(|_app, _event| {
            // macOS: clicking the Dock icon reopens the hidden window
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Reopen { .. } = _event {
                show_main(_app);
            }
        });
}
