use std::collections::HashMap;
use crate::Result;

pub trait HistoryStore {
    fn record(&self, session_name: &str) -> Result<()>;
    fn frequencies(&self) -> Result<HashMap<String, usize>>;
}
