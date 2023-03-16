
// /dev/random - not on Windows
// /dev/urandom - not on Windows
// On Intel chips only: RDRAND assembly instructions - I shall think about this one later

// Implement additional functions 
// generating not only u8 random numbers but also u16, u32, and u64 together with usize, 

//Add support for Windows too

use std::env::consts::OS;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub enum ErrorRandom { 
    IOError(std::io::Error), 
    OSError(String), 
}

impl From<std::io::Error> for ErrorRandom {
    fn from(error: std::io::Error) -> Self {
        ErrorRandom::IOError(error)
    }
}

pub struct Random {
    random: u8,
}

impl Random {
    pub fn new() -> Result<Self, ErrorRandom> {
        match OS {
            "windows" => Err(ErrorRandom::OSError("Not implemented for Windows".to_string())), 
            "android" => Err(ErrorRandom::OSError("Not implemented for Android".to_string())),
            _         => Ok(Self {random: Self::urandom()?}),
        }
        
    }

    fn urandom() -> Result<u8, ErrorRandom> {
        let mut random_file = File::open("/dev/urandom")?;
        let mut buffer: [u8; 1] = [0];  
        random_file.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    pub fn random(self: &Self) -> u8 {
        self.random
    }
}

fn main() -> Result<(), ErrorRandom> { 

    let random = Random::new()?; 
    println!("Random number: {}", &random.random());
    Ok(())
}
