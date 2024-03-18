use crate::{addressing, opcode, Clock, Counter, Memory};
use bitflags::bitflags;

mod and;
mod eor;
mod lda;
mod ldx;
mod ldy;
mod pha;
mod php;
mod pla;
mod plp;
mod sta;
mod stx;
mod sty;
mod tax;
mod tay;
mod tsx;
mod txa;
mod txs;
mod tya;

bitflags! {
    pub struct StatusFlag: u8 {
        const C = 0b00000001;
        const Z = 0b00000010;
        const I = 0b00000100;
        const D = 0b00001000;
        const B = 0b00010000;
        const V = 0b01000000;
        const N = 0b10000000;
    }
}

impl Default for StatusFlag {
    fn default() -> Self {
        StatusFlag::empty()
    }
}

#[derive(Default, Clone, Copy)]
pub enum CPUState {
    #[default]
    Fetch,
    Execute,
    Halted,
}

impl ToString for CPUState {
    fn to_string(&self) -> String {
        match *self {
            CPUState::Fetch => "F".to_string(),
            CPUState::Execute => "E".to_string(),
            CPUState::Halted => "H".to_string(),
        }
    }
}

#[derive(Default)]
pub enum ReadWrite {
    #[default]
    R,
    W,
}

impl ToString for ReadWrite {
    fn to_string(&self) -> String {
        match *self {
            ReadWrite::R => "R".to_string(),
            ReadWrite::W => "W".to_string(),
        }
    }
}

#[derive(Default)]
pub struct CPUPins {
    pub address: u16,
    pub data: u8,
    pub rw: ReadWrite,
}

#[derive(Default)]
pub struct CPUData {
    pub pins: CPUPins,
    pub mem: Memory,
    pub clock: Clock,
    pub state: CPUState,
}

#[allow(dead_code)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

#[derive(Default)]
pub struct CPU {
    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub ps: StatusFlag,
    state: CPUState,
    counter: Counter,
    opcode: u8,
    pub temp8: u8,
    pub temp16: u16,
    pub tempb: bool,
}

impl CPU {
    pub fn reset(&mut self, data: &mut CPUData) {
        self.pc = 0xFFFC;
        self.sp = 0xFF;
        self.ps = StatusFlag::empty();
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.state = CPUState::Fetch;
        data.mem.reset();
        self.counter.value = 0;
    }

    pub fn tick(&mut self, data: &mut CPUData) {
        if data.clock.state {
            match self.state {
                CPUState::Halted => return,
                CPUState::Fetch => self.fetch(data),
                CPUState::Execute => self.execute(data),
            }
            self.counter.tick(&data.clock);
        }

        data.state = self.state;
    }

    fn instruction_finish(&mut self) {
        self.state = CPUState::Fetch;
        self.counter.reset();
    }

    fn fetch(&mut self, data: &mut CPUData) {
        match self.counter.value {
            0 => {
                data.pins.address = self.pc;
                data.pins.rw = ReadWrite::R;
                self.pc += 1;
            }
            1 => {
                self.opcode = data.pins.data;
                self.state = CPUState::Execute;
                self.counter.reset();
            }
            _ => panic!("Should never reach."),
        }
    }

    pub fn run_instruction(&mut self, map: addressing::CaseHashMap, data: &mut CPUData) {
        (map[&self.counter.value])(self, data);
    }

    fn execute(&mut self, data: &mut CPUData) {
        match self.opcode {
            opcode::LDA_IM => self.LDA(AddressingMode::Immediate, data),
            opcode::LDA_ZP => self.LDA(AddressingMode::ZeroPage, data),
            opcode::LDA_ZPX => self.LDA(AddressingMode::ZeroPageX, data),
            opcode::LDA_ABS => self.LDA(AddressingMode::Absolute, data),
            opcode::LDA_ABX => self.LDA(AddressingMode::AbsoluteX, data),
            opcode::LDA_ABY => self.LDA(AddressingMode::AbsoluteY, data),
            opcode::LDA_INX => self.LDA(AddressingMode::IndexedIndirect, data),
            opcode::LDA_INY => self.LDA(AddressingMode::IndirectIndexed, data),

            opcode::LDX_IM => self.LDX(AddressingMode::Immediate, data),
            opcode::LDX_ZP => self.LDX(AddressingMode::ZeroPage, data),
            opcode::LDX_ZPY => self.LDX(AddressingMode::ZeroPageY, data),
            opcode::LDX_ABS => self.LDX(AddressingMode::Absolute, data),
            opcode::LDX_ABY => self.LDX(AddressingMode::AbsoluteY, data),

            opcode::LDY_IM => self.LDY(AddressingMode::Immediate, data),
            opcode::LDY_ZP => self.LDY(AddressingMode::ZeroPage, data),
            opcode::LDY_ZPX => self.LDY(AddressingMode::ZeroPageX, data),
            opcode::LDY_ABS => self.LDY(AddressingMode::Absolute, data),
            opcode::LDY_ABX => self.LDY(AddressingMode::AbsoluteX, data),

            opcode::STA_ZP => self.STA(AddressingMode::ZeroPage, data),
            opcode::STA_ZPX => self.STA(AddressingMode::ZeroPageX, data),
            opcode::STA_ABS => self.STA(AddressingMode::Absolute, data),
            opcode::STA_ABX => self.STA(AddressingMode::AbsoluteX, data),
            opcode::STA_ABY => self.STA(AddressingMode::AbsoluteY, data),
            opcode::STA_INX => self.STA(AddressingMode::IndexedIndirect, data),
            opcode::STA_INY => self.STA(AddressingMode::IndirectIndexed, data),

            opcode::STX_ZP => self.STX(AddressingMode::ZeroPage, data),
            opcode::STX_ZPY => self.STX(AddressingMode::ZeroPageY, data),
            opcode::STX_ABS => self.STX(AddressingMode::Absolute, data),

            opcode::STY_ZP => self.STY(AddressingMode::ZeroPage, data),
            opcode::STY_ZPX => self.STY(AddressingMode::ZeroPageX, data),
            opcode::STY_ABS => self.STY(AddressingMode::Absolute, data),

            opcode::TAX => self.TAX(data),
            opcode::TAY => self.TAY(data),
            opcode::TXA => self.TXA(data),
            opcode::TYA => self.TYA(data),
            opcode::TSX => self.TSX(data),
            opcode::TXS => self.TXS(data),

            opcode::PHA => self.PHA(data),
            opcode::PHP => self.PHP(data),
            opcode::PLA => self.PLA(data),
            opcode::PLP => self.PLP(data),

            opcode::AND_IM => self.AND(AddressingMode::Immediate, data),
            opcode::AND_ZP => self.AND(AddressingMode::ZeroPage, data),
            opcode::AND_ZPX => self.AND(AddressingMode::ZeroPageX, data),
            opcode::AND_ABS => self.AND(AddressingMode::Absolute, data),
            opcode::AND_ABX => self.AND(AddressingMode::AbsoluteX, data),
            opcode::AND_ABY => self.AND(AddressingMode::AbsoluteY, data),
            opcode::AND_INX => self.AND(AddressingMode::IndexedIndirect, data),
            opcode::AND_INY => self.AND(AddressingMode::IndirectIndexed, data),

            opcode::EOR_IM => self.EOR(AddressingMode::Immediate, data),
            opcode::EOR_ZP => self.EOR(AddressingMode::ZeroPage, data),
            opcode::EOR_ZPX => self.EOR(AddressingMode::ZeroPageX, data),
            opcode::EOR_ABS => self.EOR(AddressingMode::Absolute, data),
            opcode::EOR_ABX => self.EOR(AddressingMode::AbsoluteX, data),
            opcode::EOR_ABY => self.EOR(AddressingMode::AbsoluteY, data),
            opcode::EOR_INX => self.EOR(AddressingMode::IndexedIndirect, data),
            opcode::EOR_INY => self.EOR(AddressingMode::IndirectIndexed, data),

            opcode::JMP_ABS => match self.counter.value {
                0 => {
                    data.pins.address = self.pc;
                    data.pins.rw = ReadWrite::R;
                    self.pc += 1;
                }
                1 => {
                    self.temp16 = data.pins.data as u16;
                    data.pins.address = self.pc;
                    data.pins.rw = ReadWrite::R;
                }
                2 => {
                    let addr: u16 = self.temp16 | ((data.pins.data as u16) << 8);
                    self.pc = addr;
                    self.instruction_finish();
                }
                _ => panic!("Should never reach."),
            },
            _ => todo!("Opcode {}", self.opcode),
        }
    }
}
