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
