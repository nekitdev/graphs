macro_rules! failed {
    ($name: ident) => {
        concat!("call to `", stringify!($name), "` failed")
    };
}

pub(crate) use failed;
