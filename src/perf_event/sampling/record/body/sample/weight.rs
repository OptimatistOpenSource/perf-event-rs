use crate::syscall::bindings::perf_sample_weight;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum WeightRepr {
    Full,
    #[cfg(feature = "linux-5.12")]
    Vars,
}

#[derive(Debug, Clone)]
pub enum Weight {
    Full(u64),
    #[cfg(feature = "linux-5.12")]
    Vars {
        var1_dw: u32,
        var2_w: u16,
        var3_w: u16,
    },
}

impl Weight {
    pub(crate) const fn from_raw(raw: perf_sample_weight, repr: WeightRepr) -> Self {
        match repr {
            #[cfg(feature = "linux-5.12")]
            WeightRepr::Full => unsafe { Self::Full(raw.full) },
            #[cfg(feature = "linux-5.12")]
            WeightRepr::Vars => unsafe {
                Self::Vars {
                    var1_dw: raw.__bindgen_anon_1.var1_dw,
                    var2_w: raw.__bindgen_anon_1.var2_w,
                    var3_w: raw.__bindgen_anon_1.var3_w,
                }
            },
            #[cfg(not(feature = "linux-5.12"))]
            WeightRepr::Full => Self::Full(raw),
        }
    }
}
