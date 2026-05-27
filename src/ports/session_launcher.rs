use std::path::{Path, PathBuf};

use crate::Result;

pub trait SessionLauncher {
    fn config_path(&self, session_name: &str) -> PathBuf;
    fn ensure_config(&self, session_name: &str, project_dir: &Path) -> Result<()>;
    fn start_session(&self, session_name: &str) -> Result<()>;
}
