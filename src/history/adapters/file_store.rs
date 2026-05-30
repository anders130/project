use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::path::PathBuf;

use crate::history::ports::usage_reader::UsageReader;
use crate::history::ports::usage_recorder::UsageRecorder;
use crate::Result;

const MAX_ENTRIES: usize = 1000;

pub struct FileUsageStore {
    pub history_path: PathBuf,
}

impl UsageRecorder for FileUsageStore {
    fn record(&self, session_name: &str) -> Result<()> {
        if let Some(parent) = self.history_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut entries = self.read_entries()?;
        entries.push(session_name.to_string());
        if entries.len() > MAX_ENTRIES {
            entries.drain(..entries.len() - MAX_ENTRIES);
            std::fs::write(&self.history_path, entries.join("\n") + "\n")?;
        } else {
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.history_path)?;
            writeln!(file, "{session_name}")?;
        }
        Ok(())
    }
}

impl UsageReader for FileUsageStore {
    fn frequencies(&self) -> Result<HashMap<String, usize>> {
        Ok(self
            .read_entries()?
            .into_iter()
            .fold(HashMap::new(), |mut map, entry| {
                *map.entry(entry).or_insert(0) += 1;
                map
            }))
    }
}

impl FileUsageStore {
    fn read_entries(&self) -> Result<Vec<String>> {
        if !self.history_path.exists() {
            return Ok(Vec::new());
        }
        let file = std::fs::File::open(&self.history_path)?;
        let entries = std::io::BufReader::new(file)
            .lines()
            .map_while(|r| r.ok())
            .filter(|l| !l.is_empty())
            .collect();
        Ok(entries)
    }
}
