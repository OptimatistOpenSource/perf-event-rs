mod aux;
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

pub enum RecordBody {
    Mmap(mmap::Body),
    Lost(lost::Body),
    Comm(comm::Body),
    Exit(exit::Body),
    Throttle(throttle::Body),
    Unthrottle(throttle::Body), // Unthrottle is same to Throttle
    Fork(fork::Body),
    Read(read::Body),
    Sample(sample::Body),
    Mmap2(mmap2::Body),
    Aux(aux::Body),
    ItraceStart(intrace_start::Body),
    LostSamples(lost_samples::Body),
    Switch(switch::Body),
    SwitchCpuWide(switch_cpu_wide::Body),
    Namespaces(namespaces::Body),
    Ksymbol(ksymbol::Body),
    BpfEvent(bpf_event::Body),
    Cgroup(cgroup::Body),
    TextPoke(text_poke::Body),

    AuxOutputHwId, // TODO: missing docs in man
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
