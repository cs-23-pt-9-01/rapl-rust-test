use csv::Writer;
use once_cell::sync::OnceCell;
use std::{fs::File, sync::Once};
use thiserror::Error;

// Use the OS specific implementation
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windowss;

// Import the MSR constants per CPU type
#[cfg(amd)]
use crate::rapl::amd::{MSR_RAPL_PKG_ENERGY_STAT, MSR_RAPL_POWER_UNIT};
#[cfg(intel)]
use crate::rapl::intel::{MSR_RAPL_PKG_ENERGY_STAT, MSR_RAPL_POWER_UNIT};

// Import the OS specific functions
#[cfg(target_os = "linux")]
use self::linux::{read_msr, start_rapl_impl, stop_rapl_impl};
#[cfg(target_os = "windows")]
use self::windowss::{read_msr, start_rapl_impl, stop_rapl_impl};

#[derive(Error, Debug)]
pub enum RaplError {
    #[cfg(target_os = "windows")]
    #[error("windows error")]
    Windows(#[from] windows::core::Error),
    #[error("unknown RAPL error")]
    Unknown,
}

#[cfg(amd)]
static mut RAPL_START: (u64, u64) = (0, 0);

#[cfg(intel)]
static mut RAPL_START: (u64, u64, u64, u64) = (0, 0, 0, 0);

static RAPL_INIT: Once = Once::new();
static RAPL_POWER_UNITS: OnceCell<u64> = OnceCell::new();
static mut CSV_WRITER: Option<Writer<File>> = None;

pub fn start_rapl() {
    start_rapl_impl();

    RAPL_INIT.call_once(|| {
        // Read power unit and store in the power units variable
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

pub fn stop_rapl() {
    stop_rapl_impl();
}

// Get the CPU type based on the compile time configuration
pub fn get_cpu_type() -> &'static str {
    #[cfg(intel)]
    {
        "Intel"
    }

    #[cfg(amd)]
    {
        "AMD"
    }
}

pub fn read_rapl_power_unit() -> Result<u64, RaplError> {
    read_msr(MSR_RAPL_POWER_UNIT)
}

pub fn read_rapl_pkg_energy_stat() -> Result<u64, RaplError> {
    read_msr(MSR_RAPL_PKG_ENERGY_STAT)
}

#[cfg(amd)]
fn read_rapl_values_amd() -> (u64, u64) {
    use self::amd::AMD_MSR_CORE_ENERGY;

    let pkg = read_rapl_pkg_energy_stat().expect("failed to read pkg energy stat");
    let core = read_msr(AMD_MSR_CORE_ENERGY).unwrap();

    (pkg, core)
}

#[cfg(intel)]
fn read_rapl_values_intel() -> (u64, u64, u64, u64) {
    use self::intel::{INTEL_MSR_RAPL_DRAM, INTEL_MSR_RAPL_PP0, INTEL_MSR_RAPL_PP1};

    let pp0 = read_msr(INTEL_MSR_RAPL_PP0).expect("failed to read PP0");
    let pp1 = read_msr(INTEL_MSR_RAPL_PP1).expect("failed to read PP1");
    let dram = read_msr(INTEL_MSR_RAPL_DRAM).expect("failed to read DRAM");
    let pkg = read_rapl_pkg_energy_stat().expect("failed to read PKG_ENERGY_STAT");

    (pp0, pp1, dram, pkg)
}

#[cfg(amd)]
pub mod amd {
    /*
    https://lore.kernel.org/lkml/20180817163442.10065-2-calvin.walton@kepstin.ca/

    "A notable difference from the Intel implementation is that AMD reports
    the "Cores" energy usage separately for each core, rather than a
    per-package total"
     */
    pub const MSR_RAPL_POWER_UNIT: u64 = 0xC0010299; // Similar to Intel MSR_RAPL_POWER_UNIT
    pub const MSR_RAPL_PKG_ENERGY_STAT: u64 = 0xC001029B; // Similar to Intel PKG_ENERGY_STATUS (This is for the whole socket)

    pub const AMD_MSR_CORE_ENERGY: u64 = 0xC001029A; // Similar to Intel PP0_ENERGY_STATUS (PP1 is for the GPU)

    /*
    const AMD_TIME_UNIT_MASK: u64 = 0xF0000;
    const AMD_ENERGY_UNIT_MASK: u64 = 0x1F00;
    const AMD_POWER_UNIT_MASK: u64 = 0xF;
    */
}

#[cfg(intel)]
pub mod intel {
    pub const MSR_RAPL_POWER_UNIT: u64 = 0x606;
    pub const MSR_RAPL_PKG_ENERGY_STAT: u64 = 0x611;

    pub const INTEL_MSR_RAPL_PP0: u64 = 0x639;
    pub const INTEL_MSR_RAPL_PP1: u64 = 0x641;
    pub const INTEL_MSR_RAPL_DRAM: u64 = 0x619;
    /*
    const INTEL_TIME_UNIT_MASK: u64 = 0xF000;
    const INTEL_ENGERY_UNIT_MASK: u64 = 0x1F00;
    const INTEL_POWER_UNIT_MASK: u64 = 0x0F;

    const INTEL_TIME_UNIT_OFFSET: u64 = 0x10;
    const INTEL_ENGERY_UNIT_OFFSET: u64 = 0x08;
    const INTEL_POWER_UNIT_OFFSET: u64 = 0;
    */
}
