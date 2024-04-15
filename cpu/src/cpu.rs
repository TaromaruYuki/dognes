use std::{cell::RefCell, rc::Rc};

use crate::{addressing, opcode, Counter};
use bitflags::bitflags;

mod adc;
mod and;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod bpl;
mod brk;
mod bvc;
mod bvs;
mod clc;
mod cld;
mod cli;
mod clv;
mod cmp;
mod cpx;
mod cpy;
mod dec;
mod dex;
mod dey;
mod eor;
mod inc;
mod inx;
mod iny;
mod jmp;
mod jsr;
mod lda;
mod ldx;
mod ldy;
mod lsr;
mod nop;
mod ora;
mod pha;
mod php;
mod pla;
mod plp;
mod rol;
mod ror;
mod rti;
mod rts;
mod sbc;
mod sec;
mod sed;
mod sei;
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
    #[derive(Clone, Debug)]
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

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum CPUState {
    #[default]
    Fetch,
    Execute,
    Halted,
    Interrupt,
    Reset,
}

impl ToString for CPUState {
    fn to_string(&self) -> String {
        match *self {
            CPUState::Fetch => "F".to_string(),
            CPUState::Execute => "E".to_string(),
            CPUState::Halted => "H".to_string(),
            CPUState::Interrupt => "I".to_string(),
            CPUState::Reset => "R".to_string(),
        }
    }
}

#[derive(Default, Debug)]
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

#[derive(Default, Debug)]
pub struct CPUPins {
    pub address: u16,
    pub data: u8,
    pub rw: ReadWrite,
}

#[derive(Default, Debug)]
pub struct CPUData {
    pub pins: CPUPins,
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

#[derive(Default, Debug)]
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
    pub fn reset(&mut self) {
        self.sp = 0xFF;
        self.ps = StatusFlag::empty();
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.state = CPUState::Reset;
        self.counter.value = 0;
    }

    pub fn get_state(&self) -> CPUState {
        self.state
    }

    pub fn is_complete(&self) -> bool {
        self.state == CPUState::Fetch && self.counter.value == 0
    }

    pub fn tick(&mut self, data: &mut CPUData) {
        match self.state {
            CPUState::Halted => return,
            CPUState::Fetch => self.fetch(data),
            CPUState::Execute => self.execute(data),
            CPUState::Interrupt => {
                if self.ps.contains(StatusFlag::I) {
                    self.instruction_finish();
                }

                let map = self.irq();

                self.run_instruction(map, data);
            }
            CPUState::Reset => self.reset_state(data),
        }
        self.counter.tick();

        data.state = self.state;
    }

    pub(crate) fn instruction_finish(&mut self) {
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
        (map.get(&self.counter.value).unwrap_or_else(|| {
            panic!(
                "Failed to get instruction cycle '{}' with opcode '{}'",
                self.counter.value, self.opcode
            )
        }))(self, data);
        // (map[&self.counter.value])(self, data);
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

            opcode::ORA_IM => self.ORA(AddressingMode::Immediate, data),
            opcode::ORA_ZP => self.ORA(AddressingMode::ZeroPage, data),
            opcode::ORA_ZPX => self.ORA(AddressingMode::ZeroPageX, data),
            opcode::ORA_ABS => self.ORA(AddressingMode::Absolute, data),
            opcode::ORA_ABX => self.ORA(AddressingMode::AbsoluteX, data),
            opcode::ORA_ABY => self.ORA(AddressingMode::AbsoluteY, data),
            opcode::ORA_INX => self.ORA(AddressingMode::IndexedIndirect, data),
            opcode::ORA_INY => self.ORA(AddressingMode::IndirectIndexed, data),

            opcode::BIT_ZP => self.BIT(AddressingMode::ZeroPage, data),
            opcode::BIT_ABS => self.BIT(AddressingMode::Absolute, data),

            opcode::ADC_IM => self.ADC(AddressingMode::Immediate, data),
            opcode::ADC_ZP => self.ADC(AddressingMode::ZeroPage, data),
            opcode::ADC_ZPX => self.ADC(AddressingMode::ZeroPageX, data),
            opcode::ADC_ABS => self.ADC(AddressingMode::Absolute, data),
            opcode::ADC_ABX => self.ADC(AddressingMode::AbsoluteX, data),
            opcode::ADC_ABY => self.ADC(AddressingMode::AbsoluteY, data),
            opcode::ADC_INX => self.ADC(AddressingMode::IndexedIndirect, data),
            opcode::ADC_INY => self.ADC(AddressingMode::IndirectIndexed, data),

            opcode::SBC_IM => self.SBC(AddressingMode::Immediate, data),
            opcode::SBC_ZP => self.SBC(AddressingMode::ZeroPage, data),
            opcode::SBC_ZPX => self.SBC(AddressingMode::ZeroPageX, data),
            opcode::SBC_ABS => self.SBC(AddressingMode::Absolute, data),
            opcode::SBC_ABX => self.SBC(AddressingMode::AbsoluteX, data),
            opcode::SBC_ABY => self.SBC(AddressingMode::AbsoluteY, data),
            opcode::SBC_INX => self.SBC(AddressingMode::IndexedIndirect, data),
            opcode::SBC_INY => self.SBC(AddressingMode::IndirectIndexed, data),

            opcode::CMP_IM => self.CMP(AddressingMode::Immediate, data),
            opcode::CMP_ZP => self.CMP(AddressingMode::ZeroPage, data),
            opcode::CMP_ZPX => self.CMP(AddressingMode::ZeroPageX, data),
            opcode::CMP_ABS => self.CMP(AddressingMode::Absolute, data),
            opcode::CMP_ABX => self.CMP(AddressingMode::AbsoluteX, data),
            opcode::CMP_ABY => self.CMP(AddressingMode::AbsoluteY, data),
            opcode::CMP_INX => self.CMP(AddressingMode::IndexedIndirect, data),
            opcode::CMP_INY => self.CMP(AddressingMode::IndirectIndexed, data),

            opcode::CPX_IM => self.CPX(AddressingMode::Immediate, data),
            opcode::CPX_ZP => self.CPX(AddressingMode::ZeroPage, data),
            opcode::CPX_ABS => self.CPX(AddressingMode::Absolute, data),

            opcode::CPY_IM => self.CPY(AddressingMode::Immediate, data),
            opcode::CPY_ZP => self.CPY(AddressingMode::ZeroPage, data),
            opcode::CPY_ABS => self.CPY(AddressingMode::Absolute, data),

            opcode::INC_ZP => self.INC(AddressingMode::ZeroPage, data),
            opcode::INC_ZPX => self.INC(AddressingMode::ZeroPageX, data),
            opcode::INC_ABS => self.INC(AddressingMode::Absolute, data),
            opcode::INC_ABX => self.INC(AddressingMode::AbsoluteX, data),

            opcode::INX => self.INX(data),
            opcode::INY => self.INY(data),

            opcode::DEC_ZP => self.DEC(AddressingMode::ZeroPage, data),
            opcode::DEC_ZPX => self.DEC(AddressingMode::ZeroPageX, data),
            opcode::DEC_ABS => self.DEC(AddressingMode::Absolute, data),
            opcode::DEC_ABX => self.DEC(AddressingMode::AbsoluteX, data),

            opcode::DEX => self.DEX(data),
            opcode::DEY => self.DEY(data),

            opcode::ASL_AC => self.ASL(AddressingMode::Accumulator, data),
            opcode::ASL_ZP => self.ASL(AddressingMode::ZeroPage, data),
            opcode::ASL_ZPX => self.ASL(AddressingMode::ZeroPageX, data),
            opcode::ASL_ABS => self.ASL(AddressingMode::Absolute, data),
            opcode::ASL_ABX => self.ASL(AddressingMode::AbsoluteX, data),

            opcode::LSR_AC => self.LSR(AddressingMode::Accumulator, data),
            opcode::LSR_ZP => self.LSR(AddressingMode::ZeroPage, data),
            opcode::LSR_ZPX => self.LSR(AddressingMode::ZeroPageX, data),
            opcode::LSR_ABS => self.LSR(AddressingMode::Absolute, data),
            opcode::LSR_ABX => self.LSR(AddressingMode::AbsoluteX, data),

            opcode::ROL_AC => self.ROL(AddressingMode::Accumulator, data),
            opcode::ROL_ZP => self.ROL(AddressingMode::ZeroPage, data),
            opcode::ROL_ZPX => self.ROL(AddressingMode::ZeroPageX, data),
            opcode::ROL_ABS => self.ROL(AddressingMode::Absolute, data),
            opcode::ROL_ABX => self.ROL(AddressingMode::AbsoluteX, data),

            opcode::ROR_AC => self.ROR(AddressingMode::Accumulator, data),
            opcode::ROR_ZP => self.ROR(AddressingMode::ZeroPage, data),
            opcode::ROR_ZPX => self.ROR(AddressingMode::ZeroPageX, data),
            opcode::ROR_ABS => self.ROR(AddressingMode::Absolute, data),
            opcode::ROR_ABX => self.ROR(AddressingMode::AbsoluteX, data),

            opcode::JMP_ABS => self.JMP(AddressingMode::Absolute, data),
            opcode::JMP_IND => self.JMP(AddressingMode::Indirect, data),

            opcode::JSR => self.JSR(data),
            opcode::RTS => self.RTS(data),

            opcode::BCC => self.BCC(data),
            opcode::BCS => self.BCS(data),
            opcode::BEQ => self.BEQ(data),
            opcode::BMI => self.BMI(data),
            opcode::BNE => self.BNE(data),
            opcode::BPL => self.BPL(data),
            opcode::BVC => self.BVC(data),
            opcode::BVS => self.BVS(data),

            opcode::CLC => self.CLC(data),
            opcode::CLD => self.CLD(data),
            opcode::CLI => self.CLI(data),
            opcode::CLV => self.CLV(data),

            opcode::SEC => self.SEC(data),
            opcode::SED => self.SED(data),
            opcode::SEI => self.SEI(data),

            opcode::BRK => self.BRK(data),
            opcode::NOP => self.NOP(data),
            opcode::RTI => self.RTI(data),
            _ => todo!("Opcode {}", self.opcode),
        }
    }

    fn irq(&mut self) -> addressing::CaseHashMap {
        let mut map = addressing::implied();
        map.insert(0, |cpu, data| {
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            (cpu.sp, _) = cpu.sp.overflowing_sub(1);

            data.pins.data = ((cpu.pc & 0xFF00) >> 8) as u8;
            data.pins.rw = ReadWrite::W;
        });
        map.insert(1, |cpu, data| {
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            (cpu.sp, _) = cpu.sp.overflowing_sub(1);

            data.pins.data = (cpu.pc & 0xFF) as u8;
            data.pins.rw = ReadWrite::W;
        });
        map.insert(2, |cpu, data| {
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            (cpu.sp, _) = cpu.sp.overflowing_sub(1);

            data.pins.data = cpu.ps.bits();
            data.pins.rw = ReadWrite::W;
        });
        map.insert(3, |_, data| {
            data.pins.address = 0xFFFE;
            data.pins.rw = ReadWrite::R;
        });
        map.insert(4, |cpu, data| {
            cpu.temp16 = data.pins.data as u16;
            data.pins.address = 0xFFFF;
            data.pins.rw = ReadWrite::R;
        });
        map.insert(5, |cpu, data| {
            cpu.temp16 |= (data.pins.data as u16) << 8;
            cpu.pc = cpu.temp16;
        });
        map.insert(6, |cpu, _| {
            cpu.instruction_finish();
        });

        map
    }

    fn reset_state(&mut self, data: &mut CPUData) {
        match self.counter.value {
            0 => {
                data.pins.address = 0xFFFC;
                data.pins.rw = ReadWrite::R;
            }
            1 => {
                self.pc = data.pins.data as u16;
                data.pins.address = 0xFFFD;
                data.pins.rw = ReadWrite::R;
            }
            2 => {
                self.pc |= (data.pins.data as u16) << 8;
            }
            3 => {}
            4 => {}
            5 => {}
            6 => {
                self.instruction_finish();
            }
            _ => panic!("Should not reach"),
        }
    }
}
