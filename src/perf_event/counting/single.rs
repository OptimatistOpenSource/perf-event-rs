use crate::counting::{read_format_body, read_format_header, Attr};
use crate::infra::WrapResult;
use crate::syscall;
use crate::syscall::bindings::perf_event_attr;
use crate::syscall::{ioctl_wrapped, perf_event_open};
use std::fs::File;
use std::io;
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd};

pub struct Counting {
    // TODO
    #[allow(dead_code)]
    pub(crate) raw_attr: Box<perf_event_attr>,
    pub(crate) file: File,
}

#[derive(Debug)]
pub struct CountingResult {
    pub event_count: u64,
    pub time_enabled: u64,
    pub time_running: u64,
    pub event_id: u64,
    // TODO: the following is for sampling mode
    //#[cfg(feature = "kernel-6.0")]
    //pub event_lost: u64,
}

impl Counting {
    pub(crate) unsafe fn new(
        attr: Attr,
        pid: i32,
        cpu: i32,
        group_fd: i32,
        flags: u64,
    ) -> io::Result<Self> {
        let raw_attr = Box::new(attr.into_raw());
        let i32 = unsafe { perf_event_open(&*raw_attr as *const _, pid, cpu, group_fd, flags) };
        match i32 {
            -1 => Err(io::Error::last_os_error()),
            fd => Self {
                raw_attr,
                file: File::from_raw_fd(fd),
            }
            .wrap_ok(),
        }
    }

    pub fn result(&mut self) -> io::Result<CountingResult> {
        #[repr(C)]
        #[allow(non_camel_case_types)]
        struct read_format {
            header: read_format_header,
            body: read_format_body, // This group has only one member
        }

        let mut buf = [0_u8; std::mem::size_of::<read_format>()];
        self.file.read_exact(&mut buf)?;

        let read_format = {
            let ptr = buf.as_ptr() as *const read_format;
            unsafe { &*ptr }
        };
        CountingResult {
            event_count: read_format.body.event_count,
            time_enabled: read_format.header.time_enabled,
            time_running: read_format.header.time_running,
            event_id: read_format.body.event_id,
            // TODO: the following is for sampling mode
            //#[cfg(feature = "kernel-6.0")]
            //event_lost: read_format.value.event_lost,
        }
        .wrap_ok()
    }

    pub fn enable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(
            &self.file,
            syscall::bindings::perf_event_ioctls_ENABLE,
            None,
        )
    }

    pub fn disable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(
            &self.file,
            syscall::bindings::perf_event_ioctls_DISABLE,
            None,
        )
    }

    /*
        // TODO
        pub fn refresh(&self) -> io::Result<()> {
            //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_REFRESH)
            todo!()
        }
    */

    pub fn reset_count(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, syscall::bindings::perf_event_ioctls_RESET, None)
    }

    pub fn update_period(&self, new: u64) -> io::Result<()> {
        ioctl_wrapped(
            &self.file,
            syscall::bindings::perf_event_ioctls_PERIOD,
            Some(&new),
        )
    }

    pub fn set_output(&self, new: &File) -> io::Result<()> {
        let raw_fd = new.as_raw_fd() as i64;
        ioctl_wrapped(
            &self.file,
            syscall::bindings::perf_event_ioctls_SET_OUTPUT,
            Some(raw_fd),
        )
    }

    pub fn ignore_output(&self) -> io::Result<()> {
        ioctl_wrapped(
            &self.file,
            syscall::bindings::perf_event_ioctls_SET_OUTPUT,
            Some(-1i64),
        )
    }

    pub fn event_id(&self) -> io::Result<u64> {
        let mut id = 0_u64;
        ioctl_wrapped(
            &self.file,
            syscall::bindings::perf_event_ioctls_ID,
            Some(&mut id),
        )?;
        Ok(id)
    }
}

/*
    // TODO: tracing mode only
    pub fn set_filter(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_SET_FILTER)
        todo!()
    }

    // TODO: tracing mode only
    pub fn set_bpf(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_SET_BPF)
        todo!()
    }

    // TODO: sampling mode only
    pub fn pause_output(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_PAUSE_OUTPUT, 1i32)
    }

    // TODO: sampling mode only
    pub fn resume_output(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_PAUSE_OUTPUT, 0i32)
    }

    // TODO: tracing mode only
    pub fn query_bpf(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_QUERY_BPF)
        todo!()
    }

    // TODO: breakpoint event only
    pub fn modify_attributes(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_MODIFY_ATTRIBUTES)
        todo!()
    }
*/
