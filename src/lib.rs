#![feature(core)]

extern crate interpolate;
extern crate num;

pub mod vector;
pub mod rect;
pub mod circle;
pub mod grid;
pub mod wrapping_grid;
// pub mod continuous_grid;

pub use vector::Vec2;
pub use rect::Rect;
pub use grid::Grid;
pub use wrapping_grid::WrappingGrid;
// pub use continuous_grid::ContinuousGrod;