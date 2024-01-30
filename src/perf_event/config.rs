use std::{io, result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("PID is invalid: {0}")]
    InvalidPid(u32),
    #[error("Measures any process on any cpu is invalid")]
    InvalidProcessCpu,
    #[error("Failed to perform perf_event_open: {0}")]
    SyscallFailed(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

pub enum Process {
    Any,     // -1
    Current, // 0
    Pid(u32),
}

impl Process {
    pub(crate) const fn as_i32(&self) -> Result<i32> {
        match self {
            Self::Any => Ok(-1),
            Self::Current => Ok(0),
            Self::Pid(0) => Err(Error::InvalidPid(0)),
            Self::Pid(n) => Ok(*n as _),
        }
    }
}

pub enum Cpu {
    Any, // -1
    Id(u32),
}

impl Cpu {
    pub(crate) const fn as_i32(&self) -> i32 {
        match self {
            Self::Any => -1,
            Self::Id(n) => *n as _,
        }
    }
}
