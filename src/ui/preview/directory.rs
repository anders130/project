use std::process::Command;

pub fn render(path_str: &str) {
    println!();
    let _ = Command::new("ls")
        .args(["--group-directories-first", "--color=always", path_str])
        .status();
}
