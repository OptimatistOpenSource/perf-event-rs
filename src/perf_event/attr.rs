use crate::perf_event::counting::attr::Attr;

pub(crate) enum Inner {
    CountingMode(Attr),
}

pub struct PerfEventAttr(Inner);

impl From<Attr> for PerfEventAttr {
    fn from(value: Attr) -> Self {
        Self(Inner::CountingMode(value))
    }
}
