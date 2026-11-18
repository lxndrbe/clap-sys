use crate::host::*;

use core::ffi::CStr;
use core::ffi::c_void;

pub const CLAP_EXT_SCRATCH_MEMORY: &CStr = c"clap.scratch-memory/1";

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct clap_host_scratch_memory {
    pub reserve: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            scratch_size_bytes: u32,
            max_concurrency_hint: u32,
        ) -> bool,
    >,
    pub access: Option<unsafe extern "C" fn(host: *const clap_host) -> *mut c_void>,
}
