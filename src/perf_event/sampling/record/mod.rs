mod body;
mod sample_id;

pub use body::*;
pub use sample_id::*;

#[derive(Debug, Clone)]
pub struct Record {
    pub misc: u16,
    pub body: RecordBody,
}

#[derive(Debug, Clone)]
pub enum RecordBody {
    Mmap(Box<mmap::Body>),
    Lost(Box<lost::Body>),
    Comm(Box<comm::Body>),
    Exit(Box<exit::Body>),
    Throttle(Box<throttle::Body>),
    Unthrottle(Box<unthrottle::Body>),
    Fork(Box<fork::Body>),
    Read(Box<read::Body>),
    Sample(Box<sample::Body>),
    Mmap2(Box<mmap2::Body>),
    Aux(Box<aux::Body>),
    ItraceStart(Box<intrace_start::Body>),
    LostSamples(Box<lost_samples::Body>),
    Switch(Box<switch::Body>),
    SwitchCpuWide(Box<switch_cpu_wide::Body>),
    Namespaces(Box<namespaces::Body>),
    Ksymbol(Box<ksymbol::Body>),
    BpfEvent(Box<bpf_event::Body>),
    Cgroup(Box<cgroup::Body>),
    TextPoke(Box<text_poke::Body>),
    AuxOutputHwId(Box<aux_output_hw_id::Body>), // TODO: missing docs in manual
}
