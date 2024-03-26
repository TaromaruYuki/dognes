use super::{CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn PLA(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();
        map.insert(0, |cpu, _| {
            cpu.sp += 1;
        });
        map.insert(1, |cpu, data| {
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            data.pins.rw = ReadWrite::R;
        });
        map.insert(2, |cpu, data| {
            cpu.a = data.pins.data;
        });
        map.insert(3, |cpu, _| {
            cpu.ps.set(StatusFlag::Z, cpu.a == 0);
            cpu.ps.set(StatusFlag::N, (cpu.a & 0x80) > 0);
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
    fn PLA() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.sp = 0xFE;
        cpu.a = 0x00;

        data.mem.data[0x0000] = opcode::PLA;
        data.mem.data[0x01FF] = 0xAB;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0xAB);
        assert_eq!(cpu.pc, 0x0001);
    }
}
