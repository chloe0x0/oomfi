use oomfi::*;


fn main() {
    let mut oomfi = Bloom::new(100, 0.01);
    oomfi.insert(":3");
    assert!(oomfi.query(":3"));
    assert_ne!(oomfi.query("element"), true);
    oomfi.insert("element");
    assert!(oomfi.query("element"));

    oomfi.clear();
    assert_ne!(oomfi.query("element"), true);
    assert_ne!(oomfi.query(":3"), true);
}


