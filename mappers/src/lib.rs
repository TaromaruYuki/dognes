mod plane0;

trait Mapper {
    fn new(prog_banks: u8, char_banks: u8) -> Self;
    fn get_prog_banks(&self) -> u8;
    fn get_char_banks(&self) -> u8;

    fn cpu_read(&self, mapped_info: &mut MapperInfo) -> bool;
    fn cpu_write(&self, mapped_info: &mut MapperInfo) -> bool;

    fn ppu_read(&self, mapped_info: &mut MapperInfo) -> bool;
    fn ppu_write(&self, mapped_info: &mut MapperInfo) -> bool;
}

struct MapperInfo {
    addr: u16,
    mapped_addr: u16,
}
