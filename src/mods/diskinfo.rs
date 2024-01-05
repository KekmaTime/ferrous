use sysinfo::{DiskExt, System, SystemExt};
use colored::*;

use crate::dirsize;

pub fn print_disk_info() {
    let mut system = System::new_all();
    system.refresh_all();

    for disk in system.get_disks() {
        let total_space = dirsize::bytes_to_human_readable(disk.get_total_space());
        let available_space = dirsize::bytes_to_human_readable(disk.get_available_space());

        println!("Disk: {:?}", disk.get_name());
        println!("Total space: {}", total_space.green());
        println!("Available space: {}", available_space.red());
    }
}