use sysinfo::{System, SystemExt, CpuExt};
use std::{thread, time};

fn main() {
    let mut sys = System::new_all();
    
    println!("Starting High-Performance System Monitor...");
    println!("OS: {:?}", sys.name());
    
    loop {
        sys.refresh_all();
        
        for (i, cpu) in sys.cpus().iter().enumerate() {
            println!("CPU {}: {:.2}% Load", i, cpu.cpu_usage());
        }
        
        let total_mem = sys.total_memory();
        let used_mem = sys.used_memory();
        println!("Memory: {}/{} KB used", used_mem, total_mem);
        
        // Sleep for 2 seconds before next poll
        thread::sleep(time::Duration::from_secs(2));
    }
}
