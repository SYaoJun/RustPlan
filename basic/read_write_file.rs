use std::io::Write;
use std::io::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
fn main() -> io::Result<()> {
    let _ = read_file();
    // let _ = write_file();
    Ok(())
}
fn read_file()->io::Result<()>{
    let file = File::open("hello.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
fn write_file()->Result<(), Error>{
    let mut file = File::create("hello.txt")?;

    file.write_all(b"some bytes")?;
    Ok(())
}

