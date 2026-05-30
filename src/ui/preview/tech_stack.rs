use crate::projects::domain::lang::Lang;
use crate::ui::colors::*;

pub struct TechItem {
    pub icon: &'static str,
    pub name: &'static str,
    pub color: &'static str,
}

pub fn from_langs(langs: &[&'static Lang]) -> Vec<TechItem> {
    langs
        .iter()
        .map(|l| TechItem {
            icon: l.icon,
            name: l.name,
            color: lang_color(l.name),
        })
        .collect()
}

pub fn render(items: &[TechItem]) {
    if items.is_empty() {
        return;
    }
    let rendered: Vec<String> = items
        .iter()
        .map(|item| format!("{}{} {}{RST}", item.color, item.icon, item.name))
        .collect();
    println!();
    println!("  {}", rendered.join("  "));
}

fn lang_color(name: &str) -> &'static str {
    match name {
        "Nix" | "Go" => SKY,
        "Rust" | "Java" | "Ruby" => PCH,
        "TypeScript" | "Docker" => BLU,
        "Node" => GRN,
        "Python" => YEL,
        "PHP" => MVE,
        _ => DIM,
    }
}
