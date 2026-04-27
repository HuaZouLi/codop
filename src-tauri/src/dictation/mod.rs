#[cfg_attr(any(target_os = "macos", target_os = "windows"), path = "real.rs")]
#[cfg_attr(not(any(target_os = "macos", target_os = "windows")), path = "stub.rs")]
mod imp;

pub(crate) use imp::*;
