use crate::perf_event::PerfEventAttr;
use std::ops::Not;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum EventScope {
    User,
    Kernel,
    Hv,
    Idle,
    Host,
    Guest,
}

impl EventScope {
    pub fn all() -> Vec<Self> {
        vec![
            Self::User,
            Self::Kernel,
            Self::Hv,
            Self::Idle,
            Self::Host,
            Self::Guest,
        ]
    }

    pub fn all_but_exclude<'t>(scopes: impl IntoIterator<Item = &'t Self>) -> Vec<Self> {
        let excludes = scopes.into_iter().collect::<Vec<_>>();
        Self::all()
            .iter()
            .filter(|s| excludes.contains(s).not())
            .cloned()
            .collect()
    }

    pub(crate) fn enable_in_raw_attr(&self, perf_event_attr: &mut PerfEventAttr) {
        #[rustfmt::skip]
        match self {
            Self::User   => perf_event_attr.set_exclude_user(0),
            Self::Kernel => perf_event_attr.set_exclude_kernel(0),
            Self::Hv     => perf_event_attr.set_exclude_hv(0),
            Self::Idle   => perf_event_attr.set_exclude_idle(0),
            Self::Host   => perf_event_attr.set_exclude_host(0),
            Self::Guest  => perf_event_attr.set_exclude_guest(0),
        };
    }
}
