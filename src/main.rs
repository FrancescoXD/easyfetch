use colored::*;
use std::process::Command;
use std::{env, fs};

fn main() {
    // Shell
    let shell = match env::var("SHELL") {
        Ok(value) => value,
        Err(e) => panic!("{:?}", e),
    };

    let shell = shell.split('/').last().unwrap();

    // Kernel
    let kernel = fs::read_to_string("/proc/sys/kernel/osrelease")
        .expect("Unable to read /proc/sys/kernel/osrelease")
        .split_whitespace()
        .next()
        .expect("Unable to get kernel version.")
        .to_owned();

    // Uptime
    let uptime = fs::read_to_string("/proc/uptime").expect("Unable to read /proc/uptime.");
    let mut uptime = uptime.split(' ');

    let uptime = uptime.next().unwrap().parse::<f32>().unwrap_or(0.00);

    let mut hours = 0;
    let mut minutes = (uptime / 60.00) as i32;
    if minutes > 60 {
        hours = minutes / 60;
        minutes -= hours * 60;
    }

    // Memory
    let memory = fs::read_to_string("/proc/meminfo").expect("Unable to read /proc/meminfo.");
    let memory = memory.split_whitespace();
    let memory: Vec<&str> = memory.collect();

    let memtotal = memory
        .get(1)
        .unwrap()
        .parse::<f32>()
        .expect("Unable to get total ram.");
    let memfree = memory
        .get(4)
        .unwrap()
        .parse::<f32>()
        .expect("Unable to get free ram.");
    let buffers = memory
        .get(10)
        .unwrap()
        .parse::<f32>()
        .expect("Unable to get buffers.");
    let cached = memory
        .get(13)
        .unwrap()
        .parse::<f32>()
        .expect("Unable to get cached memory.");

    let totmemused = memtotal - memfree;
    let memused = totmemused - (buffers + cached);

    // Packages - Works only with arch
    let output = Command::new("/bin/bash")
        .args(&["-c", "pacman -Q | wc -l"])
        .output()
        .expect("Failed to execute pacman");

    let packages = String::from_utf8(output.stdout)
        .expect("Unable to get packages.")
        .split_whitespace()
        .next()
        .expect("Unable to get packages count.")
        .to_owned();

    // Disk
    let output = Command::new("/bin/bash")
        .args(&["-c", "df -h /"])
        .output()
        .expect("Failed to execute pacman");

    let disk = String::from_utf8(output.stdout).expect("Unable to get disk output.");
    let disk: Vec<&str> = disk.split_whitespace().collect();
    let diskused = disk.get(10).expect("Unable to get disk used.");
    let disktotal = disk.get(9).expect("Unable to get disk total.");

    // Print
    println!(
        "{:>8}   {} {} {} {} {}",
        "up".cyan().bold(),
        hours.to_string().bold(),
        "hours".bold(),
        "and".bold(),
        minutes.to_string().bold(),
        "minutes".bold()
    );
    println!("{:>8}   {}", "sh".blue().bold(), shell.bold());
    println!(
        "{:>9}  {} / {} MB",
        "ram".yellow().bold(),
        (memused / 1_000.00).round().to_string().bold(),
        (memtotal / 1000.00).round().to_string().bold()
    );
    println!("{:>10} {}", "pkgs".red().bold(), packages.bold());
    println!("{:>9}  {}", "ker".green().bold(), kernel.bold());
    println!(
        "{:>10} {} / {}",
        "disk".purple().bold(),
        diskused.bold(),
        disktotal.bold()
    );
}
