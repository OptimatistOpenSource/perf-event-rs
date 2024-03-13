mod extra_config;
mod extra_record;
mod new;
mod sample_record_fields;

use crate::perf_event::PerfEventAttr;
use crate::{Event, EventScope};
use std::ffi::CString;
use std::fmt::Debug;
use std::rc::Rc;

pub use extra_config::*;
pub use extra_record::*;
pub use sample_record_fields::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum OverflowBy {
    Period(u64),
    Freq(u64),
}

#[derive(Debug, Clone)]
pub struct Config {
    // This will keep the ptr of `kprobe_func` or `uprobe_path` valid if present.
    #[allow(dead_code)]
    kprobe_func_or_uprobe_path: Option<Rc<CString>>,
    perf_event_attr: PerfEventAttr,
}

impl Config {
    pub fn new<'t>(
        event: &Event,
        scopes: impl IntoIterator<Item = &'t EventScope>,
        overflow_by: &OverflowBy,
    ) -> Self {
        Self::extra_new(event, scopes, overflow_by, &Default::default())
    }

    pub fn extra_new<'t>(
        event: &Event,
        scopes: impl IntoIterator<Item = &'t EventScope>,
        overflow_by: &OverflowBy,
        extra_config: &ExtraConfig,
    ) -> Self {
        new::new(event, scopes, overflow_by, extra_config)
    }

    /// Construct from a `PerfEventAttr` struct.
    /// # Safety
    /// The `perf_event_attr` argument must be properly initialized from
    /// `PerfEventAttr` struct for counting mode.
    pub const unsafe fn from_raw(perf_event_attr: PerfEventAttr) -> Self {
        Self {
            kprobe_func_or_uprobe_path: None,
            perf_event_attr,
        }
    }

    pub fn into_raw(self) -> PerfEventAttr {
        self.perf_event_attr
    }

    pub const fn as_raw(&self) -> &PerfEventAttr {
        &self.perf_event_attr
    }
}
