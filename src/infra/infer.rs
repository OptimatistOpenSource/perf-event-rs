/// Use this to infer type for val, as same as type ascription
#[inline]
pub const fn infer<T>(val: T) -> T {
    val
}
