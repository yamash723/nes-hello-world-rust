use std::fs::File;
use std::io;
use std::io::prelude::*;

pub mod header;

use super::cassette::header::INesHeader;

#[derive(Debug, PartialEq)]
pub struct Cassette {
    pub header: INesHeader,
    pub program_rom: Vec<u8>,
    pub character_rom: Vec<u8>,
}

impl Cassette {
    const HEADER_SIZE: usize = 0x0010;         // 16 byte
    const PROGRAM_UNIT_SIZE: usize = 0x4000;   // 16384 byte
    const CHARACTER_UNIT_SIZE: usize = 0x2000; // 8192 byte

    pub fn new(path: &str) -> Result<Self, CassetteInitializeError> {
        let rom_bytes = Self::load_rom_bytes(path)?;
        let header = INesHeader::new(&rom_bytes)?;
        
        // <iNES file format>
        // Header (16 bytes)
        // Trainer, if present (0 or 512 bytes)
        // PRG ROM data (16384 * x bytes)
        // CHR ROM data, if present (8192 * y bytes)
        // 
        // refer: https://wiki.nesdev.com/w/index.php/INES

        let program_rom = Self::split_program_rom(&header, &rom_bytes);
        let character_rom = Self::split_character_rom(&header, &rom_bytes);

        Ok(Self {
            header: header,
            program_rom: program_rom,
            character_rom: character_rom,
        })
    }

    fn load_rom_bytes(path: &str) -> Result<Vec<u8>, CassetteInitializeError> {
        let mut f = File::open(path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    fn split_program_rom(header: &INesHeader, buf: &Vec<u8>) -> Vec<u8> {
        let pos_from: usize = Self::HEADER_SIZE;
        let pos_to: usize = pos_from + header.prg_size as usize * Self::PROGRAM_UNIT_SIZE;

        buf[pos_from..pos_to].to_vec()
    }

    fn split_character_rom(header: &INesHeader, buf: &Vec<u8>) -> Vec<u8> {
        let pos_from: usize = Self::HEADER_SIZE + header.prg_size as usize * Self::PROGRAM_UNIT_SIZE;
        let pos_to: usize = pos_from + header.chr_size as usize * Self::CHARACTER_UNIT_SIZE;

        buf[pos_from..pos_to].to_vec()
    }
}

pub trait NesCassette {
    fn read_program_rom(&self, addr: u16) -> u8;
    fn read_character_rom(&self, addr: u16) -> u8;
    fn program_rom_length(&self) -> usize;
}

impl NesCassette for Cassette {
    fn read_program_rom(&self, addr: u16) -> u8 {
        self.program_rom[addr as usize]
    }

    fn read_character_rom(&self, addr: u16) -> u8 {
        self.character_rom[addr as usize]
    }

    fn program_rom_length(&self) -> usize {
        self.program_rom.len()
    }
}


#[derive(Debug)]
pub enum CassetteInitializeError {
    IoError(io::Error),
    /// Rom haven't magic number
    FormatError,
}

impl From<io::Error> for CassetteInitializeError {
    fn from(err: io::Error) -> Self {
        CassetteInitializeError::IoError(err)
    }
}

#[cfg(test)]
mod cassette_test {
    use super::*;

    #[test]
    fn new_success() {
        let path = "rom/hello_world.nes";
        let cassette = Cassette::new(path);
        assert!(
            match cassette {
                Ok(_) => true,
                _ => false,
            }
        );
    }

    #[test]
    fn test_faild_wrong_path() {
        let path = "rom/hello_world_wrong.nes";
        let cassette = Cassette::new(path);
        assert!(
            match cassette {
                Err(CassetteInitializeError::IoError(_)) => true,
                _ => false,
            }
        );
    }

    #[test]
    fn split_program_rom_test() {
        let test_program_rom = [
            vec![50; 0x4000],
            vec![51; 0x4000],
            vec![52; 0x4000],
            vec![53; 0x4000],
            vec![54; 0x4000],
        ].concat();

        let rom_bytes = [
            "NES\x1A".as_bytes().to_vec(),
            vec![5, 3],   // program / character rom page count
            vec![10; 10], // dummy header
            test_program_rom.clone(),
        ].concat();

        let header = INesHeader::new(&rom_bytes.clone()).unwrap();
        let program_rom = Cassette::split_program_rom(&header, &rom_bytes);
        
        assert_eq!(test_program_rom.to_vec(), program_rom);
    }

    #[test]
    fn split_character_rom_test() {
        let test_character_rom = [
            vec![50; 0x2000],
            vec![51; 0x2000],
            vec![52; 0x2000],
        ].concat();

        let rom_bytes = [
            // [NES\x1A1 "3"] is character page count
            "NES\x1A".as_bytes().to_vec(),
            vec![1, 3],       // program / character rom page count
            vec![10; 10],     // dummy header
            vec![49; 0x4000], // dummy program_rom page(1 page)
            test_character_rom.clone(),
        ].concat();

        let header = INesHeader::new(&rom_bytes.clone()).unwrap();
        let character_rom = Cassette::split_character_rom(&header, &rom_bytes);
        
        assert_eq!(test_character_rom.to_vec(), character_rom);
    }
}
