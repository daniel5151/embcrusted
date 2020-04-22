#![deny(rust_2018_idioms)]

mod buffer;
mod frame;
mod instruction;
mod micro_rng;
mod options;
mod traits;
mod zmachine;

pub use options::Options;
pub use traits::UI;
pub use zmachine::Zmachine;
