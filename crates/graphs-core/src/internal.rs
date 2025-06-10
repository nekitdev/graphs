macro_rules! failed {
    ($method: ident) => {
        concat!("call to `", stringify!($method), "` failed")
    };
}

pub(crate) use failed;
