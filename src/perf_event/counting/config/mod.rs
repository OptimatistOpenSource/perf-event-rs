mod extra_config;
mod new;

use crate::perf_event::RawAttr;
use std::fmt::Debug;

use crate::{Event, EventScope};
pub use extra_config::*;

#[derive(Debug, Clone)]
pub struct Config {
    raw_attr: RawAttr,
}

impl Config {
    pub fn new(
        event: impl Into<Event>,
        scopes: impl IntoIterator<Item = EventScope>,
        extra_config: ExtraConfig,
    ) -> Self {
        new::new(event, scopes, extra_config)
    }

    /// Construct from a raw `perf_event_attr` struct.
    /// # Safety
    /// The `raw_attr` argument must be a properly initialized
    /// `perf_event_attr` struct for counting mode.
    pub const unsafe fn from_raw(raw_attr: RawAttr) -> Self {
        Self { raw_attr }
    }

    pub const fn into_raw(self) -> RawAttr {
        self.raw_attr
    }

    pub const fn as_raw(&self) -> &RawAttr {
        &self.raw_attr
    }
}
