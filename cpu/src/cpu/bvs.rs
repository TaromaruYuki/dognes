use super::{CPUData, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn BVS(&mut self, data: &mut CPUData) {
        let map = addressing::branch_set(self.ps.contains(StatusFlag::V));

        self.run_instruction(map, data);
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use crate::{opcode, CPUData, ReadWrite, StatusFlag, CPU};

    #[test]
    fn BVS_UNSET() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.ps.set(StatusFlag::V, false);

        data.mem.data[0x0000] = opcode::BVS;
        data.mem.data[0x0001] = 0x7F;

        for _ in 0..=7 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn BVS_SET_3_CYCLES() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.ps.set(StatusFlag::V, true);

        data.mem.data[0x0000] = opcode::BVS;
        data.mem.data[0x0001] = 0x7F;

        for _ in 0..=9 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.pc, 0x0081);
    }

    #[test]
    fn BVS_SET_4_CYCLES() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x00FD;
        cpu.ps.set(StatusFlag::V, true);

        data.mem.data[0x00FD] = opcode::BVS;
        data.mem.data[0x00FE] = 0x7F;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.pc, 0x017E);
    }

    #[test]
    fn BVS_SET_NEGATIVE() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.ps.set(StatusFlag::V, true);

        data.mem.data[0x0000] = opcode::BVS;
        data.mem.data[0x0001] = 0x80;

        for _ in 0..=9 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.pc, 0xFF82);
    }
}
