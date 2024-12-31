mod data;
mod view;

#[macro_use]
pub mod utils;

pub use {
    data::Data,
    view::View,
};

pub type Matrix = Vec<Vec<char>>;
pub type Size = (usize, usize);
