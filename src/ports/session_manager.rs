use crate::domain::entities::Session;
use crate::Result;

pub trait SessionManager {
    fn list_sessions(&self) -> Vec<Session>;
    fn get_session(&self, name: &str) -> Option<Session>;
    fn has_session(&self, name: &str) -> bool;
    fn is_in_tmux(&self) -> bool;
    fn switch_client(&self, name: &str) -> Result<()>;
    fn attach_session(&self, name: &str) -> Result<()>;
}
