use crate::Result;

pub trait UsageRecorder {
    fn record(&self, session_name: &str) -> Result<()>;
}
