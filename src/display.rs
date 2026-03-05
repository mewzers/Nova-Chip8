// Note: Default framebuffer state (all pixels off).
// FR: Etat par defaut du framebuffer (tous les pixels eteints).
fn default_pixels() -> [bool; 64 * 32] {
    [false; 64 * 32]
}

// Note: CHIP-8 monochrome framebuffer (64x32).
// FR: Framebuffer monochrome CHIP-8 (64x32).
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Display {
    // Note: Raw pixel buffer stored in row-major order.
    // FR: Tampon de pixels brut stocke en ordre ligne par ligne.
    #[serde(skip, default = "default_pixels")]
    pub pixels: [bool; 64 * 32],
}

impl Display {
    // Note: Create a cleared display.
    // FR: Cree un affichage vide.
    pub fn new() -> Self {
        Self {
            pixels: [false; 64 * 32],
        }
    }

    // Note: Clear the full display.
    // FR: Efface completement l affichage.
    pub fn clear(&mut self) {
        self.pixels = [false; 64 * 32];
    }

    // Note: Read one pixel value at (x, y).
    // FR: Lit la valeur d un pixel en (x, y).
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[y * 64 + x]
    }

    // Note: XOR one pixel and return collision flag (true if pixel was previously set).
    // FR: XOR un pixel et retourne le drapeau de collision (vrai si le pixel etait deja actif).
    pub fn set_pixel(&mut self, x: usize, y: usize) -> bool {
        let idx = y * 64 + x;
        let collision = self.pixels[idx];
        self.pixels[idx] ^= true;
        collision
    }
}
