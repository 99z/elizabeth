use core::fmt;

#[derive(Debug, Clone)]
pub struct NoShadowError;

#[derive(Debug, Clone)]
pub struct SelectorParseError;

#[derive(Debug, Clone)]
pub struct NoVariantError;

impl std::error::Error for NoShadowError {}

impl fmt::Display for NoShadowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no shadow found")
    }
}

impl std::error::Error for SelectorParseError {}

impl fmt::Display for SelectorParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse CSS selector")
    }
}

impl std::error::Error for NoVariantError {}

impl fmt::Display for NoVariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no shadow found matching game + variant combination")
    }
}