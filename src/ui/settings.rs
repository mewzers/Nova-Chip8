use eframe::egui::{self, Theme};

use crate::app::Nova;
use crate::i18n::tr;
use crate::types::{Langue, OngletsSettings};

// Note: Render and manage the detached settings viewport.
// FR: Affiche et gere la viewport detachee des parametres.
pub fn show(ctx: &egui::Context, app: &mut Nova) {
    let t = tr(app.langue);
    let text_color = if app.theme == Theme::Dark {
        egui::Color32::from_gray(235)
    } else {
        egui::Color32::from_gray(20)
    };
    let center_x = app.fenetre_principale_pos.x + app.fenetre_principale_size.x / 2.0;
    let center_y = app.fenetre_principale_pos.y + app.fenetre_principale_size.y / 2.0;

    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("settings_window"),
        egui::ViewportBuilder::default()
            .with_title(format!("⚙ {}", t.settings_title))
            .with_inner_size([560.0, 600.0])
            .with_position([center_x - 280.0, center_y - 300.0]),
        |ctx, _| {
            if app.focus_settings_requested {
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                app.focus_settings_requested = false;
            }

            ctx.style_mut(|style| {
                style.wrap_mode = Some(egui::TextWrapMode::Extend);
                style.visuals.override_text_color = Some(text_color);
            });

            egui::TopBottomPanel::bottom("settings_boutons").show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if ui.button(format!("✅ {}", t.ok)).clicked() {
                        app.apply_temp_values();
                        app.fenetre_settings = false;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button(format!("💾 {}", t.apply)).clicked() {
                        app.apply_temp_values();
                    }
                    if ui.button(format!("🔄 {}", t.defaults)).clicked() {
                        app.reset_current_settings_tab_to_default();
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(format!("❌ {}", t.cancel)).clicked() {
                            app.restore_snapshots();
                            app.fenetre_settings = false;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
                ui.add_space(4.0);
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut app.onglet_settings,
                        OngletsSettings::Emulateur,
                        format!("⚙ {}", t.tab_emulator),
                    );
                    ui.selectable_value(
                        &mut app.onglet_settings,
                        OngletsSettings::Video,
                        format!("🎨 {}", t.tab_video),
                    );
                    ui.selectable_value(
                        &mut app.onglet_settings,
                        OngletsSettings::Controles,
                        format!("🎮 {}", t.tab_controls),
                    );
                    ui.selectable_value(
                        &mut app.onglet_settings,
                        OngletsSettings::Raccourcis,
                        format!("⌨ {}", t.tab_shortcuts),
                    );
                    ui.selectable_value(
                        &mut app.onglet_settings,
                        OngletsSettings::Debug,
                        format!("🐞 {}", t.tab_debug),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let mut dark_mode = app.theme == Theme::Dark;
                        let mode_icon = if dark_mode { "🌙" } else { "☀" };
                        if ui
                            .toggle_value(&mut dark_mode, mode_icon)
                            .changed()
                        {
                            app.theme = if dark_mode { Theme::Dark } else { Theme::Light };
                        }
                        ui.label(&t.theme);
                    });
                });

                ui.separator();

                match app.onglet_settings {
                    OngletsSettings::Emulateur => show_emulateur_settings(ui, app, t),
                    OngletsSettings::Video => show_video_settings(ui, app, t),
                    OngletsSettings::Controles => show_controles_settings(ui, app, t),
                    OngletsSettings::Raccourcis => show_raccourcis_settings(ui, app, t),
                    OngletsSettings::Debug => show_debug_settings(ui, app, t),
                }
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                app.fenetre_settings = false;
            }
        },
    );
}

fn show_emulateur_settings(ui: &mut egui::Ui, app: &mut Nova, t: &crate::i18n::Translations) {
    // Note: Emulation settings section (language, CPU speed, sound toggle).
    // FR: Section des parametres d emulation (langue, vitesse CPU, activation du son).
    ui.label(&t.language);

    let before = app.temp_langue;
    egui::ComboBox::from_id_salt("langue_settings")
        .selected_text(app.temp_langue.label())
        .show_ui(ui, |ui| {
            for language in Langue::ALL {
                ui.selectable_value(&mut app.temp_langue, language, language.label());
            }
        });
    if app.temp_langue != before {
        // Note: Apply live preview in settings; Cancel restores snapshot_langue.
        // FR: Applique un apercu en direct dans les parametres; Annuler restaure snapshot_langue.
        app.langue = app.temp_langue;
    }

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label(&t.settings_cpu_hz);
        ui.add(egui::Slider::new(&mut app.temp_cycles_par_seconde, 60..=2000));
    });
    ui.checkbox(&mut app.temp_son_active, &t.settings_sound);
}

fn show_video_settings(ui: &mut egui::Ui, app: &mut Nova, t: &crate::i18n::Translations) {
    // Note: Video settings section (vsync, aspect and scale).
    // FR: Section des parametres video (vsync, ratio et echelle).
    ui.label(&t.video_config);
    ui.checkbox(&mut app.temp_vsync, &t.vsync);
    ui.checkbox(&mut app.temp_ecran_large, &t.widescreen);
    ui.label(&t.render_size);
    egui::ComboBox::from_id_salt("video_scale")
        .selected_text(format!("{}x", app.video_scale))
        .show_ui(ui, |ui| {
            for scale in [1u8, 2, 3, 4, 5] {
                ui.selectable_value(&mut app.video_scale, scale, format!("{}x", scale));
            }
        });
}

fn show_controles_settings(ui: &mut egui::Ui, app: &mut Nova, t: &crate::i18n::Translations) {
    // Note: Human-friendly labels for the CHIP-8 keypad visual layout.
    // FR: Libelles lisibles pour la disposition visuelle du clavier CHIP-8.
    fn chip8_user_label(index: usize) -> &'static str {
        match index {
            0x1 => "Haut gauche",
            0x2 => "Haut centre",
            0x3 => "Haut droite",
            0xC => "Haut action",
            0x4 => "Milieu haut gauche",
            0x5 => "Centre",
            0x6 => "Milieu haut droite",
            0xD => "Milieu action",
            0x7 => "Milieu bas gauche",
            0x8 => "Bas centre",
            0x9 => "Milieu bas droite",
            0xE => "Bas action",
            0xA => "Bas gauche",
            0x0 => "Bas milieu",
            0xB => "Bas droite",
            0xF => "Action finale",
            _ => "Touche",
        }
    }

    const CHIP8_VISUAL_ORDER: [usize; 16] = [
        0x1, 0x2, 0x3, 0xC, 0x4, 0x5, 0x6, 0xD, 0x7, 0x8, 0x9, 0xE, 0xA, 0x0, 0xB, 0xF,
    ];

    ui.label(&t.key_config);
    for i in CHIP8_VISUAL_ORDER {
        ui.horizontal(|ui| {
            ui.label(format!("{} (CHIP-8 0x{:X}):", chip8_user_label(i), i));
            ui.text_edit_singleline(&mut app.temp_touches[i]);
        });
    }
}

fn show_raccourcis_settings(ui: &mut egui::Ui, app: &mut Nova, t: &crate::i18n::Translations) {
    // Note: Keyboard shortcut mapping section.
    // FR: Section de mapping des raccourcis clavier.
    ui.label(&t.keyboard_shortcuts);
    let raccourcis = &mut app.temp_raccourcis;
    egui::Grid::new("raccourcis_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .show(ui, |ui| {
            ui.label(&t.shortcut_pause);
            ui.text_edit_singleline(&mut raccourcis.pause);
            ui.end_row();
            ui.label(&t.shortcut_reset);
            ui.text_edit_singleline(&mut raccourcis.reset);
            ui.end_row();
            ui.label(&t.shortcut_stop);
            ui.text_edit_singleline(&mut raccourcis.stop);
            ui.end_row();
            ui.label(&t.shortcut_load_game);
            ui.text_edit_singleline(&mut raccourcis.charger_jeu);
            ui.end_row();
            ui.label(&t.shortcut_fullscreen);
            ui.text_edit_singleline(&mut raccourcis.fullscreen);
            ui.end_row();
            ui.label(&t.shortcut_save_slot_1);
            ui.text_edit_singleline(&mut raccourcis.savestate_1);
            ui.end_row();
            ui.label(&t.shortcut_save_slot_2);
            ui.text_edit_singleline(&mut raccourcis.savestate_2);
            ui.end_row();
            ui.label(&t.shortcut_save_slot_3);
            ui.text_edit_singleline(&mut raccourcis.savestate_3);
            ui.end_row();
            ui.label(&t.shortcut_load_slot_1);
            ui.text_edit_singleline(&mut raccourcis.loadstate_1);
            ui.end_row();
            ui.label(&t.shortcut_load_slot_2);
            ui.text_edit_singleline(&mut raccourcis.loadstate_2);
            ui.end_row();
            ui.label(&t.shortcut_load_slot_3);
            ui.text_edit_singleline(&mut raccourcis.loadstate_3);
            ui.end_row();
        });
}

fn show_debug_settings(ui: &mut egui::Ui, app: &mut Nova, t: &crate::i18n::Translations) {
    // Note: Debug-specific settings section.
    // FR: Section des parametres specifiques au debug.
    ui.checkbox(&mut app.temp_terminal_active, &t.settings_debug_terminal);
}
