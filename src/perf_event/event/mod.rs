mod hw;
mod raw;
mod scope;
mod sw;
pub mod tracing;

pub use hw::*;
pub use raw::*;
pub use scope::*;
pub use sw::*;

pub enum Event {
    Hw(HwEvent),
    Sw(SwEvent),
    Raw(RawEvent),
}
