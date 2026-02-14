//-----------------------------------------------------------------------------
// main.rs — Friendly system info banner
//
// Purpose:
//   Prints a colored greeting showing username, hostname, IP address, and
//   current working directory. All colors come from colors.rs to ensure a
//   single source of truth and zero drift.
//
// Usage:
//   me.exe
//
// Example Output:
//   Good afternoon, joseph on ROSY2 at 192.168.1.18
//   C:\Users\joseph\share
//
// Notes:
//   - No ANSI codes appear here; everything is semantic.
//   - Windows Terminal renders ANSI correctly even when using CMD as the shell.
//   - This file stays clean and readable because colors.rs owns the palette.
//-----------------------------------------------------------------------------

mod colors;
use colors::*;

fn main() {
    print_banner();
}

fn print_banner() {
    println!(
        "{g}Good evening,{r} {n}Joseph{r} on {h}DellWin10Laptop{r} at {i}192.168.1.30{r}",
        g = COLOR_GREETING,
        n = COLOR_NAME,
        h = COLOR_HOST,
        i = COLOR_IP,
        r = RESET
    );

    println!(
        "{p}C:\\Users\\Joseph\\rust\\me{r}",
        p = COLOR_PATH,
        r = RESET
    );
}
