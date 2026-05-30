use crate::ui::colors::*;

pub struct WindowView {
    pub index: u32,
    pub name: String,
    pub active: bool,
}

pub fn render(header: Option<&str>, windows: Option<&[WindowView]>) {
    match windows {
        Some(ws) => {
            if let Some(name) = header {
                println!("{GRN}●{RST} {BLD}{name}{RST}");
            }
            for w in ws {
                if w.active {
                    println!("{BLU}{}: {} ◀{RST}", w.index, w.name);
                } else {
                    println!("{SUB}{}: {}{RST}", w.index, w.name);
                }
            }
        }
        None => {
            if let Some(name) = header {
                println!("{DIM}○ {name}{RST}");
            }
        }
    }
}
