use eframe::egui::{self, Theme};

use crate::app::Nova;
use crate::i18n::tr;
use crate::types::{Langue, OngletsSettings};

// Note: Replace regular spaces with non-breaking spaces for stable menu labels.
// FR: Remplace les espaces classiques par des espaces inseparables pour stabiliser les libelles de menu.
fn nw(text: &str) -> String {
    text.replace(' ', "\u{00A0}")
}

// Note: Compute menu width from the widest label.
// FR: Calcule la largeur du menu a partir du libelle le plus large.
fn menu_width(ui: &egui::Ui, labels: &[String]) -> f32 {
    let font_id = egui::TextStyle::Button.resolve(ui.style());
    let color = ui.visuals().text_color();
    let max_text_width = labels
        .iter()
        .map(|label| {
            ui.painter()
                .layout_no_wrap(label.clone(), font_id.clone(), color)
                .size()
                .x
        })
        .fold(0.0_f32, f32::max);
    let padding = ui.spacing().button_padding.x * 2.0;
    (max_text_width + padding).max(1.0)
}

// Note: Compute menu width including icon/checkmark reserved space.
// FR: Calcule la largeur du menu en incluant l espace reserve aux icones/coches.
fn menu_width_with_extras(
    ui: &egui::Ui,
    labels: &[String],
    has_leading_icon: bool,
    has_trailing_icon: bool,
) -> f32 {
    let mut width = menu_width(ui, labels);
    if has_leading_icon {
        width += ui.spacing().icon_width + ui.spacing().icon_spacing;
    }
    if has_trailing_icon {
        width += ui.spacing().icon_width + ui.spacing().item_spacing.x;
    }
    width
}

// Note: Force a fixed width for consistent dropdown layout.
// FR: Force une largeur fixe pour une mise en page cohérente du menu deroulant.
fn set_menu_width(ui: &mut egui::Ui, width: f32) {
    ui.set_min_width(width);
    ui.set_max_width(width);
}

// Note: Render the top menu bar and wire actions to app state changes.
// FR: Affiche la barre de menu du haut et relie les actions aux changements d etat.
pub fn show(ctx: &egui::Context, app: &mut Nova) {
    let t = tr(app.langue);
    let text_color = if app.theme == Theme::Dark {
        egui::Color32::from_gray(235)
    } else {
        egui::Color32::from_gray(20)
    };

    egui::TopBottomPanel::top("barre").show(ctx, |ui| {
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
        ui.visuals_mut().override_text_color = Some(text_color);

        let game_w_plain = menu_width(ui, &[
            format!("📂 {}", nw(&t.load_game)),
            format!("🕘 {}", nw(&t.load_recent_rom)),
            format!("▶ {}", nw(&t.resume)),
            format!("⏸ {}", nw(&t.pause)),
            format!("🔄 {}", nw(&t.reset)),
            format!("⏹ {}", nw(&t.stop)),
            format!("💾 {}", nw(&t.save_state)),
            format!("📁 {}", nw(&t.load_state)),
        ]);
        let game_w_checkbox = menu_width_with_extras(ui, &[nw(&t.fps_limiter)], true, false);
        let game_w_submenu = menu_width_with_extras(
            ui,
            &[format!("💾 {}", nw(&t.save_state)), format!("📁 {}", nw(&t.load_state))],
            false,
            true,
        );
        let game_w = game_w_plain.max(game_w_checkbox).max(game_w_submenu);
        let state_w = menu_width(ui, &[nw(&t.slot_1), nw(&t.slot_2), nw(&t.slot_3)]);
        let emulator_w = menu_width(ui, &[nw(&t.emulator_settings), nw(&t.languages)]);
        let language_w = menu_width(
            ui,
            &Langue::ALL
                .iter()
                .map(|l| nw(l.label()))
                .collect::<Vec<String>>(),
        );
        let video_w_plain = menu_width(ui, &[
            nw(&t.video_settings),
            nw(&t.render_size),
        ]);
        let video_w_checkbox = menu_width_with_extras(
            ui,
            &[nw(&t.vsync), nw(&t.widescreen), nw(&t.fullscreen)],
            true,
            false,
        );
        let video_w_submenu =
            menu_width_with_extras(ui, &[nw(&t.render_size)], false, true);
        let video_w = video_w_plain.max(video_w_checkbox).max(video_w_submenu);
        let render_w = menu_width(
            ui,
            &[
                "1x".to_owned(),
                "2x".to_owned(),
                "3x".to_owned(),
                "4x".to_owned(),
                "5x".to_owned(),
            ],
        );
        let debug_w_plain = menu_width(
            ui,
            &[nw(&t.debug_settings)],
        );
        let debug_w_checkbox =
            menu_width_with_extras(ui, &[nw(&t.settings_debug_terminal)], true, false);
        let debug_w = debug_w_plain.max(debug_w_checkbox);

        ui.horizontal(|ui| {
            ui.menu_button(format!("📺 {}", nw(&t.game)), |ui| {
                set_menu_width(ui, game_w);
                if ui.button(format!("📂 {}", nw(&t.load_game))).clicked() { app.load_rom_dialog(); ui.close(); }
                if ui
                    .add_enabled(app.last_rom_path.is_some(), egui::Button::new(format!("🕘 {}", nw(&t.load_recent_rom))))
                    .clicked()
                {
                    app.load_recent_rom();
                    ui.close();
                }
                ui.separator();
                if ui
                    .button(if app.en_pause {
                        format!("▶ {}", nw(&t.resume))
                    } else {
                        format!("⏸ {}", nw(&t.pause))
                    })
                    .clicked()
                {
                    app.en_pause = !app.en_pause;
                    ui.close();
                }
                if ui.button(format!("🔄 {}", nw(&t.reset))).clicked() { app.reset_rom(); ui.close(); }
                if ui.button(format!("⏹ {}", nw(&t.stop))).clicked() { app.stop_emulation(); ui.close(); }
                ui.separator();
                ui.menu_button(format!("💾 {}", nw(&t.save_state)), |ui| {
                    set_menu_width(ui, state_w);
                    if ui.button(nw(&t.slot_1)).clicked() { app.save_state_slot(0); ui.close(); }
                    if ui.button(nw(&t.slot_2)).clicked() { app.save_state_slot(1); ui.close(); }
                    if ui.button(nw(&t.slot_3)).clicked() { app.save_state_slot(2); ui.close(); }
                });
                ui.menu_button(format!("📁 {}", nw(&t.load_state)), |ui| {
                    set_menu_width(ui, state_w);
                    if ui.button(nw(&t.slot_1)).clicked() { app.load_state_slot(0); ui.close(); }
                    if ui.button(nw(&t.slot_2)).clicked() { app.load_state_slot(1); ui.close(); }
                    if ui.button(nw(&t.slot_3)).clicked() { app.load_state_slot(2); ui.close(); }
                });
                ui.separator();
                ui.checkbox(&mut app.vsync, nw(&t.fps_limiter));
            });

            ui.menu_button(format!("⚙ {}", nw(&t.emulator)), |ui| {
                set_menu_width(ui, emulator_w);
                if ui.button(nw(&t.emulator_settings)).clicked() {
                    app.snapshot_langue = app.langue;
                    app.temp_langue = app.langue;
                    app.snapshot_cycles_par_seconde = app.cycles_par_seconde;
                    app.snapshot_son_active = app.son_active;
                    app.snapshot_terminal_active = app.terminal_active;
                    app.temp_cycles_par_seconde = app.cycles_par_seconde;
                    app.temp_son_active = app.son_active;
                    app.temp_terminal_active = app.terminal_active;
                    app.onglet_settings = OngletsSettings::Emulateur;
                    app.fenetre_settings = true;
                    app.focus_settings_requested = true;
                    ui.close();
                }
                ui.separator();
                ui.menu_button(nw(&t.languages), |ui| {
                    set_menu_width(ui, language_w);
                    for language in Langue::ALL {
                        show_language_item(ui, app, language);
                    }
                });
            });

            ui.menu_button(format!("🎨 {}", nw(&t.video)), |ui| {
                set_menu_width(ui, video_w);
                if ui.button(nw(&t.video_settings)).clicked() {
                    app.snapshot_vsync = app.vsync;
                    app.snapshot_ecran_large = app.ecran_large;
                    app.snapshot_terminal_active = app.terminal_active;
                    app.temp_vsync = app.vsync;
                    app.temp_ecran_large = app.ecran_large;
                    app.temp_terminal_active = app.terminal_active;
                    app.onglet_settings = OngletsSettings::Video;
                    app.fenetre_settings = true;
                    app.focus_settings_requested = true;
                    ui.close();
                }
                ui.separator();
                ui.menu_button(nw(&t.render_size), |ui| {
                    set_menu_width(ui, render_w);
                    for scale in [1u8, 2, 3, 4, 5] {
                        if ui
                            .selectable_label(app.video_scale == scale, format!("{}x", scale))
                            .clicked()
                        {
                            app.video_scale = scale;
                            ui.close();
                        }
                    }
                });
                ui.checkbox(&mut app.vsync, nw(&t.vsync));
                ui.checkbox(&mut app.ecran_large, nw(&t.widescreen));
                ui.checkbox(&mut app.fullscreen, nw(&t.fullscreen));
            });

            if ui.button(format!("🎮 {}", nw(&t.controls))).clicked() {
                app.snapshot_touches = app.touches.clone();
                app.snapshot_raccourcis = app.raccourcis.clone();
                app.snapshot_terminal_active = app.terminal_active;
                app.temp_touches = app.touches.clone();
                app.temp_raccourcis = app.raccourcis.clone();
                app.temp_terminal_active = app.terminal_active;
                app.onglet_settings = OngletsSettings::Controles;
                app.fenetre_settings = true;
                app.focus_settings_requested = true;
            }

            if ui.button(format!("⌨ {}", nw(&t.shortcuts))).clicked() {
                app.snapshot_raccourcis = app.raccourcis.clone();
                app.snapshot_terminal_active = app.terminal_active;
                app.temp_raccourcis = app.raccourcis.clone();
                app.temp_terminal_active = app.terminal_active;
                app.onglet_settings = OngletsSettings::Raccourcis;
                app.fenetre_settings = true;
                app.focus_settings_requested = true;
            }

            ui.menu_button(format!("🐞 {}", nw(&t.tab_debug)), |ui| {
                set_menu_width(ui, debug_w);
                if ui.button(nw(&t.debug_settings)).clicked() {
                    app.snapshot_terminal_active = app.terminal_active;
                    app.temp_terminal_active = app.terminal_active;
                    app.onglet_settings = OngletsSettings::Debug;
                    app.fenetre_settings = true;
                    app.focus_settings_requested = true;
                    ui.close();
                }
                ui.separator();
                let resp = ui.checkbox(&mut app.terminal_active, nw(&t.settings_debug_terminal));
                if resp.changed() && app.terminal_active {
                    app.focus_terminal_requested = true;
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut dark_mode = app.theme == Theme::Dark;
                let mode_icon = if dark_mode { "🌙" } else { "☀" };
                if ui
                    .toggle_value(&mut dark_mode, mode_icon)
                    .changed()
                {
                    app.theme = if dark_mode { Theme::Dark } else { Theme::Light };
                }
                ui.label(nw(&t.theme));
            });
        });
    });
}

// Note: Render one language selection item.
// FR: Affiche un element de selection de langue.
fn show_language_item(ui: &mut egui::Ui, app: &mut Nova, language: Langue) {
    let selected = app.langue == language;
    let label = language.label();
    if ui.selectable_label(selected, nw(label)).clicked() {
        app.langue = language;
        ui.close();
    }
}


