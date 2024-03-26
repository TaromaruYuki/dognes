use super::{CPUData, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn BEQ(&mut self, data: &mut CPUData) {
        let map = addressing::branch_set(self.ps.contains(StatusFlag::Z));

        self.run_instruction(map, data);
    }
}
