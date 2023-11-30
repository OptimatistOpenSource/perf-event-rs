/*
struct {
  u64    addr;
  u16    old_len;
  u16    new_len;
  u8     bytes[];
  struct sample_id sample_id;
};
*/

use crate::infra::{ConstPtrExt, SliceExt, ZeroTerminated};
use crate::sampling::record::SampleId;

#[repr(C)]
struct Sized1 {
    pub addr: u64,
    pub old_len: u16,
    pub new_len: u16,
}

#[repr(C)]
pub(crate) struct Body;

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
    sized1_get!(old_len, &u16);
    sized1_get!(new_len, &u16);

    pub fn bytes(&self) -> &[u8] {
        let sized1_ptr = self.sized1() as *const Sized1;
        let ptr = unsafe { sized1_ptr.add(1) } as *const u8;
        let zt = unsafe { ZeroTerminated::from_ref(ptr.as_ref().unwrap()) };
        zt.as_slice()
    }

    pub fn sample_id(&self) -> &SampleId {
        let ptr = unsafe { self.bytes().follow_mem_ptr().align_as_ptr::<SampleId>() };
        unsafe { ptr.as_ref() }.unwrap()
    }
}
