mod new;

use crate::perf_event::RawAttr;
use crate::{Event, EventScope};
use std::ffi::CString;
use std::rc::Rc;

pub type ExtraConfig = crate::sampling::ExtraConfig;

#[derive(Debug, Clone)]
pub struct Config {
    // This will keep the ptr of `kprobe_func` or `uprobe_path` valid if present.
    #[allow(dead_code)]
    kprobe_func_or_uprobe_path: Option<Rc<CString>>,
    raw_attr: RawAttr,
}

impl Config {
    pub fn new<'t>(
        event: &Event,
        scopes: impl IntoIterator<Item = &'t EventScope>,
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
            kprobe_func_or_uprobe_path: None,
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
