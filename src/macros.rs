macro_rules! resource_root {
    ($e:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/res/", $e)
    }
}
macro_rules! include_res {
    ($e:expr) => {
        include_bytes!(resource_root!($e))
    }
}
macro_rules! include_res_str {
    ($e:expr) => {
        include_str!(resource_root!($e))
    }
}
