//-----------------------------------------------------------------------------
// ts.rs — Collaborative Timestamp Utility
//
// Joseph Abraham Semmel, Gemini & Copilot.
//
// Description: Quick timestamp for AI context and logging.
// Date:        February 13, 2026
// Version:     See Cargo.toml
//-----------------------------------------------------------------------------

mod colors;
use colors::*;
use chrono::Local;
use arboard::Clipboard;
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn handle_timestamp(copy: bool) {
    let now = Local::now();
    let timestamp = now.format("%A, %B %d, %Y at %I:%M:%S %p").to_string();
    
    // Uses the semantic color you just updated in colors.rs
    println!("{i}{ts}{r}", i=COLOR_IP, ts=timestamp, r=RESET);

    if copy {
        if let Ok(mut clipboard) = Clipboard::new() {
            if clipboard.set_text(&timestamp).is_ok() {
                // Using the semantic path color for the confirmation
                println!("{p}(Copied to clipboard!){r}", p=COLOR_PATH, r=RESET);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-v" | "--v" | "-version" | "--version" => {
                println!("ts version {VERSION}\nCreated by Joseph Abraham Semmel, Gemini & Copilot.");
            },
            "-c" | "--colors" => {
                println!("{h}HOST{r} {p}PATH{r} {i}IP{r} {g}GREETING{r}", 
                    h=COLOR_HOST, p=COLOR_PATH, i=COLOR_IP, g=COLOR_GREETING, r=RESET);
            },
            "-cp" | "--copy" | "-copy" => handle_timestamp(true),
            _ => handle_timestamp(false),
        }
    } else {
        handle_timestamp(false);
    }
}