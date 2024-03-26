use super::{CPUData, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn BNE(&mut self, data: &mut CPUData) {
        let map = addressing::branch_clear(self.ps.contains(StatusFlag::Z));

        self.run_instruction(map, data);
    }
}
