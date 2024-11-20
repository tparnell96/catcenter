use crate::api::Device;
use chrono::Utc;
use prettytable::{row, Table};

pub fn current_timestamp() -> u64 {
    Utc::now().timestamp() as u64
}

pub fn print_devices(devices: Vec<Device>) {
    let mut table = Table::new();
    table.add_row(row![
        "Hostname",
        "IP Address",
        "Serial Number",
        "Software Version"
    ]);

    for device in devices {
        table.add_row(row![
            device.hostname.unwrap_or_else(|| "N/A".to_string()),
            device
                .managementIpAddress
                .unwrap_or_else(|| "N/A".to_string()),
            device.serialNumber.unwrap_or_else(|| "N/A".to_string()),
            device.softwareVersion.unwrap_or_else(|| "N/A".to_string()),
        ]);
    }

    table.printstd();
}
