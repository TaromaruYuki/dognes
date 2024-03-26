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
