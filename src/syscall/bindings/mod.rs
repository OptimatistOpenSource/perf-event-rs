#[allow(clippy::useless_transmute)]
#[allow(clippy::unnecessary_cast)]
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(clippy::all, clippy::nursery, clippy::cargo_common_metadata)]
mod bindgen; // TODO: bindings may differ between kernel versions
mod r#impl;

pub use bindgen::*;
pub use r#impl::*;
