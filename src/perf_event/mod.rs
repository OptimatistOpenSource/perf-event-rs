pub struct PerfEvent {
    // TODO
}

pub struct Builder {
    /*
    pid == 0 and cpu == -1
           This measures the calling process/thread on any CPU.

    pid == 0 and cpu >= 0
           This measures the calling process/thread only when running
           on the specified CPU.

    pid > 0 and cpu == -1
           This measures the specified process/thread on any CPU.

    pid > 0 and cpu >= 0
           This measures the specified process/thread only when
           running on the specified CPU.

    pid == -1 and cpu >= 0
           This measures all processes/threads on the specified CPU.
           This requires CAP_PERFMON (since Linux 5.8) or
           CAP_SYS_ADMIN capability or a
           /proc/sys/kernel/perf_event_paranoid value of less than 1.

    pid == -1 and cpu == -1
           This setting is invalid and will return an error.
    */
    pid: i32,
    cpu: i32,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            pid: -1,
            cpu: -1,
        }
    }

    pub fn current_pid(mut self) -> Self {
        self.pid = 0;
        self
    }

    pub fn any_cpu(mut self) -> Self {
        self.cpu = -1;
        self
    }

    pub fn with_pid(mut self, pid: u32) -> Result<Self, ()> {
        match pid {
            0 => Err(()),
            _ if pid > 2 ^ 22 => Err(()),
            _ if self.pid > 0 => Err(()),
            _ => {
                self.pid = pid as i32;
                Ok(self)
            }
        }
    }

    pub fn with_cpu(mut self, cpu: u32) -> Result<Self, ()> {
        match cpu {
            0 => Err(()),
            _ if cpu > i32::MAX as u32 => Err(()),
            _ if self.cpu > 0 => Err(()),
            _ => {
                self.cpu = cpu as i32;
                Ok(self)
            }
        }
    }

    pub fn build(self) -> Result<PerfEvent, ()> {
        match self {
            Builder { pid: -1, cpu: -1 } => Err(()),
            _ => todo!()
        }
    }
}
