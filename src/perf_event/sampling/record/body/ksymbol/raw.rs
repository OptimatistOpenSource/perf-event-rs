/*
struct {
  u64    addr;
  u32    len;
  u16    ksym_type;
  u16    flags;
  char   name[];
  struct sample_id sample_id;
};
*/

use crate::infra::{ConstPtrExt, SliceExt, ZeroTerminated};
use crate::sampling::record::sample_id::SampleId;

#[repr(C)]
struct Sized1 {
    addr: u64,
    len: u32,
    ksym_type: u16,
    flags: u16,
}

#[repr(C)]
pub struct Body;

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
    sized1_get!(addr, &u64);
    sized1_get!(len, &u32);
    sized1_get!(ksym_type, &u16);
    sized1_get!(flags, &u16);

    pub fn name(&self) -> &[u8] {
        let sized1_ptr = self.sized1() as *const Sized1;
        let ptr = unsafe { sized1_ptr.add(1) } as *const u8;
        let zt = unsafe { ZeroTerminated::from_ref(ptr.as_ref().unwrap()) };
        zt.as_slice()
    }

    pub unsafe fn sample_id(&self, sample_type: u64) -> SampleId {
        let ptr = unsafe { self.name().follow_mem_ptr().align_as_ptr::<u64>() } as _;
        SampleId::from_ptr(ptr, sample_type)
    }
}