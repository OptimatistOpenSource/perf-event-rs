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

use crate::infra::NullTerminated;
use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body {
    pid: u32,
    tid: u32,
    addr: u64,
    len: u64,
    pgoff: u64,
    anon_union: anon_union,
    prot: u32,
    flags: u32,
    filename: NullTerminated<u8>,
    sample_id: sample_id,
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct anon_struct_1 {
    pub maj: u32,
    pub min: u32,
    pub ino: u64,
    pub ino_generation: u64,
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct anon_struct_2 {
    pub build_id_size: u8,
    pub __reserved_1: u8,
    pub __reserved_2: u16,
    pub build_id: [u8; 20],
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub union anon_union {
    pub anon_struct_1: anon_struct_1,
    pub anon_struct_2: anon_struct_2,
}
