use crate::history::ports::usage_recorder::UsageRecorder;
use crate::launcher::ports::session_starter::SessionStarter;
use crate::projects::domain::project::Project;
use crate::sessions::domain::session_name;
use crate::sessions::ports::session_navigator::SessionNavigator;
use crate::sessions::ports::session_store::SessionStore;
use crate::Result;

pub fn execute(
    project: &Project,
    usage: &dyn UsageRecorder,
    sessions: &dyn SessionStore,
    starter: &dyn SessionStarter,
    navigator: &dyn SessionNavigator,
) -> Result<()> {
    let name = session_name::from_project(project.repo.name.display());
    usage.record(&name)?;
    if sessions.find(&name).is_some() {
        return navigate(navigator, &name);
    }
    starter.start(&name, &project.repo.path)?;
    navigate(navigator, &name)
}

fn navigate(nav: &dyn SessionNavigator, name: &str) -> Result<()> {
    if nav.is_in_session() {
        nav.switch_to(name)
    } else {
        nav.attach_to(name)
    }
}
