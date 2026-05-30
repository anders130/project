mod navigator;
mod store;

use crate::sessions::ports::multiplexer::Multiplexer;

pub struct TmuxAdapter;

impl Multiplexer for TmuxAdapter {}
