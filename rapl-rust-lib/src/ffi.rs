use crate::rapl::windows::start_rapl_windows;

#[no_mangle]
pub extern "C" fn start_rapl() -> usize {
    start_rapl_windows()
}

#[no_mangle]
pub extern "C" fn end_rapl() -> usize {
    456
}
