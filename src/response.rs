pub struct InvalidValue;

#[repr(u8)]
pub enum Response {
    Success,
    Failure,
}

// Now that I'm looking at it maybe I do want to just smash a bunch of serde together

impl core::convert::TryFrom<u8> for Response {
    type Error = InvalidValue;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Response::Success),
            1 => Ok(Response::Failure),
            _ => Err(InvalidValue),
        }
    }
}
