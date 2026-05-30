use super::session_navigator::SessionNavigator;
use super::session_store::SessionStore;

pub trait Multiplexer: SessionStore + SessionNavigator {}
