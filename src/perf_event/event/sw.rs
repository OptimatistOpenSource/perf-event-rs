use crate::syscall::bindings::*;
use crate::Event;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SoftwareEvent {
    CpuClock,
    TaskClock,
    PageFaults,
    ContextSwitches,
    CpuMigrations,
    PageFaultsMin,
    PageFaultsMaj,
    AlignmentFaults,
    EmulationFaults,
    Dummy,
    BpfOutput,
    #[cfg(feature = "linux-5.13")]
    CgroupSwitches,
}

impl SoftwareEvent {
    pub(crate) const fn into_u64(self) -> u64 {
        use SoftwareEvent::*;
        let config = match self {
            CpuClock => PERF_COUNT_SW_CPU_CLOCK,
            TaskClock => PERF_COUNT_SW_TASK_CLOCK,
            PageFaults => PERF_COUNT_SW_PAGE_FAULTS,
            ContextSwitches => PERF_COUNT_SW_CONTEXT_SWITCHES,
            CpuMigrations => PERF_COUNT_SW_CPU_MIGRATIONS,
            PageFaultsMin => PERF_COUNT_SW_PAGE_FAULTS_MIN,
            PageFaultsMaj => PERF_COUNT_SW_PAGE_FAULTS_MAJ,
            AlignmentFaults => PERF_COUNT_SW_ALIGNMENT_FAULTS,
            EmulationFaults => PERF_COUNT_SW_EMULATION_FAULTS,
            Dummy => PERF_COUNT_SW_DUMMY,
            BpfOutput => PERF_COUNT_SW_BPF_OUTPUT,
            #[cfg(feature = "linux-5.13")]
            CgroupSwitches => PERF_COUNT_SW_CGROUP_SWITCHES,
        };
        config as _
    }
}

impl From<SoftwareEvent> for Event {
    fn from(value: SoftwareEvent) -> Self {
        Self::Software(value)
    }
}
