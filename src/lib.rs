#![no_std]

mod response;
pub mod direction;
mod errors;

pub use errors::Error;



pub struct Driver {
    address: u8,
    bytes: [u8; 8], // This is our buffer that we're going to load out with commands in it
}

type Result<T> = core::result::Result<T, crate::Error>;

impl Default for Driver {
    fn default() -> Self {
        Driver {
            address: 0xe0,
            bytes: Default::default(),
        }
    }
}

impl Driver {
    pub fn rotate<'a>(&'a mut self, direction: direction::Direction, speed: u8) -> Result<&'a mut [u8]> {
        if speed > 0x80 {
            return Err(Error::InvalidValue);
        }

        // For now we'll just build it up by hand because we're being silly
        let cmd = [self.address, 0xf6, speed & direction as u8];
        let len = cmd.len();
        self.bytes[..len].clone_from_slice(&cmd);
        self.bytes[len] = checksum(&cmd);

        Ok(&mut self.bytes[..len+1])
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
