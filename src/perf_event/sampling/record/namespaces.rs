/*
struct {
  u32    pid;
  u32    tid;
  u64    nr_namespaces;
  struct { u64 dev, inode } [nr_namespaces];
  struct sample_id sample_id;
};
*/

use crate::infra::Vla;
use crate::sampling::record::sample_id;

#[allow(non_camel_case_types)]
pub struct namespace {
    dev: u64,
    inode: u64,
}

#[repr(C)]
pub struct Body {
    pid: u32,
    tid: u32,
    namespaces: Vla<u64, namespace>,
    sample_id: sample_id,
}
