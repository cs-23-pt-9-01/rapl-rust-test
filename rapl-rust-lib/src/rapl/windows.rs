use std::{
    ffi::CString,
    io,
    sync::Once,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use thiserror::Error;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{CloseHandle, GENERIC_READ, HANDLE},
        Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
        Storage::FileSystem::{CreateFileA, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, OPEN_EXISTING},
        System::{
            Threading::{GetCurrentProcess, OpenProcessToken},
            IO::DeviceIoControl,
        },
    },
};

#[derive(Error, Debug)]
pub enum RaplError {
    #[error("windows error")]
    Windows(#[from] windows::core::Error),
}

/*
#define IOCTL_OLS_READ_MSR \
    CTL_CODE(OLS_TYPE, 0x821, METHOD_BUFFERED, FILE_ANY_ACCESS)
*/
const IOCTL_OLS_READ_MSR: u32 = 0x9C402084;

const AMD_MSR_PWR_UNIT: u32 = 0xC0010299;
const AMD_MSR_CORE_ENERGY: u32 = 0xC001029A;
const AMD_MSR_PACKAGE_ENERGY: u32 = 0xC001029B;

static RAPL_INIT: Once = Once::new();

pub fn start_rapl_impl() -> u64 {
    // Initialize RAPL driver on first call
    RAPL_INIT.call_once(|| {
        if !is_admin() {
            panic!("not running as admin");
        }
    });

    let h_device = open_driver().expect("failed to open driver handle");
    read_msr(h_device, AMD_MSR_PWR_UNIT).expect("failed to read AMD_MSR_PWR_UNIT")
}

// check if running as admin using the windows crate
fn is_admin() -> bool {
    let mut h_token = HANDLE::default();
    unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut h_token as _) }.unwrap();

    let mut token_elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
    let token_elevation_ptr = &mut token_elevation as *mut TOKEN_ELEVATION;
    let mut cb_size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

    unsafe {
        GetTokenInformation(
            h_token,
            TokenElevation,
            Some(token_elevation_ptr as _),
            cb_size,
            &mut cb_size as _,
        )
        .unwrap();
    }

    token_elevation.TokenIsElevated != 0
}

fn open_driver() -> Result<HANDLE, RaplError> {
    let driver_name = CString::new("\\\\.\\WinRing0_1_2_0").expect("failed to create driver name");
    Ok(unsafe {
        CreateFileA(
            PCSTR(driver_name.as_ptr() as *const u8), // File path
            GENERIC_READ.0,                           // Access mode (read-only in this example)
            FILE_SHARE_READ,                          // Share mode (0 for exclusive access)
            None,                                     // Security attributes (can be None)
            OPEN_EXISTING,                            // Creation disposition
            FILE_ATTRIBUTE_NORMAL,                    // File attributes (normal for regular files)
            None,                                     // Template file (not used here)
        )
    }?)
}

fn read_msr(h_device: HANDLE, msr: u32) -> Result<u64, RaplError> {
    let input_data: [u8; 4] = msr.to_le_bytes();

    let output_data: [u8; 8] = [0; 8];
    let mut lp_bytes_returned: u32 = 0;
    unsafe {
        DeviceIoControl(
            h_device,
            IOCTL_OLS_READ_MSR,
            Some(input_data.as_ptr() as _),
            input_data.len() as u32,
            Some(output_data.as_ptr() as _),
            output_data.len() as u32,
            Some(&mut lp_bytes_returned as _),
            None,
        )
    }?;

    //println!("lp_bytes_returned: {}", lp_bytes_returned);
    Ok(u64::from_le_bytes(output_data))
}
