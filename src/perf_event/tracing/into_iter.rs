use crate::sampling::record::Record;
use crate::tracing::Tracing;

pub struct IntoIter {
    inner: Tracing,
}

impl Iterator for IntoIter {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next_record()
    }
}

impl IntoIterator for Tracing {
    type Item = Record;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { inner: self }
    }
}
