use anyhow::Result;
use rapl_rust_lib::rapl::windows::start_rapl_impl;

// RAPL Intel: https://github.com/tfett/RAPL/blob/master/rapwl-read.c
// RAPL AMD: https://me.sakana.moe/2023/09/06/measuring-cpu-power-consumption/
// Read MSR on Windows: https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/blob/cada6b76b009105aadd9bb2821a7c4cae5cca431/WinRing0/OpenLibSys.c#L313
// Windows RAPL Driver: https://github.com/hubblo-org/windows-rapl-driver/tree/master

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

pub fn bench_test(n: i32) -> i32 {
    let mut val: i32 = 0;
    for _ in 0..n {
        val += 1;
    }
    val
}

fn main() -> Result<()> {
    // TODO: Logging, multiple cores (maybe only possible to read all cores at once, although Linux seems to have multiple since MSR for each CPU), multiple CPU support (Intel)
    start_rapl_impl();

    //println!("Fibonacci: {}", fibonacci(900));
    //println!("Bench test: {}", bench_test(1000000000));

    /*
    let sys = System::new_all();
    match sys.cpus().first().expect("failed getting CPU").vendor_id() {
        "GenuineIntel" => println!("Intel CPU detected"),
        "AuthenticAMD" => println!("AMD CPU detected"),
        _ => {
            println!("unknown CPU detected");
            return Ok(());
        }
    }
    */

    // TODO: Install driver ourselves: https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/blob/cada6b76b009105aadd9bb2821a7c4cae5cca431/LibreHardwareMonitorLib/Hardware/KernelDriver.cs#L40

    /*let output_number =
    read_msr(h_device, AMD_MSR_PWR_UNIT).expect("failed to read AMD_MSR_PWR_UNIT");*/
    let output_number = 1324;
    println!("output_number: {}", output_number);

    /*
    let time_unit = ((output_number & AMD_TIME_UNIT_MASK) >> 16) as f64;
    let energy_unit = ((output_number & AMD_ENERGY_UNIT_MASK) >> 8) as f64;
    let power_unit = (output_number & AMD_POWER_UNIT_MASK) as f64;
    println!(
        "time_unit: {}, energy_unit: {}, power_unit: {}",
        time_unit, energy_unit, power_unit
    );

    let time_unit_d = time_unit.powf(0.5);
    let energy_unit_d = energy_unit.powf(0.5);
    let power_unit_d = power_unit.powf(0.5);
    println!(
        "time_unit_d: {}, energy_unit_d: {}, power_unit_d: {}",
        time_unit_d, energy_unit_d, power_unit_d
    );

    let mut vec = Vec::new();
    for _ in 0..100000 {
        let core_energy_raw = read_msr(h_device, AMD_MSR_CORE_ENERGY)
            .expect("failed to read AMD_MSR_CORE_ENERGY") as f64;
        let package_raw = read_msr(h_device, AMD_MSR_PACKAGE_ENERGY)
            .expect("failed to read AMD_MSR_PACKAGE_ENERGY") as f64;
        let core_energy = (core_energy_raw * energy_unit_d) as u64;
        let package_energy = (package_raw * energy_unit_d) as u64;

        let current_time = SystemTime::now();
        let duration_since_epoch = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = duration_since_epoch.as_millis();

        vec.push((core_energy, package_energy, timestamp));
    }

    for i in 0..1000 {
        println!(
            "core_energy: {}, package_energy: {}, timestamp: {}",
            vec[i].0, vec[i].1, vec[i].2
        );
    }*/

    //return Ok(());

    // Read core energy stuff
    /*
    let core_energy_raw =
        read_msr(h_device, AMD_MSR_CORE_ENERGY).expect("failed to read AMD_MSR_CORE_ENERGY") as f64;
    let package_raw = read_msr(h_device, AMD_MSR_PACKAGE_ENERGY)
        .expect("failed to read AMD_MSR_PACKAGE_ENERGY") as f64;
    let core_energy = (core_energy_raw * energy_unit_d) as u64;
    let package_energy = (package_raw * energy_unit_d) as u64;

    println!(
        "core_energy: {}, package_energy: {}",
        core_energy, package_energy
    );

    // Sleep for 10 seconds
    println!("sleeping for 10 seconds");
    thread::sleep(Duration::from_secs(10));

    // Read core energy stuff again
    let core_energy_raw =
        read_msr(h_device, AMD_MSR_CORE_ENERGY).expect("failed to read AMD_MSR_CORE_ENERGY") as f64;
    let package_raw = read_msr(h_device, AMD_MSR_PACKAGE_ENERGY)
        .expect("failed to read AMD_MSR_PACKAGE_ENERGY") as f64;

    let core_energy_delta = (core_energy_raw * energy_unit_d) as u64;
    let package_energy_delta = (package_raw * energy_unit_d) as u64;

    println!(
        "core_energy_delta: {}, package_energy_delta: {}",
        core_energy_delta, package_energy_delta
    );

    let core_energy_diff = core_energy_delta - core_energy;
    let package_diff = package_energy_delta - package_energy;

    println!(
        "core_energy_diff: {}, package_diff: {}",
        core_energy_diff, package_diff
    );

    println!(
        "Energy used: {}W, Package: {}W",
        core_energy_diff, package_diff
    );

    unsafe { CloseHandle(h_device) }.expect("failed to close driver handle");
    */

    Ok(())
}
