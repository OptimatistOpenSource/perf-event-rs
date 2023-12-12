#[derive(Debug, Clone)]
pub struct ExtraConfig {
    pub pinned: bool,
    pub exclusive: bool,

    pub inherit: bool,
    pub inherit_stat: bool,
    pub inherit_thread: bool,

    pub enable_on_exec: bool,
    #[cfg(feature = "linux-5.13")]
    pub remove_on_exec: bool,
}

impl Default for ExtraConfig {
    fn default() -> Self {
        Self {
            pinned: false,
            exclusive: false,

            inherit: false,
            inherit_stat: false,
            inherit_thread: false,

            enable_on_exec: false,
            #[cfg(feature = "linux-5.13")]
            remove_on_exec: false,
        }
    }
}
