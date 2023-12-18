mod hw;
mod raw;
mod sw;

pub use hw::*;
pub use raw::*;
pub use sw::*;

pub enum Event {
    Hw(HwEvent),
    Sw(SwEvent),
    Raw(RawEvent),
}
