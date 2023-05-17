extern crate bitvec;

use core::panic;
use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{BuildHasher, Hash, Hasher};
use bitvec::vec::BitVec;
use bitvec::bitvec;

// ln(2)^2, used when computing optimal m
const LN2_2: f64 = std::f64::consts::LN_2 * std::f64::consts::LN_2;

/// Function to compute the optimal number of hash functions given: 
/// epsilon: The desired false positivity rate
#[inline(always)]
fn optima_k(epsilon: f64) -> u64 {
    (-epsilon.ln()/std::f64::consts::LN_2).ceil() as u64
}
 
/// Function to compute the optimal number of bits given:
/// n: number of elements to insert,
/// epislon: desired false positivity rate
#[inline(always)]
fn optima_m(n: usize, epsilon: f64) -> u64 {
    ((-1.0f64 * n as f64 * epsilon.ln()) / LN2_2).ceil() as u64
}

/// An implementation of a generic BloomFilter
pub struct Bloom {
    /// Bit-Vector of the data (more memory compact than [u8;N])
    data: BitVec,
    /// number of hash functions
    k: u64, 
    /// size of the BloomFilter in bits
    m: u64,
    /// Hash Functions
    /// Why only 2?
    /// See this paper: https://www.eecs.harvard.edu/~michaelm/postscripts/rsa2008.pdf
    h1: DefaultHasher,
    h2: DefaultHasher
}

impl Bloom {
    pub fn new(n: usize, epsilon: f64) -> Self {
        // Compute optimal m and k
        let m = optima_m(n, epsilon);
        let k = optima_k(epsilon);
        Bloom {
            /// Init all bits to 0, using m bits
            data: bitvec![0; m as usize],
            k,
            m, 
            h1: RandomState::new().build_hasher(),
            h2: RandomState::new().build_hasher()
        }
    }

    /// Construct a bloom filter with an explicit number of hash functions
    /// panics if the number is less than 2
    /// also takes as input
    /// n: the expected number of elements in the set
    /// epsilon: the desired false positive rate
    pub fn with_k(k: u64, n: usize, epsilon: f64) -> Self {
        if k < 2 {
            panic!("Too few hash functions (need at least 2)");
        }

        let m = optima_m(n, epsilon);
        Bloom {
            data: bitvec![0; m as usize],
            k,
            m,
            h1: RandomState::new().build_hasher(),
            h2: RandomState::new().build_hasher()
        }
    }

    /// Construct a Bloom Filter with m bits
    /// Will use the optimal number of hash functions
    /// panics if m == 0
    /// also takes as input:
    /// epsilon: desired false positive rate
    pub fn with_m(m: u64, epsilon: f64) -> Self {
        if m == 0 {
            panic!("Cannot have 0 bits!");
        }

        let k = optima_k(epsilon);
        Bloom {
            data: bitvec![0; m as usize],
            k,
            m,
            h1: RandomState::new().build_hasher(),
            h2: RandomState::new().build_hasher()
        }
    }

    /// Construct a bloom filter with m bits and k hash functions
    /// panics if either m==0 or k < 2
    pub fn with_km(k: u64, m: u64) -> Self {
        if k < 2 {
            panic!("Too few hash functions (need at least 2)");
        } 
        if m == 0 {
            panic!("Cannot have 0 bits!");
        }

        Bloom {
            data: bitvec![0; m as usize],
            k,
            m,
            h1: RandomState::new().build_hasher(),
            h2: RandomState::new().build_hasher()
        }
    }

    /// Clear the BitVector (removes all elements from teh set)
    #[inline(always)]
    pub fn clear(&mut self) {
        self.data.fill(false);
    }

    /// How many hash functions are used in the BloomFilter
    #[inline(always)]
    pub fn hash_functions(&self) -> u64 {
        self.k
    }

    /// How many bits are used in teh BitVector
    #[inline(always)]
    pub fn number_of_bits(&self) -> u64 {
        self.m
    }

    /// Check if the set is the empty set
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.data.count_ones() == 0
    }

    /// Get a reference to the BitVec
    #[inline(always)]
    pub fn get_vec(&self) -> &BitVec {
        &self.data
    }

    // Compute h1(x) and h2(x) for some element x
    fn hash_element(&mut self, element: impl Hash) -> (u64, u64) {
        let h1 = &mut self.h1.clone();
        let h2 = &mut self.h2.clone();

        element.hash(h1);
        element.hash(h2);

        (h1.finish(), h2.finish())
    }

    // Compute gi(x) within the bit vector for some element x given h1(x), h2(x)
    #[inline(always)]
    fn compute_index(&mut self, h1: u64, h2: u64, i: u64) -> usize {
        ((h1.wrapping_add(i.wrapping_mul(h2))) % self.m) as usize
    }

    /// Query for set membership of a Hashable element
    pub fn query(&mut self, element: impl Hash) -> bool {
        let (h1, h2) = self.hash_element(element);

        for i in 0u64..self.k {
            let gi_x = self.compute_index(h1, h2, i);
            let bit = self.data.get(gi_x).unwrap();

            if !bit {
                return false;
            }
        }
        
        true 
    }

    /// Insert a Hashable element into the BitVector
    pub fn insert(&mut self, element: impl Hash) {
        // Compute the h1(x) and h2(x) hashes
        let (h1, h2) = self.hash_element(element);
        // Iterate over the number of hash functions
        // g_i(x) = h1(x) + ih2(x), 1 <= i <= k
        for i in 0u64..self.k {
            let gi_x = self.compute_index(h1, h2, i);
            // Toggle the bit on
            self.data.set(gi_x, true);
        }
    }
}


