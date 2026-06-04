macro_rules! debug_struct {
    ($formatter: expr => $name: ident $({
        $($field: ident: $value: expr),+ $(,)?
    })?) => {
        $formatter
            .debug_struct(stringify!($name))
            $(
                $(
                    .field(stringify!($field), &$value)
                )+
            )?
    };
}

pub(crate) use debug_struct;

pub(crate) const fn copy<T: Copy>(value: &T) -> T {
    *value
}
