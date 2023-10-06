use super::{get_cpu_type, read_rapl_pkg_energy_stat, read_rapl_power_unit};
use crate::rapl::RaplError;
use csv::{Writer, WriterBuilder};
use once_cell::sync::OnceCell;
use std::{
    ffi::CString,
    fs::{File, OpenOptions},
    sync::Once,
};
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

// Use File Open on Windows instead
// https://doc.rust-lang.org/stable/std/os/windows/io/trait.FromRawHandle.html

/*
#define IOCTL_OLS_READ_MSR \
    CTL_CODE(OLS_TYPE, 0x821, METHOD_BUFFERED, FILE_ANY_ACCESS)
*/
const IOCTL_OLS_READ_MSR: u32 = 0x9C402084;

static mut RAPL_START: (u64, u64) = (0, 0);
//static RAPL_STOP: AtomicU64 = AtomicU64::new(0);

static RAPL_INIT: Once = Once::new();
static RAPL_DRIVER: OnceCell<HANDLE> = OnceCell::new();
static RAPL_POWER_UNITS: OnceCell<u64> = OnceCell::new();

static mut CSV_WRITER: Option<Writer<File>> = None;

pub fn start_rapl_impl() {
    // Initialize RAPL driver on first call
    RAPL_INIT.call_once(|| {
        // Check if running as admin due to driver requirement
        if !is_admin() {
            panic!("not running as admin, this is required for the RAPL driver to work");
        }

        let h_device = open_driver()
            .expect("failed to open driver handle, make sure the driver is installed and running");
        RAPL_DRIVER.get_or_init(|| h_device);

        // Read power unit and store it the power units variable
        let pwr_unit = read_rapl_power_unit().expect("failed to read RAPL power unit");
        RAPL_POWER_UNITS.get_or_init(|| pwr_unit);
    });

    // Safety: RAPL_START is only accessed in this function and only from a single thread
    #[cfg(amd)]
    unsafe {
        RAPL_START = read_rapl_values_amd()
    };

    #[cfg(intel)]
    unsafe {
        RAPL_START = read_rapl_values_intel()
    };
}

#[cfg(amd)]
fn read_rapl_values_amd() -> (u64, u64) {
    use super::amd::AMD_MSR_CORE_ENERGY;

    let pkg = read_rapl_pkg_energy_stat().expect("failed to read pkg energy stat");
    let core = read_msr(AMD_MSR_CORE_ENERGY).unwrap();

    (pkg, core)
}

#[cfg(intel)]
fn read_rapl_values_intel() -> (u64, u64, u64, u64) {
    use super::intel::{INTEL_MSR_RAPL_DRAM, INTEL_MSR_RAPL_PP0, INTEL_MSR_RAPL_PP1};

    let pp0 = read_msr(INTEL_MSR_RAPL_PP0).expect("failed to read pp0");
    let pp1 = read_msr(INTEL_MSR_RAPL_PP1).expect("failed to read pp1");
    let dram = read_msr(INTEL_MSR_RAPL_DRAM).expect("failed to read dram");
    let pkg = read_rapl_pkg_energy_stat().expect("failed to read pkg energy stat");

    (pp0, pp1, dram, pkg)
}

// Get all drivers: sc query type=driver
// Stop manually in CMD: sc stop R0LibreHardwareMonitor
// Delete manually in CMD: sc delete R0LibreHardwareMonitor

pub fn stop_rapl_impl() {
    // Read the RAPL end values
    let (pkg_end, core_end) = read_rapl_values_amd();

    // Load in the RAPL start value
    // Safety: RAPL_START is only accessed in this function and only from a single thread
    let (pkg_start, core_start) = unsafe { RAPL_START };

    /*
    // TODO: Revise if we can even use timestamps

    let current_time = SystemTime::now();
    let duration_since_epoch = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp = duration_since_epoch.as_millis();
    */

    let wtr = match unsafe { CSV_WRITER.as_mut() } {
        Some(wtr) => wtr,
        None => {
            // Open the file to write to CSV. First argument is CPU type, second is RAPL power units
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!(
                    "{}_{}.csv",
                    get_cpu_type(),
                    RAPL_POWER_UNITS.get().unwrap()
                ))
                .unwrap();

            // Create the CSV writer
            let mut wtr = WriterBuilder::new().from_writer(file);
            /*
            wtr.write_record([
                "PP0Start",
                "PP0End",
                "PP1Start",
                "PP1End",
                "PkgStart",
                "PkgEnd",
                "DramStart",
                "DramEnd",
            ])
            .unwrap();
            */
            wtr.write_record(["PkgStart", "PkgEnd", "CoreStart", "CoreEnd"])
                .unwrap();

            // Store the CSV writer in a static variable
            unsafe { CSV_WRITER = Some(wtr) };

            // Return a mutable reference to the CSV writer
            unsafe { CSV_WRITER.as_mut().unwrap() }
        }
    };

    wtr.serialize((pkg_start, pkg_end, core_start, core_end))
        .unwrap();
    wtr.flush().unwrap();
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

// Read the MSR using the driver
pub fn read_msr(msr: u64) -> Result<u64, RaplError> {
    Ok(read_msr_wrapper(msr as u32)?)
}

// Read the MSR using the driver with a 32 bit MSR
// __readmsr on Windows takes in an "int" as the MSR, which is 32 bits
pub fn read_msr_wrapper(msr: u32) -> Result<u64, RaplError> {
    /*
    // TODO: Validate if this works correctly. Should be used instead
    let driver_file = File::open("\\\\.\\WinRing0_1_2_0").unwrap();
    let driver_handle = HANDLE(driver_file.as_raw_handle() as _);
    */

    // Get the driver handle
    let rapl_driver = *RAPL_DRIVER.get().expect("RAPL driver not initialized");

    // Convert the MSR to a little endian byte array
    let input_data: [u8; 4] = msr.to_le_bytes();

    // Create an empty byte array to store the output
    let output_data: [u8; 8] = [0; 8];
    let mut lp_bytes_returned: u32 = 0;

    // Call the driver to read the MSR
    unsafe {
        DeviceIoControl(
            rapl_driver,
            IOCTL_OLS_READ_MSR,
            Some(input_data.as_ptr() as _),
            input_data.len() as u32,
            Some(output_data.as_ptr() as _),
            output_data.len() as u32,
            Some(&mut lp_bytes_returned as _),
            None,
        )
    }?;

    // TODO: Consider using lp_bytes_returned for error handling or logging it, it is supposed to return 8 bytes on success
    //println!("lp_bytes_returned: {}", lp_bytes_returned);
    Ok(u64::from_le_bytes(output_data))
}

/*
// Experimental. This was not a great success because Windows takes too long deleting + recreating the driver
// TODO: Consider documenting this or revisiting it later

fn install_driver() -> Result<(), RaplError> {
    let scm =
        unsafe { OpenSCManagerA(PCSTR::null(), PCSTR::null(), SC_MANAGER_ALL_ACCESS) }.unwrap();

    let driver_name = CString::new("R0LibreHardwareMonitor").expect("failed to create driver name");
    let driver_path =
        CString::new("C:\\Users\\Jakob\\Documents\\GitHub\\cs-23-pt-9-01\\rapl-rust-test\\LibreHardwareMonitor.sys").expect("failed to create driver path");

    let created_driver_service = unsafe {
        CreateServiceA(
            scm,
            PCSTR(driver_name.as_ptr() as *const u8),
            PCSTR(driver_name.as_ptr() as *const u8),
            SERVICE_ALL_ACCESS,
            SERVICE_KERNEL_DRIVER,
            SERVICE_DEMAND_START,
            SERVICE_ERROR_NORMAL,
            PCSTR(driver_path.as_ptr() as *const u8),
            None,
            None,
            None,
            None,
            None,
        )
    }
    .unwrap();

    unsafe { StartServiceA(created_driver_service, None) }.unwrap();

    unsafe { CloseServiceHandle(created_driver_service) }.unwrap();
    unsafe { CloseServiceHandle(scm) }.unwrap();

    Ok(())
}

fn stop_and_delete_driver() -> Result<(), RaplError> {
    let driver_name = CString::new("R0LibreHardwareMonitor").expect("failed to create driver name");
    let scm =
        unsafe { OpenSCManagerA(PCSTR::null(), PCSTR::null(), SC_MANAGER_ALL_ACCESS) }.unwrap();

    if let Ok(driverr) = unsafe {
        OpenServiceA(
            scm,
            PCSTR(driver_name.as_ptr() as *const u8),
            SERVICE_ALL_ACCESS,
        )
    } {
        // Stop the driver
        let mut service_status: SERVICE_STATUS = Default::default();
        unsafe {
            ControlService(
                driverr,
                SERVICE_CONTROL_STOP,
                &mut service_status as *mut SERVICE_STATUS,
            )
        }
        .unwrap();

        unsafe { DeleteService(driverr) }.unwrap();
        unsafe { CloseServiceHandle(driverr) }.unwrap();
    }
    unsafe { CloseServiceHandle(scm) }.unwrap();

    Ok(())
}
*/

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
