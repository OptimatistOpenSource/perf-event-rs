mod breakpoint;
mod dynamic_pmu;
mod hardware;
mod raw;
mod scope;
mod software;
mod tracepoint;

use crate::perf_event::PerfEventAttr;
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
    pub(crate) fn enable_in_raw_attr(&self, perf_event_attr: &mut PerfEventAttr) {
        match self {
            Self::Hardware(ev) if ev.is_cache_event() => {
                perf_event_attr.type_ = PERF_TYPE_HW_CACHE;
                perf_event_attr.config = ev.as_u64();
            }
            Self::Hardware(ev) => {
                perf_event_attr.type_ = PERF_TYPE_HARDWARE;
                perf_event_attr.config = ev.as_u64();
            }
            Self::Software(ev) => {
                perf_event_attr.type_ = PERF_TYPE_SOFTWARE;
                perf_event_attr.config = ev.as_u64();
            }
            Self::Raw(ev) => {
                perf_event_attr.type_ = PERF_TYPE_RAW;
                perf_event_attr.config = ev.as_u64();
            }
            Self::Tracepoint(ev) => {
                perf_event_attr.type_ = PERF_TYPE_TRACEPOINT;
                perf_event_attr.config = ev.id
            }
            Self::Breakpoint(ev) => {
                perf_event_attr.type_ = PERF_TYPE_BREAKPOINT;
                perf_event_attr.config = 0;
                match &ev.bp_type {
                    BreakpointType::R { addr, len } => {
                        perf_event_attr.bp_type = HW_BREAKPOINT_R;
                        perf_event_attr.__bindgen_anon_3.bp_addr = *addr;
                        perf_event_attr.__bindgen_anon_4.bp_len = len.as_u64();
                    }
                    BreakpointType::W { addr, len } => {
                        perf_event_attr.bp_type = HW_BREAKPOINT_W;
                        perf_event_attr.__bindgen_anon_3.bp_addr = *addr;
                        perf_event_attr.__bindgen_anon_4.bp_len = len.as_u64();
                    }
                    BreakpointType::Rw { addr, len } => {
                        perf_event_attr.bp_type = HW_BREAKPOINT_RW;
                        perf_event_attr.__bindgen_anon_3.bp_addr = *addr;
                        perf_event_attr.__bindgen_anon_4.bp_len = len.as_u64();
                    }
                    BreakpointType::X { addr } => {
                        perf_event_attr.bp_type = HW_BREAKPOINT_X;
                        perf_event_attr.__bindgen_anon_3.bp_addr = *addr;
                        perf_event_attr.__bindgen_anon_4.bp_len = size_of::<c_long>() as _;
                    }
                };
            }
            Self::DynamicPmu(ev) => match ev {
                DynamicPmuEvent::Other { r#type, config } => {
                    perf_event_attr.type_ = *r#type;
                    perf_event_attr.config = *config;
                }
                #[cfg(feature = "linux-4.17")]
                DynamicPmuEvent::Kprobe {
                    r#type,
                    retprobe,
                    cfg,
                } => {
                    perf_event_attr.type_ = *r#type;
                    perf_event_attr.config |= *retprobe as u64;
                    match cfg {
                        KprobeConfig::FuncAndOffset {
                            kprobe_func,
                            probe_offset,
                        } => {
                            perf_event_attr.__bindgen_anon_3.kprobe_func =
                                kprobe_func.as_ptr() as _;
                            perf_event_attr.__bindgen_anon_4.probe_offset = *probe_offset;
                        }
                        KprobeConfig::KprobeAddr(addr) => {
                            perf_event_attr.__bindgen_anon_3.kprobe_func = 0;
                            perf_event_attr.__bindgen_anon_4.kprobe_addr = *addr;
                        }
                    }
                }
                #[cfg(feature = "linux-4.17")]
                DynamicPmuEvent::Uprobe {
                    r#type,
                    retprobe,
                    cfg,
                } => {
                    perf_event_attr.type_ = *r#type;
                    perf_event_attr.config |= *retprobe as u64;
                    perf_event_attr.__bindgen_anon_3.uprobe_path = cfg.uprobe_path.as_ptr() as _;
                    perf_event_attr.__bindgen_anon_4.probe_offset = cfg.probe_offset;
                }
            },
        }
    }
}
