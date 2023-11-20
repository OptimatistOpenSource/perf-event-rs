use crate::perf_event::counting::event::{Event, Inner};
use crate::syscall::bindings::{
    __BindgenBitfieldUnit, perf_event_attr, perf_event_attr__bindgen_ty_1,
    perf_event_attr__bindgen_ty_2, perf_event_attr__bindgen_ty_3, perf_event_attr__bindgen_ty_4,
};
use crate::EventScope;

type RawAttr = perf_event_attr;

pub struct CountingAttr {
    raw_attr: RawAttr,
}

impl Default for CountingAttr {
    fn default() -> Self {
        let mut attr = RawAttr {
            type_: 0,
            size: std::mem::size_of::<RawAttr>() as libc::__u32,
            config: 0,
            __bindgen_anon_1: perf_event_attr__bindgen_ty_1::default(), // not use in counting mode
            sample_type: 0,                                             // ditto
            read_format: 0,                                             // TODO
            _bitfield_align_1: [],
            _bitfield_1: __BindgenBitfieldUnit::new([0u8; 8usize]), // set latter via attr.set_...
            __bindgen_anon_2: perf_event_attr__bindgen_ty_2::default(), // not use in counting mode
            bp_type: 0,                                             // ditto
            __bindgen_anon_3: perf_event_attr__bindgen_ty_3::default(), // ditto
            __bindgen_anon_4: perf_event_attr__bindgen_ty_4::default(), // ditto
            branch_sample_type: 0,                                  // ditto
            sample_regs_user: 0,                                    // ditto
            sample_stack_user: 0,                                   // ditto
            clockid: 0,                                             // ditto
            sample_regs_intr: 0,                                    // ditto
            aux_watermark: 0,                                       // ditto
            sample_max_stack: 0,                                    // ditto
            __reserved_2: 0,
            aux_sample_size: 0, // not use in counting mode
            __reserved_3: 0,
            sig_data: 0, // not use in counting mode
            config3: 0,  // TODO: miss doc in man
        };

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

        Self { raw_attr: attr }
    }
}

impl CountingAttr {
    // TODO: more options needed
    pub fn new(event: impl Into<Event>, scopes: impl IntoIterator<Item = EventScope>) -> Self {
        let mut attr = Self::default();

        use EventScope::*;
        let raw_attr = &mut attr.raw_attr;
        scopes.into_iter().for_each(|scope| match scope {
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

        attr
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
