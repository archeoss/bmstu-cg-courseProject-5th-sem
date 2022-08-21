use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct NotImplError {
    method: String,
}

impl NotImplError {
    pub fn new(method: &str) -> NotImplError {
        NotImplError {
            method: method.to_string(),
        }
    }
}

impl Debug for NotImplError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Not Implemented: {{ file : {}, line: {}, details: {} }}",
            file!(),
            line!(),
            self.method
        )
    }
}

impl Display for NotImplError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The method '{}' hasn't been implemented", self.method)
    }
}

impl Error for NotImplError {
    fn description(&self) -> &str {
        &self.method
    }
}
