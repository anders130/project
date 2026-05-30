use std::io::Write;
use std::process::{Command, Stdio};

use crate::projects::domain::project::Project;
use crate::ui::colors::*;
use crate::ui::picker::display::{build_category_colors, to_pick_entry};
use crate::ui::picker::port::ProjectPicker;
use crate::Result;

pub struct FzfProjectPicker {
    pub preview_cmd: Option<String>,
}

impl ProjectPicker for FzfProjectPicker {
    fn pick(&self, open: &[&Project], closed: &[&Project], query: &str) -> Result<Option<usize>> {
        let all: Vec<&Project> = open.iter().chain(closed.iter()).copied().collect();
        let colors = build_category_colors(&all);
        let open_entries: Vec<_> = open.iter().map(|p| to_pick_entry(p, &colors)).collect();
        let closed_entries: Vec<_> = closed.iter().map(|p| to_pick_entry(p, &colors)).collect();

        let pid = std::process::id();
        let tmp_meta = std::env::temp_dir().join(format!("project-fzf-meta-{pid}"));
        super::meta::write(&tmp_meta, &open_entries, &closed_entries)?;
        let tmp_meta_str = tmp_meta.to_str().ok_or("temp path not UTF-8")?;

        let sep = format!("{DIM}{}{RST}", "─".repeat(300));
        let has_both = !open_entries.is_empty() && !closed_entries.is_empty();
        let n_open = open_entries.len();

        let lines: Vec<String> = open_entries
            .iter()
            .enumerate()
            .map(|(i, e)| format!("{}\t{i}", e.display))
            .chain(has_both.then(|| format!("{sep}\t")))
            .chain(
                closed_entries
                    .iter()
                    .enumerate()
                    .map(|(i, e)| format!("{}\t{}", e.display, n_open + i)),
            )
            .collect();

        let color_arg = "--color=bg+:8,gutter:0,hl:1,hl+:1,pointer:15,marker:2,prompt:5,info:7,border:8,separator:8";

        let mut args: Vec<String> = vec![
            "--ansi".into(),
            "--delimiter=\t".into(),
            "--with-nth=1".into(),
            format!("--query={query}"),
            color_arg.into(),
            "--border=rounded".into(),
            "--layout=reverse".into(),
            "--info=inline".into(),
            "--prompt=  ".into(),
        ];
        if let Some(cmd) = &self.preview_cmd {
            args.push(format!("--preview={cmd} --show-name {tmp_meta_str} {{2}}"));
            args.push("--preview-window=right:50%:border-left:wrap".into());
        }

        let mut child = Command::new("fzf")
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take().ok_or("fzf stdin unavailable")?;
        let writer = std::thread::spawn(move || {
            let mut w = stdin;
            let _ = lines.iter().try_for_each(|line| writeln!(w, "{line}"));
        });

        let output = child.wait_with_output()?;
        let _ = writer
            .join()
            .map_err(|_| eprintln!("fzf stdin writer panicked"));
        let _ = std::fs::remove_file(&tmp_meta);

        if output.stdout.is_empty() {
            return Ok(None);
        }

        Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .rsplit_once('\t')
            .and_then(|(_, s)| s.trim().parse().ok()))
    }
}
