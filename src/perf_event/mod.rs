// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of perf-event-rs.
//
// Perf-event-rs is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// Perf-event-rs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.

pub mod config;
pub mod counting;
pub mod event;
pub mod sampling;
pub mod tracing;

use crate::syscall::bindings::perf_event_attr;
pub use event::*;

pub type RawPerfEventAttr = perf_event_attr;

#[derive(Debug, Clone)]
pub struct PerfEventAttr(pub RawPerfEventAttr);

impl std::ops::Deref for PerfEventAttr {
    type Target = RawPerfEventAttr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for PerfEventAttr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PerfEventAttr {
    /// The `disabled` bit specifies whether the counter starts out `disabled` or `enabled`.
    /// If `disabled`, the event can later be enabled by ioctl(2), prctl(2), or enable_on_exec.
    ///
    /// When  creating  an event group, typically the group leader is initialized with `disabled` set to 1 and any child events
    /// are initialized with `disabled` set to 0.  Despite `disabled` being 0, the child events will not start until the group leader is enabled.
    #[inline]
    pub fn set_disabled(&mut self, val: u64) {
        self.0.set_disabled(val)
    }

    /// The `inherit` bit specifies that this counter should count events of child tasks as well as the task specified.
    /// This applies only to new children, not to any existing children at  the  time the counter is created (nor to any new children of existing children).
    #[inline]
    pub fn set_inherit(&mut self, val: u64) {
        self.0.set_inherit(val)
    }

    /// The `pinned` bit specifies that the counter should always be on the CPU if at all possible.
    /// It applies only to hardware counters and only to group leaders.  If a `pinned` counter cannot be
    /// put onto the CPU (e.g., because there are not enough hardware counters or because of a conflict with some other event),
    /// then the counter goes into an 'error' state, where reads return end-of-file (i.e., read(2) returns 0)
    /// until the counter is subsequently enabled or disabled.
    #[inline]
    pub fn set_pinned(&mut self, val: u64) {
        self.0.set_pinned(val)
    }

    /// The `exclusive` bit specifies that when this counter's group is on the CPU, it should be the only group using the CPU's counters.
    /// In the future this may allow monitoring programs to support PMU features that need to run alone so that they do not disrupt other hardware counters.
    ///
    /// Note that many unexpected situations may prevent events with the `exclusive` bit set from ever running.
    /// This includes any users running a system-wide measurement as well as any  kernel use
    /// of the performance counters (including the commonly enabled NMI Watchdog Timer interface).
    #[inline]
    pub fn set_exclusive(&mut self, val: u64) {
        self.0.set_exclusive(val)
    }

    /// If this bit is set, the count excludes events that happen in user space.
    #[inline]
    pub fn set_exclude_user(&mut self, val: u64) {
        self.0.set_exclude_user(val)
    }

    /// If this bit is set, the count excludes events that happen in kernel space.
    #[inline]
    pub fn set_exclude_kernel(&mut self, val: u64) {
        self.0.set_exclude_kernel(val)
    }

    /// If this bit is set, the count excludes events that happen in the hypervisor.
    /// This is mainly for PMUs that have built-in support for handling this (such as POWER).
    /// Extra support is needed for handling hypervisor measurements on most machines.
    #[inline]
    pub fn set_exclude_hv(&mut self, val: u64) {
        self.0.set_exclude_hv(val)
    }

    /// If set, don't count when the CPU is running the idle task.
    /// While you can currently enable this for any event type, it is ignored for all but software events.
    #[inline]
    pub fn set_exclude_idle(&mut self, val: u64) {
        self.0.set_exclude_idle(val)
    }

    /// The `mmap` bit enables generation of PERF_RECORD_MMAP samples for every mmap(2) call that has PROT_EXEC set.
    /// This allows tools to notice new executable code being mapped into a program (dy‐namic shared libraries for example)
    /// so that addresses can be mapped back to the original code.
    #[inline]
    pub fn set_mmap(&mut self, val: u64) {
        self.0.set_mmap(val)
    }

    /// The `comm` bit enables tracking of process command name as modified by the exec(2) and prctl(PR_SET_NAME) system calls as well as writing to /proc/self/comm.
    /// If the comm_exec flag is also successfully set (possible since Linux 3.16),
    /// then the misc flag PERF_RECORD_MISC_COMM_EXEC can be used to differentiate the exec(2) case from the others.
    #[inline]
    pub fn set_comm(&mut self, val: u64) {
        self.0.set_comm(val)
    }

    /// If this bit is set, then `sample_frequency` not `sample_period` is used when setting up the sampling interval.
    #[inline]
    pub fn set_freq(&mut self, val: u64) {
        self.0.set_freq(val)
    }

    /// This bit enables saving of event counts on context switch for inherited tasks. This is meaningful only if the `inherit` field is set.
    #[inline]
    pub fn set_inherit_stat(&mut self, val: u64) {
        self.0.set_inherit_stat(val)
    }

    /// If this bit is set, a counter is automatically enabled after a call to exec(2).
    #[inline]
    pub fn set_enable_on_exec(&mut self, val: u64) {
        self.0.set_enable_on_exec(val)
    }

    /// If this bit is set, then fork/exit notifications are included in the ring buffer.
    #[inline]
    pub fn set_task(&mut self, val: u64) {
        self.0.set_task(val)
    }

    /// If set, have an overflow notification happen when we cross the `wakeup_watermark` boundary.
    /// Otherwise, overflow notifications happen after `wakeup_events` samples.
    #[inline]
    pub fn set_watermark(&mut self, val: u64) {
        self.0.set_watermark(val)
    }

    /// (since Linux 2.6.35)
    /// This controls the amount of skid.  Skid is how many instructions execute between an event of interest happening
    /// and the kernel being able to stop and record the event. Smaller skid is better and allows more accurate reporting of
    /// which events correspond to which instructions, but hardware is often limited with how small this can be.
    ///
    /// The possible values of this field are the following:
    ///     0  SAMPLE_IP can have arbitrary skid.
    ///     1  SAMPLE_IP must have constant skid.
    ///     2  SAMPLE_IP requested to have 0 skid.
    ///     3  SAMPLE_IP must have 0 skid.  See also the description of PERF_RECORD_MISC_EXACT_IP.
    #[inline]
    pub fn set_precise_ip(&mut self, val: u64) {
        self.0.set_precise_ip(val)
    }

    /// (since Linux 2.6.36)
    /// This is the counterpart of the `mmap` field.  This enables generation of PERF_RECORD_MMAP samples
    /// for mmap(2) calls that do not have PROT_EXEC set (for example data and SysV shared memory).
    #[inline]
    pub fn set_mmap_data(&mut self, val: u64) {
        self.0.set_mmap_data(val)
    }

    /// (since Linux 2.6.38)
    /// If set, then TID, TIME, ID, STREAM_ID, and CPU can additionally be included in non-PERF_RECORD_SAMPLEs if the corresponding sample_type is selected.
    /// If PERF_SAMPLE_IDENTIFIER is specified, then an additional ID value is included as the last value to ease parsing the record stream.
    /// This may lead to the id value appearing twice.
    ///
    /// The layout is described by this pseudo-structure:
    /// ```C
    ///     struct sample_id {
    ///         { u32 pid, tid; }   /* if PERF_SAMPLE_TID set */
    ///         { u64 time;     }   /* if PERF_SAMPLE_TIME set */
    ///         { u64 id;       }   /* if PERF_SAMPLE_ID set */
    ///         { u64 stream_id;}   /* if PERF_SAMPLE_STREAM_ID set  */
    ///         { u32 cpu, res; }   /* if PERF_SAMPLE_CPU set */
    ///         { u64 id;       }   /* if PERF_SAMPLE_IDENTIFIER set */
    ///     };
    /// ```
    #[inline]
    pub fn set_sample_id_all(&mut self, val: u64) {
        self.0.set_sample_id_all(val)
    }

    /// (since Linux 3.2)
    /// When conducting measurements that include processes running VM instances (i.e.,
    /// have executed a KVM_RUN ioctl(2)), only measure events happening inside a guest instance.
    /// This is only meaningful outside the guests; this setting does not change counts gathered inside of a guest.
    /// Currently, this functionality is x86 only.
    #[inline]
    pub fn set_exclude_host(&mut self, val: u64) {
        self.0.set_exclude_host(val)
    }

    /// (since Linux 3.2)
    /// When conducting measurements that include processes running VM instances (i.e., have executed a KVM_RUN ioctl(2)),
    /// do not measure events happening inside guest instances. This is only meaningful outside the guests; this setting does not
    /// change counts gathered inside of a guest.Currently, this functionality is x86 only.
    #[inline]
    pub fn set_exclude_guest(&mut self, val: u64) {
        self.0.set_exclude_guest(val)
    }

    /// (since Linux 3.7)
    /// Do not include kernel callchains.
    #[inline]
    pub fn set_exclude_callchain_kernel(&mut self, val: u64) {
        self.0.set_exclude_callchain_kernel(val)
    }

    /// (since Linux 3.7)
    /// Do not include user callchains.
    #[inline]
    pub fn set_exclude_callchain_user(&mut self, val: u64) {
        self.0.set_exclude_callchain_user(val)
    }

    /// (since Linux 3.16[Ref: man perf_event_open], feature selection from `linux-3.12`)
    /// Generate an extended executable mmap record that contains enough additional information to uniquely identify shared mappings.
    /// The `mmap` flag must also be set for this to work.
    /// FIXME (Chengdong Li)
    #[cfg(feature = "linux-3.12")]
    #[inline]
    pub fn set_mmap2(&mut self, val: u64) {
        self.0.set_mmap2(val)
    }

    /// (since Linux 3.16)
    /// This  is purely a feature-detection flag, it does not change kernel behavior.
    /// If this flag can successfully be set, then, when `comm` is enabled, the PERF_RECORD_MISC_COMM_EXEC flag will be
    /// set in the misc field of a `comm` record header if the rename event being reported was caused by a call to exec(2).
    /// This allows tools to distinguish between the various types of process renaming.
    #[cfg(feature = "linux-3.16")]
    #[inline]
    pub fn set_comm_exec(&mut self, val: u64) {
        self.0.set_comm_exec(val)
    }

    /// (since Linux 4.1)
    /// This allows selecting which internal Linux clock to use when generating timestamps via the `clockid` field.
    /// This can make it easier to correlate perf sample times with timestamps generated by other tools.
    #[cfg(feature = "linux-4.1")]
    #[inline]
    pub fn set_use_clockid(&mut self, val: u64) {
        self.0.set_use_clockid(val)
    }

    /// (since Linux 4.3)
    /// This enables the generation of PERF_RECORD_SWITCH records when a context switch occurs.
    /// It also enables the generation of PERF_RECORD_SWITCH_CPU_WIDE records when sampling in CPU-wide mode.
    /// This functionality is in addition to existing tracepoint and software events for measuring context switches.
    /// The advantage of this method is that it will give full information even with strict perf_event_paranoid settings.
    #[cfg(feature = "linux-4.3")]
    #[inline]
    pub fn set_context_switch(&mut self, val: u64) {
        self.0.set_context_switch(val)
    }

    /// (since Linux 4.6[Ref: man perf_event_open], feature selection from `linux-4.7`)
    /// This causes the ring buffer to be written from the end to the beginning.
    /// This is to support reading from overwritable ring buffer.
    ///
    // The `write_backward` was first added to the Linux kernel in 4.7
    // the man documentation incorrectly says "since Linux 4.6"
    // See: https://github.com/torvalds/linux/commit/9ecda41acb971ebd07c8fb35faf24005c0baea12
    #[cfg(feature = "linux-4.7")]
    #[inline]
    pub fn set_write_backward(&mut self, val: u64) {
        self.0.set_write_backward(val)
    }

    /// (since Linux 4.6[Ref: man perf_event_open], feature selection from `linux-4.12`)
    /// This enables the generation of PERF_RECORD_NAMESPACES records when a task enters a new namespace.
    /// Each namespace has a combination of device and inode numbers.
    #[cfg(feature = "linux-4.12")]
    #[inline]
    pub fn set_namespaces(&mut self, val: u64) {
        self.0.set_namespaces(val)
    }

    /// (since Linux 5.0[Ref: man perf_event_open], feature selection from `linux-5.1`)
    /// This enables the generation of PERF_RECORD_KSYMBOL records when new kernel symbols are registered or unregistered.
    /// This is analyzing dynamic kernel functions like eBPF.
    #[cfg(feature = "linux-5.1")]
    #[inline]
    pub fn set_ksymbol(&mut self, val: u64) {
        self.0.set_ksymbol(val)
    }

    /// (since Linux 5.0[Ref: man perf_event_open], feature selection from `linux-5.1`)
    /// This enables the generation of PERF_RECORD_BPF_EVENT records when an eBPF program is loaded or unloaded.
    #[cfg(feature = "linux-5.1")]
    #[inline]
    pub fn set_bpf_event(&mut self, val: u64) {
        self.0.set_bpf_event(val)
    }

    /// (since Linux 5.4)
    /// This allows normal (non-AUX) events to generate data for AUX events if the hardware supports it.
    #[cfg(feature = "linux-5.4")]
    #[inline]
    pub fn set_aux_output(&mut self, val: u64) {
        self.0.set_aux_output(val)
    }

    /// (since Linux 5.7)
    /// This enables the generation of PERF_RECORD_CGROUP records when a new cgroup is created (and activated).
    #[cfg(feature = "linux-5.7")]
    #[inline]
    pub fn set_cgroup(&mut self, val: u64) {
        self.0.set_cgroup(val)
    }

    /// (since Linux 5.8[Ref: man perf_event_open], feature selection from `linux-5.9`)
    /// This enables the generation of PERF_RECORD_TEXT_POKE records when there's a changes to the kernel text (i.e., self-modifying code).
    #[cfg(feature = "linux-5.9")]
    #[inline]
    pub fn set_text_poke(&mut self, val: u64) {
        self.0.set_text_poke(val)
    }

    /// (since Linux 5.12)
    /// This changes the contents in the PERF_RECORD_MMAP2 to have a build-id instead of device and inode numbers.
    #[cfg(feature = "linux-5.12")]
    #[inline]
    pub fn set_build_id(&mut self, val: u64) {
        self.0.set_build_id(val)
    }

    /// (since Linux 5.13)
    /// This disables the inheritance of the event to a child process.
    /// Only new threads in the same process (which is cloned with CLONE_THREAD) will inherit the event.
    #[cfg(feature = "linux-5.13")]
    #[inline]
    pub fn set_inherit_thread(&mut self, val: u64) {
        self.0.set_inherit_thread(val)
    }

    /// (since Linux 5.13)
    /// This closes the event when it starts a new process image by execve(2).
    #[cfg(feature = "linux-5.13")]
    #[inline]
    pub fn set_remove_on_exec(&mut self, val: u64) {
        self.0.set_remove_on_exec(val)
    }

    /// (since Linux 5.13)
    /// This enables synchronous signal delivery of SIGTRAP on event overflow.
    #[cfg(feature = "linux-5.13")]
    #[inline]
    pub fn set_sigtrap(&mut self, val: u64) {
        self.0.set_sigtrap(val)
    }
}
