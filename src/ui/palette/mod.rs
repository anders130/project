mod kitty;

pub type Palette = [String; 16];

const STANDARD: [&str; 16] = [
    "#000000", "#800000", "#008000", "#808000", "#000080", "#800080", "#008080", "#c0c0c0",
    "#808080", "#ff0000", "#00ff00", "#ffff00", "#0000ff", "#ff00ff", "#00ffff", "#ffffff",
];

pub fn detect() -> Palette {
    from_env().or_else(kitty::detect).unwrap_or_else(standard)
}

pub fn normalize(s: &str) -> String {
    let s = s.trim();
    if s.starts_with('#') {
        s.to_string()
    } else {
        format!("#{s}")
    }
}

fn standard() -> Palette {
    std::array::from_fn(|i| STANDARD[i].to_string())
}

fn from_env() -> Option<Palette> {
    let val = std::env::var("PROJECT_PALETTE").ok()?;
    let parts: Vec<&str> = val.split(',').collect();
    if parts.len() < 16 {
        return None;
    }
    Some(std::array::from_fn(|i| normalize(parts[i])))
}
