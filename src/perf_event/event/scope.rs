use std::ops::Not;

#[derive(PartialEq, Clone, Debug)]
pub enum EventScope {
    User,
    Kernel,
    Hv,
    Idle,
    Host,
    Guest,
    CallchainKernel,
    CallchainUser,
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
            Self::CallchainKernel,
            Self::CallchainUser,
        ]
    }
    pub fn all_but_exclude(scopes: impl IntoIterator<Item = EventScope>) -> Vec<Self> {
        let exclude_scopes = scopes.into_iter().collect::<Vec<_>>();
        EventScope::all()
            .into_iter()
            .filter(|s| exclude_scopes.contains(s).not())
            .collect()
    }
}
