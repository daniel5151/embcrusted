use alloc::vec::Vec;

pub struct Reader<'a> {
    buffer: &'a Buffer<'a>,
    cursor: usize,
}

impl<'a> Reader<'a> {
    pub fn byte(&mut self) -> u8 {
        let byte = self.buffer.read_byte(self.cursor);
        self.cursor += 1;
        byte
    }

    pub fn word(&mut self) -> u16 {
        let word = self.buffer.read_word(self.cursor);
        self.cursor += 2;
        word
    }

    pub fn position(&self) -> usize {
        self.cursor
    }
}

pub struct Buffer<'a> {
    dyn_mem: Vec<u8>,
    static_mem: &'a [u8],
}

impl<'a> Buffer<'a> {
    pub fn new(base: &'a [u8], static_start: usize) -> Buffer<'a> {
        Buffer {
            dyn_mem: base[..static_start].to_vec(),
            static_mem: &base[static_start..],
        }
    }

    pub fn read_byte(&self, location: usize) -> u8 {
        if location < self.dyn_mem.len() {
            self.dyn_mem[location]
        } else {
            self.static_mem[location - self.dyn_mem.len()]
        }
    }

    pub fn read_word(&self, location: usize) -> u16 {
        ((self.read_byte(location) as u16) << 8) + self.read_byte(location + 1) as u16
    }

    pub fn write_byte(&mut self, location: usize, value: u8) {
        if location < self.dyn_mem.len() {
            self.dyn_mem[location] = value;
        } else {
            panic!(
                "cannot write to static_mem {} {}",
                self.dyn_mem.len(),
                location
            );
        }
    }

    pub fn write_word(&mut self, location: usize, value: u16) {
        let top = ((value & 0xFF00) >> 8) as u8;
        let bottom = (value & 0x00FF) as u8;

        self.write_byte(location, top);
        self.write_byte(location + 1, bottom);
    }

    pub fn read(&self, mut location: usize, length: usize) -> &[u8] {
        if location < self.dyn_mem.len() {
            &self.dyn_mem[location..location + length]
        } else {
            location -= self.dyn_mem.len();
            &self.static_mem[location..location + length]
        }
    }

    pub fn write(&mut self, location: usize, buf: &[u8]) {
        for (i, b) in buf.iter().enumerate() {
            self.write_byte(location + i as usize, *b);
        }
    }

    pub fn get_reader(&self, cursor: usize) -> Reader {
        Reader {
            buffer: self,
            cursor,
        }
    }
}
