use std::io::Write;
use std::process::{Command, Stdio};

use crate::adapters::colors::{
    DIM, RST,
    H_BASE, H_SURFACE0, H_OVERLAY0, H_SUB0, H_RED, H_GRN, H_MVE, H_ROSEWATER,
};
use crate::ports::picker::{PickEntry, PickResult, Picker};
use crate::Result;

pub struct FzfPicker {
    pub preview_cmd: Option<String>,
}

impl Picker for FzfPicker {
    fn pick(&self, items: &[PickEntry], query: &str) -> Result<Option<PickResult>> {
        let sep_display = format!("{DIM}{}{RST}", "─".repeat(36));

        let lines: Vec<String> = items
            .iter()
            .map(|entry| match entry {
                PickEntry::Separator => format!("{sep_display}\t\t"),
                PickEntry::Item(item) => {
                    format!("{}\t{}\t{}", item.display, item.session_name, item.path)
                }
            })
            .collect();

        let color_arg = format!("--color=bg+:{H_SURFACE0},gutter:{H_BASE},hl:{H_RED},hl+:{H_RED},pointer:{H_ROSEWATER},marker:{H_GRN},prompt:{H_MVE},info:{H_SUB0},border:{H_OVERLAY0},separator:{H_OVERLAY0}");

        let mut fzf_args: Vec<String> = vec![
            "--ansi".into(),
            "--delimiter=\t".into(),
            "--with-nth=1".into(),
            format!("--query={query}"),
            color_arg,
            "--border=rounded".into(),
            "--layout=reverse".into(),
            "--info=inline".into(),
            "--prompt=  ".into(),
        ];
        if let Some(cmd) = &self.preview_cmd {
            fzf_args.push(format!("--preview={cmd}"));
            fzf_args.push("--preview-window=right:35%:border-left:wrap".into());
        }

        let mut child = Command::new("fzf")
            .args(&fzf_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = child.stdin.take().ok_or("fzf stdin unavailable")?;

        let writer = std::thread::spawn(move || {
            let mut stdin = stdin;
            for line in &lines {
                if writeln!(stdin, "{line}").is_err() {
                    break;
                }
            }
        });

        let output = child.wait_with_output()?;
        let _ = writer.join().map_err(|_| eprintln!("fzf stdin writer panicked"));

        if output.stdout.is_empty() {
            return Ok(None);
        }

        let raw = String::from_utf8_lossy(&output.stdout);
        let raw = raw.trim_end_matches('\n');
        let mut parts = raw.splitn(3, '\t');

        let _display = parts.next();
        let session_name = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("").to_string();

        if session_name.is_empty() {
            return Ok(None);
        }

        Ok(Some(PickResult { session_name, path }))
    }
}
