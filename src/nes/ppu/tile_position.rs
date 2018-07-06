#[derive(Debug)]
pub struct TilePosition {
    pub x: u8,
    pub y: u8,
}

impl TilePosition {
    pub fn new(x: u8, y: u8) -> Self {
        TilePosition {
            x,
            y,
        }
    }

    pub fn get_tile_number(&self) -> u16 {
        /*
            The background line consists of 32 tiles.
            +----+----+----+----+----+----+----+----+----
            |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8
            +----+----+----+----+----+----+----+----+----
            | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40
            +----+----+----+----+----+----+----+----+----
        */
        self.x as u16 + (self.y as u16 * 32)
    }

    pub fn get_block_id(&self) -> u8 {
        /*
            The background line consists of 16 blocks and One block has 2x2 tiles.
            +---+---+ +---+---+ +---+---+ +---+---+ +---+---
            | 0 | 1 | | 0 | 1 | | 0 | 1 | | 0 | 1 | | 0 | 1 
            +---+---+ +---+---+ +---+---+ +---+---+ +---+---
            | 2 | 3 | | 2 | 3 | | 2 | 3 | | 2 | 3 | | 2 | 3 
            +---+---+ +---+---+ +---+---+ +---+---+ +---+---

            Blocks detail
            +---------+---------++---------+---------
            | Tile: 0 | Tile: 1 || Tile: 2 | Tile: 3
            |         |         ||         |
            |         |         ||         |
            +---- [Block: 0] ---++---- [Block: 1] ---
            | Tile:32 | Tile:33 || Tile:34 | Tile:35
            |         |         ||         |
            |         |         ||         |
            +---------+---------++---------+---------
            +---------+---------++---------+---------
            | Tile:64 | Tile:65 || Tile:66 | Tile:67
            |         |         ||         |
            |         |         ||         |
            +---- [Block: 2] ---++---- [Block: 3] ---
            | Tile:96 | Tile:97 || Tile:98 | Tile:99
            |         |         ||         |
            |         |         ||         |
            +---------+---------++---------+---------
        */

        let x_block_pos = (self.x % 4) / 2;
        let y_block_pos = ((self.y % 4) / 2) * 2;

        x_block_pos + y_block_pos
    }

    pub fn get_attribute_id(&self) -> u8 {
        /*
            The background line consists of 8 attributes and One attribute has 2x2 blocks.
            +----+----+----+----+----+----+----+----+
            |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |
            +----+----+----+----+----+----+----+----+
            |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 |
            +----+----+----+----+----+----+----+----+

            Attributes detail
            +----------+----------++----------+----------
            | Block: 0 | Block: 1 || Block: 2 | Block: 3
            |          |          ||          |
            |          |          ||          |
            +-- [Attribute: 0] ---++-- [Attribute: 1] ---
            | Block:16 | Block:17 || Block:18 | Block:19
            |          |          ||          |
            |          |          ||          |
            +----------+----------++----------+----------
            +----------+----------++----------+----------
            | Block:32 | Block:33 || Block:34 | Block:35
            |          |          ||          |
            |          |          ||          |
            +-- [Attribute: 8] ---++-- [Attribute: 9] ---
            | Block:48 | Block:49 || Block:50 | Block:51
            |          |          ||          |
            |          |          ||          |
            +----------+----------++----------+----------
        */
        
        let attributes_pos_x = self.x / 4;
        let attributes_pos_y = (self.y / 4) * 8;

        attributes_pos_x + attributes_pos_y
    }

    pub fn get_palette_id(&self, attribute: u8) -> u8 {
        /*
            The attribute has 4 palette id for 4 blocks.
            
             3  2  1  0 <- block id
            10 11 00 01 < attribute bit

            attribute: 0xE4 -> 0b10110001 -> 10 11 00 01 -> 2 3 0 1(palette id).
        */
        let shift = self.get_block_id() * 2;
        (attribute >> shift) & 0b11
    }
}

#[cfg(test)]
mod tile_position_test {
    use super::*;
    use std::ops::Range;

    #[test]
    fn get_tile_number_test() {
        assert_eq!(TilePosition::new(0, 0).get_tile_number(), 0);
        assert_eq!(TilePosition::new(1, 0).get_tile_number(), 1);
        assert_eq!(TilePosition::new(0, 1).get_tile_number(), 32);
        assert_eq!(TilePosition::new(1, 1).get_tile_number(), 33);
    }

    #[test]
    fn get_block_id_test() {
        // block: 0
        assert_eq!(TilePosition::new(0, 0).get_block_id(), 0);
        assert_eq!(TilePosition::new(1, 0).get_block_id(), 0);
        assert_eq!(TilePosition::new(0, 1).get_block_id(), 0);
        assert_eq!(TilePosition::new(1, 1).get_block_id(), 0);

        // block: 1
        assert_eq!(TilePosition::new(2, 0).get_block_id(), 1);
        assert_eq!(TilePosition::new(3, 0).get_block_id(), 1);
        assert_eq!(TilePosition::new(2, 1).get_block_id(), 1);
        assert_eq!(TilePosition::new(3, 1).get_block_id(), 1);

        // block: 2
        assert_eq!(TilePosition::new(0, 2).get_block_id(), 2);
        assert_eq!(TilePosition::new(1, 2).get_block_id(), 2);
        assert_eq!(TilePosition::new(0, 3).get_block_id(), 2);
        assert_eq!(TilePosition::new(1, 3).get_block_id(), 2);

        // block: 3
        assert_eq!(TilePosition::new(2, 2).get_block_id(), 3);
        assert_eq!(TilePosition::new(3, 2).get_block_id(), 3);
        assert_eq!(TilePosition::new(2, 3).get_block_id(), 3);
        assert_eq!(TilePosition::new(3, 3).get_block_id(), 3);
    }

    #[test]
    fn get_attribute_id_test() {
        let assert_attribute = |tile_range_x: Range<u8>, tile_range_y: Range<u8>, attribute_id: u8| {
            for x in tile_range_x {
                for y in tile_range_y.start..tile_range_y.end {
                    assert_eq!(TilePosition::new(x, y).get_attribute_id(), attribute_id);
                }
            }
        };

        // attribute: 0, block: 0, 1, 16, 17
        assert_attribute(0..4, 0..4, 0);

        // attribute: 1, block: 2, 3, 18, 19
        assert_attribute(4..8, 0..4, 1);

        // attribute: 8, block: 32, 33, 48, 49
        assert_attribute(0..4, 4..8, 8);

        // attribute: 9, block: 34, 35, 50, 51
        assert_attribute(4..8, 4..8, 9);
    }

    #[test]
    fn get_palette_id_test() {
        let attribute = 0b10110001; // 2 3 0 1

        // block: 0
        let tile_pos = TilePosition::new(0, 0);
        assert_eq!(tile_pos.get_palette_id(attribute), 1);

        // block: 1
        let tile_pos = TilePosition::new(2, 0);
        assert_eq!(tile_pos.get_palette_id(attribute), 0);

        // block: 2
        let tile_pos = TilePosition::new(0, 2);
        assert_eq!(tile_pos.get_palette_id(attribute), 3);

        // block: 3
        let tile_pos = TilePosition::new(2, 2);
        assert_eq!(tile_pos.get_palette_id(attribute), 2);
    }
}