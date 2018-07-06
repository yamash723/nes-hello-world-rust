pub mod background;
pub mod sprite;
pub mod palette;
pub mod tile;
pub mod tile_position;

mod registers;
mod palette_ram;

use self::registers::Registers;
use self::palette_ram::PaletteRam;
use self::tile::Tile;
use self::tile_position::TilePosition;
use self::background::Background;

use nes::ram::Ram;

pub struct Ppu {
    pub cycle: usize,
    pub line: usize,
    pub registers: Registers,
    pub background: Background,
    pub context: PpuContext,
}

pub struct PpuContext {
    pub cram: Ram,
    pub vram: Ram,
    pub palette_ram: PaletteRam,
}

const CLOCK_TO_RENDER_LINE: usize = 341;

pub enum PpuRunResult {
    CountUpCycle,
    FinishedBuildBackgroundLine,
    FinishedBuildAllBackgroundLine,
}

impl Ppu {
    pub fn new(character_rom: Vec<u8>) -> Self {
        Ppu {
            cycle: 0,
            line: 0,
            registers: Registers::new(),
            background: Background::new(),
            context: PpuContext {
                cram: Ram::new(character_rom),
                vram: Ram::new(vec![0; 0x2000]),
                palette_ram: PaletteRam::new(),
            }

        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        self.registers.read(addr, &mut self.context)
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.registers.write(addr, data, &mut self.context);
    }    

    pub fn run(&mut self, cycle: usize) -> PpuRunResult {
        self.cycle += cycle;

        if self.cycle < CLOCK_TO_RENDER_LINE {
            return PpuRunResult::CountUpCycle;
        }

        self.cycle -= CLOCK_TO_RENDER_LINE;
        self.line += 1;

        // is need building a background line.
        if self.line <= 240 && self.line % 8 == 0 {
            /*
                The name table has such a structure.

                nametable: has 4 nametable. (ID: 0-3)
                x: 32 tiles(8x8 pixel) in 256 pixel.
                y: 30 tiles(8x8 pixel) in 240 pixel.
                +------------+------------+
                |            |            |
                |  0(0x2000) |  1(0x2400) |
                |            |            |
                +------------+------------+
                |            |            |
                |  2(0x2800) |  3(0x2C00) |
                |            |            |
                +------------+------------+

                name table: 0, 2 -> start x position is 0.
                name table: 1, 3 -> start x position is 32.
            */

            let nametable_id = self.registers.get_nametable_id();
            let pos_x_start = (nametable_id % 2) * 32;
            let pos_x_end = pos_x_start + 32;
            let pos_y = (self.line / 8) as u8;

            for pos_x in pos_x_start..pos_x_end {
                let tile_pos = TilePosition::new(pos_x, pos_y);
                let tile = Tile::build(tile_pos, &self.context);
                self.background.push(tile);
            }
        }

        // is not finished building all the background lines.
        if self.line < 262 {
            return PpuRunResult::FinishedBuildBackgroundLine;
        }

        self.line = 0;
        PpuRunResult::FinishedBuildAllBackgroundLine
    }
}