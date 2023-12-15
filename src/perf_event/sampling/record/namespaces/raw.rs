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
struct Sized1 {
    pid: u32,
    tid: u32,
}

#[repr(C)]
pub struct Body {
    namespaces: Vla<u64, Namespace>,
    sample_id: SampleId,
}

macro_rules! sized1_get {
    ($name:ident,$ty:ty) => {
        #[inline]
        pub fn $name(&self) -> $ty {
            &self.sized1().$name
        }
    };
}

impl Body {
    #[inline]
    fn sized1(&self) -> &Sized1 {
        let ptr = self as *const _ as *const Sized1;
        unsafe { ptr.as_ref().unwrap() }
    }
    sized1_get!(pid, &u32);
    sized1_get!(tid, &u32);

    pub fn namespaces(&self) -> &[Namespace] {
        let len_ptr = unsafe { (self.sized1() as *const Sized1).add(1) } as *const u64;
        let vla: &Vla<u64, Namespace> = unsafe { Vla::from_ptr(len_ptr).as_ref().unwrap() };
        vla.as_slice()
    }

    pub unsafe fn sample_id(&self, sample_type: u64) -> SampleId {
        let ptr = self.namespaces().follow_mem_ptr() as _;
        SampleId::from_ptr(ptr, sample_type)
    }
}
