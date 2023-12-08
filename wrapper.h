// This file is consumed by bindgen, called from our build.rs file.

#include <linux/bpf.h>
#include <linux/perf_event.h>
#include <linux/hw_breakpoint.h>

// for __NR_perf_event_open
#include <asm/unistd.h>

// bindgen won't capture preprocessor macro definitions, so we have to do this.
enum perf_event_ioctls {
    PERF_EVENT_IOCTL_ENABLE = PERF_EVENT_IOC_ENABLE,
    PERF_EVENT_IOCTL_DISABLE = PERF_EVENT_IOC_DISABLE,
    PERF_EVENT_IOCTL_REFRESH = PERF_EVENT_IOC_REFRESH,
    PERF_EVENT_IOCTL_RESET = PERF_EVENT_IOC_RESET,
    PERF_EVENT_IOCTL_PERIOD = PERF_EVENT_IOC_PERIOD,
    PERF_EVENT_IOCTL_SET_OUTPUT = PERF_EVENT_IOC_SET_OUTPUT,
    PERF_EVENT_IOCTL_SET_FILTER = PERF_EVENT_IOC_SET_FILTER,
    PERF_EVENT_IOCTL_ID = PERF_EVENT_IOC_ID,
    PERF_EVENT_IOCTL_SET_BPF = PERF_EVENT_IOC_SET_BPF,
    PERF_EVENT_IOCTL_PAUSE_OUTPUT = PERF_EVENT_IOC_PAUSE_OUTPUT,
    PERF_EVENT_IOCTL_QUERY_BPF = PERF_EVENT_IOC_QUERY_BPF,
    PERF_EVENT_IOCTL_MODIFY_ATTRIBUTES = PERF_EVENT_IOC_MODIFY_ATTRIBUTES,
};
