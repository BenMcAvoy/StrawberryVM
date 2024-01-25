pub type DynErr = Box<dyn std::error::Error>;

pub fn assert_length(parts: &[&str], n: usize) -> Result<(), Box<dyn std::error::Error>> {
    if !parts.len() == n {
        return Err(format!("Expected {} got {}", n, parts.len()).into());
    }

    Ok(())
}

pub fn split_u16(dbyte: u16) -> (u8, u8) {
    let lower = (dbyte & 0xff) as u8;
    let upper = (dbyte >> 8) as u8;

    (lower, upper)
}
