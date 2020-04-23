use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use core::fmt;

#[derive(Debug)]
pub struct Frame {
    stack: Vec<u16>,
    locals: Vec<u16>,
    pub arg_count: u8,
    pub resume: usize,
    pub store: Option<u8>,
}

impl Frame {
    pub fn new(resume: usize, store: Option<u8>, mut locals: Vec<u16>, arguments: &[u16]) -> Frame {
        for i in 0..locals.len() {
            if arguments.len() > i {
                locals[i] = arguments[i];
            }
        }

        Frame {
            stack: Vec::new(),
            arg_count: arguments.len() as u8,
            locals,
            resume,
            store,
        }
    }

    pub fn empty() -> Frame {
        Frame {
            stack: Vec::new(),
            locals: Vec::new(),
            arg_count: 0,
            resume: 0,
            store: None,
        }
    }

    pub fn read_local(&self, index: u8) -> u16 {
        let index = index as usize;

        if index > self.locals.len() {
            panic!("Trying to read out of bounds local @: {}", index);
        }

        self.locals[index]
    }

    pub fn write_local(&mut self, index: u8, value: u16) {
        let index = index as usize;

        if index > self.locals.len() {
            panic!("Trying to write out of bounds local @: {}", index);
        }

        self.locals[index] = value;
    }

    pub fn stack_push(&mut self, value: u16) {
        self.stack.push(value);
    }

    pub fn stack_pop(&mut self) -> u16 {
        self.stack.pop().expect("Can't pop off an empty stack!")
    }

    pub fn stack_peek(&self) -> u16 {
        *self.stack.last().expect("Can't peek on an empty stack!")
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stringify = |values: &Vec<u16>| {
            let mut out = String::from("[");

            for (i, val) in values.iter().enumerate() {
                if i != 0 {
                    out.push_str(", ")
                }
                out.push_str(&format!("{:04x}", val));
            }

            out.push_str("]");
            out
        };

        write!(
            f,
            "Locals: {} Stack: {} -> {:?} @ {:04x}",
            &stringify(&self.locals),
            &stringify(&self.stack),
            self.store,
            self.resume
        )
    }
}
