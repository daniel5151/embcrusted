#![no_std]
extern crate alloc;

mod buffer;
mod frame;
mod instruction;
mod ui;
mod zmachine;

pub use ui::Ui;
pub use zmachine::Zmachine;

pub struct Options {
    pub rand_seed: usize,
}
