use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn INC(&mut self, mode: AddressingMode, data: &mut CPUData) {
        fn inc_data(cpu: &mut CPU, data: &mut CPUData) {
            (cpu.temp8, _) = data.pins.data.overflowing_add(1);
        }

        fn end_inc(cpu: &mut CPU, _data: &mut CPUData) {
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
                map.insert(2, inc_data);
                map.insert(3, addressing::methods::store_temp_8_in_temp16);
                map.insert(4, end_inc);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPageX => {
                let mut map = addressing::zero_page_x();
                map.insert(2, |cpu, data| {
                    cpu.temp16 = cpu.temp8 as u16;
                    addressing::methods::get_byte_from_temp8(cpu, data);
                });
                map.insert(3, inc_data);
                map.insert(4, addressing::methods::store_temp_8_in_temp16);
                map.insert(5, end_inc);

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    cpu.temp16 = data.pins.address;
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, inc_data);
                map.insert(4, addressing::methods::store_temp_8_in_temp16);
                map.insert(5, end_inc);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteX => {
                let mut map = addressing::absolute_x();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_add_x(cpu, data);
                    data.pins.address = cpu.temp16;
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, inc_data);
                map.insert(4, addressing::methods::store_temp_8_in_temp16);
                map.insert(5, |_, _| {}); // Takes 7 cycles, the extra one comes from adding x.
                map.insert(6, end_inc);

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
    fn INC_ZP() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::INC_ZP;
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

        assert_eq!(data.mem.data[0x00AB], 0xD1);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn INC_ZPX() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.x = 0x05;

        data.mem.data[0x0000] = opcode::INC_ZPX;
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

        assert_eq!(data.mem.data[0x00B0], 0xD1);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn INC_ABS() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::INC_ABS;
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

        assert_eq!(data.mem.data[0xD007], 0xAC);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn INC_ABX() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.x = 0x09;

        data.mem.data[0x0000] = opcode::INC_ABX;
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

        assert_eq!(data.mem.data[0xD010], 0xAC);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn INC_IS_ZERO() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::INC_ZP;
        data.mem.data[0x0001] = 0xAB;
        data.mem.data[0x00AB] = 0xFF;

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
