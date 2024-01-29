mod breakpoint;
mod dynamic_pmu;
mod hardware;
mod raw;
mod scope;
mod software;
mod tracepoint;

use crate::perf_event::RawAttr;
use crate::syscall::bindings::*;
use libc::c_long;
use std::mem::size_of;

pub use breakpoint::*;
pub use dynamic_pmu::*;
pub use hardware::*;
pub use raw::*;
pub use scope::*;
pub use software::*;
pub use tracepoint::*;

#[derive(Clone, Debug)]
pub enum Event {
    Hardware(HardwareEvent),
    Software(SoftwareEvent),
    Raw(RawEvent),
    Tracepoint(TracepointEvent),
    Breakpoint(BreakpointEvent),
    DynamicPmu(DynamicPmuEvent),
}

impl Event {
    pub(crate) fn enable_in_raw_attr(&self, raw_attr: &mut RawAttr) {
        match self {
            Self::Hardware(ev) if ev.is_cache_event() => {
                raw_attr.type_ = PERF_TYPE_HW_CACHE;
                raw_attr.config = ev.as_u64();
            }
            Self::Hardware(ev) => {
                raw_attr.type_ = PERF_TYPE_HARDWARE;
                raw_attr.config = ev.as_u64();
            }
            Self::Software(ev) => {
                raw_attr.type_ = PERF_TYPE_SOFTWARE;
                raw_attr.config = ev.as_u64();
            }
            Self::Raw(ev) => {
                raw_attr.type_ = PERF_TYPE_RAW;
                raw_attr.config = ev.as_u64();
            }
            Self::Tracepoint(ev) => {
                raw_attr.type_ = PERF_TYPE_TRACEPOINT;
                raw_attr.config = ev.id
            }
            Self::Breakpoint(ev) => {
                raw_attr.type_ = PERF_TYPE_BREAKPOINT;
                raw_attr.config = 0;
                match &ev.bp_type {
                    BreakpointType::R { addr, len } => {
                        raw_attr.bp_type = HW_BREAKPOINT_R;
                        raw_attr.__bindgen_anon_3.bp_addr = *addr;
                        raw_attr.__bindgen_anon_4.bp_len = len.as_u64();
                    }
                    BreakpointType::W { addr, len } => {
                        raw_attr.bp_type = HW_BREAKPOINT_W;
                        raw_attr.__bindgen_anon_3.bp_addr = *addr;
                        raw_attr.__bindgen_anon_4.bp_len = len.as_u64();
                    }
                    BreakpointType::Rw { addr, len } => {
                        raw_attr.bp_type = HW_BREAKPOINT_RW;
                        raw_attr.__bindgen_anon_3.bp_addr = *addr;
                        raw_attr.__bindgen_anon_4.bp_len = len.as_u64();
                    }
                    BreakpointType::X { addr } => {
                        raw_attr.bp_type = HW_BREAKPOINT_X;
                        raw_attr.__bindgen_anon_3.bp_addr = *addr;
                        raw_attr.__bindgen_anon_4.bp_len = size_of::<c_long>() as _;
                    }
                };
            }
            Self::DynamicPmu(ev) => match ev {
                DynamicPmuEvent::Other { r#type, config } => {
                    raw_attr.type_ = *r#type;
                    raw_attr.config = *config;
                }
                #[cfg(feature = "linux-4.17")]
                DynamicPmuEvent::Kprobe {
                    r#type,
                    retprobe,
                    cfg,
                } => {
                    raw_attr.type_ = *r#type;
                    raw_attr.config |= *retprobe as u64;
                    match cfg {
                        KprobeConfig::FuncAndOffset {
                            kprobe_func,
                            probe_offset,
                        } => {
                            raw_attr.__bindgen_anon_3.kprobe_func = kprobe_func.as_ptr() as _;
                            raw_attr.__bindgen_anon_4.probe_offset = *probe_offset;
                        }
                        KprobeConfig::KprobeAddr(addr) => {
                            raw_attr.__bindgen_anon_3.kprobe_func = 0;
                            raw_attr.__bindgen_anon_4.kprobe_addr = *addr;
                        }
                    }
                }
                #[cfg(feature = "linux-4.17")]
                DynamicPmuEvent::Uprobe {
                    r#type,
                    retprobe,
                    cfg,
                } => {
                    raw_attr.type_ = *r#type;
                    raw_attr.config |= *retprobe as u64;
                    raw_attr.__bindgen_anon_3.uprobe_path = cfg.uprobe_path.as_ptr() as _;
                    raw_attr.__bindgen_anon_4.probe_offset = cfg.probe_offset;
                }
            },
        }
    }
}
