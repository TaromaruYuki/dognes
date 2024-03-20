use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn DEC(&mut self, mode: AddressingMode, data: &mut CPUData) {
        fn dec_data(cpu: &mut CPU, data: &mut CPUData) {
            (cpu.temp8, _) = data.pins.data.overflowing_sub(1);
        }

        fn end_dec(cpu: &mut CPU, _data: &mut CPUData) {
            cpu.ps.set(StatusFlag::Z, cpu.temp8 == 0);
            cpu.ps.set(StatusFlag::N, (cpu.temp8 & 0x80) > 0);
            cpu.instruction_finish();
        }

        match mode {
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(1, |cpu, data| {
                    cpu.temp16 = data.pins.data as u16;
                    addressing::methods::get_byte_from_data(cpu, data);
                });
                map.insert(2, dec_data);
                map.insert(3, addressing::methods::store_temp_8_in_temp16);
                map.insert(4, end_dec);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPageX => {
                let mut map = addressing::zero_page_x();
                map.insert(2, |cpu, data| {
                    cpu.temp16 = cpu.temp8 as u16;
                    addressing::methods::get_byte_from_temp8(cpu, data);
                });
                map.insert(3, dec_data);
                map.insert(4, addressing::methods::store_temp_8_in_temp16);
                map.insert(5, end_dec);

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    cpu.temp16 = data.pins.address;
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, dec_data);
                map.insert(4, addressing::methods::store_temp_8_in_temp16);
                map.insert(5, end_dec);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteX => {
                let mut map = addressing::absolute_x();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_add_x(cpu, data);
                    data.pins.address = cpu.temp16;
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, dec_data);
                map.insert(4, addressing::methods::store_temp_8_in_temp16);
                map.insert(5, |_, _| {}); // Takes 7 cycles, the extra one comes from adding x.
                map.insert(6, end_dec);

                self.run_instruction(map, data);
            }
            _ => panic!("Should never reach."),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use crate::{opcode, CPUData, ReadWrite, StatusFlag, CPU};

    #[test]
    fn DEC_ZP() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::DEC_ZP;
        data.mem.data[0x0001] = 0xAB;
        data.mem.data[0x00AB] = 0xD0;

        for _ in 0..=13 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(data.mem.data[0x00AB], 0xCF);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn DEC_ZPX() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.x = 0x05;

        data.mem.data[0x0000] = opcode::DEC_ZPX;
        data.mem.data[0x0001] = 0xAB;
        data.mem.data[0x00B0] = 0xD0;

        for _ in 0..=15 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(data.mem.data[0x00B0], 0xCF);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn DEC_ABS() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::DEC_ABS;
        data.mem.data[0x0001] = 0x07;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD007] = 0xAB;

        for _ in 0..=15 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(data.mem.data[0xD007], 0xAA);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn DEC_ABX() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.x = 0x09;

        data.mem.data[0x0000] = opcode::DEC_ABX;
        data.mem.data[0x0001] = 0x07;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD010] = 0xAB;

        for _ in 0..=17 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(data.mem.data[0xD010], 0xAA);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn DEC_IS_ZERO() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::DEC_ZP;
        data.mem.data[0x0001] = 0xAB;
        data.mem.data[0x00AB] = 0x01;

        for _ in 0..=13 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(data.mem.data[0x00AB], 0x00);
        assert!(!cpu.ps.contains(StatusFlag::N));
        assert!(cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0002);
    }
}
