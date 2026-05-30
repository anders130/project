use crate::launcher::ports::session_config_editor::SessionConfigEditor;
use crate::projects::domain::project::Project;
use crate::sessions::domain::session_name;
use crate::Result;

pub fn execute(project: &Project, editor: &dyn SessionConfigEditor) -> Result<()> {
    let name = session_name::from_project(project.repo.name.display());
    let config_path = editor.prepare_for_edit(&name, &project.repo.path)?;
    let editor_cmd = std::env::var("EDITOR").unwrap_or_else(|_| "vi".into());
    std::process::Command::new(editor_cmd)
        .arg(config_path)
        .status()?;
    Ok(())
}
