//! Demonstrates copying a string to clipboard
//!
//! This example uses OSC control sequence `Pr = 5 2` (See
//! https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands)
//! to copy data to the terminal host clipboard.
//!
//! This only works if it is enabled on the respective terminal emulator. If a terminal multiplexer
//! is used, the multiplexer will likely need to support it, too.
//!
//! cargo run --example copy -- --clipboard "Some String"
//!
//! cargo run --example copy -- --primary "Some String"
//!
//! cargo run --example copy -- "Some String"

use std::io;

use crossterm::execute;
use crossterm::terminal::{ClipboardDestination, CopyToClipboard};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut args = std::env::args();
    args.next(); // Skip to first argument

    let default_text = String::from("Example text");
    let (text, dest) = match args.next().as_deref() {
        Some("--clipboard") => (
            args.next().unwrap_or(default_text),
            ClipboardDestination::Clipboard,
        ),
        Some("--primary") => (
            args.next().unwrap_or(default_text),
            ClipboardDestination::Primary,
        ),
        Some(text) => (text.to_owned(), ClipboardDestination::Clipboard),
        None => (default_text, ClipboardDestination::Clipboard),
    };
    execute!(stdout, CopyToClipboard(text, dest))
}
