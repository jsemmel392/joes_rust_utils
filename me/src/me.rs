//-----------------------------------------------------------------------------
// me.rs — Friendly system info banner
//
// Joseph Abraham Semmel, Gemini & Copilot.
//
// Description: Dynamic greeting, local/public IP, and path display.
// Date:        February 13, 2026
// Version:     See Cargo.toml
//-----------------------------------------------------------------------------

mod colors;
use colors::*;
use chrono::{Local, Timelike};
use std::env;
use std::time::Duration;

// --- CONFIGURATION ---
const VERSION: &str = env!("CARGO_PKG_VERSION");

// --- FUNCTIONS ---

fn get_public_ip() -> String {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(2))
        .build();

    match client {
        Ok(c) => {
            match c.get("https://icanhazip.com").send() {
                Ok(res) => res.text().unwrap_or_else(|_| "Unknown".into()).trim().to_string(),
                Err(_) => "Offline".to_string(),
            }
        }
        Err(_) => "Error".to_string(),
    }
}

fn print_version() {
    println!("me version {VERSION}");
    println!("Created by Joseph Abraham Semmel, Gemini & Copilot.");
}

fn print_help() {
    let help_text = format!(
        r#"
me v{v}
Authored by Joseph Abraham Semmel, Gemini & Copilot.

Usage: me [options]

Description:
  A personalized system banner showing your identity, 
  network status, and current location.

Options:
  -v, --v, --version    Show version info and credits
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
    // 1. Time-based Greeting (Morning starts at 5 AM)
    let hour = Local::now().hour();
    let greeting = if (5..=11).contains(&hour)      { "Good morning" } 
                   else if (12..=16).contains(&hour) { "Good afternoon" } 
                   else if (17..=21).contains(&hour) { "Good evening" } 
                   else { "Good night" };

    // 2. Identity & Networking
    let user = whoami::username();
    let host = whoami::devicename();
    let local_ip = local_ipaddress::get().unwrap_or_else(|| "127.0.0.1".to_string());
    let public_ip = get_public_ip();
    
    let path = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| ".".to_string());

    // 3. Display
    println!(
        "{g}{greeting},{r} {n}{user}{r} on {h}{host}{r}",
        g = COLOR_GREETING, n = COLOR_NAME, h = COLOR_HOST, r = RESET,
    );
    
    println!(
        "Local: {i}{local}{r} | Public: {i}{public}{r}",
        i = COLOR_IP, local = local_ip, public = public_ip, r = RESET
    );

    println!("{p}{path}{r}", p = COLOR_PATH, r = RESET);
}

// --- ENTRY POINT ---

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