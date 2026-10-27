use sysinfo::{
    // Components, Disks, 
    System};

fn main() {

    let bytes_to_mib_divisor: u64 = 1024 * 1024;
    let _bytes_to_gib_divisor: u64 = 1024 * 1024 * 1024;

    let mut sys: System = System::new_all();
    sys.refresh_all();

    // OS
    let os_name: String = System::name().unwrap_or("Unknown OS".to_string());
    let os_version: String = System::os_version().unwrap_or("Unknown Version".to_string());
    let architecture: String = System::cpu_arch();
    let kernal: String = System::kernel_version().unwrap_or("Unknown Kernel".to_string());

    // CPU and GPU

    // Memory
    let memory_total: u64 = sys.total_memory() / bytes_to_mib_divisor; // Divides by a big number to
    let memory_used: u64 = sys.used_memory() / bytes_to_mib_divisor;   // convert from whatever to mebibytes

    // Output
    println!("OS: {os_name} {os_version} {architecture}");
    println!("Kernal: {kernal}");
    println!("Memory: {memory_used}MiB / {memory_total}MiB");
    println!("CPU: ");
    println!("GPU: ");
    println!("Locale: ");
}
