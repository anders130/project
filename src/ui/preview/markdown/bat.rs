#![allow(dead_code)]

use std::process::Command;

use super::MarkdownRenderer;

pub struct BatRenderer;

impl MarkdownRenderer for BatRenderer {
    fn render(&self, path: &str, width: u16) {
        let _ = Command::new("bat")
            .args([
                "--language=markdown",
                "--color=always",
                "--style=plain",
                "--paging=never",
                &format!("--terminal-width={width}"),
                path,
            ])
            .status();
    }
}
