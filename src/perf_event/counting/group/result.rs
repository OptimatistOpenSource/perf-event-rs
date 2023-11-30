use crate::syscall::bindings::{read_format_body, read_format_header};
use std::collections::HashMap;

#[derive(Debug)]
pub struct MemberResult {
    pub event_count: u64,
    /// only meaningful in sampling mode
    #[cfg(feature = "kernel-6.0")]
    pub event_lost: u64,
}

#[derive(Debug)]
pub struct GroupCountingResult {
    pub time_enabled: u64,
    pub time_running: u64,
    pub member_results: HashMap<u64, MemberResult>,
}

impl GroupCountingResult {
    pub(crate) fn from_raw(header: &read_format_header, body: &[read_format_body]) -> Self {
        Self {
            time_enabled: header.time_enabled,
            time_running: header.time_running,
            member_results: body
                .iter()
                .map(|it| {
                    (
                        it.event_id,
                        MemberResult {
                            event_count: it.event_count,
                            // only meaningful in sampling mode
                            #[cfg(feature = "kernel-6.0")]
                            event_lost: it.event_lost,
                        },
                    )
                })
                .collect::<HashMap<_, _>>(),
        }
    }
}
