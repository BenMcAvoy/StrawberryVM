use std::fmt::Write;

type DynErr = Box<dyn std::error::Error>;

/// A trait implemented on all types of memory used
/// for the virtual machine.
pub trait Addressable {
    fn dump(&self) -> String;

    /// Read function implemented uniquely for reading
    /// a single byte.
    fn read(&self, addr: u16) -> Result<u8, DynErr>;

    /// Read function implemented uniquely for writing
    /// a single byte.
    fn write(&mut self, addr: u16, value: u8) -> Result<(), DynErr>;

    fn read_u16(&self, addr: u16) -> Result<u16, DynErr> {
        let next = addr.checked_add(1).ok_or(Error::OutOfBounds(addr))?;
        let lower = self.read(addr)?;
        let upper = self.read(next)?;
        Ok(u16::from(lower) | (u16::from(upper) << 8))
    }

    fn write_u16(&mut self, addr: u16, value: u16) -> Result<(), DynErr> {
        let next = addr.checked_add(1).ok_or(Error::OutOfBounds(addr))?;
        let lower = value & 0xff;
        let upper = (value & 0xff00) >> 8;

        self.write(addr, lower as u8)?;
        self.write(next, upper as u8)
    }

    fn copy(&mut self, from: u16, to: u16, n: usize) -> Result<(), DynErr> {
        for i in 0..n {
            let offset = u16::try_from(i)?;
            let source = from.checked_add(offset).ok_or(Error::OutOfBounds(from))?;
            let destination = to.checked_add(offset).ok_or(Error::OutOfBounds(to))?;
            let val = self.read(source)?;
            self.write(destination, val)?;
        }

        Ok(())
    }

    fn load(&mut self, from: &[u8], addr: u16) -> Result<(), DynErr> {
        for (i, byte) in from.iter().enumerate() {
            let offset = u16::try_from(i)?;
            let destination = addr.checked_add(offset).ok_or(Error::OutOfBounds(addr))?;
            self.write(destination, *byte)?;
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
            Self::OutOfBounds(v) => write!(f, "OutOfBounds error occurred @ 0x{v:X}"),
            Self::OtherError => write!(f, "Another error occurred"),
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
    fn dump(&self) -> String {
        let mut result = String::with_capacity(self.bytes.len() * 4);

        for chunk in self.bytes.chunks_exact(2) {
            write!(result, "{:02x}{:02x} ", chunk[0], chunk[1]).unwrap();
        }

        result
    }

    fn read(&self, addr: u16) -> Result<u8, DynErr> {
        if (addr as usize) >= self.size {
            return Err(Error::OutOfBounds(addr).into());
        }

        Ok(self.bytes[addr as usize])
    }

    fn write(&mut self, addr: u16, value: u8) -> Result<(), DynErr> {
        if (addr as usize) >= self.size {
            return Err(Error::OutOfBounds(addr).into());
        }

        self.bytes[addr as usize] = value;

        Ok(())
    }
}
