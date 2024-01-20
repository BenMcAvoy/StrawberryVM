pub trait Addressable {
    fn read(&self, addr: u16) -> Result<u8, Error>;
    fn write(&mut self, addr: u16, value: u8) -> Result<(), Error>;

    fn read_u16(&self, addr: u16) -> Result<u16, Error> {
        if let Ok(x0) = self.read(addr) {
            if let Ok(x1) = self.read(addr + 1) {
                return Ok(u16::from(x0) | (u16::from(x1) << 8));
            }
        };

        Err(Error::OutOfBounds(addr))
    }

    fn write_u16(&mut self, addr: u16, value: u16) -> Result<(), Error> {
        let lower = value & 0xff;
        let upper = (value & 0xff00) >> 8;

        self.write(addr, lower as u8)?;
        self.write(addr + 1, upper as u8)
    }

    fn copy(&mut self, from: u16, to: u16, n: usize) -> Result<(), Error> {
        for i in 0..n {
            let val = self.read(from + u16::try_from(i).unwrap())?;
            self.write(to + u16::try_from(i).unwrap(), val)?;
        }

        Ok(())
    }

    fn load(&mut self, from: &[u8], addr: u16) -> Result<(), Error> {
        for (i, byte) in from.iter().enumerate() {
            self.write(addr + u16::try_from(i).unwrap(), *byte)?;
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    OutOfBounds(u16),
    OtherError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::OutOfBounds(v) => write!(f, "OutOfBounds error occurred @ 0x{v:X}"),
            Error::OtherError => write!(f, "Another error occurred"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// Linear memory that can have dynamic size
pub struct Linear {
    bytes: Vec<u8>,
    size: usize,
}

impl Linear {
    /// Create new linear memory of a certain size
    pub fn new(n: usize) -> Self {
        Self {
            bytes: vec![0; n],
            size: n,
        }
    }
}

impl Addressable for Linear {
    fn read(&self, addr: u16) -> Result<u8, Error> {
        if (addr as usize) >= self.size {
            return Err(Error::OutOfBounds(addr));
        }

        Ok(self.bytes[addr as usize])
    }

    fn write(&mut self, addr: u16, value: u8) -> Result<(), Error> {
        if (addr as usize) >= self.size {
            return Err(Error::OutOfBounds(addr));
        }

        self.bytes[addr as usize] = value;

        Ok(())
    }
}
