mod directory;
pub mod markdown;
mod readme;
pub mod session_status;
pub mod subprocess;
pub mod tech_stack;

use std::path::Path;

use markdown::MarkdownRenderer;
use session_status::WindowView;
use tech_stack::TechItem;

pub fn render(
    path_str: &str,
    windows: Option<&[WindowView]>,
    tech_items: &[TechItem],
    renderer: &dyn MarkdownRenderer,
    header: Option<&str>,
    width: u16,
) {
    session_status::render(header, windows);

    if path_str.is_empty() {
        return;
    }

    let path = Path::new(path_str);
    tech_stack::render(tech_items);

    if !readme::render(path, renderer, width) {
        directory::render(path_str);
    }
}
