use crate::projects::domain::project::Project;
use crate::Result;

pub struct PickEntry {
    pub display: String,
    pub session_name: String,
    pub path: String,
}

pub trait ProjectPicker {
    fn pick(&self, open: &[&Project], closed: &[&Project], query: &str) -> Result<Option<usize>>;
}
