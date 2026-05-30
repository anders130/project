#[derive(Debug, Clone)]
pub struct ProjectName(String);

impl ProjectName {
    pub fn new(raw: impl Into<String>) -> Self {
        Self(raw.into())
    }

    pub fn display(&self) -> &str {
        &self.0
    }
}
