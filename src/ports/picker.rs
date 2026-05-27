use crate::Result;

pub struct PickItem {
    pub display: String,
    pub session_name: String,
    pub path: String,
}

pub enum PickEntry {
    Item(PickItem),
    Separator,
}

pub struct PickResult {
    pub session_name: String,
    pub path: String,
}

pub trait Picker {
    fn pick(&self, items: &[PickEntry], query: &str) -> Result<Option<PickResult>>;
}
