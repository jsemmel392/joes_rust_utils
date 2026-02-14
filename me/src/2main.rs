mod colors;
use colors::*;
use chrono::{Local, Timelike};
use std::env;

fn main() {
    print_banner();
}

fn print_banner() {
    // 1. Determine Greeting based on system time
    let hour = Local::now().hour();
    let greeting = match hour {
        5..=11 => "Good morning",
        12..=16 => "Good afternoon",
        17..=21 => "Good evening",
        _ => "Good night",
    };

    // 2. Fetch system info
    let user = env::var("USERNAME").unwrap_or_else(|_| "User".to_string());
    let host = env::var("COMPUTERNAME").unwrap_or_else(|_| "Host".to_string());
    
    // 3. Get the actual LAN IP
    let ip = local_ipaddress::get().unwrap_or_else(|| "127.0.0.1".to_string());

    // 4. Get Current Working Directory
    let path = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    // Print the dynamic banner
    println!(
        "{g}{greeting},{r} {n}{user}{r} on {h}{host}{r} at {i}{ip}{r}",
        g = COLOR_GREETING,
        n = COLOR_NAME,
        h = COLOR_HOST,
        i = COLOR_IP,
        r = RESET,
        greeting = greeting,
        user = user,
        host = host,
        ip = ip
    );

    println!("{p}{path}{r}", p = COLOR_PATH, r = RESET);
}