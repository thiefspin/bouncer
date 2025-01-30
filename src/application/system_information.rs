use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::Deserialize;
use std::string::ToString;
use sysinfo::System;

#[derive(Clone, Deserialize, Serialize, JsonSchema, Debug)]
#[allow(non_snake_case)]
#[serde(crate = "rocket::serde")]
pub struct SystemOs {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, Debug)]
#[allow(non_snake_case)]
#[serde(crate = "rocket::serde")]
pub struct SystemMemory {
    pub total: u64,
    pub used: u64,
    pub swap: u64,
    pub used_swap: u64,
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, Debug)]
#[allow(non_snake_case)]
#[serde(crate = "rocket::serde")]
pub struct SystemInformation {
    pub system: SystemOs,
    pub memory: SystemMemory,
    pub cpus: Vec<String>,
}

impl SystemInformation {
    //https://docs.rs/sysinfo/latest/sysinfo/
    pub fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let unknown = "Unknown".to_string();
        let cpu_usages = sys
            .cpus()
            .iter()
            .map(|c| format!("{}%", c.cpu_usage()))
            .collect();

        SystemInformation {
            system: SystemOs {
                name: System::name().unwrap_or(unknown.clone()),
                kernel_version: System::kernel_version().unwrap_or(unknown.clone()),
                os_version: System::os_version().unwrap_or(unknown.clone()),
                host_name: System::host_name().unwrap_or(unknown.clone()),
            },
            memory: SystemMemory {
                total: bytes_to_gigabytes(sys.total_memory()),
                used: bytes_to_gigabytes(sys.used_memory()),
                swap: bytes_to_gigabytes(sys.total_swap()),
                used_swap: bytes_to_gigabytes(sys.used_swap()),
            },
            cpus: cpu_usages,
        }
    }
}

fn bytes_to_gigabytes(bytes: u64) -> u64 {
    return bytes / (1024 * 1024 * 1024);
}
