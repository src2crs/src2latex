macro_rules! texsrc {
    () => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/texsrc")
    };
}
pub(crate) use texsrc;

macro_rules! sty {
    () => {
        concat!($crate::package::macros::texsrc!(), "/sty")
    };
}
pub(crate) use sty;

macro_rules! package {
    ($name:literal) => {
        include_str!(concat!($crate::package::macros::sty!(), "/", $name, ".sty"))
    };
}
pub(crate) use package;
