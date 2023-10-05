use once_cell::sync::OnceCell;
use std::{fs::File, os::unix::prelude::FileExt};

// Running it for now: sudo ./target/debug/rapl-bin

//const MSR_RAPL_POWER_UNIT: i64 = 0x606;
const MSR_RAPL_PKG: i64 = 0x611;
static CPU0_MSR_FD: OnceCell<File> = OnceCell::new();

pub fn start_rapl_impl() {
    let f = CPU0_MSR_FD.get_or_init(|| open_msr(0));
    let result = read_msr(f, MSR_RAPL_PKG);
    println!("MSR RES START: {}", result);
}

pub fn stop_rapl_impl() {
    let f = CPU0_MSR_FD.get().unwrap();
    let result = read_msr(f, MSR_RAPL_PKG);
    println!("MSR RES STOP: {}", result);
}

// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L157
// fn rapl_init() {} // Use OnceCell for init e.g. with fd

// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L64
// fn detect_cpu() {} // Compile timed currently

// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L14
fn open_msr(core: u32) -> File {
    File::open(format!("/dev/cpu/{}/msr", core)).unwrap()
}

// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L38
fn read_msr(fd: &File, msr_offset: i64) -> u64 {
    let mut output_data: [u8; 8] = [0; 8];

    // TODO: Consider just seek here instead, same impl for Windows then
    fd.read_at(&mut output_data, msr_offset as u64).unwrap();
    //let num_bytes_read = fd.read_at(&mut output_data, msr_offset as u64).unwrap();
    //println!("number of bytes read: {}", num_bytes_read);

    u64::from_le_bytes(output_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_msr() {
        let fd = open_msr(0);
        let result = read_msr(&fd, MSR_RAPL_PKG);
        assert_eq!(result, 1234);
    }
}
