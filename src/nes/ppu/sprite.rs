use nes::ram::Ram;

#[derive(Debug)]
pub struct Sprite(Vec<Vec<u8>>);

const SPRITE_BYTES_LENGTH: usize = 16;
const CHANNEL_BYTES_LENGTH: usize = 8;

impl Sprite {
    pub fn build(sprite_number: u8, cram: &Ram) -> Self {
        let start_idx = sprite_number as usize * SPRITE_BYTES_LENGTH;
        let end_idx = start_idx + SPRITE_BYTES_LENGTH;

        let bytes = cram.read_range(start_idx..end_idx);
        let channel_1 = &bytes[0..CHANNEL_BYTES_LENGTH];
        let channel_2 = &bytes[CHANNEL_BYTES_LENGTH..SPRITE_BYTES_LENGTH];

        let overlaped_bytes = Sprite::overlap_two_channel(channel_1, channel_2);
        Sprite(overlaped_bytes)
    }

    fn overlap_two_channel(channel_1: &[u8], channel_2: &[u8]) -> Vec<Vec<u8>> {
        let mut overlapped_channel = vec![vec![0u8; CHANNEL_BYTES_LENGTH]; CHANNEL_BYTES_LENGTH];

        for y in 0..CHANNEL_BYTES_LENGTH {
            for x in 0..CHANNEL_BYTES_LENGTH {
                let shift = CHANNEL_BYTES_LENGTH - x - 1;
                let get_target_bit = |byte: u8| -> u8 {
                    let mask = 0b10000000 >> x;
                    (byte & mask) >> shift
                };

                let p1 = get_target_bit(channel_1[y]);     // channel_1: 1
                let p2 = get_target_bit(channel_2[y]) * 2; // channel_2: 2
                overlapped_channel[y][x] = p1 + p2;
            }
        }

        overlapped_channel
    }

    pub fn to_vec(&self) -> &Vec<Vec<u8>> {
        &self.0
    }
}

#[cfg(test)]
mod sprite_test {
    use super::*;

    #[test]
    fn overlap_two_channel_test() {
        let channel_1 = &[
            0b11111000,
            0b11111000,
            0b11111000,
            0b11111000,
            0b11111000,
            0b00000000,
            0b00000000,
            0b00000000,
        ];

        let channel_2 = &[
            0b00000000,
            0b00000000,
            0b00000000,
            0b00011111,
            0b00011111,
            0b00011111,
            0b00011111,
            0b00011111,
        ];

        let overlapped_channel = Sprite::overlap_two_channel(channel_1, channel_2);

        assert_eq!(overlapped_channel, vec![
            vec![1,1,1,1,1,0,0,0],
            vec![1,1,1,1,1,0,0,0],
            vec![1,1,1,1,1,0,0,0],
            vec![1,1,1,3,3,2,2,2],
            vec![1,1,1,3,3,2,2,2],
            vec![0,0,0,2,2,2,2,2],
            vec![0,0,0,2,2,2,2,2],
            vec![0,0,0,2,2,2,2,2],
        ]);
    }

    #[test]
    fn build_test() {
        let bytes = vec![
            // sprite number: 0
            0b11111000,
            0b11111000,
            0b11111000,
            0b11111000,
            0b11111000,
            0b00000000,
            0b00000000,
            0b00000000,

            0b00000000,
            0b00000000,
            0b00000000,
            0b00011111,
            0b00011111,
            0b00011111,
            0b00011111,
            0b00011111,

            // sprite number: 1
            0b00000000,
            0b00000000,
            0b00000000,
            0b00011111,
            0b00011111,
            0b00011111,
            0b00011111,
            0b00011111,

            0b11111000,
            0b11111000,
            0b11111000,
            0b11111000,
            0b11111000,
            0b00000000,
            0b00000000,
            0b00000000,
        ];

        let cram = Ram::new(bytes);

        // sprite: 0
        let sprite = Sprite::build(0, &cram);
        assert_eq!(sprite.to_vec(), &vec![
            vec![1,1,1,1,1,0,0,0],
            vec![1,1,1,1,1,0,0,0],
            vec![1,1,1,1,1,0,0,0],
            vec![1,1,1,3,3,2,2,2],
            vec![1,1,1,3,3,2,2,2],
            vec![0,0,0,2,2,2,2,2],
            vec![0,0,0,2,2,2,2,2],
            vec![0,0,0,2,2,2,2,2],
        ]);

        // sprite: 1
        let sprite = Sprite::build(1, &cram);
        assert_eq!(sprite.to_vec(), &vec![
            vec![2,2,2,2,2,0,0,0],
            vec![2,2,2,2,2,0,0,0],
            vec![2,2,2,2,2,0,0,0],
            vec![2,2,2,3,3,1,1,1],
            vec![2,2,2,3,3,1,1,1],
            vec![0,0,0,1,1,1,1,1],
            vec![0,0,0,1,1,1,1,1],
            vec![0,0,0,1,1,1,1,1],
        ]);
    }
}