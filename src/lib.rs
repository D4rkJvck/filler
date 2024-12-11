mod data;
mod utils;
mod view;

pub use data::Data;
pub use utils::{get_size, matricize};

pub type Matrix = Vec<Vec<char>>;
pub type Size = (usize, usize);
