#[derive(Debug)]
pub struct Options {
    pub log_instructions: bool,
    pub rand_seed: usize,
}

impl Options {
    pub fn default() -> Options {
        Options {
            log_instructions: false,
            rand_seed: 0,
        }
    }
}
