use super::window::Window;

#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub windows: Vec<Window>,
}
