use super::{CPUData, ReadWrite, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn PHP(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();
        map.insert(0, |cpu, data| {
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            data.pins.data = cpu.ps.bits();
            data.pins.rw = ReadWrite::W;
        });
        map.insert(1, |cpu, _| {
            cpu.sp -= 1;
        });
        map.insert(2, |cpu, _| {
            cpu.instruction_finish();
        });

        self.run_instruction(map, data);
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use crate::{opcode, CPUData, ReadWrite, StatusFlag, CPU};

    #[test]
    fn PHP() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.ps.set(StatusFlag::I, true);
        cpu.ps.set(StatusFlag::N, true);

        data.mem.data[0x0000] = opcode::PHP;

        for _ in 0..=9 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.sp, 0xFE);
        assert_eq!(data.mem.data[0x01FF], cpu.ps.bits());
        assert_eq!(cpu.pc, 0x0001);
    }
}
