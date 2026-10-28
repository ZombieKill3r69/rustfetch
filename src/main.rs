use std::path::Path;
use uzers::{get_current_uid, get_user_by_uid,};
use sysinfo::{System, Disks,};
use hostname;

fn main() {

    let bytes_to_mib_divisor: u64 = 1024 * 1024;
    let bytes_to_gib_divisor: u64 = 1024 * 1024 * 1024;

    let mut sys: System = System::new_all();
    sys.refresh_all();

    // Username
    let userinfo: uzers::User = get_user_by_uid(get_current_uid()).unwrap();
    let username: String = userinfo.name()
        .to_string_lossy()
        .to_string();

    // Hostname
    let host: String = hostname::get()
        .unwrap()
        .to_string_lossy()
        .to_string();

    // OS
    let os_name: String = System::name()
        .unwrap_or("Unknown OS".to_string());
    let os_version: String = System::os_version()
        .unwrap_or("Unknown Version".to_string());
    let architecture: String = System::cpu_arch();
    let kernel: String = System::kernel_version()
        .unwrap_or("Unknown Kernel".to_string());

    // Memory
    let memory_total: u64 = sys.total_memory() / bytes_to_mib_divisor; // Divides by a big number to
    let memory_used: u64 = sys.used_memory() / bytes_to_mib_divisor;   // convert from whatever to mebibytes

    // Uptime
    let uptime_seconds = System::uptime();
    let uptime: (u64, u64, u64) = (
        uptime_seconds / 3600,
        (uptime_seconds % 3600) / 60,
        uptime_seconds % 60,
    );

    // Storage
    let disk_info: (u64, u64) = {
        let disks: Disks = Disks::new_with_refreshed_list();
        let mut total_space: u64 = 0;
        let mut used_space: u64 = 0;
    
        for disk in disks.list() {
            if disk.mount_point() == Path::new("/") { // Check if diskis root
                total_space = disk.total_space();
                used_space = disk.total_space() - disk.available_space();
                break; // We dont care about anything but root here so we stop
            }
        }
    
        (total_space, used_space) // Return both values from one function for consiseness
    };
    let storage_used: u64 = disk_info.1 / bytes_to_gib_divisor;
    let storage_total: u64 = disk_info.0 / bytes_to_gib_divisor;

    // Output
    println!("{username}@{host}");
    println!("Uptime: {}hours {}mins {}secs", uptime.0, uptime.1, uptime.2);
    println!("OS: {os_name} {os_version} {architecture}");
    println!("Kernel: {kernel}");
    println!("Memory: {memory_used}MiB / {memory_total}MiB");
    println!("Storage: {storage_used}GiB / {storage_total}GiB");
}

