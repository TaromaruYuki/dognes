use std::{
    fs::File,
    io::{Read, Seek},
};

use mappers::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Mirror {
    Horizontal,
    Vertical,
    OnescreenLo,
    OnescreenHi,
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug)]
struct iNESHeader {
    name: [u8; 4],
    prog_rom_chunks: u8,
    char_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prog_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    unused: [u8; 5],
}

impl iNESHeader {
    pub fn from_array(content: [u8; 16]) -> Self {
        Self {
            name: [content[0], content[1], content[2], content[3]],
            prog_rom_chunks: content[4],
            char_rom_chunks: content[5],
            mapper1: content[6],
            mapper2: content[7],
            prog_ram_size: content[8],
            tv_system1: content[9],
            tv_system2: content[10],
            unused: [
                content[11],
                content[12],
                content[13],
                content[14],
                content[15],
            ],
        }
    }
}

#[derive(Debug)]
pub struct Cartridge {
    pub mapper: Box<dyn Mapper>,

    pub mapper_id: u8,
    pub prog_banks: u8,
    pub char_banks: u8,

    pub prog_mem: Vec<u8>,
    pub char_mem: Vec<u8>,

    pub mirror: Mirror,
}

impl Cartridge {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(file_name: String) -> Self {
        let mut file = File::open(file_name).unwrap();

        let mut header_raw: [u8; 16] = [0; 16];
        file.read_exact(&mut header_raw).unwrap();

        let header = iNESHeader::from_array(header_raw);
        println!("{:#?}", header);

        // ignore trainer
        if header.mapper1 & 0x04 > 0 {
            println!("Ignoring trainer");
            file.seek(std::io::SeekFrom::Current(512)).unwrap();
        }

        let mapper_id = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);
        let mirror = if header.mapper1 & 0x01 > 0 {
            Mirror::Vertical
        } else {
            Mirror::Horizontal
        };
        println!(
            "Mapper ID: {mapper_id}\nMirror: {}",
            if mirror == Mirror::Vertical {
                "Vertical"
            } else {
                "Horizontal"
            }
        );

        let prog_banks = header.prog_rom_chunks;
        let char_banks = header.char_rom_chunks;
        println!("Program Banks: {prog_banks}\nChar Banks: {char_banks}");

        let mut prog_mem: Vec<u8> = vec![0; (prog_banks as usize) * 0x4000];
        file.read_exact(&mut prog_mem).unwrap();

        let mut char_mem: Vec<u8> = vec![0; (char_banks as usize) * 0x2000];
        file.read_exact(&mut char_mem).unwrap();

        let mapper = match mapper_id {
            0 => Box::new(INES_000::new(prog_banks, char_banks)),
            _ => todo!(),
        };

        Self {
            mapper,
            mapper_id,
            prog_banks,
            char_banks,
            prog_mem,
            char_mem,
            mirror,
        }
    }

    pub fn cpu_read(&self, address: u16) -> Option<u8> {
        if let Some(mapped_addr) = self.mapper.cpu_read(address) {
            return Some(self.prog_mem[mapped_addr as usize]);
        }

        None
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) -> Option<()> {
        if let Some(mapped_addr) = self.mapper.cpu_write(address) {
            self.prog_mem[mapped_addr as usize] = data;

            return Some(());
        }

        None
    }

    pub fn ppu_read(&self, address: u16) -> Option<u8> {
        if let Some(mapped_addr) = self.mapper.ppu_read(address) {
            return Some(self.char_mem[mapped_addr as usize]);
        }

        None
    }

    pub fn ppu_write(&mut self, address: u16, data: u8) -> Option<()> {
        if let Some(mapped_addr) = self.mapper.ppu_write(address) {
            self.char_mem[mapped_addr as usize] = data;

            return Some(());
        }

        None
    }
}
