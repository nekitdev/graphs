/// Copies the given value.
pub const fn copy<T: Copy>(value: &T) -> T {
    *value
}
