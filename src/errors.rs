pub enum Error {
    InvalidValue,
}

impl Error {
    pub fn as_str(&self) -> &'static str {
        match self {
            Error::InvalidValue => "Invalid value",
        }
    }
}
