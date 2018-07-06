use std::ops::Range;

pub struct Ram {
    pub buf: Vec<u8>,
}

impl Ram {
    pub fn new(buf: Vec<u8>) -> Self {
        Self {
            buf: buf,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.buf[addr as usize]
    }

    pub fn read_range(&self, range: Range<usize>) -> &[u8] {
        &self.buf[range]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.buf[addr as usize] = data;
    }
}

#[cfg(test)]
mod ram_test {
    use super::*;

    #[test]
    fn can_be_read_and_write_test() {
        let buf = vec![0x00; 2024];
        let mut ram = Ram::new(buf);
        ram.write(0x0000, 0x4F);

        assert_eq!(ram.read(0x0000), 0x4F);
    }

    #[test]
    fn read_range_test() {
        let buf = vec![0x00; 2024];
        let mut ram = Ram::new(buf);
        ram.write(0x0000, 0x00);
        ram.write(0x0001, 0x01);
        ram.write(0x0002, 0x02);

        assert_eq!(ram.read_range(0..3), &[0x00, 0x01, 0x02]);
    }
}
