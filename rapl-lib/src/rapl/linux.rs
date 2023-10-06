use super::{get_cpu_type, read_rapl_pkg_energy_stat, read_rapl_power_unit, RaplError};
use csv::{Writer, WriterBuilder};
use once_cell::sync::OnceCell;
use std::{
    fs::{File, OpenOptions},
    os::unix::prelude::FileExt,
    sync::Once,
};

// Running it for now: sudo ./target/debug/rapl-bin

static CPU0_MSR_FD: OnceCell<File> = OnceCell::new();
static mut RAPL_START: u64 = 0;
static mut CSV_WRITER: Option<Writer<File>> = None;

static RAPL_INIT: Once = Once::new();
static RAPL_POWER_UNITS: OnceCell<u64> = OnceCell::new();

pub fn start_rapl_impl() {
    RAPL_INIT.call_once(|| {
        // Read power unit and store it the power units variable
        let pwr_unit = read_rapl_power_unit().unwrap();
        RAPL_POWER_UNITS.get_or_init(|| pwr_unit);
    });

    let result = read_rapl_pkg_energy_stat().unwrap();
    unsafe { RAPL_START = result };
}

pub fn stop_rapl_impl() {
    let rapl_end_val = read_rapl_pkg_energy_stat().unwrap();

    let rapl_start_val = unsafe { RAPL_START };

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
            wtr.write_record(["PkgStart", "PkgEnd"]).unwrap();

            // Store the CSV writer in a static variable
            unsafe { CSV_WRITER = Some(wtr) };

            // Return a mutable reference to the CSV writer
            unsafe { CSV_WRITER.as_mut().unwrap() }
        }
    };

    wtr.serialize((rapl_start_val, rapl_end_val)).unwrap();
    wtr.flush().unwrap();
}

// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L14
fn open_msr(core: u32) -> File {
    File::open(format!("/dev/cpu/{}/msr", core)).unwrap()
}

// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L38
pub fn read_msr(msr_offset: u64) -> Result<u64, RaplError> {
    let f = CPU0_MSR_FD.get_or_init(|| open_msr(0));

    let mut output_data: [u8; 8] = [0; 8];

    // TODO: Consider just seek here instead, same impl for Windows then
    f.read_at(&mut output_data, msr_offset).unwrap();

    Ok(u64::from_le_bytes(output_data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_msr() {
        let result = read_rapl_pkg_energy_stat().unwrap();
        assert_eq!(result, 1234);
    }
}
