use crate::perf_event::counting::event::{Event, Inner};
use crate::syscall::bindings::{
    perf_event_attr, perf_event_attr__bindgen_ty_1, perf_event_attr__bindgen_ty_2,
    perf_event_attr__bindgen_ty_3, perf_event_attr__bindgen_ty_4,
};

type RawAttr = perf_event_attr;

pub struct PerfEventCountingAttr {
    raw_attr: RawAttr,
}

pub enum PerfEventScope {
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
        // TODO: satisfy clippy
        let mut attr = RawAttr::default(); // Fields defaults to 0

        attr.type_ = 0; // TODO
        attr.size = std::mem::size_of::<RawAttr>() as libc::__u32;
        attr.config = 0; // TODO
        attr.__bindgen_anon_1 = perf_event_attr__bindgen_ty_1::default(); // not for counting mode
        attr.sample_type = 0; // ditto
        attr.read_format = 0; // TODO

        attr.set_disabled(1); // TODO
        attr.set_inherit(1); // TODO
        attr.set_pinned(0); // TODO
        attr.set_exclusive(0); // TODO

        attr.set_exclude_user(1);
        attr.set_exclude_kernel(1);
        attr.set_exclude_hv(1);
        attr.set_exclude_idle(1);

        attr.set_mmap(0); // not for counting mode
        attr.set_comm(0); // ditto
        attr.set_freq(0); // ditto
        attr.set_inherit_stat(1); // TODO
        attr.set_enable_on_exec(0); // TODO
        attr.set_task(0); // not for counting mode
        attr.set_watermark(0); // ditto
        attr.set_precise_ip(0); // TODO
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
        attr.set_inherit_thread(0); // TODO
        attr.set_remove_on_exec(0); // TODO
        attr.set_sigtrap(0); // TODO

        attr.__bindgen_anon_2 = perf_event_attr__bindgen_ty_2::default(); // not for counting mode
        attr.bp_type = 0; // ditto
        attr.__bindgen_anon_3 = perf_event_attr__bindgen_ty_3::default(); // ditto
        attr.__bindgen_anon_4 = perf_event_attr__bindgen_ty_4::default(); // ditto
        attr.branch_sample_type = 0; // ditto
        attr.sample_regs_user = 0; // ditto
        attr.sample_stack_user = 0; // ditto
        attr.clockid = 0; // ditto
        attr.sample_regs_intr = 0; // ditto
        attr.aux_watermark = 0; // ditto
        attr.sample_max_stack = 0; // ditto
        attr.aux_sample_size = 0; // ditto
        attr.sig_data = 0; // ditto

        Self { raw_attr: attr }
    }
}

impl PerfEventCountingAttr {
    pub fn new(event: impl Into<Event>, scopes: impl Iterator<Item = PerfEventScope>) -> Self {
        let mut attr = Self::default();

        use PerfEventScope::*;
        let raw_attr = &mut attr.raw_attr;
        scopes.for_each(|scope| match scope {
            User => raw_attr.set_exclude_user(0),
            Kernel => raw_attr.set_exclude_kernel(0),
            Hv => raw_attr.set_exclude_hv(0),
            Idle => raw_attr.set_exclude_idle(0),
            Host => raw_attr.set_exclude_host(0),
            Guest => raw_attr.set_exclude_guest(0),
            CallchainKernel => raw_attr.set_exclude_callchain_kernel(0),
            CallchainUser => raw_attr.set_exclude_callchain_user(0),
        });

        use crate::syscall::bindings::*;
        match event.into().into_inner() {
            Inner::Hw(ev) if ev.is_cache_event() => {
                raw_attr.type_ = perf_type_id_PERF_TYPE_HW_CACHE;
                raw_attr.config = ev.into_u64();
            }
            Inner::Hw(ev) => {
                raw_attr.type_ = perf_type_id_PERF_TYPE_HARDWARE;
                raw_attr.config = ev.into_u64();
            }
            Inner::Sw(ev) => {
                raw_attr.type_ = perf_type_id_PERF_TYPE_SOFTWARE;
                raw_attr.config = ev.into_u64();
            }
            Inner::Raw(ev) => {
                raw_attr.type_ = perf_type_id_PERF_TYPE_RAW;
                raw_attr.config = ev.into_u64();
            }
        }

        todo!()
    }

    /// Construct from a raw `perf_event_attr` struct.
    /// # Safety
    /// The `raw_attr` argument must be a properly initialized
    /// `perf_event_attr` struct for counting mode.
    pub unsafe fn from_raw(raw_attr: RawAttr) -> Self {
        Self { raw_attr }
    }

    pub fn into_raw(self) -> RawAttr {
        self.raw_attr
    }
}
