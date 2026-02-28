pub struct Spring {
    pub value: f32,
    pub velocity: f32,
}

impl Spring {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            velocity: 0.0,
        }
    }

    pub fn update(&mut self, target: f32, stiffness: f32, damping: f32) {
        let force = (target - self.value) * stiffness;
        self.velocity = (self.velocity + force) * damping;
        self.value += self.velocity;
    }
}
