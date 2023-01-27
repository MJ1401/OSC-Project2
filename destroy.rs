use std::{fs, env::args};

fn main() -> std::io::Result<()> {
    for arg in args().skip(1) {
        fs::remove_file(arg)?;
    }
    Ok(())
}