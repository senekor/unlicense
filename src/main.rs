use std::io::IsTerminal;

static USAGE: &str = "\
Access the text of the Unlicense. <https://unlicense.org>

When used interactively, writes to ./UNLICENSE.
Otherwise (e.g. in a pipe), writes to stdout.

When given any arguments, displays this help page.
";

fn main() {
    if std::env::args().count() > 1 {
        print!("{}", USAGE);
        std::process::exit(0);
    }

    if !std::io::stdout().is_terminal() {
        print!("{}", unlicense::TEXT);
        std::process::exit(0);
    }

    let path = std::path::PathBuf::from("UNLICENSE");
    if path.exists() {
        eprintln!("ERROR: The file UNLICENSE already exists.");
        std::process::exit(1);
    } else {
        std::fs::write(path, unlicense::TEXT).expect("should write to file");
    }
}
