use crate::perf_event::counting::attr::CountingAttr;

pub(crate) enum Inner {
    CountingMode(CountingAttr),
}

pub struct PerfEventAttr(Inner);

impl From<CountingAttr> for PerfEventAttr {
    fn from(value: CountingAttr) -> Self {
        Self(Inner::CountingMode(value))
    }
}
