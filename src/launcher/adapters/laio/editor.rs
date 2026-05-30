use std::path::{Path, PathBuf};

use crate::launcher::ports::session_config_editor::SessionConfigEditor;
use crate::Result;

use super::LaioSessionStarter;

impl SessionConfigEditor for LaioSessionStarter {
    fn prepare_for_edit(&self, session_name: &str, project_dir: &Path) -> Result<PathBuf> {
        self.ensure_config(session_name, project_dir)?;
        Ok(self.config_path(session_name))
    }
}
