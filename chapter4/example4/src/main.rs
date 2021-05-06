fn hello() {
    println!("Hello");
}

fn test_unit() {
    println!("# test_unit()");
    let ret = hello();
    assert_eq!(ret, ());
    assert_eq!(std::mem::size_of::<()>(), 0);
}

fn main() {
    test_unit();
}
