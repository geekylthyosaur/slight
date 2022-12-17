use std::cmp::Ordering;

pub struct Range {
    curr: usize,
    max: usize,
}

impl Range {
    pub fn new(curr: usize, max: usize) -> Self {
        Self { curr, max }
    }

    pub fn to_value(&self, new: usize) -> Box<dyn Iterator<Item = usize>> {
        let r: Box<dyn Iterator<Item = usize>>;
        match new.cmp(&self.curr) {
            Ordering::Greater => r = Box::new(self.curr..=new),
            Ordering::Less => r = Box::new((new..=self.curr).rev()),
            Ordering::Equal => r = Box::new(std::iter::empty()),
        }
        r
    }

    pub fn by_value(&self, diff: isize) -> impl Iterator<Item = usize> {
        let new = (self.curr as isize).checked_add(diff).unwrap_or(0) as usize;
        self.to_value(new)
    }

    pub fn by_percent_exp(&self, percent: f32, exp: f32) -> Box<dyn Iterator<Item = usize>> {
        // TODO:
        let (curr, max) = (self.curr, self.max);
        let r: Box<dyn Iterator<Item = usize>> = match percent.is_sign_positive() {
            true => Box::new(
                (0..=max)
                    .filter(move |&v| v > curr)
                    .take((percent * exp) as usize),
            ),
            false => Box::new(
                (0..=max)
                    .filter(move |&v| v < curr)
                    .rev()
                    .take((percent.copysign(1.0) * exp) as usize),
            ),
        };
        r
    }
}
