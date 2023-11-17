use crate::syscall::bindings::perf_event_attr;

type RawAttr = perf_event_attr;

pub struct PerfEventCountingAttr {
    inner: RawAttr,
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

impl Default for PerfEventCountingAttr {
    fn default() -> Self {
        let mut attr = RawAttr::default();

        attr.size = std::mem::size_of::<RawAttr>() as libc::__u32;

        attr.set_disabled(1);
        attr.set_inherit(1);
        attr.set_pinned(0);
        attr.set_exclusive(0);

        attr.set_exclude_user(1);
        attr.set_exclude_kernel(1);
        attr.set_exclude_hv(1);
        attr.set_exclude_idle(1);

        attr.set_mmap(0); // not for counting mode
        attr.set_comm(0); // ditto
        attr.set_freq(0); // ditto
        attr.set_inherit_stat(1);
        attr.set_enable_on_exec(0);
        attr.set_task(0); // not for counting mode
        attr.set_watermark(0); // ditto
        attr.set_precise_ip(0);
        attr.set_mmap_data(0); // not for counting mode
        attr.set_sample_id_all(0); // ditto

        attr.set_exclude_host(1);
        attr.set_exclude_guest(1);
        attr.set_exclude_callchain_kernel(1);
        attr.set_exclude_callchain_user(1);

        attr.set_mmap2(0); // not for counting mode
        attr.set_comm_exec(0); // ditto
        attr.set_use_clockid(0); // ditto
        attr.set_context_switch(0); // ditto
        attr.set_write_backward(0); // ditto
        attr.set_namespaces(0); // ditto
        attr.set_ksymbol(0); // ditto
        attr.set_bpf_event(0); // ditto
        attr.set_aux_output(0); // ditto
        attr.set_cgroup(0); // ditto
        attr.set_text_poke(0); // ditto
        attr.set_build_id(0); // ditto
        attr.set_inherit_thread(0);
        attr.set_remove_on_exec(0);
        attr.set_sigtrap(0);

        Self { inner: attr }
    }
}

impl PerfEventCountingAttr {
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
