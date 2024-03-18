use super::{CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn PLP(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();
        map.insert(0, |cpu, _| {
            cpu.sp += 1;
        });
        map.insert(1, |cpu, data| {
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            data.pins.rw = ReadWrite::R;
        });
        map.insert(2, |cpu, data| {
            cpu.ps = StatusFlag::from_bits_retain(data.pins.data);
        });
        map.insert(3, |cpu, _| {
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
    fn PLP() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.sp = 0xFE;
        cpu.ps = StatusFlag::empty();

        data.mem.data[0x0000] = opcode::PLP;
        data.mem.data[0x01FF] = 0xAA;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.ps.bits(), 0xAA);
        assert_eq!(cpu.pc, 0x0001);
    }
}
