use super::{CPUData, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn NOP(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();
        map.insert(0, |_, _| {});
        map.insert(1, |cpu, _| {
            cpu.instruction_finish();
        });

        self.run_instruction(map, data);
    }
}
