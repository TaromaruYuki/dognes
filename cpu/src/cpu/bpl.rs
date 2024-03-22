use super::{CPUData, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn BPL(&mut self, data: &mut CPUData) {
        let map = addressing::branch_clear(self.ps.contains(StatusFlag::N));

        self.run_instruction(map, data);
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use crate::{opcode, CPUData, ReadWrite, StatusFlag, CPU};

    #[test]
    fn BPL_SET() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.ps.set(StatusFlag::N, true);

        data.mem.data[0x0000] = opcode::BPL;
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
    fn BPL_UNSET_3_CYCLES() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.ps.set(StatusFlag::N, false);

        data.mem.data[0x0000] = opcode::BPL;
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
    fn BPL_UNSET_4_CYCLES() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x00FD;
        cpu.ps.set(StatusFlag::N, false);

        data.mem.data[0x00FD] = opcode::BPL;
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
    fn BPL_UNSET_NEGATIVE() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.ps.set(StatusFlag::N, false);

        data.mem.data[0x0000] = opcode::BPL;
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