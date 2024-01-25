#[allow(clippy::useless_transmute)]
#[allow(clippy::unnecessary_cast)]
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(clippy::all, clippy::nursery, clippy::cargo_common_metadata)]
#[rustfmt::skip]
mod bindgen;
mod r#impl;
mod read_format;

#[rustfmt::skip]
pub use bindgen::*;
#[allow(unused_imports)]
pub use r#impl::*;
pub use read_format::*;

#[cfg(feature = "linux-5.12")]
#[allow(non_camel_case_types)]
pub type perf_sample_weight = bindgen::perf_sample_weight;

#[cfg(not(feature = "linux-5.12"))]
#[allow(non_camel_case_types)]
pub type perf_sample_weight = u64;
