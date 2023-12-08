#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
/// read_format body in PERF_FORMAT_GROUP
pub struct read_format_body {
    pub event_count: u64, // u64 value;
    pub event_id: u64,    // u64 id;

    /// only meaningful in sampling mode
    #[cfg(feature = "linux-6.0")]
    pub event_lost: u64, // u64 lost;
}

#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
/// read_format header in PERF_FORMAT_GROUP
pub struct read_format_header {
    pub members_len: u64,  // u64 nr;
    pub time_enabled: u64, // u64 time_enabled;
    pub time_running: u64, // u64 time_running;
                           // follows: struct { .. } values[nr];
}
