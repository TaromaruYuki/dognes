use super::{CPUData, ReadWrite, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn RTS(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();

        map.insert(0, |cpu, _| {
            cpu.sp += 1;
        });
        map.insert(1, |cpu, data| {
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            data.pins.rw = ReadWrite::R;
        });
        map.insert(2, |cpu, data| {
            cpu.temp16 = data.pins.data as u16;
            cpu.sp += 1;
        });
        map.insert(3, |cpu, data| {
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            data.pins.rw = ReadWrite::R;
        });
        map.insert(4, |cpu, data| {
            addressing::methods::create_address_from_data_save_addr(cpu, data);
            cpu.pc = cpu.temp16;
        });
        map.insert(5, |cpu, _| {
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
    fn RTS() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.sp = 0xFD;

        data.mem.data[0x0000] = opcode::RTS;
        data.mem.data[0x01FF] = 0xD0;
        data.mem.data[0x01FE] = 0x07;

        for _ in 0..=15 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.sp, 0xFF);
        assert_eq!(cpu.pc, 0xD007);
    }
}
