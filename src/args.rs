use std::fmt::{Debug, Formatter};
use std::path::PathBuf;

fn get_nth_element(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

pub struct Args {
    pub image_1: PathBuf,
    pub image_2: PathBuf,
    pub output: PathBuf
}

impl Args {
    pub fn new() -> Self {
        Args {
            image_1: PathBuf::from(get_nth_element(1)),
            image_2: PathBuf::from(get_nth_element(2)),
            output: PathBuf::from(get_nth_element(3))
        }
    }
}

impl Debug for Args {
    fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Args")
            .field("image_1", &self.image_1)
            .field("image_2", &self.image_2)
            .field("output", &self.output)
            .finish()
    }
}