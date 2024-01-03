use crate::counting::group::guard::CounterGuard;
use crate::infra::WrapResult;
use crate::syscall::bindings::{read_format_body, read_format_header};
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub struct MemberResult {
    pub event_count: u64,
    /// only meaningful in sampling mode
    #[cfg(feature = "linux-6.0")]
    pub event_lost: u64,
}

#[derive(Debug, Clone)]
pub struct CounterGroupResult {
    pub time_enabled: u64,
    pub time_running: u64,
    member_results: HashMap<u64, MemberResult>,
}

impl CounterGroupResult {
    pub fn member_count(&self, guard: &CounterGuard) -> io::Result<u64> {
        self.member_results
            .get(&guard.event_id())
            .unwrap()
            .event_count
            .wrap_ok()
    }

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
                            #[cfg(feature = "linux-6.0")]
                            event_lost: it.event_lost,
                        },
                    )
                })
                .collect(),
        }
    }
}
