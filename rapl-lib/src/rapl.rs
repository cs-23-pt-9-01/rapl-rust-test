use thiserror::Error;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windowss;

#[cfg(amd)]
use crate::rapl::amd::{MSR_RAPL_PKG_ENERGY_STAT, MSR_RAPL_POWER_UNIT};

#[cfg(intel)]
use crate::rapl::intel::{MSR_RAPL_PKG, MSR_RAPL_POWER_UNIT};

use self::windowss::read_msr;

#[derive(Error, Debug)]
pub enum RaplError {
    #[error("windows error")]
    Windows(#[from] windows::core::Error),
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
pub mod amd {
    /*
    https://lore.kernel.org/lkml/20180817163442.10065-2-calvin.walton@kepstin.ca/

    "A notable difference from the Intel implementation is that AMD reports
    the "Cores" energy usage separately for each core, rather than a
    per-package total"
     */
    pub const MSR_RAPL_POWER_UNIT: u32 = 0xC0010299; // Similar to Intel MSR_RAPL_POWER_UNIT
    pub const AMD_MSR_CORE_ENERGY: u32 = 0xC001029A; // Similar to Intel PP0_ENERGY_STATUS (PP1 is for the GPU)
    pub const MSR_RAPL_PKG_ENERGY_STAT: u32 = 0xC001029B; // Similar to Intel PKG_ENERGY_STATUS (This is for the whole socket)

    /*
    const AMD_TIME_UNIT_MASK: u64 = 0xF0000;
    const AMD_ENERGY_UNIT_MASK: u64 = 0x1F00;
    const AMD_POWER_UNIT_MASK: u64 = 0xF;
    */
}

#[cfg(intel)]
pub mod intel {
    pub const MSR_RAPL_POWER_UNIT: u32 = 0x606;
    pub const MSR_RAPL_PKG_ENERGY_STAT: u32 = 0x611;
    /*
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
}
