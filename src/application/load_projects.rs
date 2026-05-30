use std::collections::HashSet;

use crate::history::ports::usage_reader::UsageReader;
use crate::projects::domain::project::{Project, UsageFrequency};
use crate::projects::domain::ranker;
use crate::projects::ports::project_source::ProjectSource;
use crate::sessions::domain::session_name;
use crate::sessions::ports::session_store::SessionStore;
use crate::Result;

pub fn execute(
    source: &dyn ProjectSource,
    sessions: &dyn SessionStore,
    usage: &dyn UsageReader,
) -> Result<(Vec<Project>, Vec<Project>)> {
    let all_sessions = sessions.all();
    let open_names: HashSet<&str> = all_sessions.iter().map(|s| s.name.as_str()).collect();
    let frequencies = usage.frequencies()?;

    let (open, closed): (Vec<_>, Vec<_>) = source
        .find_all()?
        .into_iter()
        .map(|repo| {
            let sname = session_name::from_project(repo.name.display());
            let freq = frequencies.get(&sname).copied().unwrap_or(0);
            let is_open = open_names.contains(sname.as_str());
            (
                is_open,
                Project {
                    repo,
                    frequency: UsageFrequency(freq),
                },
            )
        })
        .partition(|(is_open, _)| *is_open);

    let open = open.into_iter().map(|(_, p)| p).collect();
    let closed = closed.into_iter().map(|(_, p)| p).collect();

    Ok(ranker::rank(open, closed))
}
