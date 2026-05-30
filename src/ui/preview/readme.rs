use std::path::Path;

use super::markdown::MarkdownRenderer;

pub fn render(path: &Path, renderer: &dyn MarkdownRenderer, width: u16) -> bool {
    ["README.md", "readme.md", "README"]
        .iter()
        .find_map(|name| {
            let readme = path.join(name);
            readme.exists().then(|| {
                println!();
                if let Some(p) = readme.to_str() {
                    renderer.render(p, width);
                }
            })
        })
        .is_some()
}
