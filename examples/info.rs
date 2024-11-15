use sys_stats::CPUStats;
use sys_stats::GPUStats;
use sys_stats::MemoryStats;

fn main() {
    let gpu = GPUStats::get_gpu_info().unwrap();
    println!("----------------------");
    println!("{:#?}", gpu);
    println!("----------------------");

    let memory_stats = MemoryStats::get_system_memory_info().unwrap();
    println!("----------------------");
    println!("{:#?}", memory_stats);
    println!("----------------------");

    let cpu_stats = CPUStats::get_cpu_info().unwrap();
    println!("----------------------");
    println!("{:#?}", cpu_stats);
    println!("----------------------");
}
