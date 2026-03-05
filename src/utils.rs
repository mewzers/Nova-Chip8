use eframe::egui;

use crate::constants::{BARRE_BAS, BARRE_HAUT, CHIP8_H, CHIP8_W, PIXEL_BASE};

// Note: Compute window size from the selected render scale.
// FR: Calcule la taille de fenetre selon l echelle de rendu choisie.
pub fn fenetre_size(scale: u8) -> [f32; 2] {
    let w = CHIP8_W * PIXEL_BASE * scale as f32;
    let h = CHIP8_H * PIXEL_BASE * scale as f32 + BARRE_HAUT + BARRE_BAS;
    [w, h]
}

// Note: Default CHIP-8 keypad mapping on keyboard.
// FR: Mapping clavier CHIP-8 par defaut.
pub fn default_touches() -> [String; 16] {
    [
        "X".into(), "1".into(), "2".into(), "3".into(),
        "Q".into(), "W".into(), "E".into(), "A".into(),
        "S".into(), "D".into(), "Z".into(), "C".into(),
        "4".into(), "R".into(), "F".into(), "V".into(),
    ]
}

// Note: Convert a user-facing key label into an egui key enum.
// FR: Convertit un libelle de touche utilisateur en enum de touche egui.
pub fn key_from_label(label: &str) -> Option<egui::Key> {
    let key = label.trim().to_uppercase();
    match key.as_str() {
        "0" => Some(egui::Key::Num0),
        "1" => Some(egui::Key::Num1),
        "2" => Some(egui::Key::Num2),
        "3" => Some(egui::Key::Num3),
        "4" => Some(egui::Key::Num4),
        "5" => Some(egui::Key::Num5),
        "6" => Some(egui::Key::Num6),
        "7" => Some(egui::Key::Num7),
        "8" => Some(egui::Key::Num8),
        "9" => Some(egui::Key::Num9),
        "A" => Some(egui::Key::A),
        "B" => Some(egui::Key::B),
        "C" => Some(egui::Key::C),
        "D" => Some(egui::Key::D),
        "E" => Some(egui::Key::E),
        "F" => Some(egui::Key::F),
        "G" => Some(egui::Key::G),
        "H" => Some(egui::Key::H),
        "I" => Some(egui::Key::I),
        "J" => Some(egui::Key::J),
        "K" => Some(egui::Key::K),
        "L" => Some(egui::Key::L),
        "M" => Some(egui::Key::M),
        "N" => Some(egui::Key::N),
        "O" => Some(egui::Key::O),
        "P" => Some(egui::Key::P),
        "Q" => Some(egui::Key::Q),
        "R" => Some(egui::Key::R),
        "S" => Some(egui::Key::S),
        "T" => Some(egui::Key::T),
        "U" => Some(egui::Key::U),
        "V" => Some(egui::Key::V),
        "W" => Some(egui::Key::W),
        "X" => Some(egui::Key::X),
        "Y" => Some(egui::Key::Y),
        "Z" => Some(egui::Key::Z),
        "F1" => Some(egui::Key::F1),
        "F2" => Some(egui::Key::F2),
        "F3" => Some(egui::Key::F3),
        "F4" => Some(egui::Key::F4),
        "F5" => Some(egui::Key::F5),
        "F6" => Some(egui::Key::F6),
        "F7" => Some(egui::Key::F7),
        "F8" => Some(egui::Key::F8),
        "F9" => Some(egui::Key::F9),
        "F10" => Some(egui::Key::F10),
        "F11" => Some(egui::Key::F11),
        "F12" => Some(egui::Key::F12),
        "ESC" | "ECHAP" | "ESCAPE" => Some(egui::Key::Escape),
        "ENTER" => Some(egui::Key::Enter),
        "SPACE" => Some(egui::Key::Space),
        "TAB" => Some(egui::Key::Tab),
        "LEFT" => Some(egui::Key::ArrowLeft),
        "RIGHT" => Some(egui::Key::ArrowRight),
        "UP" => Some(egui::Key::ArrowUp),
        "DOWN" => Some(egui::Key::ArrowDown),
        _ => None,
    }
}
