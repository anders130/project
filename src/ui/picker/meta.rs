use std::path::Path;

use super::port::PickEntry;

pub fn write(path: &Path, open: &[PickEntry], closed: &[PickEntry]) -> std::io::Result<()> {
    let lines: String = open
        .iter()
        .map(|e| format!("{}\t{}\t1", e.session_name, e.path))
        .chain(
            closed
                .iter()
                .map(|e| format!("{}\t{}\t0", e.session_name, e.path)),
        )
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(path, lines)
}

pub fn resolve(meta_path: &str, index: usize) -> Option<(String, String, bool)> {
    let metas = std::fs::read_to_string(meta_path).ok()?;
    let line = metas.lines().nth(index)?;
    let mut parts = line.splitn(3, '\t');
    let session_name = parts.next()?.to_string();
    let path = parts.next()?.to_string();
    let is_open = parts.next().map(|s| s == "1").unwrap_or(false);
    Some((session_name, path, is_open))
}
