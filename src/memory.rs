pub trait Addressable {
    fn read(&self, addr: u16) -> Result<u8, MemoryError>;
    fn write(&mut self, addr: u16, value: u8) -> Result<(), MemoryError>;

    fn read_u16(&self, addr: u16) -> Result<u16, MemoryError> {
        if let Ok(x0) = self.read(addr) {
            if let Ok(x1) = self.read(addr + 1) {
                return Ok((x0 as u16) | ((x1 as u16) << 8));
            }
        };

        Err(MemoryError::OutOfBounds)
    }

    fn write_u16(&mut self, addr: u16, value: u16) -> Result<(), MemoryError> {
        let lower = value & 0xff;
        let upper = value & 0xff00 >> 8;

        self.write(addr, lower as u8)?;
        self.write(addr + 1, upper as u8)
    }

    fn copy(&mut self, from: u16, to: u16, n: usize) -> Result<(), MemoryError> {
        for i in 0..n {
            let val = self.read(from + i as u16)?;
            self.write(to + i as u16, val)?;
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum MemoryError {
    OutOfBounds,
    OtherError,
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MemoryError::OutOfBounds => write!(f, "OutOfBounds error occurred"),
            MemoryError::OtherError => write!(f, "Another error occurred"),
        }
    }
}

impl std::error::Error for MemoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            MemoryError::OutOfBounds => None,
            MemoryError::OtherError => None,
        }
    }
}

pub struct LinearMemory {
    bytes: Vec<u8>,
    size: usize,
}

impl LinearMemory {
    pub fn new(n: usize) -> Self {
        Self {
            bytes: vec![0; n],
            size: n,
        }
    }
}

impl Addressable for LinearMemory {
    fn read(&self, addr: u16) -> Result<u8, MemoryError> {
        if (addr as usize) > self.size {
            return Err(MemoryError::OutOfBounds);
        }

        Ok(self.bytes[addr as usize])
    }

    fn write(&mut self, addr: u16, value: u8) -> Result<(), MemoryError> {
        if (addr as usize) > self.size {
            return Err(MemoryError::OutOfBounds);
        }

        self.bytes[addr as usize] = value;

        Ok(())
    }
}
