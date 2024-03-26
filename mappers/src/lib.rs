mod plane0;

pub trait Mapper {
    fn get_prog_banks(&self) -> u8;
    fn get_char_banks(&self) -> u8;

    fn cpu_read(&self, mapped_info: &mut MapperInfo) -> bool;
    fn cpu_write(&self, mapped_info: &mut MapperInfo) -> bool;

    fn ppu_read(&self, mapped_info: &mut MapperInfo) -> bool;
    fn ppu_write(&self, mapped_info: &mut MapperInfo) -> bool;
}

pub struct MapperInfo {
    pub addr: u16,
    pub mapped_addr: u16,
}

impl MapperInfo {
    pub fn new(addr: u16) -> Self {
        Self {
            addr,
            mapped_addr: 0,
        }
    }
}

pub mod prelude {
    pub use crate::Mapper;
    pub use crate::MapperInfo;

    pub use crate::plane0::ines_000::INES_000;
}
