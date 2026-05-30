use crate::sessions::domain::session::Session;

pub trait SessionStore {
    fn all(&self) -> Vec<Session>;
    fn find(&self, name: &str) -> Option<Session>;
}
