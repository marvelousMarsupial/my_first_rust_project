mod storage;
mod parser;
mod executor;
mod types;

fn main() -> std::io::Result<()> {
    storage::write_int_to_file("example.txt", 1)?;
    let num = storage::read_int_from_file("example.txt").unwrap();
    println!("We just read this from a file: {:?}!!!", num);
    Ok(())
}