use std::collections::HashMap;

use crate::domain::entities::{Project, Repo, Session};

pub fn build_projects(
    repos: Vec<Repo>,
    sessions: &[Session],
    frequencies: &HashMap<String, usize>,
) -> Vec<Project> {
    repos
        .into_iter()
        .map(|repo| {
            let session = sessions.iter().find(|s| s.name == repo.session_name).cloned();
            let frequency = frequencies.get(&repo.session_name).copied().unwrap_or(0);
            Project { repo, session, frequency }
        })
        .collect()
}

pub fn sort_projects(projects: Vec<Project>) -> (Vec<Project>, Vec<Project>) {
    let cmp = |a: &Project, b: &Project| {
        b.frequency
            .cmp(&a.frequency)
            .then_with(|| a.repo.display_name.to_lowercase().cmp(&b.repo.display_name.to_lowercase()))
    };

    let (mut open, mut closed): (Vec<_>, Vec<_>) =
        projects.into_iter().partition(|p| p.session.is_some());

    open.sort_by(cmp);
    closed.sort_by(cmp);

    (open, closed)
}
