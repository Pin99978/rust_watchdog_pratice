use sysinfo::System;

fn main() {
    let mut sys = System::new_all();

    sys.refresh_memory();
    println!("Total memory: {}", sys.total_memory());
    println!("Used memory: {}", sys.used_memory());

    for cpu in sys.cpus() {
        println!("{}", cpu.name());
        println!("{}", cpu.cpu_usage());
    }
}
