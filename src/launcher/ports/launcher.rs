use super::session_config_editor::SessionConfigEditor;
use super::session_starter::SessionStarter;

pub trait Launcher: SessionStarter + SessionConfigEditor {}
