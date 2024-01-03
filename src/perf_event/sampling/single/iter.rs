use crate::sampling::record::Record;
use crate::sampling::Sampler;

impl Sampler {
    #[inline]
    pub fn iter(&mut self) -> Iter<'_> {
        Iter { inner: self }
    }
}

pub struct Iter<'t> {
    inner: &'t mut Sampler,
}

impl Iterator for Iter<'_> {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next_record()
    }
}
