use std::{env, fs};
use colored::*;
use std::process::Command;

fn main() {
    // Shell
    let key = "SHELL";

    let shell = match env::var(key) {
        Ok(value) => value,
        Err(e) => panic!("{:?}", e),
    };

    let shell = shell.split("/").last().unwrap();

    // Kernel
    let kernel = fs::read_to_string("/proc/sys/kernel/osrelease").unwrap();

    // Uptime
    let uptime = fs::read_to_string("/proc/uptime").unwrap();
    let mut uptime = uptime.split(" ");

    let uptime = match uptime.nth(0).unwrap().parse::<f32>() {
        Ok(value) => value,
        Err(_) => 0.00,
    };

    let uptime = (uptime / 60.00) as i32;

    // Memory
    let memory = fs::read_to_string("/proc/meminfo").unwrap();
    let memory = memory.split_whitespace();
    let memory: Vec<&str> = memory.collect();

    let memtotal = memory.get(1).unwrap().parse::<f32>().expect("Unable to get total ram.");
    let memfree = memory.get(4).unwrap().parse::<f32>().expect("Unable to get free ram.");
    let buffers = memory.get(10).unwrap().parse::<f32>().expect("Unable to get buffers.");
    let cached = memory.get(13).unwrap().parse::<f32>().expect("Unable to get cached memory.");

    let totmemused = memtotal - memfree;
    let memused = totmemused - (buffers + cached);

    // Packages - needs to be fixed
    let output = Command::new("/bin/bash")
        .args(&["-c", "pacman -Q | wc -l"])
        .output()
        .expect("Failed to execute pacman");
    //println!("{:?}", output);

    // Print
    println!("{:>9}  {} {}", "up".cyan().bold(), uptime.to_string().bold(), "minutes".bold());
    println!("{:>9}  {}", "sh".blue().bold(), shell.bold());
    println!("{:>10} {:.1} / {:.1} GB", "ram".yellow().bold(), (memused / 1_000_000.00).to_string().bold(), (memtotal / 1_000_000.00).to_string().bold());
    println!("{:>10} {}", "ker".green().bold(), kernel.bold());
}
