//-----------------------------------------------------------------------------
// main.rs — Friendly system info banner
//
// Joseph Abraham Semmel & Gemini.
//
// Description: Dynamic greeting, system info, and path display.
// Date:        February 12, 2026
// Version:     See Cargo.toml (Pulled via env!)
//-----------------------------------------------------------------------------

mod colors;
use colors::*;
use chrono::{Local, Timelike};
use std::env;

// --- CONFIGURATION (The "Global" Constants) ---
const VERSION: &str = env!("CARGO_PKG_VERSION");

const MORNING: std::ops::RangeInclusive<u32>   = 5..=11;
const AFTERNOON: std::ops::RangeInclusive<u32> = 12..=16;
const EVENING: std::ops::RangeInclusive<u32>   = 17..=21;

// --- FUNCTIONS (The Logic Blocks) ---

fn print_version() {
    println!("me version {VERSION}");
}

fn print_help() {
    let help_text = format!(
        r#"
me v{v}
Authored by Joseph Abraham Semmel & Gemini.

Usage: me [options]

Options:
  -v, --v, --version    Show version info
  -c, --colors          Test the current color palette
  ?, -h, --help         Show this help message
        "#,
        v = VERSION
    );
    println!("{}", help_text.trim());
}

fn print_colors() {
    println!("--- Current Palette (colors.rs) ---");
    println!("{g}Greeting: {COLOR_GREETING}COLOR_GREETING{r}", g=COLOR_GREETING, r=RESET);
    println!("{n}Name:     {COLOR_NAME}COLOR_NAME{r}", n=COLOR_NAME, r=RESET);
    println!("{h}Host:     {COLOR_HOST}COLOR_HOST{r}", h=COLOR_HOST, r=RESET);
    println!("{i}IP:       {COLOR_IP}COLOR_IP{r}", i=COLOR_IP, r=RESET);
    println!("{p}Path:     {COLOR_PATH}COLOR_PATH{r}", p=COLOR_PATH, r=RESET);
}

fn print_banner() {
    // 1. Determine Greeting
    let hour = Local::now().hour();
    let greeting = if MORNING.contains(&hour) { "Good morning" } 
                   else if AFTERNOON.contains(&hour) { "Good afternoon" } 
                   else if EVENING.contains(&hour) { "Good evening" } 
                   else { "Good night" };

    // 2. Gather System Data (Result handling for 2024 edition crates)
    let user = whoami::username().unwrap_or_else(|_| "User".to_string());
    let host = whoami::devicename().unwrap_or_else(|_| "Host".to_string());
    let ip   = local_ipaddress::get().unwrap_or_else(|| "127.0.0.1".to_string());
    
    let path = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| ".".to_string());

    // 3. Display the banner
    println!(
        "{g}{greeting},{r} {n}{user}{r} on {h}{host}{r} at {i}{ip}{r}",
        g = COLOR_GREETING, n = COLOR_NAME, h = COLOR_HOST, i = COLOR_IP, r = RESET,
    );
    println!("{p}{path}{r}", p = COLOR_PATH, r = RESET);
}

// --- ENTRY POINT (The Switchboard) ---

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-v" | "--v" | "-version" | "--version" => print_version(),
            "?"  | "-h"  | "--help"    | "/?"       => print_help(),
            "-c" | "--colors"                       => print_colors(),
            _                                       => print_banner(),
        }
    } else {
        print_banner();
    }
}