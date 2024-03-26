use super::{CPUData, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn BCC(&mut self, data: &mut CPUData) {
        let map = addressing::branch_clear(self.ps.contains(StatusFlag::C));

        self.run_instruction(map, data);
    }
}
