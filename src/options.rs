#[derive(Debug)]
pub struct Options {
    pub log_instructions: bool,
    pub rand_seed: [u32; 4],
}

impl Options {
    pub fn default() -> Options {
        Options {
            log_instructions: false,
            rand_seed: [90, 111, 114, 107],
        }
    }
}
