use std::path::{Path, PathBuf};

use crate::ports::session_launcher::SessionLauncher;
use crate::Result;

fn yaml_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

fn builtin_template(session_name: &str, project_dir: &Path) -> String {
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
    panes:
      - commands: []
",
        name = yaml_quote(session_name),
        dir = yaml_quote(&project_dir.display().to_string()),
    )
}

pub struct LaioAdapter {
    pub config_dir: PathBuf,
    pub template_path: PathBuf,
}

impl SessionLauncher for LaioAdapter {
    fn config_path(&self, session_name: &str) -> PathBuf {
        self.config_dir.join(format!("{session_name}.yaml"))
    }

    fn ensure_config(&self, session_name: &str, project_dir: &Path) -> Result<()> {
        let path = self.config_path(session_name);
        if path.exists() {
            return Ok(());
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = if self.template_path.exists() {
            std::fs::read_to_string(&self.template_path)?
                .replace("{name}", &yaml_quote(session_name))
                .replace("{path}", &yaml_quote(&project_dir.display().to_string()))
        } else {
            builtin_template(session_name, project_dir)
        };

        std::fs::write(&path, content)?;
        Ok(())
    }

    fn start_session(&self, session_name: &str) -> Result<()> {
        let config_path = self.config_path(session_name);
        let config_path_str = config_path
            .to_str()
            .ok_or("laio config path is not valid UTF-8")?;
        let status = std::process::Command::new("laio")
            .args(["start", "--file", config_path_str, "--skip-attach"])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()?;
        if !status.success() {
            return Err(format!("laio start failed for '{session_name}'").into());
        }

        // Visit shell then code so tmux records shell as last-window.
        // Without this a new session has no last-window and prefix-L fails.
        let _ = std::process::Command::new("tmux")
            .args(["select-window", "-t", &format!("{session_name}:shell")])
            .status();
        let _ = std::process::Command::new("tmux")
            .args(["select-window", "-t", &format!("{session_name}:code")])
            .status();

        Ok(())
    }
}
