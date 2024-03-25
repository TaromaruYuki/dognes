use super::{CPUData, ReadWrite, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn JSR(&mut self, data: &mut CPUData) {
        let mut map = addressing::absolute();
        map.insert(2, |cpu, data| {
            addressing::methods::create_address_from_data_save_addr(cpu, data);
            (cpu.pc, _) = cpu.pc.overflowing_sub(1);

            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            (cpu.sp, _) = cpu.sp.overflowing_sub(1);

            data.pins.data = ((cpu.pc & 0xFF00) >> 8) as u8;
            data.pins.rw = ReadWrite::W;
        });
        map.insert(3, |cpu, data| {
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            (cpu.sp, _) = cpu.sp.overflowing_sub(1);

            data.pins.data = (cpu.pc & 0xFF) as u8;
            data.pins.rw = ReadWrite::W;
        });
        map.insert(4, |_, _| {});
        map.insert(5, |cpu, _| {
            cpu.pc = cpu.temp16;
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
    fn JSR() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::JSR;
        data.mem.data[0x0001] = 0x07;
        data.mem.data[0x0002] = 0xD0;

        for _ in 0..=15 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(data.mem.data[0x01FF], 0x00);
        assert_eq!(data.mem.data[0x01FE], 0x02);
        assert_eq!(cpu.pc, 0xD007);
    }
}
