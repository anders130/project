use std::process::Command;

use crate::projects::domain::project::Project;
use crate::ui::picker::display::{build_category_colors, to_pick_entry};
use crate::ui::picker::port::ProjectPicker;
use crate::Result;

pub struct TvProjectPicker {
    pub preview_cmd: Option<String>,
}

fn strip_ansi(s: &str) -> String {
    s.chars()
        .fold((String::new(), 0u8), |(mut out, st), c| match (st, c) {
            (0, '\x1b') => (out, 1),
            (1, '[') => (out, 2),
            (1, _) => {
                out.push('\x1b');
                out.push(c);
                (out, 0)
            }
            (2, _) if c.is_ascii_alphabetic() => (out, 0),
            (2, _) => (out, 2),
            _ => {
                out.push(c);
                (out, 0)
            }
        })
        .0
}

fn visible_len(s: &str) -> usize {
    s.chars()
        .fold((0usize, 0u8), |(n, st), c| match (st, c) {
            (0, '\x1b') => (n, 1),
            (1, '[') => (n, 2),
            (1, _) => (n + 2, 0),
            (2, _) if c.is_ascii_alphabetic() => (n, 0),
            (2, _) => (n, 2),
            _ => (n + 1, 0),
        })
        .0
}

impl ProjectPicker for TvProjectPicker {
    fn pick(&self, open: &[&Project], closed: &[&Project], query: &str) -> Result<Option<usize>> {
        let all: Vec<&Project> = open.iter().chain(closed.iter()).copied().collect();
        let colors = build_category_colors(&all);
        let open_entries: Vec<_> = open.iter().map(|p| to_pick_entry(p, &colors)).collect();
        let closed_entries: Vec<_> = closed.iter().map(|p| to_pick_entry(p, &colors)).collect();

        let sep_display = format!("\x1b[2m{}\x1b[0m", "─".repeat(300));
        let sep_pre_pad = " ".repeat(250_usize.saturating_sub(300));
        let sep_post_pad = " ".repeat(250);
        let separator = format!("{}{}║{}║", sep_display, sep_pre_pad, sep_post_pad);

        // Entries: "{ansi_display}{pre_pad}║{index}{post_pad}║{plain_display}║{pid}"
        // Fields (split on ║):
        //   0: ansi_display + pre_pad  — list display
        //   1: index + post_pad        — bare integer after trim()
        //   2: plain_display           — preview panel header
        //   3: pid                     — run-unique nonce; prevents TV frecency from
        //                                promoting closed items above the separator
        let pid = std::process::id();
        let has_both = !open_entries.is_empty() && !closed_entries.is_empty();
        let n_open = open_entries.len();

        let fmt = |idx: usize, e: &crate::ui::picker::port::PickEntry| {
            let plain = strip_ansi(&e.display);
            let name = plain.get(3..).unwrap_or(&plain).to_owned();
            let pre_pad = " ".repeat(250_usize.saturating_sub(visible_len(&e.display)));
            let post_pad = " ".repeat(250);
            format!("{}{}║{idx}{}║{}║{pid}", e.display, pre_pad, post_pad, name)
        };

        let source_lines: Vec<String> = open_entries
            .iter()
            .enumerate()
            .map(|(i, e)| fmt(i, e))
            .chain(has_both.then_some(separator))
            .chain(
                closed_entries
                    .iter()
                    .enumerate()
                    .map(|(i, e)| fmt(n_open + i, e)),
            )
            .collect();

        // Meta file: session_name\tpath per line, no separator entry.
        let meta_lines: Vec<String> = open_entries
            .iter()
            .chain(closed_entries.iter())
            .map(|e| format!("{}\t{}", e.session_name, e.path))
            .collect();

        let tmp_dir = std::env::temp_dir();
        let tmp_source = tmp_dir.join(format!("project-tv-source-{pid}"));
        let tmp_meta = tmp_dir.join(format!("project-tv-meta-{pid}"));

        std::fs::write(&tmp_source, source_lines.join("\n"))?;
        std::fs::write(&tmp_meta, meta_lines.join("\n"))?;

        let tmp_source_str = tmp_source.to_str().ok_or("temp path not UTF-8")?;
        let tmp_meta_str = tmp_meta.to_str().ok_or("temp path not UTF-8")?;

        let mut args: Vec<String> = vec![
            format!("--source-command=cat {tmp_source_str}"),
            "--ansi".into(),
            "--source-output={split:║:1}".into(),
            "--preview-header={split:║:2}".into(),
            "--input-header=project".into(),
            "--no-status-bar".into(),
            format!("--input={query}"),
        ];
        if let Some(cmd) = &self.preview_cmd {
            args.push(format!(
                "--preview-command={cmd} {tmp_meta_str} {{split:║:1}}"
            ));
        }

        let output = Command::new("tv")
            .args(&args)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .output();

        let _ = std::fs::remove_file(&tmp_source);
        let _ = std::fs::remove_file(&tmp_meta);

        let output = output?;
        if output.stdout.is_empty() {
            return Ok(None);
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().parse().ok())
    }
}
