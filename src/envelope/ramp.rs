pub(crate) struct Ramp {
    steps: usize,
    value: f32,
    increment: f32,
}

impl Ramp {
    pub fn new(start: f32, end: f32, steps: usize) -> Self {
        // Divide by zero results in NaN which is OK
        let increment = (end - start) / steps as f32;

        Self {
            steps,
            value: start,
            increment,
        }
    }
}

impl Iterator for Ramp {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps > 0 {
            self.steps -= 1;
            self.value -= self.increment;
            Some(self.value)
        } else {
            None
        }
    }
}
