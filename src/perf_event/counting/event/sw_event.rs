use crate::syscall::bindings::*;

pub enum SwEvent {
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
    CgroupSwitches,
}

impl SwEvent {
    fn into_u64(self) -> u64 {
        use SwEvent::*;
        let config = match self {
            CpuClock => perf_sw_ids_PERF_COUNT_SW_CPU_CLOCK,
            TaskClock => perf_sw_ids_PERF_COUNT_SW_TASK_CLOCK,
            PageFaults => perf_sw_ids_PERF_COUNT_SW_PAGE_FAULTS,
            ContextSwitches => perf_sw_ids_PERF_COUNT_SW_CONTEXT_SWITCHES,
            CpuMigrations => perf_sw_ids_PERF_COUNT_SW_CPU_MIGRATIONS,
            PageFaultsMin => perf_sw_ids_PERF_COUNT_SW_PAGE_FAULTS_MIN,
            PageFaultsMaj => perf_sw_ids_PERF_COUNT_SW_PAGE_FAULTS_MAJ,
            AlignmentFaults => perf_sw_ids_PERF_COUNT_SW_ALIGNMENT_FAULTS,
            EmulationFaults => perf_sw_ids_PERF_COUNT_SW_EMULATION_FAULTS,
            Dummy => perf_sw_ids_PERF_COUNT_SW_DUMMY,
            BpfOutput => perf_sw_ids_PERF_COUNT_SW_BPF_OUTPUT,
            CgroupSwitches => perf_sw_ids_PERF_COUNT_SW_CGROUP_SWITCHES,
        };
        config as u64
    }
}
