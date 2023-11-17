use std::fmt;

pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &_ => {
                write!(f, "{}", &self.message)
            }
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Error::new(format!("Parse error: {}", value.to_string()))
    }
}

impl From<home_assistant_rest::errors::Error> for Error {
    fn from(value: home_assistant_rest::errors::Error) -> Self {
        Error::new(format!("Home Assistant: {}", value.to_string()))
    }
}
