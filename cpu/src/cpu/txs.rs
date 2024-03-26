use super::{CPUData, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn TXS(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();
        map.insert(0, |cpu, _| {
            cpu.sp = cpu.x;
        });
        map.insert(1, |cpu, _| {
            cpu.instruction_finish();
        });

        self.run_instruction(map, data);
    }
}
