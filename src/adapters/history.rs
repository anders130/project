use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::path::PathBuf;

use crate::ports::history_store::HistoryStore;
use crate::Result;

const MAX_ENTRIES: usize = 1000;

pub struct FileHistoryStore {
    pub history_path: PathBuf,
}

impl HistoryStore for FileHistoryStore {
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

    fn frequencies(&self) -> Result<HashMap<String, usize>> {
        let mut map = HashMap::new();
        for entry in self.read_entries()? {
            *map.entry(entry).or_insert(0) += 1;
        }
        Ok(map)
    }
}

impl FileHistoryStore {
    fn read_entries(&self) -> Result<Vec<String>> {
        if !self.history_path.exists() {
            return Ok(Vec::new());
        }
        let file = std::fs::File::open(&self.history_path)?;
        let entries = std::io::BufReader::new(file)
            .lines()
            .filter_map(|l| l.ok())
            .filter(|l| !l.is_empty())
            .collect();
        Ok(entries)
    }
}
