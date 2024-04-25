/*
    Most of the content here is from OLC's emulator and video series (EP 4/5)
    He explains EVERYTHING very well.
    GitHub: https://github.com/OneLoneCoder/olcNES/
    Youtube: https://www.youtube.com/@javidx9
*/
use bitflags::bitflags;
use std::{cell::RefCell, rc::Rc};

const PAL_WIDTH: i32 = 256;
const PAL_HEIGHT: i32 = 240;
pub const PAL_PALETTE: [(u8, u8, u8); 0x40] = [
    (0, 30, 116),
    (84, 84, 84),
    (8, 16, 144),
    (48, 0, 136),
    (68, 0, 100),
    (92, 0, 48),
    (84, 4, 0),
    (60, 24, 0),
    (32, 42, 0),
    (8, 58, 0),
    (0, 64, 0),
    (0, 60, 0),
    (0, 50, 60),
    (0, 0, 0),
    (0, 0, 0),
    (0, 0, 0),
    (152, 150, 152),
    (8, 76, 196),
    (48, 50, 236),
    (92, 30, 228),
    (136, 20, 176),
    (160, 20, 100),
    (152, 34, 32),
    (120, 60, 0),
    (84, 90, 0),
    (40, 114, 0),
    (8, 124, 0),
    (0, 118, 40),
    (0, 102, 120),
    (0, 0, 0),
    (0, 0, 0),
    (0, 0, 0),
    (236, 238, 236),
    (76, 154, 236),
    (120, 124, 236),
    (176, 98, 236),
    (228, 84, 236),
    (236, 88, 180),
    (236, 106, 100),
    (212, 136, 32),
    (160, 170, 0),
    (116, 196, 0),
    (76, 208, 32),
    (56, 204, 108),
    (56, 180, 204),
    (60, 60, 60),
    (0, 0, 0),
    (0, 0, 0),
    (236, 238, 236),
    (168, 204, 236),
    (188, 188, 236),
    (212, 178, 236),
    (236, 174, 236),
    (236, 174, 212),
    (236, 180, 176),
    (228, 196, 144),
    (204, 210, 120),
    (180, 222, 120),
    (168, 226, 144),
    (152, 226, 180),
    (160, 214, 228),
    (160, 162, 160),
    (0, 0, 0),
    (0, 0, 0),
];

pub type PPUBuf = [[u8; PAL_WIDTH as usize]; PAL_HEIGHT as usize];
pub type TableNameBuf = [[u8; 0x400]; 2];
pub type TablePatternBuf = [[u8; 0x1000]; 2];
pub type TablePalette = [u8; 32];

bitflags! {
    #[derive(Debug)]
    struct PPUStatus: u8 {
        const V_BLANK = 0b100_00000;
        const SPR_0_HIT = 0b010_00000;
        const SPR_OVERFLOW = 0b001_00000;
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct PPUMask: u8 {
        const GRAYSCALE = 0b0000_0001;
        const REND_BG_LEFT = 0b0000_0010;
        const REND_SPR_LEFT = 0b0000_0100;
        const REND_BG = 0b0000_1000;
        const REND_SPR = 0b0001_0000;
        const ENH_RED = 0b0010_0000;
        const ENH_GREEN = 0b0100_0000;
        const ENH_BLUE = 0b1000_0000;
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct PPUControl: u8 {
        const NAMETBL_X = 0b0000_0001;
        const NAMETBL_Y = 0b0100_0010;
        const INC_MODE = 0b0000_0100;
        const PTRN_SPR = 0b0000_1000;
        const PTRN_BG = 0b0001_0000;
        const SPR_SIZE = 0b0010_0000;
        const SLV_MODE = 0b0100_0000;
        const EN_NMI = 0b1000_0000;
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct LoopyReg {
    value: u16,
}

#[allow(dead_code)]
impl LoopyReg {
    pub fn get_data(&self) -> u16 {
        self.value
    }

    pub fn get_coarse_x(&self) -> u8 {
        (self.value & 0x1F) as u8
    }

    pub fn get_coarse_y(&self) -> u8 {
        ((self.value & 0x3E0) >> 5) as u8
    }

    pub fn get_nametable_x(&self) -> bool {
        (self.value & 0x400) > 0
    }

    pub fn get_nametable_y(&self) -> bool {
        (self.value & 0x800) > 0
    }

    pub fn get_fine_y(&self) -> u8 {
        ((self.value & 0x7000) >> 12) as u8
    }

    fn set_value(&mut self, value: u8, not_bitmask: u16, shift: u8) {
        self.value = (self.value & not_bitmask) | ((value as u16) << shift);
    }

    pub fn set_coarse_x(&mut self, value: u8) {
        self.set_value(value & 0x1F, 0xFFE0, 0);
    }

    pub fn set_coarse_y(&mut self, value: u8) {
        self.set_value(value & 0x1F, 0xFC1F, 5);
    }

    pub fn set_nametable_x(&mut self, value: bool) {
        self.set_value(value as u8, 0xFBFF, 10);
    }

    pub fn set_nametable_y(&mut self, value: bool) {
        self.set_value(value as u8, 0xF7FF, 11);
    }

    pub fn set_fine_y(&mut self, value: u8) {
        self.set_value(value & 7, 0x8FFF, 12);
    }

    pub fn increment_data(&mut self, inc_count: u8) {
        (self.value, _) = self.value.overflowing_add(inc_count as u16);
    }

    pub fn set_data(&mut self, data: u16) {
        self.value = data;
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct PPU {
    cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
    pub scanline: i16,
    pub cycle: i16,
    pub frame_complete: bool,
    pub buf: PPUBuf,
    pub nmi: bool,
    fine_x: u8,
    address_latch: u8,
    data_buffer: u8,
    status: PPUStatus,
    mask: PPUMask,
    control: PPUControl,
    vram_addr: LoopyReg,
    tram_addr: LoopyReg,
    table_name: TableNameBuf,
    table_pattern: TablePatternBuf,
    table_palette: TablePalette,

    bg_next_tile_id: u8,
    bg_next_tile_attrib: u8,
    bg_next_tile_lsb: u8,
    bg_next_tile_msb: u8,
    bg_shifter_pattern_lo: u16,
    bg_shifter_pattern_hi: u16,
    bg_shifter_attrib_lo: u16,
    bg_shifter_attrib_hi: u16,
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            cartridge: Option::default(),
            scanline: i16::default(),
            cycle: i16::default(),
            frame_complete: bool::default(),
            buf: [[0x0F; PAL_WIDTH as usize]; PAL_HEIGHT as usize],
            nmi: bool::default(),
            fine_x: u8::default(),
            address_latch: u8::default(),
            data_buffer: u8::default(),
            status: PPUStatus::empty(),
            mask: PPUMask::empty(),
            control: PPUControl::empty(),
            vram_addr: LoopyReg::default(),
            tram_addr: LoopyReg::default(),
            table_name: [[0; 0x400]; 2],
            table_pattern: [[0; 0x1000]; 2],
            table_palette: TablePalette::default(),

            bg_next_tile_id: 0x00,
            bg_next_tile_attrib: 0x00,
            bg_next_tile_lsb: 0x00,
            bg_next_tile_msb: 0x00,
            bg_shifter_pattern_lo: 0x0000,
            bg_shifter_pattern_hi: 0x0000,
            bg_shifter_attrib_lo: 0x0000,
            bg_shifter_attrib_hi: 0x0000,
        }
    }
}

impl PPU {
    pub fn attach_cart(&mut self, cart: Rc<RefCell<cartridge::Cartridge>>) {
        self.cartridge = Some(cart);
    }

    pub fn get_color(&self, palette: u8, pixel: u8) -> u8 {
        self.ppu_read(0x3F00 + ((palette as u16) << 2) + (pixel as u16)) & 0x3F
    }

    pub fn get_pattern_table(&self, i: bool, palette: u8) -> [[u8; 128]; 128] {
        let mut res = [[0_u8; 128]; 128];

        for tile_y in 0_u16..16 {
            for tile_x in 0_u16..16 {
                let offset: u16 = tile_y * 256 + tile_x * 16;

                for row in 0_u16..8 {
                    let mut tile_lsb = self.ppu_read((i as u16) * 0x1000 + offset + row);
                    let mut tile_msb = self.ppu_read((i as u16) * 0x1000 + offset + row + 0x0008);

                    for col in 0_u16..8 {
                        let pixel = (tile_lsb & 0x01) + (tile_msb & 0x01);

                        tile_lsb >>= 1;
                        tile_msb >>= 1;

                        res[(tile_y * 8 + row) as usize][(tile_x * 8 + (7 - col)) as usize] =
                            self.get_color(palette, pixel);
                    }
                }
            }
        }

        res
    }

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        match address {
            0x0000 => 0x00,
            0x0001 => 0x00,
            0x0002 => {
                let data: u8 = self.status.bits();
                self.status.set(PPUStatus::V_BLANK, false);
                self.address_latch = 0;

                data
            }
            0x0003 => 0x00,
            0x0004 => 0x00,
            0x0005 => 0x00,
            0x0006 => 0x00,
            0x0007 => {
                let mut data = self.data_buffer;
                self.data_buffer = self.ppu_read(self.vram_addr.get_data());

                if self.vram_addr.get_data() >= 0x3F00 {
                    data = self.data_buffer;
                }

                self.vram_addr
                    .increment_data(if self.control.contains(PPUControl::INC_MODE) {
                        32
                    } else {
                        1
                    });

                data
            }
            _ => panic!("Should never reach"),
        }
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) {
        match address {
            0x0000 => {
                self.control = PPUControl::from_bits_retain(data);
                self.tram_addr
                    .set_nametable_x(self.control.contains(PPUControl::NAMETBL_X));
                self.tram_addr
                    .set_nametable_y(self.control.contains(PPUControl::NAMETBL_Y));
            }
            0x0001 => {
                self.mask = PPUMask::from_bits_retain(data);
            }
            0x0002 => {}
            0x0003 => {}
            0x0004 => {}
            0x0005 => {
                if self.address_latch == 0 {
                    self.fine_x = data & 0x07;
                    self.tram_addr.set_coarse_x(data >> 3);
                    self.address_latch = 1;
                } else {
                    self.tram_addr.set_fine_y(data & 0x07);
                    self.tram_addr.set_coarse_x(data >> 3);
                    self.address_latch = 0;
                }
            }
            0x0006 => {
                if self.address_latch == 0 {
                    let data_set =
                        (((data & 0x3F) as u16) << 8) | (self.tram_addr.get_data() & 0x00FF);
                    self.tram_addr.set_data(data_set);
                    self.address_latch = 1;
                } else {
                    let data_set = (self.tram_addr.get_data() & 0xFF00) | data as u16;
                    self.tram_addr.set_data(data_set);
                    self.vram_addr = self.tram_addr;
                    self.address_latch = 0;
                }
            }
            0x0007 => {
                self.ppu_write(self.vram_addr.get_data(), data);

                self.vram_addr
                    .increment_data(if self.control.contains(PPUControl::INC_MODE) {
                        32
                    } else {
                        1
                    });
            }
            _ => panic!("Should never reach"),
        }
    }

    pub fn ppu_read(&self, mut address: u16) -> u8 {
        address &= 0x3FFF;

        if let Some(data) = self.cartridge.as_ref().unwrap().borrow().ppu_read(address) {
            return data;
        } else if (0x0000..=0x1FFF).contains(&address) {
            return self.table_pattern[((address & 0x1000) >> 12) as usize]
                [(address & 0x0FFF) as usize];
        } else if (0x2000..=0x3EFF).contains(&address) {
            address &= 0x0FFF;

            let cart = self.cartridge.as_ref().unwrap().borrow();
            let index_addr = (address & 0x03FF) as usize;

            match cart.mirror {
                cartridge::Mirror::Vertical => {
                    if (0x0000..=0x03FF).contains(&address) {
                        return self.table_name[0][index_addr];
                    } else if (0x0400..=0x07FF).contains(&address) {
                        return self.table_name[1][index_addr];
                    } else if (0x0800..=0x0BFF).contains(&address) {
                        return self.table_name[0][index_addr];
                    } else if (0x0C00..=0x0FFF).contains(&address) {
                        return self.table_name[1][index_addr];
                    }
                }
                cartridge::Mirror::Horizontal => {
                    if (0x0000..=0x07FF).contains(&address) {
                        return self.table_name[0][index_addr];
                    } else if (0x0800..=0x0FFF).contains(&address) {
                        return self.table_name[1][index_addr];
                    }
                }
                _ => todo!(),
            }
        } else if (0x3F00..=0x3FFF).contains(&address) {
            address &= 0x1F;
            address = if address == 0x10 {
                0x0
            } else if address == 0x14 {
                0x4
            } else if address == 0x18 {
                0x8
            } else if address == 0x1C {
                0xC
            } else {
                address
            };

            return self.table_palette[address as usize]
                & (if self.mask.contains(PPUMask::GRAYSCALE) {
                    0x30
                } else {
                    0x3F
                });
        }

        0x00
    }
    pub fn ppu_write(&mut self, mut address: u16, data: u8) {
        address &= 0x3FFF;

        if self
            .cartridge
            .as_mut()
            .unwrap()
            .borrow_mut()
            .ppu_write(address, data)
            .is_some()
        {
        } else if (0x0000..=0x1FFF).contains(&address) {
            self.table_pattern[((address & 0x1000) >> 12) as usize][(address & 0x0FFF) as usize] =
                data;
        } else if (0x2000..=0x3EFF).contains(&address) {
            address &= 0x0FFF;

            let cart = self.cartridge.as_ref().unwrap().borrow();
            let index_addr = (address & 0x03FF) as usize;

            match cart.mirror {
                cartridge::Mirror::Vertical => {
                    if (0x0000..=0x03FF).contains(&address) {
                        self.table_name[0][index_addr] = data;
                    } else if (0x0400..=0x07FF).contains(&address) {
                        self.table_name[1][index_addr] = data;
                    } else if (0x0800..=0x0BFF).contains(&address) {
                        self.table_name[0][index_addr] = data;
                    } else if (0x0C00..=0x0FFF).contains(&address) {
                        self.table_name[1][index_addr] = data;
                    }
                }
                cartridge::Mirror::Horizontal => {
                    if (0x0000..=0x07FF).contains(&address) {
                        self.table_name[0][index_addr] = data;
                    } else if (0x0800..=0x0FFF).contains(&address) {
                        self.table_name[1][index_addr] = data;
                    }
                }
                _ => todo!(),
            }
        } else if (0x3F00..=0x3FFF).contains(&address) {
            address &= 0x1F;
            address = if address == 0x10 {
                0x0
            } else if address == 0x14 {
                0x4
            } else if address == 0x18 {
                0x8
            } else if address == 0x1C {
                0xC
            } else {
                address
            };

            self.table_palette[address as usize] = data;
        }
    }

    #[allow(dead_code)]
    pub fn tick(&mut self) {
        fn inc_scroll_x(ppu: &mut PPU) {
            if !ppu.mask.contains(PPUMask::REND_BG) || !ppu.mask.contains(PPUMask::REND_SPR) {
                return;
            }

            if ppu.vram_addr.get_coarse_x() == 31 {
                ppu.vram_addr.set_coarse_x(0);
                ppu.vram_addr
                    .set_nametable_x(!ppu.vram_addr.get_nametable_x());
            } else {
                ppu.vram_addr.set_coarse_x(ppu.vram_addr.get_coarse_x() + 1);
            }
        }

        fn inc_scroll_y(ppu: &mut PPU) {
            if !ppu.mask.contains(PPUMask::REND_BG) || !ppu.mask.contains(PPUMask::REND_SPR) {
                return;
            }

            if ppu.vram_addr.get_fine_y() < 7 {
                ppu.vram_addr.set_fine_y(ppu.vram_addr.get_fine_y() + 1);
            } else {
                ppu.vram_addr.set_fine_y(0);
                let cy = ppu.vram_addr.get_coarse_y();

                if cy == 29 {
                    ppu.vram_addr.set_coarse_y(0);
                    ppu.vram_addr
                        .set_nametable_y(!ppu.vram_addr.get_nametable_y());
                } else if cy == 31 {
                    ppu.vram_addr.set_coarse_y(0);
                } else {
                    ppu.vram_addr.set_coarse_y(ppu.vram_addr.get_coarse_y() + 1);
                }
            }
        }

        fn mv_addr_x(ppu: &mut PPU) {
            if !ppu.mask.contains(PPUMask::REND_BG) || !ppu.mask.contains(PPUMask::REND_SPR) {
                return;
            }

            ppu.vram_addr
                .set_nametable_x(ppu.tram_addr.get_nametable_x());
            ppu.vram_addr.set_coarse_x(ppu.tram_addr.get_coarse_x());
        }

        fn mv_addr_y(ppu: &mut PPU) {
            if !ppu.mask.contains(PPUMask::REND_BG) || !ppu.mask.contains(PPUMask::REND_SPR) {
                return;
            }

            ppu.vram_addr.set_fine_y(ppu.tram_addr.get_fine_y());
            ppu.vram_addr
                .set_nametable_y(ppu.tram_addr.get_nametable_y());
            ppu.vram_addr.set_coarse_y(ppu.tram_addr.get_coarse_y());
        }

        fn load_bg_shifters(ppu: &mut PPU) {
            ppu.bg_shifter_pattern_lo =
                (ppu.bg_shifter_pattern_lo & 0xFF00) | (ppu.bg_next_tile_lsb as u16);
            ppu.bg_shifter_pattern_hi =
                (ppu.bg_shifter_pattern_hi & 0xFF00) | (ppu.bg_next_tile_msb as u16);

            ppu.bg_shifter_attrib_lo = (ppu.bg_shifter_attrib_lo & 0xFF00)
                | (if (ppu.bg_next_tile_attrib & 0b01) > 0 {
                    0xFF
                } else {
                    0x00
                });
            ppu.bg_shifter_attrib_hi = (ppu.bg_shifter_attrib_hi & 0xFF00)
                | (if (ppu.bg_next_tile_attrib & 0b10) > 0 {
                    0xFF
                } else {
                    0x00
                });
        }

        fn update_shifters(ppu: &mut PPU) {
            if !ppu.mask.contains(PPUMask::REND_BG) {
                return;
            }

            ppu.bg_shifter_pattern_lo <<= 1;
            ppu.bg_shifter_pattern_hi <<= 1;

            ppu.bg_shifter_attrib_lo <<= 1;
            ppu.bg_shifter_attrib_hi <<= 1;
        }

        if (-1..240).contains(&self.scanline) {
            if self.scanline == 0 && self.cycle == 0 {
                self.cycle = 1;
            }

            if self.scanline == -1 && self.cycle == 0 {
                self.status.set(PPUStatus::V_BLANK, false);
            }

            if (2..258).contains(&self.cycle) || (321..338).contains(&self.cycle) {
                update_shifters(self);

                match (self.cycle - 1) % 8 {
                    0 => {
                        load_bg_shifters(self);
                        self.bg_next_tile_id =
                            self.ppu_read(0x2000 | (self.vram_addr.get_data() & 0x0FFF));
                    }
                    2 => {
                        let address = 0x23C0
                            | ((self.vram_addr.get_nametable_y() as u16) << 11)
                            | ((self.vram_addr.get_nametable_x() as u16) << 10)
                            | (((self.vram_addr.get_coarse_y() as u16) >> 2) << 3)
                            | ((self.vram_addr.get_coarse_x() as u16) >> 2);

                        self.bg_next_tile_attrib = self.ppu_read(address);

                        if (self.vram_addr.get_coarse_y() & 0x02) > 0 {
                            self.bg_next_tile_attrib >>= 4;
                        }

                        if (self.vram_addr.get_coarse_x() & 0x02) > 0 {
                            self.bg_next_tile_attrib >>= 2;
                        }

                        self.bg_next_tile_attrib &= 0x03;
                    }
                    4 => {
                        let address = ((self.control.contains(PPUControl::PTRN_BG) as u16) << 12)
                            + ((self.bg_next_tile_id as u16) << 4)
                            + (self.vram_addr.get_fine_y() as u16);

                        self.bg_next_tile_lsb = self.ppu_read(address);
                    }
                    6 => {
                        let address = ((self.control.contains(PPUControl::PTRN_BG) as u16) << 12)
                            + ((self.bg_next_tile_id as u16) << 4)
                            + ((self.vram_addr.get_fine_y() as u16) + 8);

                        self.bg_next_tile_msb = self.ppu_read(address);
                    }
                    7 => inc_scroll_x(self),
                    _ => {}
                }
            }

            if self.cycle == 256 {
                inc_scroll_y(self);
            }

            if self.cycle == 257 {
                load_bg_shifters(self);
                mv_addr_x(self);
            }

            if self.cycle == 338 || self.cycle == 340 {
                self.bg_next_tile_id = self.ppu_read(0x2000 | (self.vram_addr.get_data() & 0x0FFF));
            }

            if self.scanline == -1 && (280..305).contains(&self.cycle) {
                mv_addr_y(self);
            }
        }

        if self.scanline == 241 && self.cycle == 1 {
            self.status.set(PPUStatus::V_BLANK, true);

            if self.control.contains(PPUControl::EN_NMI) {
                self.nmi = true;
            }
        }

        // Composition

        let mut bg_pixel = 0x00_u8;
        let mut bg_palette = 0x00_u8;

        if self.mask.contains(PPUMask::REND_BG) {
            let bit_mux = 0x8000 >> self.fine_x;

            let p0_pixel = ((self.bg_shifter_pattern_lo & bit_mux) > 0) as u8;
            let p1_pixel = ((self.bg_shifter_pattern_hi & bit_mux) > 0) as u8;
            bg_pixel = (p1_pixel << 1) | p0_pixel;

            let bg_pal0 = ((self.bg_shifter_attrib_lo & bit_mux) > 0) as u8;
            let bg_pal1 = ((self.bg_shifter_attrib_hi & bit_mux) > 0) as u8;
            bg_palette = (bg_pal1 << 1) | bg_pal0;
        }

        let (x, y) = ((self.cycle - 1) as i32, self.scanline as i32);

        // sprScreen->SetPixel(cycle - 1, scanline, GetColourFromPaletteRam(bg_palette, bg_pixel));
        if (0..PAL_WIDTH).contains(&x) && (0..PAL_HEIGHT).contains(&y) {
            self.buf[y as usize][x as usize] = self.get_color(bg_palette, bg_pixel);
        }

        self.cycle += 1;
        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline >= 261 {
                self.scanline = -1;
                self.frame_complete = true;
            }
        }
    }
}
