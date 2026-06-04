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

macro_rules! debug_wrap_enum {
    ($formatter: expr => $wrap: expr => {
        $($variant: ident),+ $(,)?
    }) => {
        {
            let formatter = $formatter;

            match $wrap {
                $(
                    Self::$variant(wrapped) => {
                        write!(
                            formatter,
                            "{variant}({wrapped:?})",
                            variant = stringify!($variant)
                        )
                    }
                )+
            }
        }
    };
}

pub(crate) use debug_wrap_enum;
