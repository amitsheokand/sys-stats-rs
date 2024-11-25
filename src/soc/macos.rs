use crate::soc::{Soc, SocCollection};
use crate::SocDetails;
use serde_json::Error;
use std::process::Command;

impl SocDetails {
    /// Get the SOC details by name
    /// TODO: when multiple SOCs are available, match more details or ID etc
    pub fn get_soc_info_by_name(name: &str) -> Soc {
        // parse the soc.json file and return the Soc struct

        let s = include_str!("db/apple/soc.json");
        let soc_details: Result<SocCollection, Error> = serde_json::from_str(s);

        match soc_details {
            Ok(soc) => {
                for s in soc.0 {
                    if s.name == Some(name.to_string()) {
                        return s;
                    }
                }
                Soc::new(None, None, None, 0, None, None, None, None, 0)
            }
            Err(_) => Soc::new(None, None, None, 0, None, None, None, None, 0),
        }
    }

    pub fn get_current_soc_info() -> Soc {
        // parse the soc.json file and return the Soc struct

        let s = include_str!("db/apple/soc.json");
        let soc_details: Result<SocCollection, Error> = serde_json::from_str(s);

        let (name, cc) = Self::get_name_and_core_count();

        match soc_details {
            Ok(soc) => {
                for s in &soc.0 {
                    println!("s: {:#?}", s);
                    if let (Some(s_name), s_cc) = (s.name.as_ref(), s.num_of_cores()) {
                        // println!("s_name: {}, s_cc: {}", s_name, s_cc);
                        // println!("name: {}, cc: {}", name, cc);
                        if s_name == &name && s_cc == cc {
                            return s.clone();
                        }
                    }
                }
                Soc::new(None, None, None, 0, None, None, None, None, 0)
            }
            Err(_) => Soc::new(None, None, None, 0, None, None, None, None, 0),
        }
    }

    fn get_name_and_core_count() -> (String, u32) {
        let output = Command::new("sh")
            .arg("-c")
            .arg("sysctl -n machdep.cpu.brand_string")
            .output()
            .expect("Failed to execute command");

        let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let core_count = sys_info::cpu_num().unwrap();

        (name.to_string(), core_count)
    }
}