use sysinfo::{Components, Disks, System,};

fn main() {

    let bytes_to_mib_divisor: u64 = 1024 * 1024;
    let _bytes_to_gib_divisor: u64 = 1024 * 1024 * 1024;

    let mut sys: System = System::new_all();
    sys.refresh_all();

    // Output values
    // OS
    
    // Memory
    let memory_total: u64 = sys.total_memory() / bytes_to_mib_divisor; // Divides by a big number to
    let memory_used: u64 = sys.used_memory() / bytes_to_mib_divisor;   // convert from whatever to mebibytes


    // Output
    println!("Memory: {memory_used}/{memory_total}");
}
