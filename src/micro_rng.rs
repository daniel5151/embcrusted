pub struct MicroRng(usize);

impl MicroRng {
    pub fn new(seed: usize) -> MicroRng {
        MicroRng(seed)
    }

    // https://en.wikipedia.org/wiki/Linear_congruential_generator
    pub fn gen(&mut self) -> usize {
        let a = 16807usize;
        let m = 2_147_483_647;
        self.0 = (a.wrapping_mul(self.0)) % m;
        self.0 / m
    }

    pub fn reseed(&mut self, seed: usize) {
        self.0 = seed;
    }
}
