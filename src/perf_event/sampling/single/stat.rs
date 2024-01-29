use crate::infra::{SizedExt, WrapResult};
use crate::sampling::{ReadFormatHead, ReadFormatValue, Sampler};
use std::io;
use std::io::Read;
use std::mem::size_of;

#[derive(Debug, Clone)]
pub struct SamplerStat {
    pub event_id: u64,
    pub event_count: u64,
    #[cfg(feature = "linux-6.0")]
    pub event_lost: u64,
    pub time_enabled: u64,
    pub time_running: u64,
}

#[inline]
pub fn sampler_stat(sampler: &mut Sampler) -> io::Result<SamplerStat> {
    #[repr(C)]
    struct Layout {
        head: ReadFormatHead,
        value: ReadFormatValue,
    }

    let mut buf = unsafe { <[u8; size_of::<Layout>()]>::uninit() };
    sampler.file.read_exact(&mut buf)?;

    let layout = unsafe { &*(buf.as_ptr() as *const Layout) };
    SamplerStat {
        event_id: layout.value.event_id,
        event_count: layout.value.event_count,
        #[cfg(feature = "linux-6.0")]
        event_lost: layout.value.event_lost,
        time_enabled: layout.head.time_enabled,
        time_running: layout.head.time_running,
    }
    .wrap_ok()
}
