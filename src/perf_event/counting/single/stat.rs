use crate::counting::{Counter, ReadFormatHead, ReadFormatValue};
use crate::infra::{SizedExt, WrapResult};
use std::io;
use std::io::Read;
use std::mem::size_of;

#[derive(Debug, Clone)]
pub struct CounterStat {
    pub event_id: u64,
    pub event_count: u64,
    pub time_enabled: u64,
    pub time_running: u64,
}

#[inline]
pub fn counter_stat(counter: &mut Counter) -> io::Result<CounterStat> {
    #[repr(C)]
    struct Layout {
        head: ReadFormatHead,
        value: ReadFormatValue,
    }

    let mut buf = unsafe { <[u8; size_of::<Layout>()]>::uninit() };
    counter.file.read_exact(&mut buf)?;

    let layout = unsafe { &*(buf.as_ptr() as *const Layout) };
    CounterStat {
        event_id: layout.value.event_id,
        event_count: layout.value.event_count,
        time_enabled: layout.head.time_enabled,
        time_running: layout.head.time_running,
    }
    .wrap_ok()
}
