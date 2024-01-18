use crate::perf_event::RawAttr;
use std::ops::Not;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ExtraRecord {
    Mmap,
    Mmap2,
    ContextSwitch,
    Namespaces,
    Ksymbol,
    BpfEvent,
    #[cfg(feature = "linux-5.7")]
    Cgroup,
    #[cfg(feature = "linux-5.8")]
    TextPoke,
    ForkAndExit,
}

impl ExtraRecord {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Mmap,
            Self::Mmap2,
            Self::ContextSwitch,
            Self::Namespaces,
            Self::Ksymbol,
            Self::BpfEvent,
            #[cfg(feature = "linux-5.7")]
            Self::Cgroup,
            #[cfg(feature = "linux-5.8")]
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
        match self {
            Self::Mmap => raw_attr.set_mmap(1),
            Self::Mmap2 => raw_attr.set_mmap2(1),
            Self::ContextSwitch => raw_attr.set_context_switch(1),
            Self::Namespaces => raw_attr.set_namespaces(1),
            Self::Ksymbol => raw_attr.set_ksymbol(1),
            Self::BpfEvent => raw_attr.set_bpf_event(1),
            #[cfg(feature = "linux-5.7")]
            Self::Cgroup => raw_attr.set_cgroup(1),
            #[cfg(feature = "linux-5.8")]
            Self::TextPoke => raw_attr.set_text_poke(1),
            Self::ForkAndExit => raw_attr.set_task(1),
        }
    }
}
