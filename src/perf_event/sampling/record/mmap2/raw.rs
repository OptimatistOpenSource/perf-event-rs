/*
struct {
  u32    pid;
  u32    tid;
  u64    addr;
  u64    len;
  u64    pgoff;
  union {
      struct {
          u32    maj;
          u32    min;
          u64    ino;
          u64    ino_generation;
      };
      struct {   /* if PERF_RECORD_MISC_MMAP_BUILD_ID */
          u8     build_id_size;
          u8     __reserved_1;
          u16    __reserved_2;
          u8     build_id[20];
      };
  };
  u32    prot;
  u32    flags;
  char   filename[];
  struct sample_id sample_id;
};
*/

use crate::infra::{ConstPtrExt, SliceExt, ZeroTerminated};
use crate::sampling::record::SampleId;

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct anon_struct_1 {
    pub maj: u32,
    pub min: u32,
    pub ino: u64,
    pub ino_generation: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct anon_struct_2 {
    pub build_id_size: u8,
    pub __reserved_1: u8,
    pub __reserved_2: u16,
    pub build_id: [u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub union anon_union {
    pub anon_struct_1: anon_struct_1,
    pub anon_struct_2: anon_struct_2,
}

#[repr(C)]
struct Sized1 {
    pub pid: u32,
    pub tid: u32,
    pub addr: u64,
    pub len: u64,
    pub pgoff: u64,
    pub anon_union: anon_union,
    pub prot: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct Body {
    filename: ZeroTerminated<u8>,
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
    sized1_get!(addr, &u64);
    sized1_get!(len, &u64);
    sized1_get!(pgoff, &u64);
    sized1_get!(anon_union, &anon_union);
    sized1_get!(prot, &u32);
    sized1_get!(flags, &u32);

    pub fn filename(&self) -> &[u8] {
        let sized1_ptr = self.sized1() as *const Sized1;
        let ptr = unsafe { sized1_ptr.add(1) } as *const u8;
        let zt = unsafe { ZeroTerminated::from_ref(ptr.as_ref().unwrap()) };
        zt.as_slice()
    }

    pub fn sample_id(&self) -> &SampleId {
        let ptr = unsafe { self.filename().follow_mem_ptr().align_as_ptr::<SampleId>() };
        unsafe { ptr.as_ref() }.unwrap()
    }
}
