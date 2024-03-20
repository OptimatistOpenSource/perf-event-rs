// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of perf-event-rs.
//
// Perf-event-rs is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// Perf-event-rs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.

use crate::perf_event::event::Event;
use std::num::ParseIntError;
use std::ops::Not;
use std::path::PathBuf;
use std::{fs, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Event name is invalid")]
    InvalidEventName,
    #[error("Event is unsupported")]
    UnsupportedEvent,
    #[error("Failed to find tracefs")]
    FailedToFindTracefs,
    #[error("Failed to parse id file: {0}")]
    FailedToParseIdFile(ParseIntError),
    #[error("I/O error: {0}")]
    IoError(io::Error),
}

#[derive(Clone, Debug)]
pub struct TracepointEvent {
    /// The content of `/sys/kernel/debug/tracing/events/*/*/id`
    pub id: u64,
}

impl TracepointEvent {
    pub const fn new(id: u64) -> Self {
        Self { id }
    }

    /// The format of the event name is `lhs:rhs`, for example: `sched:sched_switch`
    ///
    /// For all available events, see: `/sys/kernel/debug/tracing/available_events`
    pub fn from_event_name(event_name: &str) -> Result<Self, Error> {
        let mut split = event_name.split(':');
        let path = match (split.next(), split.next()) {
            (_, None) => return Err(Error::InvalidEventName),
            (None, _) => return Err(Error::InvalidEventName),
            (Some(lhs), Some(rhs)) => {
                let mut path = tracefs_path()?;
                path.push("events");
                path.push(lhs);
                path.push(rhs);
                path.push("id");
                path
            }
        };

        if path.exists().not() {
            return Err(Error::UnsupportedEvent);
        }

        let contents = fs::read_to_string(path).map_err(Error::IoError)?;
        let id =
            str::parse::<u64>(&contents.replace('\n', "")).map_err(Error::FailedToParseIdFile)?;

        Ok(Self { id })
    }

    /// Get all available event names from `/sys/kernel/debug/tracing/available_events`
    pub fn available_event_names() -> Result<Vec<String>, Error> {
        let mut path = tracefs_path()?;
        path.push("available_events");

        let contents = fs::read_to_string(path).map_err(Error::IoError)?;
        let lines: Vec<String> = contents.lines().map(|it| it.to_string()).collect();

        Ok(lines)
    }
}

fn tracefs_path() -> Result<PathBuf, Error> {
    let contents = fs::read_to_string("/proc/mounts").map_err(Error::IoError)?;

    contents
        .lines()
        .find(|line| line.starts_with("tracefs"))
        .and_then(|line| line.split(' ').nth(1))
        .ok_or_else(|| Error::FailedToFindTracefs)
        .map(PathBuf::from)
}

impl From<TracepointEvent> for Event {
    fn from(value: TracepointEvent) -> Self {
        Self::Tracepoint(value)
    }
}

#[test]
fn test_from_event_name() {
    let ev_name = "kmem:kfree";
    let ev = TracepointEvent::from_event_name(ev_name);
    dbg!(ev.unwrap());
}

#[test]
fn test_available_event_names() {
    let ev_names = TracepointEvent::available_event_names();
    dbg!(&ev_names);
    let ev_names = ev_names.unwrap();
    assert!(ev_names.len() > 0);
}
