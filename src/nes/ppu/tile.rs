use nes::ram::Ram;
use super::PpuContext;
use super::sprite::Sprite;
use super::palette::PaletteGroup;
use super::palette_ram::PaletteType;
use super::tile_position::TilePosition;

#[derive(Debug)]
pub struct Tile {
    pub sprite: Sprite,
    pub position: TilePosition,
    pub palettes: PaletteGroup,
}

impl Tile {
    pub fn build(position: TilePosition, ppu_context: &PpuContext) -> Tile {
        let attributes_id = position.get_attribute_id();

        // ToDo: refactoring here.
        let attribute_addr = attributes_id as u16 + 0x03C0; // 0x03C0 is name table size.
        let attribute = ppu_context.vram.read(attribute_addr);
        
        let palette_id = position.get_palette_id(attribute);
        let palettes = ppu_context.palette_ram.get_palettes(palette_id, PaletteType::Background);
        let sprite_number = ppu_context.vram.read(position.get_tile_number() as u16);

        let sprite = Sprite::build(sprite_number, &ppu_context.cram);

        Tile {
            sprite,
            position,
            palettes,
        }
    }
}

#[cfg(test)]
mod tile_test {
    use super::*;
    use super::super::PaletteRam;
    use super::super::TilePosition;

    #[test]
    fn build_test() {
        /* build a workd 'H' 
        Sprite vecter.
        3 3 1 0 0 3 3 1
        3 3 1 0 0 3 3 1
        3 3 1 0 0 3 3 1
        3 3 3 3 3 3 3 1
        3 3 1 1 1 3 3 1
        3 3 1 0 0 3 3 1
        3 3 1 0 0 3 3 1
        1 1 1 0 0 1 1 1

        Sprite palette
        [0: 15, 1: 0, 2: 16, 3: 32]

        Sprite position
        x: 0, y: 0
        */

        let word_vec = [
            // channel 1
            0b11100111,
            0b11100111,
            0b11100111,
            0b11111111,
            0b11111111,
            0b11100111,
            0b11100111,
            0b11100111,

            // channel 2
            0b11000110,
            0b11000110,
            0b11000110,
            0b11111110,
            0b11000110,
            0b11000110,
            0b11000110,
            0b00000000,
        ];

        let mut character_rom = Vec::new();
        character_rom.append(&mut word_vec.to_vec());
        let cram = Ram::new(character_rom.clone());

        // write a palette id: 0 in attribute id: 0
        let mut vram = Ram::new(vec!(0;0x0FFF));
        vram.write(0x03C0, 0x00);

        // write a palettes number in palette id: 0
        let mut palette_ram = PaletteRam::new();
        let palette_numbers = [15, 0, 16, 32];
        palette_ram.write(0x00, palette_numbers[0]);
        palette_ram.write(0x01, palette_numbers[1]);
        palette_ram.write(0x02, palette_numbers[2]);
        palette_ram.write(0x03, palette_numbers[3]);

        let ppu_context = PpuContext {
            cram: cram,
            vram: vram,
            palette_ram: palette_ram,
        };
        let tile_pos = TilePosition::new(0, 0);
        let tile = Tile::build(tile_pos, &ppu_context);

        // Assert sprite
        let expect_sprite = Sprite::build(0, &Ram::new(character_rom));
        assert_eq!(tile.sprite.to_vec(), expect_sprite.to_vec());

        // assert palettes
        let expect_palettes = PaletteGroup::build(&palette_numbers);
        assert_eq!(tile.palettes, expect_palettes);

        // Assert tile position
        assert_eq!(tile.position.x, 0);
        assert_eq!(tile.position.y, 0);
    }
}