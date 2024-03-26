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
