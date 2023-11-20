mod hw_event;
mod raw_event;
mod sw_event;

pub use hw_event::*;
pub use raw_event::*;
pub use sw_event::*;

pub(crate) enum Event {
    Hw(HwEvent),
    Sw(SwEvent),
    Raw(RawEvent),
}
