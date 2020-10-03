use core::fmt;

#[derive(Debug, Clone)]
pub struct NoShadowError {
    pub name: String,
    pub game: String,
}

#[derive(Debug, Clone)]
pub struct NoVariantError {
    pub shadow_name: String,
    pub game: String,
    pub variant: Vec<String>,
}

impl std::error::Error for NoShadowError {}

impl fmt::Display for NoShadowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Shadow not found: {} for game: {}", self.name, self.game)
    }
}

impl std::error::Error for NoVariantError {}

impl fmt::Display for NoVariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Shadow {} not found in: {} of variant: {:#?}", self.shadow_name, self.game, self.variant)
    }
}