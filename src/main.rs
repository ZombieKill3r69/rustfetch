use std::path::Path;
use uzers::{get_current_uid, get_user_by_uid,};
use sysinfo::{System, Disks,};
use hostname;

struct Info {
    username:       String,
    host:           String,
    uptime:         (u64, u64, u64), // Hours, mins, secs
    os_name:        String,
    os_version:     String,
    arch:           String,
    kernel:         String,
    memory_used:    u64,
    memory_total:   u64,
    storage_used:   u64,
    storage_total:  u64,
}

fn main() {

    let bytes_to_mib_divisor: u64 = 1_024 * 1_024;
    let bytes_to_gib_divisor: u64 = 1_024 * 1_024 * 1_024;

    let underline_char = "-";

    let mut sys: System = System::new_all();
    sys.refresh_all();
    
    let userinfo: uzers::User = get_user_by_uid(get_current_uid()).unwrap();

    // Add all the info to stuct for simple reference
    let stats: Info = Info {
        username: userinfo.name().to_string_lossy().to_string(),
        host: hostname::get().unwrap().to_string_lossy().to_string(),
        os_name: System::name().unwrap_or("Unknown OS".to_string()),
        os_version: System::os_version().unwrap_or("Unknown Version".to_string()),
        arch: System::cpu_arch(),
        kernel: System::kernel_version().unwrap_or("Unknown Kernel".to_string()),
        uptime: get_system_uptime(),
        memory_used: sys.used_memory() / bytes_to_mib_divisor,
        memory_total: sys.total_memory() / bytes_to_mib_divisor,
        storage_used: get_disk_info().1 / bytes_to_gib_divisor,
        storage_total: get_disk_info().0 / bytes_to_gib_divisor,
    };

    // Line
    let line: String = std::iter::repeat(underline_char)
        .take(format!("{}@{}", stats.username, stats.host).chars().count())
        .collect();

    // Output
    craft_bulk_print(stats, line);
}

fn get_system_uptime() -> (u64, u64, u64) {
    let uptime_seconds: u64 = System::uptime(); 
    
    return (
        uptime_seconds / 3600,         // Hours
        (uptime_seconds % 3600) / 60,  // Mins
        uptime_seconds % 60,           // Secs
    );
}

fn get_disk_info() -> (u64, u64) {
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
}

fn craft_bulk_print(stats: Info, line: String) {

    let mut out = String::new();

    out.push_str(&format!("\x1b[32m{}\x1b[0m@\x1b[32m{}\x1b[0m\n", stats.username, stats.host));
    out.push_str(&format!("\x1b[32m{line}\n"));
    out.push_str(&format!("\x1b[32mUptime  -\x1b[0m> {} hours {} mins {} secs\n", stats.uptime.0, stats.uptime.1, stats.uptime.2));
    out.push_str(&format!("\x1b[32mOS      -\x1b[0m> {} {} {}\n", stats.os_name, stats.os_version, stats.arch));
    out.push_str(&format!("\x1b[32mKernel  -\x1b[0m> {}\n", stats.kernel));
    out.push_str(&format!("\x1b[32mMemory  -\x1b[0m> {}MiB / {}MiB\n", stats.memory_used, stats.memory_total));
    out.push_str(&format!("\x1b[32mStorage -\x1b[0m> {}GiB / {}GiB", stats.storage_used, stats.storage_total));

    println!("{}", out);
}
