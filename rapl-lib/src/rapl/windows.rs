use once_cell::sync::OnceCell;
use std::{
    ffi::CString,
    sync::{
        atomic::{AtomicU64, Ordering},
        Once,
    },
};
use sysinfo::{CpuExt, System, SystemExt};
use thiserror::Error;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{GENERIC_READ, HANDLE},
        Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
        Storage::FileSystem::{CreateFileA, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, OPEN_EXISTING},
        System::{
            Threading::{GetCurrentProcess, OpenProcessToken},
            IO::DeviceIoControl,
        },
    },
};

// RAPL Intel: https://github.com/tfett/RAPL/blob/master/rapwl-read.c
// RAPL AMD: https://me.sakana.moe/2023/09/06/measuring-cpu-power-consumption/
// Read MSR on Windows: https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/blob/cada6b76b009105aadd9bb2821a7c4cae5cca431/WinRing0/OpenLibSys.c#L313
// Windows RAPL Driver: https://github.com/hubblo-org/windows-rapl-driver/tree/master

// TODO: Install driver ourselves: https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/blob/cada6b76b009105aadd9bb2821a7c4cae5cca431/LibreHardwareMonitorLib/Hardware/KernelDriver.cs#L40
/*
Sample with making driver service and starting it:

#include <windows.h>

int main() {
    SC_HANDLE scm, service;

    scm = OpenSCManager(NULL, NULL, SC_MANAGER_ALL_ACCESS);
    if (scm == NULL) {
        // Handle error
        return 1;
    }

    service = CreateService(scm, L"YourDriverName", L"Your Driver Display Name",
        SERVICE_ALL_ACCESS, SERVICE_KERNEL_DRIVER, SERVICE_DEMAND_START, SERVICE_ERROR_NORMAL,
        L"Path to your driver file", NULL, NULL, NULL, NULL, NULL);

    if (service == NULL) {
        // Handle error
        CloseServiceHandle(scm);
        return 2;
    }

    StartService(service, 0, NULL);

    CloseServiceHandle(service);
    CloseServiceHandle(scm);

    return 0;
}
*/

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

// AMD
const AMD_MSR_PWR_UNIT: u32 = 0xC0010299;
/*
const AMD_MSR_CORE_ENERGY: u32 = 0xC001029A;
const AMD_MSR_PACKAGE_ENERGY: u32 = 0xC001029B;

const AMD_TIME_UNIT_MASK: u64 = 0xF0000;
const AMD_ENERGY_UNIT_MASK: u64 = 0x1F00;
const AMD_POWER_UNIT_MASK: u64 = 0xF;
*/

// Intel
const MSR_RAPL_POWER_UNIT: u32 = 0x606;
/*
const MSR_RAPL_PKG: u32 = 0x611;
const MSR_RAPL_PP0: u32 = 0x639;
const MSR_RAPL_PP1: u32 = 0x641;
const MSR_RAPL_DRAM: u32 = 0x619;

const INTEL_TIME_UNIT_MASK: u64 = 0xF000;
const INTEL_ENGERY_UNIT_MASK: u64 = 0x1F00;
const INTEL_POWER_UNIT_MASK: u64 = 0x0F;

const INTEL_TIME_UNIT_OFFSET: u64 = 0x10;
const INTEL_ENGERY_UNIT_OFFSET: u64 = 0x08;
const INTEL_POWER_UNIT_OFFSET: u64 = 0;
*/

static RAPL_START: AtomicU64 = AtomicU64::new(0);
//static RAPL_STOP: AtomicU64 = AtomicU64::new(0);

static RAPL_INIT: Once = Once::new();
static RAPL_DRIVER: OnceCell<HANDLE> = OnceCell::new();

static PROCESSOR_TYPE: OnceCell<ProcessorType> = OnceCell::new();
#[allow(clippy::upper_case_acronyms)]
enum ProcessorType {
    Intel,
    AMD,
}

// TODO: CloseHandle on driver handle
pub fn start_rapl_impl() {
    // Initialize RAPL driver on first call
    RAPL_INIT.call_once(|| {
        if !is_admin() {
            panic!("not running as admin");
        }

        let h_device = open_driver().expect("failed to open driver handle");
        RAPL_DRIVER.get_or_init(|| h_device);

        let sys = System::new_all();
        let cpu = sys.cpus().first().expect("failed getting CPU").vendor_id();
        match cpu {
            "GenuineIntel" => PROCESSOR_TYPE.get_or_init(|| ProcessorType::Intel),
            "AuthenticAMD" => PROCESSOR_TYPE.get_or_init(|| ProcessorType::AMD),
            _ => {
                panic!("unknown CPU detected: {}", cpu);
            }
        };
    });

    // Read MSR based on the processor type
    let msr_val = match PROCESSOR_TYPE.get().unwrap() {
        ProcessorType::Intel => read_msr(*RAPL_DRIVER.get().unwrap(), MSR_RAPL_POWER_UNIT)
            .expect("failed to read MSR_RAPL_POWER_UNIT"),
        ProcessorType::AMD => read_msr(*RAPL_DRIVER.get().unwrap(), AMD_MSR_PWR_UNIT)
            .expect("failed to read AMD_MSR_PWR_UNIT"),
    };

    RAPL_START.store(msr_val, Ordering::Relaxed);
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
