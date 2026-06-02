mod editor;
mod starter;

use std::path::{Path, PathBuf};

use crate::launcher::ports::launcher::Launcher;
use crate::Result;

pub struct LaioSessionStarter {
    pub config_dir: PathBuf,
}

impl LaioSessionStarter {
    pub(self) fn config_path(&self, session_name: &str) -> PathBuf {
        self.config_dir.join(format!("{session_name}.yaml"))
    }

    pub(self) fn ensure_config(&self, session_name: &str, project_dir: &Path) -> Result<()> {
        let path = self.config_path(session_name);
        if path.exists() {
            return Ok(());
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let name = yaml_quote(session_name);
        let dir = yaml_quote(&project_dir.display().to_string());
        std::fs::write(
            &path,
            format!(
                "\
name: {name}
path: {dir}

windows:
  - name: code
    panes:
      - commands:
          - command: nvim
            args:
              - .
  - name: shell
    focus: true
    panes:
      - commands: []
"
            ),
        )?;
        Ok(())
    }
}

fn yaml_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

impl Launcher for LaioSessionStarter {}
