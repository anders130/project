use std::path::Path;

use crate::Result;

pub trait SessionStarter {
    fn start(&self, session_name: &str, project_dir: &Path) -> Result<()>;
}
