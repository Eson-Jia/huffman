
use std::fs::File;
use std::env;
use std::io::prelude::*;
fn main() -> std::io::Result<()> {
    let mut param=env::args();
    if param.len() != 1{
        Err(std::io::Error::new(std::io::ErrorKind::Other,"usage:bintool inputfile"))
    }
    else{
        let _=param.next();
        let inputfile=param.next();
        let outputfile=param.next();
        let mut _inputfile = File::open("1.jpg")?;
        let mut _outputfile= File::create("2.jpg")?;
        let mut buffer:[u8;16]=[0;16];
        while let std::io::Result::Ok(len) = _inputfile.read(&mut buffer){
            //println!("{}",len);
            if len == 0 {
                break;
            }
            else{
                _outputfile.write(&mut buffer[..len]);
            }
        }
        Ok(())
    }
}