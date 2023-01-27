fn main() -> std::io::Result<()> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    if arguments.len() != 2 {
        println!("Usage: newname old_name new_name")
    } else {
        std::fs::rename(arguments[0].as_str(), arguments[1].as_str())?;
    }
    Ok(())
}