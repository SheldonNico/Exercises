#![allow(stable_features, dead_code)]
pub mod backtracking;
pub mod bit_manipulation;
pub mod ciphers;
pub mod dynamic_programming;
pub mod geometry;

pub struct Timer<'s> {
    s: std::time::Instant,
    tag: &'s str,
}

impl<'s> Timer<'s> {
    pub fn new(tag: &'s str) -> Self {
        Self {
            s: std::time::Instant::now(), tag
        }
    }
}

impl Default for Timer<'static> {
    fn default() -> Self {
        Self::new("noname")
    }
}

impl<'s> Drop for Timer<'s> {
    fn drop(&mut self) {
        println!("`{}` elapsed: {} ms", self.tag, self.s.elapsed().as_millis());
    }
}
