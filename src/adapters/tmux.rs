use std::collections::HashMap;
use std::process::Command;

use crate::domain::entities::{Session, Window};
use crate::ports::session_manager::SessionManager;
use crate::Result;

pub struct TmuxAdapter;

impl TmuxAdapter {
    fn parse_windows(output: &[u8]) -> Vec<Window> {
        String::from_utf8_lossy(output)
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(3, '\t');
                let index = parts.next()?.parse::<u32>().ok()?;
                let name = parts.next()?.to_string();
                let active = parts.next()? == "1";
                Some(Window { index, name, active })
            })
            .collect()
    }
}

impl SessionManager for TmuxAdapter {
    fn list_sessions(&self) -> Vec<Session> {
        let out = Command::new("tmux")
            .args(["list-windows", "-a", "-F", "#{session_name}\t#I\t#W\t#{window_active}"])
            .output();

        let out = match out {
            Ok(o) if o.status.success() => o,
            _ => return vec![],
        };

        let mut map: HashMap<String, Vec<Window>> = HashMap::new();
        let mut order: Vec<String> = Vec::new();

        for line in String::from_utf8_lossy(&out.stdout).lines() {
            let mut parts = line.splitn(4, '\t');
            let Some(session_name) = parts.next() else { continue };
            let index = parts.next().and_then(|s| s.parse::<u32>().ok());
            let name = parts.next().map(str::to_string);
            let active = parts.next().map(|s| s == "1");

            if !map.contains_key(session_name) {
                order.push(session_name.to_string());
                map.insert(session_name.to_string(), Vec::new());
            }

            if let (Some(index), Some(name), Some(active)) = (index, name, active) {
                map.get_mut(session_name).unwrap().push(Window { index, name, active });
            }
        }

        order
            .into_iter()
            .map(|name| Session { windows: map.remove(&name).unwrap_or_default(), name })
            .collect()
    }

    fn get_session(&self, name: &str) -> Option<Session> {
        let out = Command::new("tmux")
            .args(["list-windows", "-t", name, "-F", "#I\t#W\t#{window_active}"])
            .output()
            .ok()
            .filter(|o| o.status.success())?;

        Some(Session {
            name: name.to_string(),
            windows: Self::parse_windows(&out.stdout),
        })
    }

    fn has_session(&self, name: &str) -> bool {
        self.get_session(name).is_some()
    }

    fn is_in_tmux(&self) -> bool {
        std::env::var("TMUX").is_ok()
    }

    fn switch_client(&self, name: &str) -> Result<()> {
        let status = Command::new("tmux").args(["switch-client", "-t", name]).status()?;
        if !status.success() {
            return Err(format!("tmux switch-client failed for '{name}'").into());
        }
        Ok(())
    }

    fn attach_session(&self, name: &str) -> Result<()> {
        let status = Command::new("tmux").args(["attach-session", "-t", name]).status()?;
        if !status.success() {
            return Err(format!("tmux attach-session failed for '{name}'").into());
        }
        Ok(())
    }
}
