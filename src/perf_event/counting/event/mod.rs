mod hw_event;
mod raw_event;
mod sw_event;

pub use hw_event::*;
pub use raw_event::*;
pub use sw_event::*;

pub(crate) enum Inner {
    Hw(HwEvent),
    Sw(SwEvent),
    Raw(RawEvent),
}

pub struct Event(Inner);

impl Event {
    pub(crate) fn into_inner(self) -> Inner {
        self.0
    }
}
