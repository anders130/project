use super::{normalize, Palette};

pub fn detect() -> Option<Palette> {
    std::env::var("KITTY_WINDOW_ID").ok()?;
    let config_home = std::env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap_or_default()));
    ["current-theme.conf", "kitty.conf"]
        .iter()
        .find_map(|name| {
            let path = format!("{config_home}/kitty/{name}");
            std::fs::read_to_string(&path).ok().and_then(|c| parse(&c))
        })
}

fn parse(content: &str) -> Option<Palette> {
    let found = content
        .lines()
        .filter(|l| !l.trim().starts_with('#') && !l.trim().is_empty())
        .filter_map(|line| {
            let line = line.trim();
            let rest = line.strip_prefix("color")?;
            let (idx_str, hex) = rest.split_once(|c: char| c.is_whitespace())?;
            let idx: usize = idx_str.trim().parse().ok()?;
            (idx < 16).then_some((idx, normalize(hex)))
        })
        .fold(vec![None::<String>; 16], |mut acc, (idx, hex)| {
            acc[idx] = Some(hex);
            acc
        });
    found
        .iter()
        .all(|c| c.is_some())
        .then(|| std::array::from_fn(|i| found[i].clone().unwrap()))
}
