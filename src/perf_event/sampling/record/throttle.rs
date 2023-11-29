/*
struct {
  u64    time;
  u64    id;
  u64    stream_id;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
#[derive(Debug)]
pub struct Body {
    pub time: u64,
    pub id: u64,
    pub stream_id: u64,
    pub sample_id: sample_id,
}
