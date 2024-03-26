use super::{CPUData, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn CLV(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();

        map.insert(0, |cpu, _| {
            cpu.ps.set(StatusFlag::V, false);
        });

        map.insert(1, |cpu, _| {
            cpu.instruction_finish();
        });

        self.run_instruction(map, data);
    }
}
