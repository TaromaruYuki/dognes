mod plane0;

pub trait Mapper: std::fmt::Debug {
    fn get_prog_banks(&self) -> u8;
    fn get_char_banks(&self) -> u8;

    fn cpu_read(&self, address: u16) -> Option<u16>;
    fn cpu_write(&self, address: u16) -> Option<u16>;

    fn ppu_read(&self, address: u16) -> Option<u16>;
    fn ppu_write(&self, address: u16) -> Option<u16>;
}

pub mod prelude {
    pub use crate::Mapper;

    pub use crate::plane0::ines_000::INES_000;
}
