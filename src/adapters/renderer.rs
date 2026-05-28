use std::path::Path;
use std::process::Command;

use crate::adapters::colors::*;
use crate::ports::session_manager::SessionManager;

pub fn render_preview(session_name: &str, path_str: &str, tmux: &dyn SessionManager) {
    match tmux.get_session(session_name) {
        Some(session) => {
            println!("{GRN}●{RST} {BLD}{session_name}{RST}");
            for w in &session.windows {
                if w.active {
                    println!("  {BLU}{}: {} ◀{RST}", w.index, w.name);
                } else {
                    println!("  {SUB}{}: {}{RST}", w.index, w.name);
                }
            }
        }
        None => println!("{DIM}○ {session_name}{RST}"),
    }

    if path_str.is_empty() {
        return;
    }

    let path = Path::new(path_str);
    let techs = detect_techs(path);

    if !techs.is_empty() {
        println!();
        println!("  {}", techs.join("  "));
    }

    for readme in &["README.md", "readme.md", "README"] {
        let readme_path = path.join(readme);
        if !readme_path.exists() {
            continue;
        }
        let cols = std::env::var("FZF_PREVIEW_COLUMNS").unwrap_or_else(|_| "80".into());
        println!();
        if let Some(p) = readme_path.to_str() {
            let _ = Command::new("bat")
                .args([
                    "--language=markdown",
                    "--color=always",
                    "--style=plain",
                    "--paging=never",
                    &format!("--terminal-width={cols}"),
                    p,
                ])
                .status();
        }
        return;
    }

    println!();
    let _ = Command::new("ls")
        .args(["--group-directories-first", "--color=always", path_str])
        .status();
}

struct Lang {
    name: &'static str,
    icon: &'static str,
    color: &'static str,
    indicators: &'static [&'static str],
    exclude_if: &'static [&'static str],
}

impl Lang {
    fn detected(&self, path: &Path) -> bool {
        self.exclude_if.iter().all(|f| !path.join(f).exists())
            && self.indicators.iter().any(|f| path.join(f).exists())
    }

    fn display(&self) -> String {
        format!("{}{} {}{RST}", self.color, self.icon, self.name)
    }
}

const LANGS: &[Lang] = &[
    Lang {
        name: "Nix",
        icon: "\u{F313}",
        color: SKY,
        indicators: &["flake.nix", "default.nix"],
        exclude_if: &[],
    },
    Lang {
        name: "Rust",
        icon: "\u{E7A8}",
        color: PCH,
        indicators: &["Cargo.toml"],
        exclude_if: &[],
    },
    Lang {
        name: "TypeScript",
        icon: "\u{E628}",
        color: BLU,
        indicators: &["tsconfig.json"],
        exclude_if: &[],
    },
    Lang {
        name: "Node",
        icon: "\u{E718}",
        color: GRN,
        indicators: &["package.json"],
        exclude_if: &["tsconfig.json"],
    },
    Lang {
        name: "Python",
        icon: "\u{E606}",
        color: YEL,
        indicators: &["pyproject.toml", "requirements.txt", "setup.py"],
        exclude_if: &[],
    },
    Lang {
        name: "Go",
        icon: "\u{E724}",
        color: SKY,
        indicators: &["go.mod"],
        exclude_if: &[],
    },
    Lang {
        name: "Java",
        icon: "\u{E738}",
        color: PCH,
        indicators: &["pom.xml", "build.gradle"],
        exclude_if: &[],
    },
    Lang {
        name: "Ruby",
        icon: "\u{E23E}",
        color: PCH,
        indicators: &["Gemfile"],
        exclude_if: &[],
    },
    Lang {
        name: "PHP",
        icon: "\u{E608}",
        color: MVE,
        indicators: &["composer.json"],
        exclude_if: &[],
    },
    Lang {
        name: "Docker",
        icon: "\u{E7B0}",
        color: BLU,
        indicators: &["Dockerfile", "docker-compose.yml"],
        exclude_if: &[],
    },
];

fn detect_techs(path: &Path) -> Vec<String> {
    LANGS
        .iter()
        .filter(|l| l.detected(path))
        .map(|l| l.display())
        .collect()
}
