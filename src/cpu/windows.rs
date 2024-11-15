use crate::cpu::{CPUData, CPUUsage};
use anyhow::Result;
use winapi::um::sysinfoapi::GetSystemInfo;
use winapi::um::sysinfoapi::SYSTEM_INFO;

impl CPUUsage {
    pub fn get_cpu_info() -> Result<CPUData, String> {
        let mut sys_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
        unsafe {
            GetSystemInfo(&mut sys_info);
        }

        let mut cpu_data = CPUData {
            name: Self::get_cpu_name(),
            architecture: Self::get_cpu_architecture(),
            num_of_cores: 0,
            average_cpu_usage: 0.0,
        };
        cpu_data.num_of_cores = Self::num_of_cores();
        cpu_data.average_cpu_usage = Self::average_usage();
        Ok(cpu_data)
    }

    pub fn num_of_cores() -> u32 {
        // let mut sys_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
        // unsafe {
        //     GetSystemInfo(&mut sys_info);
        // }
        // sys_info.dwNumberOfProcessors

        sys_info::cpu_num().unwrap() as u32
    }

    pub fn average_usage() -> f32 {
        sys_info::loadavg().unwrap().one as f32
    }

    fn get_cpu_name() -> String {
        // use wmic
        let output = std::process::Command::new("wmic")
            .args(&["cpu", "get", "name"])
            .output()
            .expect("failed to execute process");

        let output = String::from_utf8_lossy(&output.stdout);
        let output = output.split("\n").collect::<Vec<&str>>();
        let output = output[1].trim();
        output.to_string()
    }

    fn get_cpu_architecture() -> String {
        // using winapi to get the architecture
        let mut sys_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
        unsafe {
            GetSystemInfo(&mut sys_info);
        }
        match unsafe { sys_info.u.s().wProcessorArchitecture } {
            0 => "x86".to_string(),
            5 => "ARM".to_string(),
            9 => "x64".to_string(),
            12 => "ARM64".to_string(),
            14 => "RISC-V".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}
