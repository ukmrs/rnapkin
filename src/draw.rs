pub mod colors;
mod gather;
mod plot;
mod point;

pub use gather::gather_bubbles;
pub use plot::{plot, Mirror};
pub use point::Point;
