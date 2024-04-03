use std::io::IsTerminal;

fn main() {
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
