use crate::sampling::record::sample::WeightRepr;
use crate::syscall::bindings::*;

/// Select the fields contained in `sample::Body`
#[derive(Debug, Clone, Default)]
pub struct SampleRecordFields {
    #[cfg(feature = "linux-3.12")]
    pub sample_id: bool, // PERF_SAMPLE_IDENTIFIER
    pub ip: bool,          // PERF_SAMPLE_IP
    pub pid_and_tid: bool, // PERF_SAMPLE_TID
    pub time: bool,        // PERF_SAMPLE_TIME
    pub addr: bool,        // PERF_SAMPLE_ADDR
    pub id: bool,          // PERF_SAMPLE_ID
    pub stream_id: bool,   // PERF_SAMPLE_STREAM_ID
    pub cpu: bool,         // PERF_SAMPLE_CPU
    pub period: bool,      // PERF_SAMPLE_PERIOD
    pub v: bool,           // PERF_SAMPLE_READ

    /// Wrap `sample_max_stack` with `Some` to enable this field
    pub ips: Option<u16>, // PERF_SAMPLE_CALLCHAIN

    pub data_raw: bool, // PERF_SAMPLE_RAW

    /// Wrap `sample_regs_user` with `Some` to enable this field
    pub abi_and_regs_user: Option<u64>, // PERF_SAMPLE_REGS_USER

    /// Wrap `sample_stack_user` with `Some` to enable this field, \
    /// `sample_stack_user` must be `n * 8` in value
    /*
    Line 12137 of kernel/events/core.c:
    ```c
    /*
     * We have __u32 type for the size, but so far
     * we can only use __u16 as maximum due to the
     * __u16 sample size limit.
     */
    if (attr->sample_stack_user >= USHRT_MAX)
        return -EINVAL;
    else if (!IS_ALIGNED(attr->sample_stack_user, sizeof(u64)))
        return -EINVAL;
    ```
    */
    pub data_stack_user: Option<u16>, // PERF_SAMPLE_STACK_USER

    /// Some(WeightRepr::Full) for Weight::Full(u64)\
    /// Some(WeightRepr::Vars) for Weight::Vars { ... }
    // PERF_SAMPLE_WEIGHT when WeightRepr::Full
    // PERF_SAMPLE_WEIGHT_STRUCT when WeightRepr::Vars
    pub weight: Option<WeightRepr>,

    pub data_src: bool, // PERF_SAMPLE_DATA_SRC
    #[cfg(feature = "linux-3.13")]
    pub transaction: bool, // PERF_SAMPLE_TRANSACTION

    /// Wrap `sample_regs_intr` with `Some` to enable this field
    #[cfg(feature = "linux-3.19")]
    pub abi_and_regs_intr: Option<u64>, // PERF_SAMPLE_REGS_INTR

    // The `PERF_RECORD_KSYMBOL` was first added to the Linux kernel in 4.14
    // the man documentation incorrectly says "since Linux 4.13"
    // See: https://github.com/torvalds/linux/commit/fc7ce9c74c3ad232b084d80148654f926d01ece7
    #[cfg(feature = "linux-4.14")]
    pub phys_addr: bool, // PERF_SAMPLE_PHYS_ADDR
    #[cfg(feature = "linux-5.7")]
    pub cgroup: bool, // PERF_SAMPLE_CGROUP
    #[cfg(feature = "linux-5.11")]
    pub data_page_size: bool, // PERF_SAMPLE_DATA_PAGE_SIZE
    #[cfg(feature = "linux-5.11")]
    pub code_page_size: bool, // PERF_SAMPLE_CODE_PAGE_SIZE
}

impl SampleRecordFields {
    #[allow(clippy::cognitive_complexity)]
    pub(crate) const fn as_sample_type(&self) -> u64 {
        macro_rules! gen {
            ($(
                $(@#[$attr: meta])*
                $cond: expr,
                $mask: expr
            )+) => {
                let mut sample_type = 0_u64;
                $(
                    $(#[$attr])*
                    {
                        if $cond {
                            sample_type |= $mask as u64;
                        }
                    }
                )+
                sample_type
            };
        }

        gen! {
            @#[cfg(feature = "linux-3.12")]
            self.sample_id                  , PERF_SAMPLE_IDENTIFIER
            self.ip                         , PERF_SAMPLE_IP
            self.pid_and_tid                , PERF_SAMPLE_TID
            self.time                       , PERF_SAMPLE_TIME
            self.addr                       , PERF_SAMPLE_ADDR
            self.id                         , PERF_SAMPLE_ID
            self.stream_id                  , PERF_SAMPLE_STREAM_ID
            self.cpu                        , PERF_SAMPLE_CPU
            self.period                     , PERF_SAMPLE_PERIOD
            self.v                          , PERF_SAMPLE_READ
            self.ips.is_some()              , PERF_SAMPLE_CALLCHAIN
            self.data_raw                   , PERF_SAMPLE_RAW
            // TODO: Not all hw supports PERF_SAMPLE_BRANCH_STACK
            self.abi_and_regs_user.is_some(), PERF_SAMPLE_REGS_USER
            self.data_stack_user.is_some()  , PERF_SAMPLE_STACK_USER
            matches!(self.weight, Some(WeightRepr::Full)), PERF_SAMPLE_WEIGHT
            @#[cfg(feature = "linux-5.12")]
            matches!(self.weight, Some(WeightRepr::Vars)), PERF_SAMPLE_WEIGHT_STRUCT
            self.data_src                   , PERF_SAMPLE_DATA_SRC
            @#[cfg(feature = "linux-3.13")]
            self.transaction                , PERF_SAMPLE_TRANSACTION
            @#[cfg(feature = "linux-3.19")]
            self.abi_and_regs_intr.is_some(), PERF_SAMPLE_REGS_INTR
            @#[cfg(feature = "linux-4.14")]
            self.phys_addr                  , PERF_SAMPLE_PHYS_ADDR
            @#[cfg(feature = "linux-5.7")]
            self.cgroup                     , PERF_SAMPLE_CGROUP
            @#[cfg(feature = "linux-5.11")]
            self.data_page_size             , PERF_SAMPLE_DATA_PAGE_SIZE
            @#[cfg(feature = "linux-5.11")]
            self.code_page_size             , PERF_SAMPLE_CODE_PAGE_SIZE
        }
    }
}
