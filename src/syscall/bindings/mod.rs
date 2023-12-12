#[allow(clippy::useless_transmute)]
#[allow(clippy::unnecessary_cast)]
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(clippy::all, clippy::nursery, clippy::cargo_common_metadata)]
mod bindgen;
mod r#impl;
mod read_format;

pub use bindgen::*;
pub use r#impl::*;
pub use read_format::*;
