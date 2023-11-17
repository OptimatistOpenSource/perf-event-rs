use crate::syscall::bindings::perf_event_attr;

pub struct PerfEventAttr {
    inner: perf_event_attr,
}

pub enum PerfEventCount {
    User,
    Kernel,
    Hv,
    Idle,
    Host,
    Guest,
    CallchainKernel,
    CallchainUser,
}

impl Default for PerfEventAttr {
    fn default() -> Self {
        let mut attr = perf_event_attr::default();
        attr.size = std::mem::size_of_val(&attr) as libc::__u32;
        attr.set_exclude_user(1);
        attr.set_exclude_kernel(1);
        attr.set_exclude_hv(1);
        attr.set_exclude_idle(1);
        attr.set_exclude_host(1);
        attr.set_exclude_guest(1);
        attr.set_exclude_callchain_kernel(1);
        attr.set_exclude_callchain_user(1);
        Self { inner: attr }
    }
}

impl PerfEventAttr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn include_count(&mut self, count: PerfEventCount) {
        use PerfEventCount::*;

        let inner = &mut self.inner;
        match count {
            User => inner.set_exclude_user(0),
            Kernel => inner.set_exclude_kernel(0),
            Hv => inner.set_exclude_hv(0),
            Idle => inner.set_exclude_idle(0),
            Host => inner.set_exclude_host(0),
            Guest => inner.set_exclude_guest(0),
            CallchainKernel => inner.set_exclude_callchain_kernel(0),
            CallchainUser => inner.set_exclude_callchain_user(0),
        }
    }
    pub fn include_counts(&mut self, includes: impl Iterator<Item = PerfEventCount>) {
        includes.for_each(|include| self.include_count(include));
    }
}
