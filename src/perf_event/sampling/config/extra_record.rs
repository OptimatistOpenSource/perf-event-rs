use crate::perf_event::RawAttr;
use std::ops::Not;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ExtraRecord {
    Mmap,

    // `PERF_RECORD_MMAP2` was first added to the Linux kernel in 3.12
    // the man documentation incorrectly says "since Linux 3.16"
    // See: https://github.com/torvalds/linux/commit/13d7a2410fa637f450a29ecb515ac318ee40c741
    #[cfg(feature = "linux-3.12")]
    Mmap2,

    #[cfg(feature = "linux-4.3")]
    ContextSwitch,

    // `PERF_RECORD_NAMESPACES` was first added to the Linux kernel in 4.12
    // the man documentation incorrectly says "since Linux 4.11"
    // See: https://github.com/torvalds/linux/commit/e422267322cd319e2695a535e47c5b1feeac45eb
    #[cfg(feature = "linux-4.12")]
    Namespaces,

    // `PERF_RECORD_KSYMBOL` was first added to the Linux kernel in 5.1
    // the man documentation incorrectly says "since Linux 5.0"
    // See: https://github.com/torvalds/linux/commit/76193a94522f1d4edf2447a536f3f796ce56343b
    #[cfg(feature = "linux-5.1")]
    Ksymbol,

    // `PERF_RECORD_BPF_EVENT` was first added to the Linux kernel in 5.1
    // the man documentation incorrectly says "since Linux 5.0"
    // See: https://github.com/torvalds/linux/commit/6ee52e2a3fe4ea35520720736e6791df1fb67106
    #[cfg(feature = "linux-5.1")]
    BpfEvent,

    #[cfg(feature = "linux-5.7")]
    Cgroup,

    // `PERF_RECORD_TEXT_POKE` was first added to the Linux kernel in 5.9
    // the man documentation incorrectly says "since Linux 5.8"
    // See: https://github.com/torvalds/linux/commit/e17d43b93e544f5016c0251d2074c15568d5d963
    #[cfg(feature = "linux-5.9")]
    TextPoke,

    ForkAndExit,
}

impl ExtraRecord {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Mmap,
            #[cfg(feature = "linux-3.12")]
            Self::Mmap2,
            #[cfg(feature = "linux-4.3")]
            Self::ContextSwitch,
            #[cfg(feature = "linux-4.12")]
            Self::Namespaces,
            #[cfg(feature = "linux-5.1")]
            Self::Ksymbol,
            #[cfg(feature = "linux-5.1")]
            Self::BpfEvent,
            #[cfg(feature = "linux-5.7")]
            Self::Cgroup,
            #[cfg(feature = "linux-5.9")]
            Self::TextPoke,
            Self::ForkAndExit,
        ]
    }

    pub fn all_but_exclude<'t>(records: impl IntoIterator<Item = &'t Self>) -> Vec<Self> {
        let excludes = records.into_iter().collect::<Vec<_>>();
        Self::all()
            .iter()
            .filter(|s| excludes.contains(s).not())
            .cloned()
            .collect()
    }

    pub(crate) fn enable_in_raw_attr(&self, raw_attr: &mut RawAttr) {
        #[rustfmt::skip]
        let val = match self {
            Self::Mmap          => raw_attr.set_mmap(1),
            #[cfg(feature = "linux-3.12")]
            Self::Mmap2         => raw_attr.set_mmap2(1),
            #[cfg(feature = "linux-4.3")]
            Self::ContextSwitch => raw_attr.set_context_switch(1),
            #[cfg(feature = "linux-4.12")]
            Self::Namespaces    => raw_attr.set_namespaces(1),
            #[cfg(feature = "linux-5.1")]
            Self::Ksymbol       => raw_attr.set_ksymbol(1),
            #[cfg(feature = "linux-5.1")]
            Self::BpfEvent      => raw_attr.set_bpf_event(1),
            #[cfg(feature = "linux-5.7")]
            Self::Cgroup        => raw_attr.set_cgroup(1),
            #[cfg(feature = "linux-5.9")]
            Self::TextPoke      => raw_attr.set_text_poke(1),
            Self::ForkAndExit   => raw_attr.set_task(1),
        };
        val
    }
}
