use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Repo {
    pub display_name: String,
    pub session_name: String,
    pub path: PathBuf,
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct Window {
    pub index: u32,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub windows: Vec<Window>,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub repo: Repo,
    pub session: Option<Session>,
    pub frequency: usize,
}
