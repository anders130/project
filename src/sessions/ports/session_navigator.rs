use crate::Result;

pub trait SessionNavigator {
    fn is_in_session(&self) -> bool;
    fn switch_to(&self, name: &str) -> Result<()>;
    fn attach_to(&self, name: &str) -> Result<()>;
    fn on_session_started(&self, _name: &str) {}
}
