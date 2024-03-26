use super::{CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn RTI(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();

        map.insert(0, |cpu, _| {
            (cpu.sp, _) = cpu.sp.overflowing_add(1);
        });
        map.insert(1, |cpu, data| {
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            data.pins.rw = ReadWrite::R;
        });
        map.insert(2, |cpu, data| {
            (cpu.sp, _) = cpu.sp.overflowing_add(1);
            cpu.temp8 = data.pins.data;
        });
        map.insert(3, |cpu, data| {
            cpu.ps = StatusFlag::from_bits_retain(cpu.temp8);

            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            data.pins.rw = ReadWrite::R;
        });
        map.insert(4, |cpu, data| {
            cpu.pc = data.pins.data as u16;

            (cpu.sp, _) = cpu.sp.overflowing_add(1);
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            data.pins.rw = ReadWrite::R;
        });
        map.insert(5, |cpu, data| {
            cpu.pc |= (data.pins.data as u16) << 8;
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
    fn RTI() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.sp = 0xFC;

        data.mem.data[0x0000] = opcode::RTI;

        data.mem.data[0x01FF] = 0xD0;
        data.mem.data[0x01FE] = 0x07;
        data.mem.data[0x01FD] = StatusFlag::Z.bits() | StatusFlag::C.bits();

        for _ in 0..=15 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.pc, 0xD007);
        assert!(cpu.ps.contains(StatusFlag::Z));
        assert!(cpu.ps.contains(StatusFlag::C));
    }
}
