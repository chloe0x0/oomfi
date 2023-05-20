use oomfi::*;
use std::time::Instant;

fn main() {
    const N: u32 = 500_000;
    let mut set = Bloom::new(10, 0.01);
    let mut set2 = Bloom::new(10 , 0.01);

    let elements = [0, 1, 2, 3];
    set.insert_all(elements);

    let elements2 = [4, 5, 6, 7];
    set2.insert_all(elements2);
}
