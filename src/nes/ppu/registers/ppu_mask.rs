pub struct PpuMask {
  // 0: normal color, 1: produce a greyscale display
  grayscale: bool,
  // 1: Show background in leftmost 8 pixels of screen, 0: Hide
  show_background_in_leftmost: bool,
  // 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
  show_sprites_in_leftmost: bool,
  // 1: Show background
  show_background: bool,
  // 1: Show sprites
  show_sprites: bool,
  // Emphasize red*
  emphasize_red: bool,
  // Emphasize green*
  emphasize_green: bool,
  // Emphasize blue*
  emphasize_blue: bool,
}

impl PpuMask {
    pub fn new() -> Self {
        PpuMask {
          grayscale: false,
          show_background_in_leftmost: false,
          show_sprites_in_leftmost: false,
          show_background: false,
          show_sprites: false,
          emphasize_red: false,
          emphasize_green: false,
          emphasize_blue: false,
        }
    }

    pub fn write(&mut self, data: u8) {
        self.grayscale                   =  data & 0b00000001       == 1;
        self.show_background_in_leftmost = (data & 0b00000010) >> 1 == 1;
        self.show_sprites_in_leftmost    = (data & 0b00000100) >> 2 == 1;
        self.show_background             = (data & 0b00001000) >> 3 == 1;
        self.show_sprites                = (data & 0b00010000) >> 4 == 1;
        self.emphasize_red               = (data & 0b00100000) >> 5 == 1;
        self.emphasize_green             = (data & 0b01000000) >> 6 == 1;
        self.emphasize_blue              = (data & 0b10000000) >> 7 == 1;
    }

    pub fn read(self) -> u8 {
        (self.grayscale as u8) |
        (self.show_background_in_leftmost as u8) << 1 |
        (self.show_sprites_in_leftmost as u8)    << 2 |
        (self.show_background as u8)             << 3 |
        (self.show_sprites as u8)                << 4 |
        (self.emphasize_red as u8)               << 5 |
        (self.emphasize_green as u8)             << 6 |
        (self.emphasize_blue as u8)              << 7
    }
}


#[cfg(test)]
mod ppu_mask_test {
    use super::*;

    #[test]
    fn write_test() {
        let mut ppu_mask = PpuMask::new();
        ppu_mask.write(0b10101010);

        assert_eq!(ppu_mask.grayscale, false);
        assert_eq!(ppu_mask.show_background_in_leftmost, true);
        assert_eq!(ppu_mask.show_sprites_in_leftmost, false);
        assert_eq!(ppu_mask.show_background, true);
        assert_eq!(ppu_mask.show_sprites, false);
        assert_eq!(ppu_mask.emphasize_red, true);
        assert_eq!(ppu_mask.emphasize_green, false);
        assert_eq!(ppu_mask.emphasize_blue, true);
    }

    #[test]
    fn read_test() {
        let mut ppu_mask = PpuMask::new();
        ppu_mask.write(0b10101010);
        assert_eq!(ppu_mask.read(), 0b10101010);
    }
}