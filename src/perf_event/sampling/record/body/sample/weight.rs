use crate::syscall::bindings::perf_sample_weight;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum WeightRepr {
    Full,
    Vars,
}

#[derive(Debug, Clone)]
pub enum Weight {
    Full(u64),
    Vars {
        var1_dw: u32,
        var2_w: u16,
        var3_w: u16,
    },
}

impl Weight {
    pub(crate) fn from_raw(raw: perf_sample_weight, repr: WeightRepr) -> Self {
        unsafe {
            match repr {
                WeightRepr::Full => Self::Full(raw.full),
                WeightRepr::Vars => Self::Vars {
                    var1_dw: raw.__bindgen_anon_1.var1_dw,
                    var2_w: raw.__bindgen_anon_1.var2_w,
                    var3_w: raw.__bindgen_anon_1.var3_w,
                },
            }
        }
    }
}
