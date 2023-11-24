use crate::syscall::bindings::perf_event_header;

mod aux;
mod aux_output_hw_id;
mod bpf_event;
mod cgroup;
mod comm;
mod exit;
mod fork;
mod intrace_start;
mod ksymbol;
mod lost;
mod lost_samples;
mod mmap;
mod mmap2;
mod namespaces;
mod read;
mod sample;
mod switch;
mod switch_cpu_wide;
mod text_poke;
mod throttle;
mod unthrottle;

pub enum RecordBody {
    Mmap(*const mmap::Body),
    Lost(*const lost::Body),
    Comm(*const comm::Body),
    Exit(*const exit::Body),
    Throttle(*const throttle::Body),
    Unthrottle(*const unthrottle::Body),
    Fork(*const fork::Body),
    Read(*const read::Body),
    Sample(*const sample::Body),
    Mmap2(*const mmap2::Body),
    Aux(*const aux::Body),
    ItraceStart(*const intrace_start::Body),
    LostSamples(*const lost_samples::Body),
    Switch(*const switch::Body),
    SwitchCpuWide(*const switch_cpu_wide::Body),
    Namespaces(*const namespaces::Body),
    Ksymbol(*const ksymbol::Body),
    BpfEvent(*const bpf_event::Body),
    Cgroup(*const cgroup::Body),
    TextPoke(*const text_poke::Body),
    AuxOutputHwId(*const aux_output_hw_id::Body), // TODO: missing docs in manual
}

#[allow(non_camel_case_types)]
pub struct sample_id {
    pid: u32,
    tid: u32,
    time: u64,
    id1: u64,
    stream_id: u64,
    cpu: u32,
    res: u32,
    id2: u64,
}

impl perf_event_header {
    fn follow_mem_ptr(&self) -> *const perf_event_header {
        let ptr = self as *const perf_event_header;
        unsafe { ptr.offset(1) }
    }

    pub fn body(&self) -> RecordBody {
        use crate::syscall::bindings::*;
        use RecordBody::*;
        let ptr = self.follow_mem_ptr();

        macro_rules! match_enum {
            ($(($perf_event_type:ident,$enum_type:expr,$ptr_type:ty)),+ $(,)?) => {
                match self.type_ {
                    $($perf_event_type => {
                        $enum_type(ptr as *const $ptr_type)
                    })+
                    _ => unreachable!()
                }
            };
        }

        match_enum! {
            (perf_event_type_PERF_RECORD_MMAP,Mmap,mmap::Body),
            (perf_event_type_PERF_RECORD_LOST,Lost,lost::Body),
            (perf_event_type_PERF_RECORD_COMM,Comm,comm::Body),
            (perf_event_type_PERF_RECORD_EXIT,Exit,exit::Body),
            (perf_event_type_PERF_RECORD_THROTTLE,Throttle,throttle::Body),
            (perf_event_type_PERF_RECORD_UNTHROTTLE,Unthrottle,unthrottle::Body),
            (perf_event_type_PERF_RECORD_FORK,Fork,fork::Body),
            (perf_event_type_PERF_RECORD_READ,Read,read::Body),
            (perf_event_type_PERF_RECORD_SAMPLE,Sample,sample::Body),
            (perf_event_type_PERF_RECORD_MMAP2,Mmap2,mmap2::Body),
            (perf_event_type_PERF_RECORD_AUX,Aux,aux::Body),
            (perf_event_type_PERF_RECORD_ITRACE_START,ItraceStart,intrace_start::Body),
            (perf_event_type_PERF_RECORD_LOST_SAMPLES,LostSamples,lost_samples::Body),
            (perf_event_type_PERF_RECORD_SWITCH,Switch,switch::Body),
            (perf_event_type_PERF_RECORD_SWITCH_CPU_WIDE,SwitchCpuWide,switch_cpu_wide::Body),
            (perf_event_type_PERF_RECORD_NAMESPACES,Namespaces,namespaces::Body),
            (perf_event_type_PERF_RECORD_KSYMBOL,Ksymbol,ksymbol::Body),
            (perf_event_type_PERF_RECORD_BPF_EVENT,BpfEvent,bpf_event::Body),
            (perf_event_type_PERF_RECORD_CGROUP,Cgroup,cgroup::Body),
            (perf_event_type_PERF_RECORD_TEXT_POKE,TextPoke,text_poke::Body),
            (perf_event_type_PERF_RECORD_AUX_OUTPUT_HW_ID,AuxOutputHwId,aux_output_hw_id::Body),
        }
    }
}
