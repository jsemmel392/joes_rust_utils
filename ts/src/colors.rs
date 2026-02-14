//-----------------------------------------------------------------------------
// colors.rs — Shared ANSI color palette
//
// Purpose:
//   Provides a single source of truth for all ANSI color codes used across
//   your Rust tools. Base colors define the actual escape sequences.
//   Semantic colors map UI meaning (greeting, host, path, etc.) to base colors.
//
// Usage:
//   use colors::*;
//   println!("{COLOR_GREETING}Hello{RESET}");
//
// Notes:
//   - No hard-coded ANSI values should appear outside this file.
//   - Changing a base color updates all semantic colors automatically.
//   - Designed for Windows Terminal and Linux terminals with ANSI support.
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// BASE COLORS (raw ANSI escape codes)
//-----------------------------------------------------------------------------
pub const RESET:  & str = "\x1b[0m"; // reset all attributes

pub const ORANGE:  & str = "\x1b[38;5;214m"; // warm amber (alerts, hostnames)
pub const GREEN:  & str = "\x1b[38;5;82m"; // bright green (success, names)
pub const CYAN:  & str = "\x1b[38;5;51m"; // cyan (IP addresses, values)	
pub const GRAY:  & str = "\x1b[38;5;245m"; // soft gray (paths, labels)
pub const DIM:  & str = "\x1b[38;5;240m"; // dark gray (secondary text)
pub const BLUE: &str = "\x1b[38;5;39m"; // hostname (distinct blue)
pub const MAGENTA: &str = "\x1b[38;5;201m"; // new (free slot)
pub const AQUA: &str = "\x1b[38;5;86m";   // green‑teal, not blue
pub const YELLOW: &str = "\x1b[38;5;226m";
pub const GOLD_SOFT: &str = "\x1b[38;5;222m"; // soft yellow-gold (between yellow & amber)
pub const PURPLE_DARK: &str = "\x1b[38;5;90m"; // dark purple, very subdued
pub const RED: &str = "\x1b[38;5;196m"; // bright red
pub const RED_DARK: &str = "\x1b[38;5;160m"; // deep red
pub const PURPLE_SOFT: &str = "\x1b[38;5;97m"; // soft violet, no clash with green


//-----------------------------------------------------------------------------
// SEMANTIC COLORS (meaning-based, no raw codes here)
//-----------------------------------------------------------------------------
pub const COLOR_GREETING:  & str = ORANGE; // greeting text
pub const COLOR_NAME:  & str = ORANGE; // user's name
pub const COLOR_HOST:  & str = GRAY; // hostname / machine name
//pub const COLOR_IP:  & str = GRAY; // IP address
pub const COLOR_PATH:  & str = GREEN; // filesystem path

// Change COLOR_IP from GRAY to CYAN (or YELLOW)
pub const COLOR_IP: &str = CYAN; 

// If you want the success message to be AQUA instead of GREEN
pub const COLOR_SUCCESS: &str = AQUA;



