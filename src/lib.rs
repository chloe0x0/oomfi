mod bloom;
pub use bloom::*;

#[cfg(test)]
mod tests {
    use crate::Bloom;
    use std::hash::Hash;

    /// Test that the clear trait works
    #[test]
    fn clear_works() {
        let mut set = Bloom::new(3, 0.01);
        set.insert(":3");
        set.insert("uwu");
        set.insert("owo");
        set.clear();
        assert_ne!(set.query(":3"), true);
        assert_ne!(set.query("uwu"), true);
        assert_ne!(set.query("owo"), true);
    }

    /// Test that the bloom filter works for other data types
    #[test]
    fn test_types() {
        let mut set = Bloom::new(25, 0.01);

        for i in 0..26 {
            set.insert(i);
            assert!(set.query(i));
        }

        set.clear();

        #[derive(Clone, Copy, Debug)]
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

        let Malori = Mage {name: &"Malori", level: u64::MAX, mana: f64::MAX};
        set.insert(Malori);
        assert!(set.query(&Malori));
    }

    /// Test that the optimal values are correct
    #[test]
    fn test_optima() {
        // Optimal values for n = 100, epsilon= 0.01 ought to be m=959, k=7
        let set = Bloom::new(100, 0.01);
        assert_eq!(set.hash_functions(), 7);
        assert_eq!(set.number_of_bits(), 959);
        // n=100000, epsilon=0.0001, m = 1917012, k = 13
        let set = Bloom::new(100_000, 0.0001);
        assert_eq!(set.hash_functions(), 14);
        assert_eq!(set.number_of_bits(), 1917012);
        let set = Bloom::new(10, 0.0001);
        assert_eq!(set.hash_functions(), 14);
        assert_eq!(set.number_of_bits(), 192);
    }
}
