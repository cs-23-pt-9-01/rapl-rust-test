#[cfg(target_os = "linux")]
use crate::rapl::linux::start_rapl_impl;

#[cfg(target_os = "windows")]
use crate::rapl::windows::start_rapl_impl;

#[no_mangle]
pub extern "C" fn start_rapl() -> usize {
    start_rapl_impl()
}

#[no_mangle]
pub extern "C" fn end_rapl() -> usize {
    456
}
