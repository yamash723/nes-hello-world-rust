#[derive(Debug)]
pub struct Palette(u8);

impl Palette {
  pub fn new(palette_number: u8) -> Self {
    Palette(palette_number)
  }

  pub fn get_palette_number(&self) -> u8 {
    self.0
  }
}

impl PartialEq for Palette {
    fn eq(&self, other: &Palette) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug)]
pub struct PaletteGroup([Palette;4]);

impl PaletteGroup {
  pub fn build(palette_numbers: &[u8;4]) -> Self {
    let palettes = [
      Palette::new(palette_numbers[0]),
      Palette::new(palette_numbers[1]),
      Palette::new(palette_numbers[2]),
      Palette::new(palette_numbers[3]),
    ];

    PaletteGroup(palettes)
  }

  pub fn get(&self, num: usize) -> &Palette {
    &self.0[num]
  }
}

impl PartialEq for PaletteGroup {
    fn eq(&self, other: &PaletteGroup) -> bool {
        self.0[0] == other.0[0] &&
        self.0[1] == other.0[1] &&
        self.0[2] == other.0[2] &&
        self.0[3] == other.0[3]
    }
}

#[cfg(test)]
mod palette_test {
  use super::*;

  #[test]
  fn eq_test() {
      let palette_1 = Palette::new(0x10);
      let palette_2 = Palette::new(0x10);
      assert_eq!(palette_1, palette_2);

      let palette_1 = Palette::new(0x10);
      let palette_2 = Palette::new(0x20);
      assert_ne!(palette_1, palette_2);
  }
}

#[cfg(test)]
mod palette_group_test {
  use super::*;

  #[test]
  fn build_test() {
    let palette_numbers = [0x10, 0x20, 0x30, 0x40];
    let palette_group = PaletteGroup::build(&palette_numbers);

    assert_eq!(palette_group.get(0).get_palette_number(), 0x10);
    assert_eq!(palette_group.get(1).get_palette_number(), 0x20);
    assert_eq!(palette_group.get(2).get_palette_number(), 0x30);
    assert_eq!(palette_group.get(3).get_palette_number(), 0x40);
  }
  
  #[test]
  fn eq_test() {
      let palettes_1 = PaletteGroup::build(&[0x10, 0x11, 0x12, 0x13]);
      let palettes_2 = PaletteGroup::build(&[0x10, 0x11, 0x12, 0x13]);
      assert!(palettes_1 == palettes_2);

      let palettes_1 = PaletteGroup::build(&[0x10, 0x11, 0x12, 0x13]);
      let palettes_2 = PaletteGroup::build(&[0x00, 0x11, 0x12, 0x13]);
      assert!(palettes_1 != palettes_2);
  }
}