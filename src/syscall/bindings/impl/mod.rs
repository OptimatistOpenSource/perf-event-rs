mod perf_event_attr;
#[allow(non_snake_case)]
mod perf_event_attr__bindgen_ty_1;
#[allow(non_snake_case)]
mod perf_event_attr__bindgen_ty_2;
#[allow(non_snake_case)]
mod perf_event_attr__bindgen_ty_3;
#[allow(non_snake_case)]
mod perf_event_attr__bindgen_ty_4;
#[cfg(feature = "linux-5.9")]
mod perf_event_mmap_page;
#[cfg(feature = "linux-5.9")]
#[allow(non_snake_case)]
mod perf_event_mmap_page__bindgen_ty_1;
#[cfg(feature = "linux-5.12")]
mod perf_sample_weight;

#[allow(unused_imports)]
pub use perf_event_attr::*;
#[allow(unused_imports)]
pub use perf_event_attr__bindgen_ty_1::*;
#[allow(unused_imports)]
pub use perf_event_attr__bindgen_ty_2::*;
#[allow(unused_imports)]
pub use perf_event_attr__bindgen_ty_3::*;
#[allow(unused_imports)]
pub use perf_event_attr__bindgen_ty_4::*;
#[cfg(feature = "linux-5.9")]
#[allow(unused_imports)]
pub use perf_event_mmap_page::*;
#[cfg(feature = "linux-5.9")]
#[allow(unused_imports)]
pub use perf_event_mmap_page__bindgen_ty_1::*;

#[macro_export]
macro_rules! debug_union {
    (
        name: $name: ident
        self: $self: ident
        fmt: $f: ident
        fields: $($(#[$attr: meta])* $field: ident)+
    ) => {{
        let mut ds = $f.debug_struct(stringify!($name));
        $(
            $(#[$attr])*
            ds.field(stringify!($field), &unsafe { $self.$field });
        )+
        ds.finish()?;
    }};
}

#[macro_export]
macro_rules! debug_struct {
    (
        name: $name: ident
        self: $self: ident
        fmt: $f: ident
        fields: $($(#[$attr: meta])* $field:ident)+
    ) => {{
        let mut ds = $f.debug_struct(stringify!($name));
        $(
            $(#[$attr])*
            ds.field(stringify!($field), &$self.$field);
        )+
        ds.finish()?;
    }};
}

#[macro_export]
macro_rules! debug_struct_fn {
    (
        name: $name: ident
        self: $self: ident
        fmt: $f: ident
        fields: $($(#[$attr: meta])* $field:ident)+
    ) => {{
        let mut ds = $f.debug_struct(stringify!($name));
        $(
            $(#[$attr])*
            ds.field(stringify!($field), &$self.$field());
        )+
        ds.finish()?;
    }};
}
