#[derive(Default, Debug)]
pub struct Clock {
    pub state: bool,
}

impl Clock {
    pub fn tick(&mut self) {
        self.state = !self.state;
    }
}
