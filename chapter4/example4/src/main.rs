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

fn test_integer() {
    println!("# test_integer()");

    let n1 = 10_000; // i32(デフォルトの型)
    let n2 = 0u8; // u8(suffixで型を明示)
    println!("{} {}", n1, n2);

    let n3 = -100_isize; //isize
    let n4 = 10; // 下でisize型のn3に加算しているのでn4はisize型になる
    let n5 = n3 + n4;
    println!("{}", n5);

    let h1 = 0xff; // 16進法
    let o1 = 0o744; // 8進法
    let b1 = 0b1010_0110_1110_1001; // 2進法
    println!("{} {} {}", h1, o1, b1);

    let n6 = b'A'; // ASCII文字'A'の文字コード
    assert_eq!(n6, 65u8);
}

fn test_overflow() {
    println!("# test_overflow()");

    let n1 = 200u8;
    let n2 = 3u8;

    // n1*n2=600はオーバーフローする

    assert_eq!(n1.checked_mul(n2), None);
    assert_eq!(n1.saturating_mul(n2), 255);
    assert_eq!(n1.wrapping_mul(n2), 88); // 600 mod 256 = 88
    assert_eq!(n1.overflowing_mul(n2), (88, true));
}

fn main() {
    test_unit();
    test_bool();
    test_integer();
    test_overflow();
}
