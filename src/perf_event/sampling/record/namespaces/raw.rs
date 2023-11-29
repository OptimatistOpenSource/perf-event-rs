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
use crate::sampling::record::sample_id;

struct Sized1 {
    pid: u32,
    tid: u32,
}

#[repr(C)]
pub struct Body {
    namespaces: Vla<u64, Namespace>,
    sample_id: sample_id,
}

macro_rules! sized1_get {
    ($name:ident,$ty:ty) => {
        pub fn $name(&self) -> $ty {
            &self.sized1().$name
        }
    };
}

impl Body {
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

    pub fn sample_id(&self) -> &sample_id {
        let ptr = unsafe { self.namespaces().follow_mem_ptr().add(1) } as *const sample_id;
        unsafe { ptr.as_ref() }.unwrap()
    }
}
