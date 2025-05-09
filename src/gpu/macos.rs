use crate::gpu::{DriverVersionData, GPUData, GPUUsage};
use objc2::msg_send;
use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice};
use os_version::OsVersion;

// Import CoreGraphics as it's required for Metal on Intel macs
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

impl GPUUsage {
    pub fn get_gpu_info() -> Result<GPUData, Box<dyn std::error::Error>> {
        let mut result: GPUData = GPUData {
            name: "".to_string(),
            architecture: "".to_string(),
            vendor_id: 0,
            total_memory: 0,
            free_memory: 0,
            used_memory: 0,
            has_unified_memory: false,
            is_integrated:false,
            adapter_index: 0,
            driver_version: DriverVersionData {
                major: 0,
                minor: 0,
                build: 0,
                revision: 0,
            },
        };

        unsafe {
            let mtl_device = { MTLCreateSystemDefaultDevice() };
            let mtl_device = match mtl_device.as_ref() {
                Some(device) => device,
                None => return Err("Failed to get MTLDevice".into()),
            };

            result.name = mtl_device.name().to_string();

            let mut mac_version: u8 = 0;

            if let Ok(OsVersion::MacOS(macos)) = os_version::detect() {
                if let Some(major_version) = macos.version.split('.').next() {
                    if let Ok(major_version_num) = major_version.parse::<u8>() {
                        mac_version = major_version_num;
                    }
                }
            }

            if mac_version >= 14 {
                if std::env::consts::ARCH == "aarch64" {
                    result.architecture = mtl_device.architecture().name().to_string();
                } else {
                    result.architecture = "Intel".to_string()
                }
            } else {
                result.architecture = "Unknown".to_string()
            };

            if std::env::consts::ARCH == "aarch64" {
                result.vendor_id = 4203;
                result.has_unified_memory = true;
                result.is_integrated = true;
            }
            

            // handling memory calculations separately, because apple does not provide a direct way to get the free/used gpu memory
            result.total_memory = Self::total_gpu_memory()?;
            result.used_memory = Self::current_gpu_memory_usage()?;
            result.free_memory = Self::current_gpu_memory_free()?;

            result.has_unified_memory = Self::has_unified_memory().unwrap()
        }
        Ok(result)
    }

    pub fn get_gpus_list() -> Result<Vec<GPUData>, Box<dyn std::error::Error>> {
        let mut results: Vec<GPUData> = Vec::new();
        results.push(GPUUsage::get_gpu_info()?);
        Ok(results)
    }

    pub fn total_gpu_memory() -> Result<u64, Box<dyn std::error::Error>> {
        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();

            let mtl_device = match mtl_device.as_ref() {
                Some(device) => device,
                None => return Err("Failed to get MTLDevice".into()),
            };

            let recommended_max_working_set_size: u64 =
                msg_send![mtl_device, recommendedMaxWorkingSetSize];

            if recommended_max_working_set_size == 0 {
                return Err("Failed to get total GPU memory".into());
            }

            Ok(recommended_max_working_set_size)
        }
    }

    pub fn current_gpu_memory_usage() -> Result<u64, Box<dyn std::error::Error>> {
        // this approach is not accurate, but it's the only way to get the current allocated size
        // as apple does not provide a way to get the free/used gpu memory
        // rough estimate of the current used memory

        let total = Self::total_gpu_memory()?;
        let free = Self::current_gpu_memory_free()?;

        if total < free {
            eprintln!("Free can not be more than total");
            return Ok(0);
        }

        Ok(total - free)
    }

    pub fn current_gpu_memory_free() -> Result<u64, Box<dyn std::error::Error>> {
        let free_memory: u64;

        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();

            let mtl_device = match mtl_device.as_ref() {
                Some(device) => device,
                None => return Err("Failed to get MTLDevice".into()),
            };

            let is_unified: bool = Self::has_unified_memory()?;

            // If the memory is unified, we can use the CPU memory to get the free memory
            // also apple does not provide a way to get the free/used gpu memory
            if is_unified {
                free_memory = Self::current_cpu_memory_free()?; // convert to bytes
            } else {
                let total_memory = mtl_device.recommendedMaxWorkingSetSize();
                let used_memory = mtl_device.currentAllocatedSize() as u64;
                free_memory = total_memory - (used_memory);
            }
        }
        Ok(free_memory)
    }

    pub fn has_unified_memory() -> Result<bool, Box<dyn std::error::Error>> {
        unsafe {
            if std::env::consts::ARCH == "aarch64" {
                let mtl_device = MTLCreateSystemDefaultDevice();

                let mtl_device = match mtl_device.as_ref() {
                    Some(device) => device,
                    None => return Err("Failed to get MTLDevice".into()),
                };

                let is_unified: bool = msg_send![mtl_device, hasUnifiedMemory];
                Ok(is_unified)
            } else {
                Ok(false)
            }
        }
    }

    fn current_cpu_memory_free() -> Result<u64, Box<dyn std::error::Error>> {
        let mem_info = sys_info::mem_info()?;
        Ok(mem_info.free * 1024) // convert to bytes
    }
}
