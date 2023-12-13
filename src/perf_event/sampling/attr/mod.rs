mod extra_config;
mod extra_record;
mod new;
mod sample_record_fields;

use crate::perf_event::RawAttr;
use crate::{Event, EventScope};
pub use extra_record::*;
use std::fmt::Debug;

pub use extra_config::*;
pub use extra_record::*;
pub use sample_record_fields::*;

pub enum OverflowBy {
    Period(u64),
    Freq(u64),
}

#[derive(Debug, Clone)]
pub struct Attr {
    raw_attr: RawAttr,
}

impl Attr {
    pub fn new(
        event: impl Into<Event>,
        scopes: impl IntoIterator<Item = EventScope>,
        overflow_by: OverflowBy,
        extra_config: &ExtraConfig,
    ) -> Self {
        new::new(event, scopes, overflow_by, extra_config)
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
