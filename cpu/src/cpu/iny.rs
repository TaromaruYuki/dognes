use super::{CPUData, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn INY(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();
        map.insert(0, |cpu, _| {
            (cpu.y, _) = cpu.y.overflowing_add(1);
        });
        map.insert(1, |cpu, _| {
            cpu.ps.set(StatusFlag::Z, cpu.y == 0);
            cpu.ps.set(StatusFlag::N, (cpu.y & 0x80) > 0);
            cpu.instruction_finish();
        });

        self.run_instruction(map, data);
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use crate::{opcode, CPUData, ReadWrite, CPU};

    #[test]
    fn INY() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.y = 0xAB;

        data.mem.data[0x0000] = opcode::INY;

        for _ in 0..=7 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.y, 0xAC);
        assert_eq!(cpu.pc, 0x0001);
    }
}