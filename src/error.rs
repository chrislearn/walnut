use std::error::Error as StdError;
use std::fmt::{self, Display};

pub type Result<T> = ::std::result::Result<T, Error>;
pub enum Error {
    General(String),
    ParseUrl(url::ParseError),
    ParseInt(std::num::ParseIntError),
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::ParseUrl(err)
    }
}
impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl StdError for Error {
    fn description(&self) -> &str{
        match *self {
            Error::General(ref msg) => &msg,
            Error::ParseUrl(_) => "url parse error.",
            Error::ParseInt(_) => "int parse error.",
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::General(ref e) =>
                format!("{}: {}", self.description(), e).fmt(f),
            Error::ParseUrl(ref e) =>
                format!("{}: {}", self.description(), e).fmt(f),
            _ => self.description().to_string().fmt(f),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&*self.description()).ok();
        if self.source().is_some() {
            write!(f, ": {:?}", self.source().unwrap()).ok(); // recurse
        }
        Ok(())
    }
}
