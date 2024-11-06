pub struct CircularBuffer {
    buffer: Vec<u8>,
    pointer: usize,
    size: usize,
}

impl CircularBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: vec![0; size],
            pointer: 0,
            size,
        }
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn write(&mut self, data: &[u8]) {
        for byte in data {
            if self.pointer >= self.size {
                self.pointer = 0;
            }

            self.buffer[self.pointer] = *byte;
            self.pointer += 1;
        }
    }
}
