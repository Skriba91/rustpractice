use std::io;
use std::io::prelude::*;
//use std::fs::File;
use std::fs::OpenOptions;

fn main() -> io::Result<()> {
    let mut f = OpenOptions::new().read(true).write(true).open("foo.txt")?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer)?;
    println!("{:?}", buffer);
    f.write_all(b"Szia Vilag")?;
    Ok(())
}