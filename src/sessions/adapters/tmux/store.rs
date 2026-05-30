use std::collections::HashMap;
use std::process::Command;

use crate::sessions::domain::session::Session;
use crate::sessions::domain::window::Window;
use crate::sessions::ports::session_store::SessionStore;

use super::TmuxAdapter;

impl SessionStore for TmuxAdapter {
    fn all(&self) -> Vec<Session> {
        let out = Command::new("tmux")
            .args([
                "list-windows",
                "-a",
                "-F",
                "#{session_name}\t#I\t#W\t#{window_active}",
            ])
            .output();

        let out = match out {
            Ok(o) if o.status.success() => o,
            _ => return vec![],
        };

        let mut map: HashMap<String, Vec<Window>> = HashMap::new();
        let mut order: Vec<String> = Vec::new();

        for line in String::from_utf8_lossy(&out.stdout).lines() {
            let mut parts = line.splitn(4, '\t');
            let Some(session_name) = parts.next() else {
                continue;
            };
            let index = parts.next().and_then(|s| s.parse::<u32>().ok());
            let name = parts.next().map(str::to_string);
            let active = parts.next().map(|s| s == "1");

            if !map.contains_key(session_name) {
                order.push(session_name.to_string());
                map.insert(session_name.to_string(), Vec::new());
            }

            if let (Some(index), Some(name), Some(active)) = (index, name, active) {
                map.get_mut(session_name).unwrap().push(Window {
                    index,
                    name,
                    active,
                });
            }
        }

        order
            .into_iter()
            .map(|name| Session {
                windows: map.remove(&name).unwrap_or_default(),
                name,
            })
            .collect()
    }

    fn find(&self, name: &str) -> Option<Session> {
        let out = Command::new("tmux")
            .args(["list-windows", "-t", name, "-F", "#I\t#W\t#{window_active}"])
            .output()
            .ok()
            .filter(|o| o.status.success())?;

        Some(Session {
            name: name.to_string(),
            windows: parse_windows(&out.stdout),
        })
    }
}

fn parse_windows(output: &[u8]) -> Vec<Window> {
    String::from_utf8_lossy(output)
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(3, '\t');
            let index = parts.next()?.parse::<u32>().ok()?;
            let name = parts.next()?.to_string();
            let active = parts.next()? == "1";
            Some(Window {
                index,
                name,
                active,
            })
        })
        .collect()
}
