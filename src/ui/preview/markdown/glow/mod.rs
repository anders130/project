use std::process::Command;

mod style;

use super::MarkdownRenderer;
use crate::ui::palette;

pub struct GlowRenderer {
    pub style: String,
}

impl MarkdownRenderer for GlowRenderer {
    fn render(&self, path: &str, width: u16) {
        let tmp_path;
        let style: &str = if self.style == "auto" {
            let json = style::generate(&palette::detect());
            tmp_path = std::env::temp_dir().join(format!("glow-style-{}.json", std::process::id()));
            let _ = std::fs::write(&tmp_path, &json);
            tmp_path.to_str().unwrap_or("dark")
        } else {
            &self.style
        };

        let _ = Command::new("glow")
            .env("CLICOLOR_FORCE", "1")
            .args(["--style", style, "--width", &width.to_string(), path])
            .status();
    }
}
