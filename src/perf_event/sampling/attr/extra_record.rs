#[derive(Debug, Clone)]
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
}
