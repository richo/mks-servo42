#![no_std]

mod response;
pub mod direction;
mod errors;

pub use errors::Error;

/// The core object in the API. Represents a single motor.
///
/// Has a Default impl which uses the default address, but if you want to stack them on a bus you
/// should use `with_id`.
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
    /// Create a driver with a different ID.
    pub fn with_id(address: u8) -> Self {
        Driver {
            address,
            .. Default::default()
        }
    }

    /// Start the motor rotating.
    ///
    /// It will continue doing this until stopped. Direction is relative to the configured
    /// direction in the motor.
    ///
    /// Speed is a value between 0 and 0x80. The values can be calculated with more infomration
    /// about how the motor is configured but that's not implemented.
    pub fn rotate<'a>(&'a mut self, direction: direction::Direction, speed: u8) -> Result<&'a mut [u8]> {
        if speed > 0x80 {
            return Err(Error::InvalidValue);
        }

        Ok(self.set_bytes(&[self.address, 0xf6, speed & direction as u8]))
    }

    /// Stop the motor.
    pub fn stop<'a>(&'a mut self) -> Result<&'a mut [u8]> {
        Ok(self.set_bytes(&[self.address, 0xf7]))
    }

    /// Setup these bytes in the internal buffer, build the checksum, and then return the correct
    /// slice.
    fn set_bytes(&mut self, cmd: &[u8]) -> &mut [u8] {
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
