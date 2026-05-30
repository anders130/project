use std::process::Command;

use crate::sessions::ports::session_navigator::SessionNavigator;
use crate::Result;

use super::TmuxAdapter;

impl SessionNavigator for TmuxAdapter {
    fn is_in_session(&self) -> bool {
        std::env::var("TMUX").is_ok()
    }

    fn switch_to(&self, name: &str) -> Result<()> {
        let status = Command::new("tmux")
            .args(["switch-client", "-t", name])
            .status()?;
        if !status.success() {
            return Err(format!("tmux switch-client failed for '{name}'").into());
        }
        Ok(())
    }

    fn attach_to(&self, name: &str) -> Result<()> {
        let status = Command::new("tmux")
            .args(["attach-session", "-t", name])
            .status()?;
        if !status.success() {
            return Err(format!("tmux attach-session failed for '{name}'").into());
        }
        Ok(())
    }

    fn on_session_started(&self, name: &str) {
        // Seed tmux last-window so prefix-L works after attach.
        // TODO: remove once laio supports window-level focus natively.
        let _ = Command::new("tmux")
            .args(["select-window", "-t", &format!("{name}:shell")])
            .status();
        let _ = Command::new("tmux")
            .args(["select-window", "-t", &format!("{name}:code")])
            .status();
    }
}
