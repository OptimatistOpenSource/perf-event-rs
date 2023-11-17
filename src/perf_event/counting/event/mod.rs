use crate::perf_event::counting::event::hw_event::HwEvent;
use crate::perf_event::counting::event::raw_event::RawEvent;
use crate::perf_event::counting::event::sw_event::SwEvent;

mod hw_event;
mod raw_event;
mod sw_event;

pub use hw_event::*;
pub use raw_event::*;
pub use sw_event::*;

enum Inner {
    Hw(HwEvent),
    Sw(SwEvent),
    Raw(RawEvent),
}

pub struct Event(Inner);

impl From<HwEvent> for Event {
    fn from(value: HwEvent) -> Self {
        Self(Inner::Hw(value))
    }
}

impl From<SwEvent> for Event {
    fn from(value: SwEvent) -> Self {
        Self(Inner::Sw(value))
    }
}

impl From<RawEvent> for Event {
    fn from(value: RawEvent) -> Self {
        Self(Inner::Raw(value))
    }
}
