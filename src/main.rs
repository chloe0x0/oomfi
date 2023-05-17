use oomfi::*;

fn main() {
    // BloomFilter representing the set {:3, uwu, owo}
    // n=3, false positive rate of 1%
    let mut oomfi = Bloom::new(3, 0.01);
    println!("Hash Functions: {}, Bits: {}", oomfi.hash_functions(), oomfi.number_of_bits());

    // Insert elements into the set
    oomfi.insert(":3");
    oomfi.insert("uwu");
    oomfi.insert("owo");
    assert_ne!(oomfi.is_empty(), true);
    // Assert that the elements are in the set
    assert!(oomfi.query(":3"));
    assert!(oomfi.query("uwu"));
    assert!(oomfi.query("owo"));
    // This should only fail ~1% of the time ^_^
    assert_ne!(oomfi.query("OWO"), true);
    // Clear the set
    oomfi.clear();
    // Assert that the set's previous elements are empty
    assert_ne!(oomfi.query(":3"), true);
    assert_ne!(oomfi.query("uwu"), true);
    assert_ne!(oomfi.query("owo"), true);

    assert!(oomfi.is_empty());
}
