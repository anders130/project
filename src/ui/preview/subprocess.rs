use std::path::Path;

use crate::projects::adapters::lang_detector;
use crate::sessions::ports::session_store::SessionStore;
use crate::ui::picker::meta;
use crate::ui::preview;
use crate::ui::preview::markdown::MarkdownRenderer;
use crate::ui::preview::session_status::WindowView;
use crate::ui::preview::tech_stack;
use crate::Result;

pub fn handle(
    args: &[String],
    store: &dyn SessionStore,
    renderer: &dyn MarkdownRenderer,
) -> Result<()> {
    let show_name = args.first().map(|s| s == "--show-name").unwrap_or(false);
    let offset = usize::from(show_name);
    let meta_path = args.get(offset).map(|s| s.as_str()).unwrap_or("");
    let index: usize = args
        .get(1 + offset)
        .and_then(|s| s.parse().ok())
        .unwrap_or(usize::MAX);

    let Some((session_name, path, is_open)) = meta::resolve(meta_path, index) else {
        return Ok(());
    };

    let header = if show_name {
        Some(session_name.as_str())
    } else {
        None
    };

    let windows: Option<Vec<WindowView>> = is_open
        .then(|| {
            store.find(&session_name).map(|s| {
                s.windows
                    .iter()
                    .map(|w| WindowView {
                        index: w.index,
                        name: w.name.clone(),
                        active: w.active,
                    })
                    .collect()
            })
        })
        .flatten();

    let tech_items = if !path.is_empty() {
        tech_stack::from_langs(&lang_detector::detect(Path::new(&path)))
    } else {
        Default::default()
    };

    preview::render(
        &path,
        windows.as_deref(),
        &tech_items,
        renderer,
        header,
        preview_width(),
    );
    Ok(())
}

fn preview_width() -> u16 {
    std::env::var("FZF_PREVIEW_COLUMNS")
        .or_else(|_| std::env::var("COLUMNS"))
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(80)
        .min(100)
}
