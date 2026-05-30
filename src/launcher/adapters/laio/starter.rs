use std::path::Path;
use std::process::{Command, Stdio};

use crate::launcher::ports::session_starter::SessionStarter;
use crate::Result;

use super::LaioSessionStarter;

impl SessionStarter for LaioSessionStarter {
    fn start(&self, session_name: &str, project_dir: &Path) -> Result<()> {
        self.ensure_config(session_name, project_dir)?;

        let config_path = self.config_path(session_name);
        let config_str = config_path
            .to_str()
            .ok_or("laio config path is not valid UTF-8")?;

        let status = Command::new("laio")
            .args(["start", "--file", config_str, "--skip-attach"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        if !status.success() {
            return Err(format!("laio start failed for '{session_name}'").into());
        }

        Ok(())
    }
}
