
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
    fn new() -> Result<Self, ErrorRandom> {
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

    fn random(self: &Self) -> u8 {
        self.random
    }

    pub fn new_u8() -> Result<u8, ErrorRandom> {
        Ok(Self::new()?.random())
    }

    pub fn new_u16() -> Result<u16, ErrorRandom> {
        let random_1 = Self::new()?.random() as u16; 
        let random_2 = Self::new()?.random() as u16;
        let random_u16 = random_1 << 8 | random_2;
        Ok(random_u16)
    }

    pub fn new_u32() -> Result<u32, ErrorRandom> {
        let random_1 = Self::new_u16()? as u32; 
        let random_2 = Self::new_u16 as u32;
        let random_u32 = random_1 << 16 | random_2;
        Ok(random_u32)
    }

    pub fn new_u64() -> Result<u64, ErrorRandom> {
        match std::mem::size_of::<&char>() {
            8         => {
                            let random_1 = Self::new_u32()? as u64; 
                            let random_2 = Self::new_u32 as u64;
                            let random_u64 = random_1 << 32 | random_2;
                            Ok(random_u64)
                        },
            4 => Err(ErrorRandom::OSError("Not implemented for 32-bit architecture".to_string())), 
            _ => Err(ErrorRandom::OSError("Not implemented for this architecture".to_string())),
        }
    }

    pub fn new_usize() -> Result<usize, ErrorRandom> {
        match std::mem::size_of::<&char>() {
            8 => Ok(Self::new_u64()? as usize),
            4 => Ok(Self::new_u32()? as usize), 
            _ => Err(ErrorRandom::OSError("Not implemented for this architecture".to_string())),
        }
    }
}

fn main() -> Result<(), ErrorRandom> { 

    let random = Random::new_usize()?; 
    println!("Random number: {:x}", &random);

    Ok(())
}
