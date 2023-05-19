use oomfi::*;
use std::time::{Instant};

fn main() {
    const N: u32 = 500_000;
    let mut set = Bloom::new(500_000, 0.01);
    //let mut set = Bloom::new(N as usize, 0.01);

    println!("Using {} hash functions!", set.hash_functions());
    println!("Using {} bytes!", set.number_of_bits() * 8);
 
    let start = Instant::now();
    
    for i in 0..N as i32 {
        set.insert(i);
    }
    
    println!("Time to insert {} elements = {:?}", N, start.elapsed());
}
