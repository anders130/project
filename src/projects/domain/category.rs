#[derive(Debug, Clone)]
pub struct Category(String);

impl Category {
    pub fn new(raw: impl Into<String>) -> Self {
        Self(raw.into())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn initial(&self) -> char {
        self.0.chars().next().unwrap_or('?').to_ascii_uppercase()
    }
}
