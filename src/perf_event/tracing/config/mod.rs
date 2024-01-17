mod new;

use crate::perf_event::RawAttr;
use crate::{Event, EventScope};
use std::ffi::CString;

pub type ExtraConfig = crate::sampling::ExtraConfig;

#[derive(Debug, Clone)]
pub struct Config {
    #[allow(dead_code)]
    kprobe_func: Option<CString>, // This keep ptr to `kprobe_func` valid, if it exists.
    #[allow(dead_code)]
    uprobe_path: Option<CString>, // This keep ptr to `uprobe_path` valid, if it exists.
    raw_attr: RawAttr,
}

impl Config {
    pub fn new(
        event: impl Into<Event>,
        scopes: impl IntoIterator<Item = EventScope>,
        extra_config: &ExtraConfig,
    ) -> Self {
        new::new(event, scopes, extra_config)
    }

    /// Construct from a raw `perf_event_attr` struct.
    /// # Safety
    /// The `raw_attr` argument must be a properly initialized
    /// `perf_event_attr` struct for counting mode.
    pub const unsafe fn from_raw(raw_attr: RawAttr) -> Self {
        Self {
            kprobe_func: None,
            uprobe_path: None,
            raw_attr,
        }
    }

    pub fn into_raw(self) -> RawAttr {
        self.raw_attr
    }

    pub const fn as_raw(&self) -> &RawAttr {
        &self.raw_attr
    }
}
