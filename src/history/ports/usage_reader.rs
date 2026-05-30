use std::collections::HashMap;

use crate::Result;

pub trait UsageReader {
    fn frequencies(&self) -> Result<HashMap<String, usize>>;
}
