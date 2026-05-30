use std::path::{Path, PathBuf};

use crate::Result;

pub trait SessionConfigEditor {
    fn prepare_for_edit(&self, session_name: &str, project_dir: &Path) -> Result<PathBuf>;
}
