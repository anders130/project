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

fn detect_techs(path: &Path) -> Vec<String> {
    let mut techs = Vec::new();

    let check = |file: &str| path.join(file).exists();

    if check("flake.nix") || check("default.nix") {
        techs.push(format!("{SKY}\u{F1125} Nix{RST}"));
    }
    if check("Cargo.toml") {
        techs.push(format!("{PCH}🦀 Rust{RST}"));
    }
    if check("tsconfig.json") {
        techs.push(format!("{BLU}\u{F02E6} TypeScript{RST}"));
    } else if check("package.json") {
        techs.push(format!("{GRN}\u{E718} Node{RST}"));
    }
    if check("pyproject.toml") || check("requirements.txt") || check("setup.py") {
        techs.push(format!("{YEL}🐍 Python{RST}"));
    }
    if check("go.mod") {
        techs.push(format!("{SKY}🐹 Go{RST}"));
    }
    if check("pom.xml") || check("build.gradle") {
        techs.push(format!("{PCH}☕ Java{RST}"));
    }
    if check("Gemfile") {
        techs.push(format!("{PCH}💎 Ruby{RST}"));
    }
    if check("composer.json") {
        techs.push(format!("{MVE}🐘 PHP{RST}"));
    }
    if check("Dockerfile") || check("docker-compose.yml") {
        techs.push(format!("{BLU}🐳 Docker{RST}"));
    }

    techs
}
