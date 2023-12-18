#[derive(Debug, Clone, Default)]
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
