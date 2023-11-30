#[derive(Debug)]
pub struct ExtraConfig {
    pub inherit: bool,
    pub pinned: bool,
    pub exclusive: bool,

    pub inherit_stat: bool,
    pub enable_on_exec: bool,

    pub inherit_thread: bool,
    pub remove_on_exec: bool,
}

impl Default for ExtraConfig {
    fn default() -> Self {
        Self {
            inherit: true,
            pinned: false,
            exclusive: false,

            inherit_stat: true,
            enable_on_exec: false,

            inherit_thread: false,
            remove_on_exec: false,
        }
    }
}
