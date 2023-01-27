fn main() -> std::io::Result<()> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    if arguments.len() != 2 {
        println!("Usage: duplicate file_wanting_to_copy new_name_for_copied_file")
    } else {
        std::fs::copy(arguments[0].as_str(), arguments[1].as_str())?;
    }
    Ok(())
}