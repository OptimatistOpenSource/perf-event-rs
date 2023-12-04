use crate::sampling::Wakeup::Events;

#[derive(Debug)]
pub struct ExtraConfig {
    pub pinned: bool,
    pub exclusive: bool,
    pub comm: bool,
    pub comm_exec: bool,
    pub enable_on_exec: bool,
    pub mmap_data: bool,
    #[cfg(feature = "kernel-5.4")]
    pub aux_output: bool,
    #[cfg(feature = "kernel-5.12")]
    pub build_id: bool,
    #[cfg(feature = "kernel-5.13")]
    pub remove_on_exec: bool,

    pub clockid: Option<ClockId>,
    pub precise_ip: SampleIpSkid,
    pub wakeup: Wakeup,
}

impl Default for ExtraConfig {
    fn default() -> Self {
        Self {
            clockid: None,
            pinned: false,
            exclusive: false,
            comm: false,
            enable_on_exec: false,
            precise_ip: SampleIpSkid::Arbitrary,
            mmap_data: false,
            comm_exec: false,
            #[cfg(feature = "kernel-5.4")]
            aux_output: false,
            #[cfg(feature = "kernel-5.12")]
            build_id: false,
            #[cfg(feature = "kernel-5.13")]
            remove_on_exec: false,
            wakeup: Events(0),
        }
    }
}

#[derive(Debug)]
pub enum SampleIpSkid {
    /// SAMPLE_IP can have arbitrary skid
    Arbitrary, // 0
    /// SAMPLE_IP must have constant skid.
    Constant, // 1
    /// SAMPLE_IP requested to have 0 skid.
    TryZero, // 2
    /// SAMPLE_IP must have 0 skid.  See also the
    /// description of PERF_RECORD_MISC_EXACT_IP.
    Zero, // 3
}

#[derive(Debug)]
pub enum ClockId {
    Monotonic,
    MonotonicRaw,
    RealTime,
    BootTime,
    Tai,
}

#[derive(Debug)]
pub enum Wakeup {
    Events(u32),
    Watermark(u32),
}
