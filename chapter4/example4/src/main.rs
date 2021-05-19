fn hello() {
    println!("Hello");
}

fn test_unit() {
    println!("# test_unit()");
    let ret = hello();
    assert_eq!(ret, ());
    assert_eq!(std::mem::size_of::<()>(), 0);
}

fn test_bool() {
    println!("# test_bool()");
    let b1 = true;
    let b2 = !b1;
    println!("{} {}", b1, b2);

    let n1 = 8;
    let n2 = 12;
    let b3 = n1 >= 10;
    let b4 = n2 >= 10;
    let b5 = b3 && b4;
    let b6 = b3 || b4;
    println!("{} {}", b5, b6);

    assert_eq!(std::mem::size_of::<bool>(), 1);
}

fn main() {
    test_unit();
    test_bool();
}
