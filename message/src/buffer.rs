#[deprecated(note = "use Message instead")]
pub struct Buffer {
    //capacity: mut usize;
    offset: usize,
    length: usize,
    buf: Box<Vec<u8>>,
}

#[allow(deprecated)]
impl Buffer {
    pub fn new(capacity: usize) -> Buffer {
        Buffer {
            //capacity: capacity,
            offset: 0,
            length: 0,
            buf: Box::<Vec<u8>>::new(Vec::<u8>::with_capacity(capacity))
        }
    }

    pub fn append(&mut self, bytes: &[u8], length: usize) -> usize {
        self.adjust_capacity(length);
        // copy byte from bytes to buf(self)
        self.buf[self.length..self.length + length].copy_from_slice(bytes);
        // adjust length
        self.length += length;
        
        return length;
    }
    
    fn adjust_capacity(&mut self, length: usize) {
        // adjust the buf to ample space
        while self.buf.capacity() < self.length + length {
            self.buf.reserve_exact(self.buf.capacity() * 2);
        }
    }
}
