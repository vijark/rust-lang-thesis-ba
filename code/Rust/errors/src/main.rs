use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", read_username1()?);
    println!("{}", read_username2()?);

    Ok(())
}

fn read_username1() -> Result<String, io::Error> {
    let f = File::open("username.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("username.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
