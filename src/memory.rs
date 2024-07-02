#[allow(dead_code)]
/// Addressable trait
pub trait Addressable {
    /// Read a byte from the address
    fn read(&self, addr: u16) -> Option<u8>;
    /// Write a byte to the address
    fn write(&mut self, addr: u16, value: u8) -> bool;

    /// Read a word from the address
    fn read_word(&self, addr: u16) -> Option<u16> {
        let low = self.read(addr)? as u16;
        let high = self.read(addr + 1)? as u16;

        Some(high << 8 | low)
    }

    /// Write a word to the address
    fn write_word(&mut self, addr: u16, value: u16) -> bool {
        let low = value as u8;
        let high = (value >> 8) as u8;

        self.write(addr, low) && self.write(addr + 1, high)
    }

    /// Copy `n` bytes from one address to another
    fn copy(&mut self, from: u16, to: u16, n: usize) -> bool {
        for i in 0..n {
            let value = match self.read(from + i as u16) {
                Some(value) => value,
                None => return false,
            };

            self.write(to + i as u16, value);
        }

        true
    }
}

/// Memory for the VM using a fixed-size array
pub struct Memory {
    memory: [u8; 0xFF], // 256 bytes
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: [0; 0xFF], // 256 bytes
        }
    }
}

impl Addressable for Memory {
    fn read(&self, addr: u16) -> Option<u8> {
        self.memory.get(addr as usize).copied()
    }

    fn write(&mut self, addr: u16, value: u8) -> bool {
        self.memory.get_mut(addr as usize).map(|slot| {
            *slot = value;
        }).is_some()
    }
}
