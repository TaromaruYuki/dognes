use super::{CPUData, StatusFlag, CPU};

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn BRK(&mut self, data: &mut CPUData) {
        self.ps.set(StatusFlag::B, true);
        let map = self.irq();
        self.run_instruction(map, data);
    }
}
