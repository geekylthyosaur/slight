use std::cmp::Ordering;

pub struct Range {
    curr: usize,
    max: usize,
}

impl Range {
    pub fn new(curr: usize, max: usize) -> Self {
        Self { curr, max }
    }

    pub fn to_value(&self, new: usize) -> impl Iterator<Item=usize> {
        todo!()
    }

    pub fn by_value(&self, new: usize) -> impl Iterator<Item=usize> {
        todo!()
    }

    pub fn by_percent(percent: f32) -> impl Iterator<Item=usize> {
        todo!()
    }

    pub fn by_percent_exp(percent: f32, exp: f32) -> impl Iterator<Item=usize> {
        todo!()
    }

    fn basic(&self) -> impl Iterator<Item=usize> {
        0..=self.max
    }
}
