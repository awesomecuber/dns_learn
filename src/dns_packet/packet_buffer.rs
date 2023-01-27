pub struct PacketBuffer<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> PacketBuffer<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self { buf: bytes, pos: 0 }
    }

    pub fn read_u8(&mut self) -> Result<u8, String> {
        if self.pos >= self.buf.len() {
            return Err("End of buffer".into());
        }
        self.pos += 1;
        Ok(self.buf[self.pos - 1])
    }

    pub fn read_u16(&mut self) -> Result<u16, String> {
        Ok(u16::from_be_bytes([self.read_u8()?, self.read_u8()?]))
    }

    pub fn read_u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_be_bytes([
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
        ]))
    }

    pub fn read_qname(&mut self) -> Result<String, String> {
        Ok(todo!())
    }
}

#[cfg(test)]
mod test {
    use super::PacketBuffer;

    #[test]
    fn read_u8() {
        let mut buf = PacketBuffer::from_bytes(&[3, 5]);
        assert_eq!(buf.read_u8(), Ok(3));
        assert_eq!(buf.read_u8(), Ok(5));
        assert_eq!(buf.read_u8(), Err("End of buffer".into()));
    }

    #[test]
    fn read_u16() {
        let mut buf = PacketBuffer::from_bytes(&[0, 1]);
        assert_eq!(buf.read_u16(), Ok(1));
    }
}
