use std::ops::Sub;

#[derive(Clone, Debug)]
pub(crate) struct Ramp {
    steps: usize,
    pub(crate) value: f32,
    increment: f32,
}

impl Ramp {
    pub fn new<T>(start: T, end: T, steps: usize) -> Self
    where
        f32: From<<T as Sub>::Output> + From<T>,
        T: Sub + Copy,
    {
        // Divide by zero results in NaN which is OK
        let range: f32 = (end - start).into();
        let increment = range / steps as f32;

        Self {
            steps,
            value: start.into(),
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
