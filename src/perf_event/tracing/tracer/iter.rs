use crate::sampling::record::Record;
use crate::tracing::tracer::Tracer;

impl Tracer {
    #[inline]
    pub fn iter(&mut self) -> Iter<'_> {
        Iter { inner: self }
    }
}

pub struct Iter<'t> {
    inner: &'t mut Tracer,
}

impl Iterator for Iter<'_> {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next_record()
    }
}
