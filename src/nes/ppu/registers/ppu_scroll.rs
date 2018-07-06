pub struct PpuScroll {
    pub x: u8,
    pub y: u8,
    write_target_is_x: bool,
}

impl PpuScroll {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            write_target_is_x: true
        }
    }

    pub fn write(&mut self, data: u8) {
        if self.write_target_is_x {
            self.x = data;
        } else {
            self.y = data;
        };

        self.write_target_is_x = !self.write_target_is_x;
    }
}

#[cfg(test)]
mod ppu_scroll_test {
    use super::*;

    #[test]
    fn switching_target_at_write() {
        let mut ppu_scroll = PpuScroll::new();

        // write: x
        let x_pos = 0x10;
        ppu_scroll.write(x_pos);
        assert_eq!(ppu_scroll.x, x_pos);
        assert_eq!(ppu_scroll.y, 0x00);

        // write: y
        let y_pos = 0x20;
        ppu_scroll.write(y_pos);
        assert_eq!(ppu_scroll.x, x_pos);
        assert_eq!(ppu_scroll.y, y_pos);

        // write: x
        let new_x_pos = 0x30;
        ppu_scroll.write(new_x_pos);
        assert_eq!(ppu_scroll.x, new_x_pos);
        assert_eq!(ppu_scroll.y, y_pos);
    }
}