/*
pub mod aux;
pub mod aux_output_hw_id;
pub mod bpf_event;
pub mod cgroup;
pub mod comm;
pub mod exit;
pub mod fork;
pub mod intrace_start;
pub mod ksymbol;
*/
pub mod lost;
pub mod lost_samples;
/*
pub mod mmap;
pub mod mmap2;
pub mod namespaces;
pub mod read;
*/
pub mod sample;
/*
pub mod switch;
pub mod switch_cpu_wide;
pub mod text_poke;
*/

pub mod throttle;
pub mod unthrottle;

#[derive(Debug)]
pub struct Record {
    pub misc: u16,
    pub body: RecordBody,
}

#[derive(Debug)]
pub enum RecordBody {
    /*
    Mmap(*const mmap::Body),
    */
    Lost(Box<lost::Body>),
    /*
    Comm(*const comm::Body),
    Exit(*const exit::Body),
     */
    Throttle(Box<throttle::Body>),
    Unthrottle(Box<unthrottle::Body>),
    /*
    Fork(*const fork::Body),
    Read(*const read::Body),
    */
    Sample(Box<sample::Body>),
    /*
    Mmap2(*const mmap2::Body),
    Aux(*const aux::Body),
    ItraceStart(*const intrace_start::Body),
    */
    LostSamples(Box<lost_samples::Body>),
    /*
    Switch(*const switch::Body),
    SwitchCpuWide(*const switch_cpu_wide::Body),
    Namespaces(*const namespaces::Body),
    Ksymbol(*const ksymbol::Body),
    BpfEvent(*const bpf_event::Body),
    Cgroup(*const cgroup::Body),
    TextPoke(*const text_poke::Body),
    AuxOutputHwId(*const aux_output_hw_id::Body), // TODO: missing docs in manual
    */
}

#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct sample_id {
    pub pid: u32,
    pub tid: u32,
    pub time: u64,
    pub id1: u64,
    pub stream_id: u64,
    pub cpu: u32,
    pub res: u32,
    pub id2: u64,
}
