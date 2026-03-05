// Note: Launch as a GUI app on Windows (no attached console window).
// FR: Lance l application en mode GUI sur Windows (sans fenetre console attachee).
#![windows_subsystem = "windows"]

mod app;
mod constants;
mod cpu;
mod display;
mod fonts;
mod gamepad;
mod i18n;
mod keypad;
mod memory;
mod types;
mod ui;
mod utils;

use eframe::egui;

use crate::app::Nova;
use crate::fonts::setup_custom_fonts;
use crate::utils::fenetre_size;

// Note: Acquire a process-wide named mutex to enforce single instance mode.
// FR: Acquiert un mutex nomme global au processus pour imposer une seule instance.
#[cfg(target_os = "windows")]
fn single_instance_guard() -> Option<windows_sys::Win32::Foundation::HANDLE> {
    use windows_sys::Win32::Foundation::{ERROR_ALREADY_EXISTS, GetLastError, HANDLE};
    use windows_sys::Win32::System::Threading::CreateMutexW;

    let name: Vec<u16> = "Local\\NovaSingleInstanceMutex"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    // Note: Keep this mutex handle alive for the whole process lifetime.
    // FR: Conserve ce handle de mutex vivant pendant toute la duree du processus.
    let handle: HANDLE = unsafe { CreateMutexW(std::ptr::null(), 0, name.as_ptr()) };
    if handle.is_null() {
        return None;
    }
    let already_running = unsafe { GetLastError() } == ERROR_ALREADY_EXISTS;
    if already_running {
        None
    } else {
        Some(handle)
    }
}

// Note: Application entry point.
// FR: Point d entree de l application.
fn main() {
    #[cfg(target_os = "windows")]
    let _single_instance = match single_instance_guard() {
        Some(handle) => handle,
        None => return,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(fenetre_size(2))
            .with_resizable(false),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Nova",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            let app: Nova = if let Some(storage) = cc.storage {
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
            } else {
                Nova::default()
            };
            Ok(Box::new(app))
        }),
    );
}
