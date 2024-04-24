use super::super::Mapper;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct INES_000 {
    prog_banks: u8,
    char_banks: u8,
}

impl INES_000 {
    pub fn new(prog_banks: u8, char_banks: u8) -> Self {
        Self {
            prog_banks,
            char_banks,
        }
    }
}

impl Mapper for INES_000 {
    fn get_prog_banks(&self) -> u8 {
        self.prog_banks
    }

    fn get_char_banks(&self) -> u8 {
        self.char_banks
    }

    fn cpu_read(&self, address: u16) -> Option<u16> {
        if address >= 0x8000 {
            return Some(address & (if self.prog_banks > 1 { 0x7FFF } else { 0x3FFF }));
        }

        None
    }

    fn cpu_write(&self, address: u16) -> Option<u16> {
        self.cpu_read(address) // It's the same read for write
    }

    fn ppu_read(&self, address: u16) -> Option<u16> {
        if address <= 0x1FFF {
            return Some(address);
        }

        None
    }

    fn ppu_write(&self, address: u16) -> Option<u16> {
        if address <= 0x1FFF && self.char_banks == 0 {
            // Treat as RAM
            return Some(address);
        }

        None
    }
}
