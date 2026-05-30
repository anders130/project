use std::path::Path;

use crate::projects::domain::lang::{Lang, LANGS};

pub fn detect(path: &Path) -> Vec<&'static Lang> {
    LANGS
        .iter()
        .filter(|l| {
            l.exclude_if.iter().all(|f| !path.join(f).exists())
                && l.indicators.iter().any(|f| path.join(f).exists())
        })
        .collect()
}
