use super::{CPUData, StatusFlag, CPU};

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn BRK(&mut self, data: &mut CPUData) {
        self.ps.set(StatusFlag::B, true);
        let map = self.irq();
        self.run_instruction(map, data);
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use crate::{opcode, CPUData, ReadWrite, StatusFlag, CPU};

    #[test]
    fn BRK() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0x10000),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::BRK;
        data.mem.data[0xFFFE] = 0x07;
        data.mem.data[0xFFFF] = 0xD0;

        for _ in 0..=17 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.pc, 0xD007);
        assert_eq!(data.mem.data[0x1FF], 0x00);
        assert_eq!(data.mem.data[0x1FE], 0x01);
        assert_eq!(data.mem.data[0x1FD], StatusFlag::B.bits());
    }
}
