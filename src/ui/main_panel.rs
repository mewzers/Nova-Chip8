use eframe::egui;

use crate::app::Nova;
use crate::constants::{CHIP8_H, CHIP8_W, PIXEL_BASE};

// Note: Draw CHIP-8 framebuffer in the central panel, centered in window/fullscreen.
// FR: Dessine le framebuffer CHIP-8 dans le panneau central, centre en fenetre/plein ecran.
pub fn show(ui: &mut egui::Ui, app: &mut Nova) {
    let pixel_size = app.video_scale as f32 * PIXEL_BASE;
    let screen_width = CHIP8_W * pixel_size;
    let screen_height = CHIP8_H * pixel_size;

    let (offset_x, offset_y) = if app.fullscreen {
        let avail = ui.available_size();
        ((avail.x - screen_width) / 2.0, (avail.y - screen_height) / 2.0)
    } else {
        (
            (app.fenetre_principale_size.x - screen_width) / 2.0,
            (app.fenetre_principale_size.y - screen_height) / 2.0,
        )
    };

    let (_response, painter) = ui.allocate_painter(
        egui::vec2(ui.available_width(), ui.available_height()),
        egui::Sense::hover(),
    );

    let origin = egui::pos2(offset_x, offset_y);

    // Note: Paint each CHIP-8 pixel as a filled rectangle.
    // FR: Dessine chaque pixel CHIP-8 comme un rectangle rempli.
    for y in 0..32 {
        for x in 0..64 {
            let color = if app.display.get_pixel(x, y) {
                egui::Color32::WHITE
            } else {
                egui::Color32::BLACK
            };

            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(
                        origin.x + x as f32 * pixel_size,
                        origin.y + y as f32 * pixel_size,
                    ),
                    egui::vec2(pixel_size, pixel_size),
                ),
                0.0,
                color,
            );
        }
    }
}
