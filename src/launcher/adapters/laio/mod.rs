mod config;
mod editor;
mod starter;

use std::path::{Path, PathBuf};

use config::SessionConfig;

use crate::launcher::ports::launcher::Launcher;
use crate::Result;

pub struct LaioSessionStarter {
    pub config_dir: PathBuf,
    pub template_path: PathBuf,
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

        let cfg = if self.template_path.exists() {
            let template = std::fs::read_to_string(&self.template_path)?;
            SessionConfig::from_template(&template, session_name, project_dir)
        } else {
            SessionConfig::builtin(session_name, project_dir)
        };

        std::fs::write(&path, cfg.content)?;
        Ok(())
    }
}

impl Launcher for LaioSessionStarter {}
