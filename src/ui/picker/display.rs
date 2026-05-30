use std::collections::HashMap;

use crate::projects::domain::project::Project;
use crate::sessions::domain::session_name;
use crate::ui::colors::*;
use crate::ui::picker::port::PickEntry;

pub fn build_category_colors(projects: &[&Project]) -> HashMap<String, &'static str> {
    const PALETTE: &[&str] = &[BLU, GRN, MVE, PCH, YEL, SKY];
    let mut cats: Vec<&str> = projects.iter().map(|p| p.repo.category.name()).collect();
    cats.sort_unstable();
    cats.dedup();
    cats.into_iter()
        .enumerate()
        .map(|(i, cat)| (cat.to_string(), PALETTE[i % PALETTE.len()]))
        .collect()
}

pub fn to_pick_entry(project: &Project, cat_colors: &HashMap<String, &'static str>) -> PickEntry {
    PickEntry {
        display: format_ansi(project, cat_colors),
        session_name: session_name::from_project(project.repo.name.display()),
        path: project.repo.path.display().to_string(),
    }
}

fn format_ansi(project: &Project, cat_colors: &HashMap<String, &'static str>) -> String {
    let col = cat_colors
        .get(project.repo.category.name())
        .copied()
        .unwrap_or(DIM);
    let label = project.repo.category.initial();
    let name = project.repo.name.display();
    match name.split_once('/') {
        Some((prefix, suffix)) => format!("{col}{label}{RST}  {DIM}{prefix}/{RST}{suffix}"),
        None => format!("{col}{label}{RST}  {name}"),
    }
}
