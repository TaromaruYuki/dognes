use super::{CPUData, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn BMI(&mut self, data: &mut CPUData) {
        let map = addressing::branch_set(self.ps.contains(StatusFlag::N));

        self.run_instruction(map, data);
    }
}
