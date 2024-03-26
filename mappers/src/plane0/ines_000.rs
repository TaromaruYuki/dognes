use super::super::{Mapper, MapperInfo};

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

    fn cpu_read(&self, mapped_info: &mut MapperInfo) -> bool {
        if mapped_info.addr >= 0x8000 {
            mapped_info.mapped_addr =
                mapped_info.addr & (if self.prog_banks > 1 { 0x7FFF } else { 0x3FFF });
            return true;
        }

        false
    }

    fn cpu_write(&self, mapped_info: &mut MapperInfo) -> bool {
        self.cpu_read(mapped_info) // It's the same read for write
    }

    fn ppu_read(&self, mapped_info: &mut MapperInfo) -> bool {
        if mapped_info.addr <= 0x1FFF {
            mapped_info.mapped_addr = mapped_info.addr;

            return true;
        }

        false
    }

    fn ppu_write(&self, mapped_info: &mut MapperInfo) -> bool {
        if mapped_info.addr <= 0x1FFF && self.char_banks == 0 {
            // Treat as RAM
            mapped_info.mapped_addr = mapped_info.addr;

            return true;
        }

        false
    }
}
