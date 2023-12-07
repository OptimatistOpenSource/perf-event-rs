use crate::sampling::record::SampleId;
use crate::syscall::bindings::PERF_RECORD_MISC_MMAP_BUILD_ID;
use std::ffi::CString;

mod raw;

#[derive(Debug)]
pub enum AnonEnum {
    Normal {
        maj: u32,
        min: u32,
        ino: u64,
        ino_generation: u64,
    },
    BuildId(Vec<u8>),
}

#[derive(Debug)]
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

type RawBody = raw::Body;

impl Body {
    pub unsafe fn from_ptr(ptr: *const u8, misc: u16, sample_id_all: bool) -> Self {
        let raw = (ptr as *const RawBody).as_ref().unwrap();

        Self {
            pid: *raw.pid(),
            tid: *raw.tid(),
            addr: *raw.addr(),
            len: *raw.len(),
            pgoff: *raw.pgoff(),
            anon_enum: match misc as _ {
                PERF_RECORD_MISC_MMAP_BUILD_ID => {
                    let build_id_len = raw.anon_union().anon_struct_2.build_id_size as _;
                    let build_id =
                        raw.anon_union().anon_struct_2.build_id[0..build_id_len].to_vec();
                    AnonEnum::BuildId(build_id)
                }
                _ => AnonEnum::Normal {
                    maj: raw.anon_union().anon_struct_1.maj,
                    min: raw.anon_union().anon_struct_1.min,
                    ino: raw.anon_union().anon_struct_1.ino,
                    ino_generation: raw.anon_union().anon_struct_1.ino_generation,
                },
            },
            prot: *raw.prot(),
            flags: *raw.flags(),
            filename: CString::from_vec_unchecked(raw.filename().to_vec()),
            sample_id: sample_id_all.then(|| raw.sample_id().clone()),
        }
    }
}
