pub struct SamplingGuard {
    pub event_id: u64,
}

impl SamplingGuard {
    pub(crate) const fn new(event_id: u64) -> Self {
        Self { event_id }
    }
}
