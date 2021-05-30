fn main() {
    println!("Scalar types!");
    let _decimal: isize = 98_222;
    let _hex: usize = 0xff;
    let _octal: i16 = 0o77;
    let _binary: u16 = 0b1111_0000;
    let _byte: u8 = b'A';

    println!("Integer overflow!");
    assert_eq!((i32::MAX - 2).checked_add(1), Some(i32::MAX - 1));
    assert_eq!((i32::MAX - 2).checked_add(3), None);
}
