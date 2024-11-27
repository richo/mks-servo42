#![no_std]

#[derive(Default)]
pub struct Driver {
    // Address: we'll default to e0
    bytes: [u8; 8], // This is our buffer that we're going to load out with commands in it
}

impl Driver {
    pub fn rotate<'a>(&'a mut self) -> &'a mut [u8] {
        // For now we'll just build it up by hand because we're being silly
        let cmd = [0xe0, 0xf6, 0x01];
        let len = cmd.len();
        self.bytes[..len].clone_from_slice(&cmd);
        self.bytes[len] = checksum(&cmd);

        &mut self.bytes[..len+1]
    }
}

fn checksum(bytes: &[u8]) -> u8 {
    let mut total: u64 = 0;
    for b in bytes {
        total += *b as u64;
    }
    return (total & 0xff) as u8;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksums() {
        assert_eq!(0xd7, checksum(&[0xe0, 0xf6, 0x01]));
    }
}
