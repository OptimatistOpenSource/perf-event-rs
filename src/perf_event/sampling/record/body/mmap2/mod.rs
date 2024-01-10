use crate::sampling::record::sample_id::SampleId;
#[cfg(feature = "linux-5.12")]
use crate::syscall::bindings::PERF_RECORD_MISC_MMAP_BUILD_ID;
use std::ffi::CString;

mod raw;

#[derive(Debug, Clone)]
pub enum AnonEnum {
    Normal {
        maj: u32,
        min: u32,
        ino: u64,
        ino_generation: u64,
    },
    BuildId(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Body {
    pub pid: u32,
    pub tid: u32,
    pub addr: u64,
    pub len: u64,
    pub pgoff: u64,
    pub anon_enum: AnonEnum,
    pub prot: u32,
    pub flags: u32,
    pub filename: CString,
    pub sample_id: Option<SampleId>,
}

impl Body {
    pub(crate) unsafe fn from_ptr(
        ptr: *const u8,
        misc: u16,
        sample_type: u64,
        sample_id_all: bool,
    ) -> Self {
        let mut raw = raw::Raw {
            read_ptr: ptr,
            sample_type,
        };

        let sizd = raw.sized();
        Self {
            pid: sizd.pid,
            tid: sizd.tid,
            addr: sizd.addr,
            len: sizd.len,
            pgoff: sizd.pgoff,
            anon_enum: {
                let anon_union = sizd.anon_union;
                match misc as _ {
                    #[cfg(feature = "linux-5.12")]
                    PERF_RECORD_MISC_MMAP_BUILD_ID => {
                        let build_id_len = anon_union.anon_struct_2.build_id_size as _;
                        let build_id = anon_union.anon_struct_2.build_id[0..build_id_len].to_vec();
                        AnonEnum::BuildId(build_id)
                    }
                    _ => AnonEnum::Normal {
                        maj: anon_union.anon_struct_1.maj,
                        min: anon_union.anon_struct_1.min,
                        ino: anon_union.anon_struct_1.ino,
                        ino_generation: anon_union.anon_struct_1.ino_generation,
                    },
                }
            },
            prot: sizd.prot,
            flags: sizd.flags,
            filename: CString::from_vec_unchecked(raw.filename().to_vec()),
            sample_id: sample_id_all.then(|| raw.sample_id()),
        }
    }
}
