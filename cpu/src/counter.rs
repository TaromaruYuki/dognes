#[derive(Default, Debug)]
pub(crate) struct Counter {
    pub value: i8,
}

impl Counter {
    pub fn tick(&mut self) {
        if self.value + 1 > 0xF {
            self.value = 0;
            return;
        }

        self.value += 1;
    }

    pub fn reset(&mut self) {
        self.value = -1;
    }
}
