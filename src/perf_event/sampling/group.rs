use crate::sampling::Sampling;
use libc::pid_t;

pub struct SamplingGroup {
    pid: pid_t,
    cpu: i32,
    members: Vec<Sampling>, // members[0] is the group leader, if it exists.
}

impl SamplingGroup {
    pub(crate) const unsafe fn new(pid: pid_t, cpu: i32) -> Self {
        Self {
            pid,
            cpu,
            members: vec![],
        }
    }
}
