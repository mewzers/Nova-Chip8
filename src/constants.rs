// Note: Build/version string injected from Cargo metadata.
// FR: Chaine de version injectee depuis les metadonnees Cargo.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Note: Base size for one CHIP-8 pixel in UI coordinates.
// FR: Taille de base d un pixel CHIP-8 dans les coordonnees UI.
pub const PIXEL_BASE: f32 = 10.0;
// Note: CHIP-8 display width in pixels.
// FR: Largeur de l ecran CHIP-8 en pixels.
pub const CHIP8_W: f32 = 64.0;
// Note: CHIP-8 display height in pixels.
// FR: Hauteur de l ecran CHIP-8 en pixels.
pub const CHIP8_H: f32 = 32.0;
// Note: Top bar reserved height.
// FR: Hauteur reservee pour la barre du haut.
pub const BARRE_HAUT: f32 = 28.0;
// Note: Bottom bar reserved height.
// FR: Hauteur reservee pour la barre du bas.
pub const BARRE_BAS: f32 = 24.0;
