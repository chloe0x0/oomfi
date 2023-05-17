# oomfi 🌸
A small, 🔥 *blazingly fast* 🔥, bloom filter implemented in Rust (yes, another one).

A bloom filter is a probabalistic data structure used for fast, memory efficient representations of Sets. 

More specifically, a bloom filter is a vector of M bits. Elements are added to the set by putting them through K hash functions which map the element to an index in the bit vector. All of these bits are then set to 1. To query for set membership, pass the element through the hash functions and if any of the bits at the hashed indecies are 0 we know the element cannot be a member of the set. Otherwise, it is *probably* an element of the set. There is a non-zero (though usually small) probability of a false positive (An element is said to be in the set when it was never inserted).

## Optimal number of hash functions and bits

I wont go over the derivation (though it is pretty trivial) 

for a desired false positivity rate $\epsilon \in (0, 1)$
and an expected capacity of n elements, 

the optimal number of hash functions is given as $-\frac{ln(\epsilon)}{ln(2)}$
and the optimal number of bits is $-\frac{nln(\epsilon)}{(ln(2))^2}$

In oomfi only 2 hash functions are used! This is because of an efficient scheme proposed by [Kirsch and Mitzenmacher](https://www.eecs.harvard.edu/~michaelm/postscripts/rsa2008.pdf) in which 2 hash functions can be combined to construct k hash functions. 
$$g_i(x) = h_1(x) + ih_2(x)$$
$$i \in [0, k)$$

This is a significant speedup compared to using K seperate hash functions! With this idea oomfi only really computes 2 hash values using Rust's DefaultHasher for query/ insertion calls regardless of the number of hash functions. It was proven to not impact the asymptotic false positive rate of a standard bloom filter. 

# Getting Started

### Dependencies
oomfi currently only depends on the [bitvec](https://crates.io/crates/bitvec) crate.
(future releases may depend on Serde for serialization of Bloom Filters)

## Installation

either run
```console
λ >>> cargo add oomfi
```
in your project directory

or add the following line to the dependencies in your cargo.toml
```toml
oomfi = "0.1.0"
```

## Usage

Lets represent the set {:3,uwu,owo}
with a false positivity rate of ~1%

```Rust
use oomfi::*;

fn main() {
    // Lets representing the set {:3, uwu, owo}
    // n=3, false positive rate of 1%
    // uses the optimal number of hash functions and bits
    // It can store any datatype which implements Hash
    let mut set = Bloom::new(3, 0.01);
    // Insert elements into the set
    set.insert(":3");
    set.insert("uwu");
    set.insert("owo");
    // Assert that the elements are in the set
    assert!(set.query(":3"));
    assert!(set.query("uwu"));
    assert!(set.query("owo"));
    // This should only fail ~1% of the time ^_^
    assert_ne!(set.query("OWO"), true);
    // Clear the set
    set.clear();
    // Assert that the set's previous elements are properly removed
    assert_ne!(set.query(":3"), true);
    assert_ne!(set.query("uwu"), true);
    assert_ne!(set.query("owo"), true);
}
```

using a custom type

```Rust
use oomfi::*;

#[derive(Clone, Copy)]
struct Mage<'a> {
    name: &'a str,
    level: u64,
    mana: f64
}

impl Hash for Mage<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.level.hash(state);
    }
}

fn main() {
    let Malori = Mage {name: &"Malori", level: u64::MAX, mana: f64::MAX};

    let set = Bloom::new(3, 0.01);
    set.insert(Malori);
    assert!(set.query(&Malori));
}
```

to check if a set is the empty set
```Rust
if set.is_empty() {
    println!("set = ∅");
} else {
    println!("set != ∅");
}
```

to get the number of hash functions used

```Rust
let hashes_used: u64 = set.hash_functions();
```
and the number of bits used in the BitVector

```Rust
let bits: u64 = set.number_of_bits();
```

to get a reference to the set's BitVec

```Rust
let bitvec: &BitVec = set.get_vec();
```

to construct a Bloom Filter with an explicit number of hash functions

```Rust
// Same set as the example, just with 7 hash functions
// The optimal number of bits will be used
let set: Bloom = Bloom::with_k(7, 3, 0.01);
```
Q: Why would you want a number of hash functions which is not the optimal number? 
A: Performance. It can be possible to use less than the optimal number of hash functions and still get few false positives. This will consequently use less compute. 

to construct a Bloom Filter with an explicit number of bits

```Rust
// Same set as the example, just with 100 bits
// The optimal number of hash functions are used
let set: Boom = Bloom::with_m(100, 3, 0.01);
```
You may use this for similar reasons as an explicit number of hash functions. It is possible to use less bits than the optimal amount to reduce memory usage. It is also possible to use more bits than the optimal amount.

To construct a Bloom Filter with an explicit number of bits and hash functions
```Rust
// Set with 7 hash functions and 100 bits
let set: Bloom = Bloom::with_km(7, 100);
```
