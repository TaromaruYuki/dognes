use super::{CPUData, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn CLD(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();

        map.insert(0, |cpu, _| {
            cpu.ps.set(StatusFlag::D, false);
        });

        map.insert(1, |cpu, _| {
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
    fn CLD() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.ps.set(StatusFlag::D, true);

        data.mem.data[0x0000] = opcode::CLD;

        for _ in 0..=7 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert!(!cpu.ps.contains(StatusFlag::D));
        assert_eq!(cpu.pc, 0x0001);
    }
}
