use std::ops::Not;

#[derive(PartialEq, Eq, Clone, Debug)]
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

    pub fn all_but_exclude(scopes: impl IntoIterator<Item = Self>) -> Vec<Self> {
        let excludes = scopes.into_iter().collect::<Vec<_>>();
        Self::all()
            .into_iter()
            .filter(|s| excludes.contains(s).not())
            .collect()
    }
}
