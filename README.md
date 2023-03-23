## lowrandom - library to generate random numbers; Rust implementation; no 3rd party dependencies, uses only standard library.

lowrandom library reads "/dev/urandom" file, hence its functionality is limited only to Linux and UNIX operating systems.  

### Functionality: 
1. u8 random number generation: pub fn new_u8() -> Result<u8, ErrorRandom>; 
1. u16 random number generation: pub fn new_u16() -> Result<u16, ErrorRandom>;   
1. u32 random number generation: pub fn new_u32() -> Result<u32, ErrorRandom>;
1. u64 random number generation - 64-bit OS only: pub fn new_u64() -> Result<u64, ErrorRandom>;
1. usize random number - generates u64 for the 64-bit OS or generates u32 for the 32-bit OS: pub fn new_usize() -> Result<usize, ErrorRandom>.

### How to use this library: 

1. Add to Cargo.toml file: 

```Toml
    [dependencies]
    lowrandom = {git = "https://github.com/azavgo/lowrandom", branch = "main"}
```
2. Generates a u32 random number and prints it to the terminal:  
```Rust
use lowrandom::*;

fn main() -> Result<(), ErrorRandom> { 

    let random_u32 = Random::new_u32()?; 
    println!("Random u32 number: {}", &random_u32);

    Ok(())
}
