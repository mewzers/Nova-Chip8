use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use eframe::egui::{self, Theme};

use crate::constants::VERSION;
use crate::cpu::CPU;
use crate::display::Display;
use crate::gamepad;
use crate::i18n::tr;
use crate::keypad::Keypad;
use crate::types::{Langue, OngletsSettings, Raccourcis};
use crate::ui;
use crate::utils::{default_touches, fenetre_size, key_from_label};

#[derive(Clone)]
pub struct EmuSnapshot {
    pub cpu: CPU,
    pub display: Display,
}

fn default_savestates() -> [Option<EmuSnapshot>; 3] {
    [None, None, None]
}

fn default_instant() -> Instant {
    Instant::now()
}

fn default_theme_dark() -> Theme {
    Theme::Dark
}

fn default_langue_fr() -> Langue {
    Langue::Francais
}

fn default_sound_volume() -> u8 {
    100
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Nova {
    pub(crate) theme: Theme,
    pub(crate) cpu: CPU,
    pub(crate) display: Display,
    pub(crate) langue: Langue,
    pub(crate) video_scale: u8,
    pub(crate) vsync: bool,
    pub(crate) ecran_large: bool,
    pub(crate) fullscreen: bool,
    pub(crate) en_pause: bool,
    pub(crate) rom_chargee: bool,
    pub(crate) touches: [String; 16],
    pub(crate) raccourcis: Raccourcis,
    pub(crate) fenetre_settings: bool,
    pub(crate) onglet_settings: OngletsSettings,
    pub(crate) temp_langue: Langue,
    pub(crate) temp_vsync: bool,
    pub(crate) temp_ecran_large: bool,
    pub(crate) temp_touches: [String; 16],
    pub(crate) temp_raccourcis: Raccourcis,
    pub(crate) cycles_par_seconde: u16,
    pub(crate) son_active: bool,
    #[serde(default = "default_sound_volume")]
    pub(crate) sound_volume: u8,
    pub(crate) temp_cycles_par_seconde: u16,
    pub(crate) temp_son_active: bool,
    pub(crate) temp_terminal_active: bool,
    pub(crate) snapshot_langue: Langue,
    pub(crate) snapshot_vsync: bool,
    pub(crate) snapshot_ecran_large: bool,
    pub(crate) snapshot_touches: [String; 16],
    pub(crate) snapshot_raccourcis: Raccourcis,
    pub(crate) snapshot_cycles_par_seconde: u16,
    pub(crate) snapshot_son_active: bool,
    pub(crate) snapshot_terminal_active: bool,
    pub(crate) terminal_active: bool,
    #[serde(skip, default)]
    pub(crate) status_message: String,
    #[serde(skip, default)]
    pub(crate) terminal_logs: Vec<String>,
    #[serde(skip, default)]
    pub(crate) last_status_logged: String,
    #[serde(skip, default)]
    pub(crate) focus_settings_requested: bool,
    #[serde(skip, default)]
    pub(crate) focus_terminal_requested: bool,
    #[serde(skip, default)]
    pub(crate) focus_main_requested: bool,
    #[serde(skip, default)]
    pub(crate) terminal_position_initialized: bool,
    #[serde(skip, default)]
    pub(crate) terminal_boot_logs_seeded: bool,
    #[serde(skip, default)]
    pub(crate) configured_game_fps: f32,
    #[serde(skip, default = "default_theme_dark")]
    pub(crate) last_logged_theme: Theme,
    #[serde(skip, default = "default_langue_fr")]
    pub(crate) last_logged_langue: Langue,
    #[serde(skip)]
    pub(crate) last_logged_vsync: bool,
    #[serde(skip)]
    pub(crate) last_logged_ecran_large: bool,
    #[serde(skip)]
    pub(crate) last_logged_fullscreen: bool,
    #[serde(skip)]
    pub(crate) last_logged_son_active: bool,
    #[serde(skip, default = "default_instant")]
    pub(crate) terminal_clock_origin: Instant,
    #[serde(skip)]
    pub(crate) fenetre_principale_pos: egui::Pos2,
    #[serde(skip)]
    pub(crate) fenetre_principale_size: egui::Vec2,
    #[serde(skip)]
    pub(crate) video_scale_precedent: u8,
    #[serde(skip)]
    pub(crate) fullscreen_precedent: bool,
    #[serde(skip)]
    pub(crate) keypad: Keypad,
    #[serde(skip)]
    pub(crate) rom_data: Vec<u8>,
    #[serde(skip)]
    pub(crate) rom_path: Option<PathBuf>,
    pub(crate) last_rom_path: Option<PathBuf>,
    #[serde(skip)]
    pub(crate) cpu_step_accumulator: f32,
    #[serde(skip)]
    pub(crate) timer_accumulator: f32,
    #[serde(skip)]
    pub(crate) audio_latch: bool,
    #[serde(skip)]
    pub(crate) terminal_keypad_states: [bool; 16],
    #[serde(skip, default = "default_savestates")]
    pub(crate) savestates: [Option<EmuSnapshot>; 3],
}

impl Default for Nova {
    // Note: Build the full default app state used on first launch.
    // FR: Construit l etat complet par defaut de l application pour le premier lancement.
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            cpu: CPU::new(),
            display: Display::new(),
            langue: Langue::Francais,
            video_scale: 2,
            vsync: true,
            ecran_large: false,
            fullscreen: false,
            en_pause: false,
            rom_chargee: false,
            touches: default_touches(),
            raccourcis: Raccourcis::default(),
            fenetre_settings: false,
            onglet_settings: OngletsSettings::Emulateur,
            temp_langue: Langue::Francais,
            temp_vsync: true,
            temp_ecran_large: false,
            temp_touches: default_touches(),
            temp_raccourcis: Raccourcis::default(),
            cycles_par_seconde: 700,
            son_active: true,
            sound_volume: 100,
            temp_cycles_par_seconde: 700,
            temp_son_active: true,
            temp_terminal_active: false,
            snapshot_langue: Langue::Francais,
            snapshot_vsync: true,
            snapshot_ecran_large: false,
            snapshot_touches: default_touches(),
            snapshot_raccourcis: Raccourcis::default(),
            snapshot_cycles_par_seconde: 700,
            snapshot_son_active: true,
            snapshot_terminal_active: false,
            terminal_active: false,
            status_message: String::new(),
            terminal_logs: Vec::new(),
            last_status_logged: String::new(),
            focus_settings_requested: false,
            focus_terminal_requested: false,
            focus_main_requested: false,
            terminal_position_initialized: false,
            terminal_boot_logs_seeded: false,
            configured_game_fps: 60.0,
            last_logged_theme: Theme::Dark,
            last_logged_langue: Langue::Francais,
            last_logged_vsync: true,
            last_logged_ecran_large: false,
            last_logged_fullscreen: false,
            last_logged_son_active: true,
            terminal_clock_origin: Instant::now(),
            fenetre_principale_pos: egui::Pos2::ZERO,
            fenetre_principale_size: egui::vec2(1280.0, 720.0),
            video_scale_precedent: 2,
            fullscreen_precedent: false,
            keypad: Keypad::new(),
            rom_data: Vec::new(),
            rom_path: None,
            last_rom_path: None,
            cpu_step_accumulator: 0.0,
            timer_accumulator: 0.0,
            audio_latch: false,
            terminal_keypad_states: [false; 16],
            savestates: [None, None, None],
        }
    }
}

impl Nova {
    // Note: Emit a short platform beep when CHIP-8 sound timer is active.
    // FR: Emet un bip court de plateforme quand le timer sonore CHIP-8 est actif.
    fn play_sound_beep() {
        #[cfg(target_os = "windows")]
        {
            // Note: Emit a short 880 Hz beep while the CHIP-8 sound timer is active.
            // FR: Emet un bip court a 880 Hz tant que le timer sonore CHIP-8 est actif.
            unsafe {
                let _ = windows_sys::Win32::System::Diagnostics::Debug::Beep(880, 20);
            }
        }
    }

    // Note: Return true when the shortcut label matches a key press on this frame.
    // FR: Retourne vrai quand le libelle du raccourci correspond a une touche appuyee sur cette frame.
    fn shortcut_pressed(ctx: &egui::Context, label: &str) -> bool {
        if let Some(key) = key_from_label(label) {
            return ctx.input(|i| i.key_pressed(key));
        }
        false
    }

    // Note: Process global keyboard shortcuts bound in settings.
    // FR: Traite les raccourcis clavier globaux definis dans les parametres.
    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        // Note: Ignore shortcuts while text fields are being edited.
        // FR: Ignore les raccourcis pendant l edition des champs texte.
        if ctx.wants_keyboard_input() {
            return;
        }

        if Self::shortcut_pressed(ctx, &self.raccourcis.pause) {
            self.en_pause = !self.en_pause;
        }
        if Self::shortcut_pressed(ctx, &self.raccourcis.reset) {
            self.reset_rom();
        }
        if Self::shortcut_pressed(ctx, &self.raccourcis.stop) {
            self.stop_emulation();
        }
        if Self::shortcut_pressed(ctx, &self.raccourcis.charger_jeu) {
            self.load_rom_dialog();
        }
        if Self::shortcut_pressed(ctx, &self.raccourcis.fullscreen) {
            self.fullscreen = !self.fullscreen;
        }
        if Self::shortcut_pressed(ctx, &self.raccourcis.savestate_1) {
            self.save_state_slot(0);
        }
        if Self::shortcut_pressed(ctx, &self.raccourcis.savestate_2) {
            self.save_state_slot(1);
        }
        if Self::shortcut_pressed(ctx, &self.raccourcis.savestate_3) {
            self.save_state_slot(2);
        }
        if Self::shortcut_pressed(ctx, &self.raccourcis.loadstate_1) {
            self.load_state_slot(0);
        }
        if Self::shortcut_pressed(ctx, &self.raccourcis.loadstate_2) {
            self.load_state_slot(1);
        }
        if Self::shortcut_pressed(ctx, &self.raccourcis.loadstate_3) {
            self.load_state_slot(2);
        }

        // Note: Keep Alt+Enter as an additional fullscreen shortcut.
        // FR: Conserve Alt+Entree comme raccourci plein ecran supplementaire.
        if ctx.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.alt) {
            self.fullscreen = !self.fullscreen;
        }
    }

    // Note: Apply temporary settings values into live runtime state.
    // FR: Applique les valeurs temporaires des parametres a l etat runtime en direct.
    pub(crate) fn apply_temp_values(&mut self) {
        let terminal_was_active = self.terminal_active;
        self.langue = self.temp_langue;
        self.vsync = self.temp_vsync;
        self.ecran_large = self.temp_ecran_large;
        self.touches = self.temp_touches.clone();
        self.raccourcis = self.temp_raccourcis.clone();
        self.cycles_par_seconde = self.temp_cycles_par_seconde;
        self.son_active = self.temp_son_active;
        self.terminal_active = self.temp_terminal_active;
        if !terminal_was_active && self.terminal_active {
            self.focus_terminal_requested = true;
            self.terminal_position_initialized = false;
        }
    }

    // Note: Restore snapshots captured before opening settings.
    // FR: Restaure les snapshots captures avant l ouverture des parametres.
    pub(crate) fn restore_snapshots(&mut self) {
        self.langue = self.snapshot_langue;
        self.vsync = self.snapshot_vsync;
        self.ecran_large = self.snapshot_ecran_large;
        self.touches = self.snapshot_touches.clone();
        self.raccourcis = self.snapshot_raccourcis.clone();
        self.cycles_par_seconde = self.snapshot_cycles_par_seconde;
        self.son_active = self.snapshot_son_active;
        self.terminal_active = self.snapshot_terminal_active;
    }

    // Note: Reset only the currently selected settings tab to defaults.
    // FR: Reinitialise uniquement l onglet de parametres actuellement selectionne.
    pub(crate) fn reset_current_settings_tab_to_default(&mut self) {
        let defaut = Nova::default();
        match self.onglet_settings {
            OngletsSettings::Emulateur => {
                self.temp_langue = defaut.langue;
                self.temp_cycles_par_seconde = defaut.cycles_par_seconde;
                self.temp_son_active = defaut.son_active;
            }
            OngletsSettings::Video => {
                self.temp_vsync = defaut.vsync;
                self.temp_ecran_large = defaut.ecran_large;
            }
            OngletsSettings::Controles => {
                self.temp_touches = defaut.touches.clone();
            }
            OngletsSettings::Raccourcis => {
                self.temp_raccourcis = defaut.raccourcis.clone();
            }
            OngletsSettings::Debug => {
                self.temp_terminal_active = defaut.terminal_active;
            }
        }
    }

    // Note: Reset runtime accumulators/timers not persisted across ROM changes.
    // FR: Reinitialise les accumulateurs/timers runtime non persistes entre changements de ROM.
    pub(crate) fn reset_runtime_clocks(&mut self) {
        self.cpu_step_accumulator = 0.0;
        self.timer_accumulator = 0.0;
        self.audio_latch = false;
    }

    // Note: Open file dialog and load selected ROM.
    // FR: Ouvre le selecteur de fichiers et charge la ROM choisie.
    pub(crate) fn load_rom_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("CHIP-8", &["ch8", "rom", "bin"])
            .pick_file()
        {
            let t = tr(self.langue);
            match self.load_rom_path(path) {
                Ok(loaded) => {
                    self.status_message = format!(
                        "{} ({} {})",
                        t.status_rom_loaded, loaded, t.bytes_unit
                    );
                }
                Err(err) => {
                    self.status_message = format!("{}: {}", t.status_rom_load_failed, err);
                }
            }
        }
    }

    // Note: Load a ROM from a known path and reset runtime state.
    // FR: Charge une ROM depuis un chemin connu et reinitialise l etat runtime.
    pub(crate) fn load_rom_path(&mut self, path: PathBuf) -> Result<usize, String> {
        let rom = fs::read(&path).map_err(|e| e.to_string())?;
        self.rom_data = rom;
        self.last_rom_path = Some(path.clone());
        self.rom_path = Some(path);
        self.cpu.hard_reset();
        self.display.clear();
        self.keypad.clear();
        self.rom_chargee = true;
        self.en_pause = false;
        self.reset_runtime_clocks();
        Ok(self.cpu.load_program(&self.rom_data))
    }

    // Note: Reload the most recently loaded ROM path.
    // FR: Recharge la ROM du chemin charge le plus recent.
    pub(crate) fn load_recent_rom(&mut self) {
        let t = tr(self.langue);
        let Some(path) = self.last_rom_path.clone() else {
            self.status_message = t.status_no_rom_loaded.clone();
            return;
        };
        match self.load_rom_path(path) {
            Ok(loaded) => {
                self.status_message = format!("{} ({} {})", t.status_rom_loaded, loaded, t.bytes_unit);
            }
            Err(err) => {
                self.status_message = format!("{}: {}", t.status_rom_load_failed, err);
            }
        }
    }

    // Note: Reset currently loaded ROM while keeping path/data.
    // FR: Reinitialise la ROM actuellement chargee en conservant chemin/donnees.
    pub(crate) fn reset_rom(&mut self) {
        let t = tr(self.langue);
        if self.rom_data.is_empty() {
            self.status_message = t.status_no_rom_loaded.clone();
            return;
        }
        self.cpu.hard_reset();
        self.display.clear();
        self.keypad.clear();
        let loaded = self.cpu.load_program(&self.rom_data);
        self.en_pause = false;
        self.rom_chargee = true;
        self.reset_runtime_clocks();
        self.status_message = format!("{} ({} {})", t.status_rom_reset, loaded, t.bytes_unit);
    }

    // Note: Stop emulation and clear loaded ROM state.
    // FR: Arrete l emulation et nettoie l etat de ROM chargee.
    pub(crate) fn stop_emulation(&mut self) {
        let t = tr(self.langue);
        self.cpu.hard_reset();
        self.display.clear();
        self.keypad.clear();
        self.rom_data.clear();
        self.rom_path = None;
        self.en_pause = false;
        self.rom_chargee = false;
        self.reset_runtime_clocks();
        self.status_message = t.status_emulation_stopped.clone();
    }

    // Note: Save full emulator snapshot into selected slot.
    // FR: Sauvegarde un snapshot complet de l emulateur dans le slot selectionne.
    pub(crate) fn save_state_slot(&mut self, slot: usize) {
        let t = tr(self.langue);
        if slot >= self.savestates.len() {
            return;
        }
        self.savestates[slot] = Some(EmuSnapshot {
            cpu: self.cpu.clone(),
            display: self.display.clone(),
        });
        self.status_message = format!("{} {}", t.status_state_saved_slot, slot + 1);
    }

    // Note: Load emulator snapshot from selected slot.
    // FR: Charge un snapshot d emulateur depuis le slot selectionne.
    pub(crate) fn load_state_slot(&mut self, slot: usize) {
        let t = tr(self.langue);
        if slot >= self.savestates.len() {
            return;
        }
        if let Some(snapshot) = &self.savestates[slot] {
            self.cpu = snapshot.cpu.clone();
            self.display = snapshot.display.clone();
            self.rom_chargee = true;
            self.en_pause = false;
            self.reset_runtime_clocks();
            self.status_message = format!("{} {}", t.status_state_loaded_slot, slot + 1);
        } else {
            self.status_message = format!("{} {}", t.status_slot_empty, slot + 1);
        }
    }

    // Note: Merge keyboard/gamepad/terminal-key states into the CHIP-8 keypad.
    // FR: Fusionne les etats clavier/manette/terminal dans le clavier CHIP-8.
    fn update_keypad_from_input(&mut self, ctx: &egui::Context) {
        let mut states = [false; 16];
        ctx.input(|i| {
            for (idx, key_name) in self.touches.iter().enumerate() {
                if let Some(key) = key_from_label(key_name) {
                    states[idx] = i.key_down(key);
                }
            }
        });
        let gamepad_states = gamepad::poll_chip8_keys();
        for idx in 0..16 {
            states[idx] = states[idx] || self.terminal_keypad_states[idx] || gamepad_states[idx];
        }
        self.keypad.set_all(states);
    }

    // Note: Run emulation clocks (CPU cycles + 60 Hz timers) for this frame.
    // FR: Execute les horloges d emulation (cycles CPU + timers 60 Hz) pour cette frame.
    fn run_emulator_step(&mut self, dt: f32) {
        if !self.rom_chargee || self.en_pause {
            return;
        }

        self.cpu_step_accumulator += dt * self.cycles_par_seconde as f32;
        // Note: Cap catch-up cycles to avoid UI stalls during move/resize.
        // FR: Limite les cycles de rattrapage pour eviter les freezes UI pendant deplacement/redimensionnement.
        const MAX_CYCLES_PER_FRAME: usize = 2_000;
        let mut executed_cycles = 0usize;
        while self.cpu_step_accumulator >= 1.0 && executed_cycles < MAX_CYCLES_PER_FRAME {
            self.cpu.cycle(&mut self.display, &self.keypad);
            self.cpu_step_accumulator -= 1.0;
            executed_cycles += 1;
        }
        if self.cpu_step_accumulator > MAX_CYCLES_PER_FRAME as f32 {
            // Note: Trim backlog when the emulator falls too far behind.
            // FR: Reduit l arriere-log quand l emulateur prend trop de retard.
            self.cpu_step_accumulator = MAX_CYCLES_PER_FRAME as f32;
        }

        self.timer_accumulator += dt * 60.0;
        while self.timer_accumulator >= 1.0 {
            if self.son_active && self.sound_volume > 0 && self.cpu.sound_timer > 0 && !self.audio_latch {
                Self::play_sound_beep();
                self.audio_latch = true;
            }
            self.cpu.tick_timers();
            if self.cpu.sound_timer == 0 {
                self.audio_latch = false;
            }
            self.timer_accumulator -= 1.0;
        }
    }

    // Note: Append user-facing status into terminal logs when it changes.
    // FR: Ajoute le statut utilisateur dans les logs terminal quand il change.
    fn update_terminal_log(&mut self) {
        if self.status_message.is_empty() || self.status_message == self.last_status_logged {
            return;
        }
        self.push_terminal_log_line("I", self.status_message.clone());
        self.last_status_logged = self.status_message.clone();
    }

    // Note: Format elapsed terminal uptime as HH:MM:SS.mmm.
    // FR: Formate le temps terminal ecoule en HH:MM:SS.mmm.
    fn terminal_timestamp(&self) -> String {
        let elapsed = self.terminal_clock_origin.elapsed();
        let total_secs = elapsed.as_secs();
        let h = total_secs / 3600;
        let m = (total_secs % 3600) / 60;
        let s = total_secs % 60;
        let ms = elapsed.subsec_millis();
        format!("{:02}:{:02}:{:02}.{:03}", h, m, s, ms)
    }

    // Note: Push one timestamped terminal log line with level marker.
    // FR: Ajoute une ligne de log terminal horodatee avec marqueur de niveau.
    fn push_terminal_log_line(&mut self, level: &str, message: String) {
        let line = format!("{} |{}| {}", self.terminal_timestamp(), level, message);
        self.terminal_logs.push(line);
    }

    // Note: Push one raw terminal line without timestamp.
    // FR: Ajoute une ligne terminal brute sans horodatage.
    fn push_terminal_plain_line(&mut self, message: String) {
        self.terminal_logs.push(message);
    }

    // Note: Return localized state word for enabled/disabled values.
    // FR: Retourne le mot localise pour les etats active/desactive.
    fn state_label_for_langue(&self, value: bool) -> String {
        let t = tr(self.langue);
        if value {
            t.terminal_enabled.clone()
        } else {
            t.terminal_disabled.clone()
        }
    }

    // Note: Return localized theme label.
    // FR: Retourne le libelle localise du theme.
    fn theme_label_for_langue(&self, theme: Theme) -> String {
        let t = tr(self.langue);
        match theme {
            Theme::Dark => t.terminal_dark.clone(),
            Theme::Light => t.terminal_light.clone(),
        }
    }

    // Note: Seed startup/debug context lines once per app run.
    // FR: Initialise les lignes de contexte demarrage/debug une seule fois par execution.
    fn seed_terminal_boot_logs(&mut self) {
        if self.terminal_boot_logs_seeded {
            return;
        }
        self.terminal_clock_origin = Instant::now();
        let t = tr(self.langue);
        self.push_terminal_plain_line(t.terminal_legend.clone());
        self.push_terminal_plain_line(t.terminal_info_legend.clone());
        self.push_terminal_plain_line(String::new());
        self.push_terminal_log_line("I", format!("Nova v{}", VERSION));
        self.push_terminal_log_line(
            "I",
            format!("{}: {} ({})", t.terminal_platform, std::env::consts::OS, std::env::consts::ARCH),
        );
        self.push_terminal_log_line("I", t.terminal_runtime_ready.clone());
        self.push_terminal_log_line("I", format!("{}: {}", t.terminal_theme, self.theme_label_for_langue(self.theme)));
        self.push_terminal_log_line("I", format!("{}: {}", t.terminal_language, self.langue.label()));
        self.push_terminal_log_line(
            "I",
            format!(
                "{} : {}",
                t.terminal_fps,
                if self.vsync { "60".to_owned() } else { t.terminal_unlimited.clone() }
            ),
        );
        self.push_terminal_log_line("I", format!("{}: {}", t.terminal_vsync, self.state_label_for_langue(self.vsync)));
        self.push_terminal_log_line(
            "I",
            format!("{}: {}", t.terminal_widescreen, self.state_label_for_langue(self.ecran_large)),
        );
        self.push_terminal_log_line(
            "I",
            format!("{}: {}", t.terminal_fullscreen, self.state_label_for_langue(self.fullscreen)),
        );
        self.push_terminal_log_line("I", format!("{}: {}", t.terminal_sound, self.state_label_for_langue(self.son_active)));
        self.push_terminal_log_line("I", t.terminal_core_init.clone());
        self.push_terminal_log_line("I", t.terminal_display_init.clone());
        self.push_terminal_log_line("I", t.terminal_input_init.clone());
        self.push_terminal_log_line(
            "I",
            format!(
                "{} ({})",
                t.terminal_audio_init,
                if self.son_active {
                    t.terminal_enabled.clone()
                } else {
                    t.terminal_disabled.clone()
                }
            ),
        );
        self.last_logged_theme = self.theme;
        self.last_logged_langue = self.langue;
        self.last_logged_vsync = self.vsync;
        self.last_logged_ecran_large = self.ecran_large;
        self.last_logged_fullscreen = self.fullscreen;
        self.last_logged_son_active = self.son_active;
        self.terminal_boot_logs_seeded = true;
    }

    // Note: Log runtime configuration changes when values differ from last logged state.
    // FR: Journalise les changements de configuration runtime quand les valeurs different du dernier etat loggue.
    fn log_config_changes(&mut self) {
        let t = tr(self.langue);
        if self.theme != self.last_logged_theme {
            self.push_terminal_log_line("I", format!("{}: {}", t.terminal_theme, self.theme_label_for_langue(self.theme)));
            self.last_logged_theme = self.theme;
        }
        if self.langue != self.last_logged_langue {
            self.push_terminal_log_line("I", format!("{}: {}", t.terminal_language, self.langue.label()));
            self.last_logged_langue = self.langue;
        }
        if self.vsync != self.last_logged_vsync {
            self.push_terminal_log_line("I", format!("{}: {}", t.terminal_vsync, self.state_label_for_langue(self.vsync)));
            self.push_terminal_log_line(
                "I",
                format!(
                    "{} : {}",
                    t.terminal_fps,
                    if self.vsync { "60".to_owned() } else { t.terminal_unlimited.clone() }
                ),
            );
            self.last_logged_vsync = self.vsync;
        }
        if self.ecran_large != self.last_logged_ecran_large {
            self.push_terminal_log_line(
                "I",
                format!("{}: {}", t.terminal_widescreen, self.state_label_for_langue(self.ecran_large)),
            );
            self.last_logged_ecran_large = self.ecran_large;
        }
        if self.fullscreen != self.last_logged_fullscreen {
            self.push_terminal_log_line(
                "I",
                format!("{}: {}", t.terminal_fullscreen, self.state_label_for_langue(self.fullscreen)),
            );
            self.last_logged_fullscreen = self.fullscreen;
        }
        if self.son_active != self.last_logged_son_active {
            self.push_terminal_log_line("I", format!("{}: {}", t.terminal_sound, self.state_label_for_langue(self.son_active)));
            self.last_logged_son_active = self.son_active;
        }
    }
}

impl eframe::App for Nova {
    // Note: Main frame update loop (input, emulation tick, UI rendering, viewport sync).
    // FR: Boucle principale de mise a jour de frame (entrees, tick emulation, rendu UI, sync viewport).
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.fenetre_principale_size = ctx.content_rect().size();
        if let Some(rect) = ctx.input(|i| i.viewport().outer_rect) {
            self.fenetre_principale_pos = rect.min;
        }
        if self.focus_main_requested {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            self.focus_main_requested = false;
        }
        if ctx.input(|i| i.pointer.any_pressed()) {
            if self.terminal_active {
                self.focus_terminal_requested = true;
            }
            if self.fenetre_settings {
                self.focus_settings_requested = true;
            }
        }

        if self.video_scale != self.video_scale_precedent {
            if !self.fullscreen {
                let size = fenetre_size(self.video_scale);
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(size[0], size[1])));
            }
            self.video_scale_precedent = self.video_scale;
        }

        self.handle_shortcuts(ctx);

        if self.fullscreen != self.fullscreen_precedent {
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));
            self.fullscreen_precedent = self.fullscreen;
        }

        self.update_keypad_from_input(ctx);
        let dt = ctx.input(|i| i.stable_dt).max(0.0);
        self.configured_game_fps = if self.vsync { 60.0 } else { f32::INFINITY };
        self.run_emulator_step(dt);
        self.seed_terminal_boot_logs();
        self.log_config_changes();
        self.update_terminal_log();

        ctx.set_visuals(match self.theme {
            Theme::Dark => egui::Visuals::dark(),
            Theme::Light => egui::Visuals::light(),
        });

        ui::top_bar::show(ctx, self);
        ui::bottom_bar::show(ctx, self);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui::main_panel::show(ui, self);
        });

        if self.fenetre_settings {
            ui::settings::show(ctx, self);
        }
        if self.terminal_active {
            ui::debug_terminal::show(ctx, self);
        } else {
            self.terminal_position_initialized = false;
            self.terminal_keypad_states = [false; 16];
        }

        if self.rom_chargee && !self.en_pause {
            ctx.request_repaint();
        }
    }

    // Note: Persist user-configurable state while stripping runtime-only transient data.
    // FR: Persiste l etat configurable utilisateur en retirant les donnees runtime transitoires.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let mut clone = self.clone();
        clone.fenetre_settings = false;
        clone.keypad = Keypad::new();
        clone.rom_data.clear();
        clone.rom_path = None;
        clone.cpu_step_accumulator = 0.0;
        clone.timer_accumulator = 0.0;
        clone.audio_latch = false;
        clone.terminal_keypad_states = [false; 16];
        clone.terminal_logs.clear();
        clone.last_status_logged.clear();
        clone.focus_settings_requested = false;
        clone.focus_terminal_requested = false;
        clone.focus_main_requested = false;
        clone.terminal_position_initialized = false;
        clone.terminal_boot_logs_seeded = false;
        clone.last_logged_theme = clone.theme;
        clone.last_logged_langue = clone.langue;
        clone.last_logged_vsync = clone.vsync;
        clone.last_logged_ecran_large = clone.ecran_large;
        clone.last_logged_fullscreen = clone.fullscreen;
        clone.last_logged_son_active = clone.son_active;
        clone.configured_game_fps = if clone.vsync { 60.0 } else { f32::INFINITY };
        clone.savestates = [None, None, None];
        eframe::set_value(storage, eframe::APP_KEY, &clone);
    }
}
