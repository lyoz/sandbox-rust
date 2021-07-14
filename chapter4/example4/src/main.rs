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

fn test_float() {
    println!("# test_float()");

    let f1 = 10.0; // f64(デフォルトの型)
    let f2 = -1_234.56f32; // f32(suffixで型を明示)
    let f3 = 578.6E+77; // f64(指数部を指定)
    println!("{} {} {}", f1, f2, f3);
}

fn test_char() {
    println!("# test_char()");

    let c1 = 'A';
    let c2 = 'a';
    println!("{} {}", c1, c2);
    assert!(c1 < c2);
    assert!(c1.is_uppercase());

    let c3 = '0';
    assert!(c3.is_digit(10));

    let c4 = '\t'; // タブ文字
    let c5 = '\n'; // 改行文字
    let c6 = '\''; // シングルクオート
    let c7 = '\\'; // バックスラッシュ
    let c8 = '\x7f'; // 8ビットコード
    println!("{} {} {} {} {}", c4, c5, c6, c7, c8);

    let c9 = '漢';
    println!("{}", c9);

    let c10 = '\u{5b57}'; // U+5b57(字)
    let c11 = '\u{1f600}'; // U+1f600(😀)
    println!("{} {}", c10, c11);
}

fn f1(mut n: u32) {
    println!("f1: n={}", n);
    n = 1;
    println!("f1: n={}", n);
}

fn f2(r: &mut u32) {
    println!("f1: *r={}", *r);
    *r = 1;
    println!("f1: *r={}", *r);
}

fn test_reference() {
    println!("# test_reference()");
    let mut n = 0;
    println!("test_reference: n={}", n);
    f1(n);
    println!("test_reference: n={}", n);
    f2(&mut n);
    println!("test_reference: n={}", n);

    let c1 = 'A';
    let c1_ref = &c1;
    assert_eq!(*c1_ref, 'A');

    let mut n1 = 0;
    let n1_ref = &mut n1;
    assert_eq!(*n1_ref, 0);
    *n1_ref = 123;
    assert_eq!(*n1_ref, 123);
    assert_eq!(n1, 123);
}

fn test_pointer() {
    println!("# test_pointer()");
    let c1 = 'A';
    let c1_ptr: *const char = &c1;
    assert_eq!(unsafe { *c1_ptr }, 'A');

    let mut n1 = 0;
    let n1_ptr: *mut i32 = &mut n1;
    assert_eq!(unsafe { *n1_ptr }, 0);

    unsafe {
        *n1_ptr = 1000;
        assert_eq!(*n1_ptr, 1000);
    }
}

fn main() {
    test_unit();
    test_bool();
    test_integer();
    test_overflow();
    test_float();
    test_char();
    test_reference();
    test_pointer();
}
