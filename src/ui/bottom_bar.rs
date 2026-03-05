use eframe::egui;
use eframe::egui::Theme;

use crate::app::Nova;
use crate::constants::VERSION;
use crate::i18n::tr;

// Note: Render bottom status bar (version, ROM state, runtime state, CPU/FPS, sound, status text).
// FR: Affiche la barre de statut du bas (version, etat ROM, etat runtime, CPU/FPS, son, statut).
pub fn show(ctx: &egui::Context, app: &mut Nova) {
    let t = tr(app.langue);
    let text_color = if app.theme == Theme::Dark {
        egui::Color32::from_gray(235)
    } else {
        egui::Color32::from_gray(20)
    };
    egui::TopBottomPanel::bottom("barre_bas").show(ctx, |ui| {
        ui.visuals_mut().override_text_color = Some(text_color);
        ui.horizontal(|ui| {
            ui.label(format!("Nova v{}", VERSION));
            ui.separator();
            ui.label(if app.rom_chargee {
                &t.bottom_rom_loaded
            } else {
                &t.bottom_rom_none
            });
            ui.separator();
            ui.label(if app.en_pause {
                &t.bottom_state_paused
            } else {
                &t.bottom_state_running
            });
            ui.separator();
            ui.label(format!("{}: {}", t.bottom_cpu_hz, app.cycles_par_seconde));
            ui.separator();
            if app.configured_game_fps.is_finite() {
                ui.label(format!("FPS : {:.0}", app.configured_game_fps));
            } else {
                ui.label("FPS : illimite");
            }
            ui.separator();
            ui.label(format!("{}: {}", t.settings_sound, app.sound_volume));
            ui.add(egui::Slider::new(&mut app.sound_volume, 0..=100).show_value(false));

            if !app.status_message.is_empty() {
                ui.separator();
                ui.label(&app.status_message);
            }
        });
    });
}
