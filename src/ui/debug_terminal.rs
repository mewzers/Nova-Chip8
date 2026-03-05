use eframe::egui;
use eframe::egui::text::LayoutJob;
use eframe::egui::RichText;
use std::sync::Arc;

use crate::app::Nova;
use crate::i18n::tr;
use crate::utils::key_from_label;

const CONSOLE_FONT_SIZE: f32 = 14.0;

// Note: Render detached debug terminal viewport and synchronize inputs/focus with main app.
// FR: Affiche la viewport detachee du terminal de debug et synchronise les entrees/focus avec l application principale.
pub fn show(ctx: &egui::Context, app: &mut Nova) {
    fn shortcut_pressed_in_terminal(input: &egui::InputState, label: &str) -> bool {
        if let Some(key) = key_from_label(label) {
            return input.key_pressed(key);
        }
        false
    }

    let terminal_w = 520.0;
    let terminal_h = 420.0;
    let margin = 16.0;
    let terminal_x = app.fenetre_principale_pos.x + app.fenetre_principale_size.x + margin;
    let terminal_y = app.fenetre_principale_pos.y;

    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("debug_terminal_window"),
        egui::ViewportBuilder::default()
            .with_title(&tr(app.langue).terminal_title)
            .with_icon(terminal_window_icon())
            .with_inner_size([terminal_w, terminal_h])
            .with_resizable(true),
        |ctx, _| {
            let t = tr(app.langue);
            let (bg_color, text_color) = match app.theme {
                egui::Theme::Light => (ctx.style().visuals.window_fill(), egui::Color32::BLACK),
                egui::Theme::Dark => (egui::Color32::BLACK, egui::Color32::from_gray(220)),
            };
            let mut terminal_keys = [false; 16];
            ctx.input(|i| {
                for (idx, key_name) in app.touches.iter().enumerate() {
                    if let Some(key) = key_from_label(key_name) {
                        terminal_keys[idx] = i.key_down(key);
                    }
                }
            });
            app.terminal_keypad_states = terminal_keys;
            let (
                sc_pause,
                sc_reset,
                sc_stop,
                sc_load_game,
                sc_fullscreen,
                sc_save_1,
                sc_save_2,
                sc_save_3,
                sc_load_1,
                sc_load_2,
                sc_load_3,
                sc_alt_enter,
            ) = {
                let r = &app.raccourcis;
                ctx.input(|i| {
                    (
                        shortcut_pressed_in_terminal(i, &r.pause),
                        shortcut_pressed_in_terminal(i, &r.reset),
                        shortcut_pressed_in_terminal(i, &r.stop),
                        shortcut_pressed_in_terminal(i, &r.charger_jeu),
                        shortcut_pressed_in_terminal(i, &r.fullscreen),
                        shortcut_pressed_in_terminal(i, &r.savestate_1),
                        shortcut_pressed_in_terminal(i, &r.savestate_2),
                        shortcut_pressed_in_terminal(i, &r.savestate_3),
                        shortcut_pressed_in_terminal(i, &r.loadstate_1),
                        shortcut_pressed_in_terminal(i, &r.loadstate_2),
                        shortcut_pressed_in_terminal(i, &r.loadstate_3),
                        i.key_pressed(egui::Key::Enter) && i.modifiers.alt,
                    )
                })
            };
            if sc_pause {
                app.en_pause = !app.en_pause;
            }
            if sc_reset {
                app.reset_rom();
            }
            if sc_stop {
                app.stop_emulation();
            }
            if sc_load_game {
                app.load_rom_dialog();
            }
            if sc_fullscreen || sc_alt_enter {
                app.fullscreen = !app.fullscreen;
            }
            if sc_save_1 {
                app.save_state_slot(0);
            }
            if sc_save_2 {
                app.save_state_slot(1);
            }
            if sc_save_3 {
                app.save_state_slot(2);
            }
            if sc_load_1 {
                app.load_state_slot(0);
            }
            if sc_load_2 {
                app.load_state_slot(1);
            }
            if sc_load_3 {
                app.load_state_slot(2);
            }

            if !app.terminal_position_initialized {
                ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(
                    terminal_x,
                    terminal_y,
                )));
                app.terminal_position_initialized = true;
            }
            if app.focus_terminal_requested {
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                app.focus_terminal_requested = false;
            }
            if ctx.input(|i| i.pointer.any_pressed()) {
                app.focus_main_requested = true;
            }

            egui::TopBottomPanel::top("terminal_header").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(if app.configured_game_fps.is_finite() {
                            format!("{} : {:.0}", t.terminal_fps, app.configured_game_fps)
                        } else {
                            format!("{} : {}", t.terminal_fps, t.terminal_unlimited)
                        })
                                .monospace()
                                .size(CONSOLE_FONT_SIZE)
                                .color(text_color),
                        );
                });
            });

            egui::CentralPanel::default()
                .frame(
                    egui::Frame::default()
                        .fill(bg_color)
                        .inner_margin(egui::Margin::symmetric(10, 6)),
                )
                .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        for line in &app.terminal_logs {
                            render_log_line(
                                ui,
                                line,
                                text_color,
                                &t.terminal_enabled,
                                &t.terminal_disabled,
                            );
                        }
                    });
                });

            if ctx.input(|i| i.viewport().close_requested()) {
                app.terminal_active = false;
                app.temp_terminal_active = false;
                app.terminal_keypad_states = [false; 16];
            }
        },
    );
}

// Note: Render one terminal line with conditional coloring for enabled/disabled words.
// FR: Affiche une ligne du terminal avec coloration conditionnelle des mots actif/inactif.
fn render_log_line(
    ui: &mut egui::Ui,
    line: &str,
    normal: egui::Color32,
    active_word: &str,
    inactive_word: &str,
) {
    let active_color = egui::Color32::from_rgb(70, 210, 90);
    let inactive_color = egui::Color32::from_rgb(220, 70, 70);

    let mut push_line = |before: &str, word: &str, word_color: egui::Color32, after: &str| {
        let mut job = LayoutJob::default();
        if !before.is_empty() {
            job.append(
                before,
                0.0,
                egui::TextFormat {
                    font_id: egui::FontId::monospace(CONSOLE_FONT_SIZE),
                    color: normal,
                    ..Default::default()
                },
            );
        }
        job.append(
            word,
            0.0,
            egui::TextFormat {
                font_id: egui::FontId::monospace(CONSOLE_FONT_SIZE),
                color: word_color,
                ..Default::default()
            },
        );
        if !after.is_empty() {
            job.append(
                after,
                0.0,
                egui::TextFormat {
                    font_id: egui::FontId::monospace(CONSOLE_FONT_SIZE),
                    color: normal,
                    ..Default::default()
                },
            );
        }
        ui.label(job);
    };

    if let Some(idx) = line.find(inactive_word) {
        let before = &line[..idx];
        let after = &line[idx + inactive_word.len()..];
        push_line(before, inactive_word, inactive_color, after);
        return;
    }

    if let Some(idx) = line.find(active_word) {
        let before = &line[..idx];
        let after = &line[idx + active_word.len()..];
        push_line(before, active_word, active_color, after);
        return;
    }

    let mut job = LayoutJob::default();
    job.append(
        line,
        0.0,
        egui::TextFormat {
            font_id: egui::FontId::monospace(CONSOLE_FONT_SIZE),
            color: normal,
            ..Default::default()
        },
    );
    ui.label(job);
}

// Note: Build a tiny terminal-like icon for the debug viewport.
// FR: Construit une petite icone type terminal pour la viewport de debug.
fn terminal_window_icon() -> Arc<egui::IconData> {
    const W: u32 = 32;
    const H: u32 = 32;
    let mut rgba = vec![0u8; (W * H * 4) as usize];

    for y in 0..H {
        for x in 0..W {
            let i = ((y * W + x) * 4) as usize;
            let border = x == 0 || y == 0 || x == W - 1 || y == H - 1;
            let header = y < 6;
            let bg = if border {
                [30, 30, 30, 255]
            } else if header {
                [50, 50, 50, 255]
            } else {
                [12, 12, 12, 255]
            };
            rgba[i..i + 4].copy_from_slice(&bg);
        }
    }

    // Note: Draw a minimal prompt glyph (>_) in the terminal icon.
    // FR: Dessine un glyphe d invite minimal (>_) dans l icone du terminal.
    for (x, y) in [(8, 14), (10, 15), (8, 16), (11, 15), (12, 15), (13, 15), (14, 15)] {
        let i = (((y as u32) * W + (x as u32)) * 4) as usize;
        rgba[i..i + 4].copy_from_slice(&[120, 220, 120, 255]);
    }

    Arc::new(egui::IconData { rgba, width: W, height: H })
}
