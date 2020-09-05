use core::fmt;

#[derive(Debug, Clone)]
pub struct PageParseError;

#[derive(Debug, Clone)]
pub struct NoShadowError;

impl fmt::Display for PageParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "table data not found")
    }
}

impl std::error::Error for PageParseError {}

impl fmt::Display for NoShadowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no shadow specified or no matching variant found")
    }
}

impl std::error::Error for NoShadowError {}