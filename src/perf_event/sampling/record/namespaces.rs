/*
struct {
  u32    pid;
  u32    tid;
  u64    nr_namespaces;
  struct { u64 dev, inode } [nr_namespaces];
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body {
    pid: u32,
    tid: u32,
    nr_namespaces: u64,
    // TODO
    sample_id: sample_id,
}
