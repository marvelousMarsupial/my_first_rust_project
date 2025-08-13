use std::fs::{File};
use std::io::{self, Read, Write}; 

pub fn write_int_to_file(path: &str, content: i32) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&content.to_le_bytes())?;
    Ok(())
} 

pub fn read_int_from_file(path: &str) -> io::Result<i32> {
    let mut file = File::open(path)?;
     let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer)?;
   
    let content= i32::from_le_bytes(buffer);
    
    Ok( content )
}