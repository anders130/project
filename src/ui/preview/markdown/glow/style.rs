use crate::ui::palette::Palette;

const TEMPLATE: &str = include_str!("style_template.json");

pub fn generate(p: &Palette) -> String {
    TEMPLATE
        .replace("%C1%", &p[1])
        .replace("%C2%", &p[2])
        .replace("%C3%", &p[3])
        .replace("%C4%", &p[4])
        .replace("%C5%", &p[5])
        .replace("%C6%", &p[6])
        .replace("%C7%", &p[7])
        .replace("%C9%", &p[9])
}
