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
use crate::sampling::record::sample_id;

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
    // TODO: use inline fn
    sized1_get!(pid, &u32);
    sized1_get!(tid, &u32);
    sized1_get!(addr, &u64);
    sized1_get!(len, &u64);
    sized1_get!(pgoff, &u64);
    sized1_get!(anon_union, &anon_union);
    sized1_get!(prot, &u32);
    sized1_get!(flags, &u32);

    // TODO: return &[u8]
    pub fn filename(&self) -> &ZeroTerminated<u8> {
        let sized1_ptr = self.sized1() as *const Sized1;
        let ptr = unsafe { sized1_ptr.add(1) } as *const u8;
        unsafe { ZeroTerminated::from_ref(ptr.as_ref().unwrap()) }
    }

    // TODO: check alignment
    pub fn sample_id(&self) -> &sample_id {
        let ptr = unsafe {
            self.filename()
                .as_slice()
                .follow_mem_ptr()
                .align_as_ptr::<sample_id>()
        };
        unsafe { ptr.as_ref() }.unwrap()
    }
}
