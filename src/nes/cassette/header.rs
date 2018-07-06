use super::CassetteInitializeError;

#[derive(Debug, PartialEq)]
pub struct INesHeader {
    /// ASCII letters 'NES' followed by 0x1A(EOF)
    pub magic_numbers: [u8; 4],
    /// Number of pages for The program rom
    pub prg_size: u8,
    /// Number of pages for The character rom
    pub chr_size: u8,
}

impl INesHeader {
    pub fn new(buf: &Vec<u8>) -> Result<Self, CassetteInitializeError> { 
        // <iNES file format header>
        // 0-3: Constant $4E $45 $53 $1A ("NES" followed by MS-DOS end-of-file)
        // 4: Size of PRG ROM in 16 KB units
        // 5: Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)
        // 
        // refer: https://wiki.nesdev.com/w/index.php/INES

        let magic_numbers = *array_ref!(buf, 0, 4);
        if &magic_numbers != "NES\x1A".as_bytes() {
            return Err(CassetteInitializeError::FormatError)
        };

        Ok(INesHeader {
            magic_numbers: magic_numbers,
            prg_size: buf[4],
            chr_size: buf[5],
        })
    }
}

#[cfg(test)]
mod ines_header_test {
    use super::*;

    #[test]
    fn new_success() {
        // "N" "E" "S" "\x1A" "5" "3"
        let rom_bytes = [78, 69, 83, 26, 53, 51];
        assert_eq!(rom_bytes, *"NES\x1A53".as_bytes());

        let ines_header = INesHeader::new(&rom_bytes.to_vec()).unwrap();
        assert_eq!(ines_header, INesHeader {
            magic_numbers: [
                rom_bytes[0],
                rom_bytes[1],
                rom_bytes[2],
                rom_bytes[3],
            ],
            prg_size: rom_bytes[4],
            chr_size: rom_bytes[5],
        });
    }

    #[test]
    fn new_format_error() {
        // "N" "N" "S" "\x1A" "5" "3"
        let rom_bytes = [78, 78, 83, 26, 53, 51];
        assert_eq!(rom_bytes, *"NNS\x1A53".as_bytes());

        let ines_header = INesHeader::new(&rom_bytes.to_vec());
        assert!(
            match ines_header {
                Err(CassetteInitializeError::FormatError) => true,
                _ => false,
            }
        );
    }
}