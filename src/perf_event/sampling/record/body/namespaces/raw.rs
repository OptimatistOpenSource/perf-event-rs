/*
struct {
  u32    pid;
  u32    tid;
  u64    nr_namespaces;
  struct { u64 dev, inode } [nr_namespaces];
  struct sample_id sample_id;
};
*/

use crate::infra::{SliceExt, Vla};
use crate::sampling::record::namespaces::Namespace;
use crate::sampling::record::sample_id::SampleId;

#[repr(C)]
pub struct Sized {
    pub pid: u32,
    pub tid: u32,
}

pub struct Raw {
    pub read_ptr: *const u8,
    pub sample_type: u64,
}

// TODO: use read_ptr
impl Raw {
    pub unsafe fn sized(&mut self) -> &Sized {
        let ptr = self.read_ptr as *const Sized;
        self.read_ptr = ptr.add(1) as _;
        &*ptr
    }

    pub unsafe fn namespaces(&mut self) -> &[Namespace] {
        let len_ptr = self.read_ptr as *const u64;
        let vla: &Vla<u64, Namespace> = Vla::from_ptr(len_ptr);
        let slice = vla.as_slice();
        self.read_ptr = slice.follow_mem_ptr() as _;
        slice
    }

    pub unsafe fn sample_id(&self) -> SampleId {
        SampleId::from_ptr(self.read_ptr, self.sample_type)
    }
}
