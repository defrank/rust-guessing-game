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

    println!("Floating-point types!");
    let _x = 2.0;  // f64
    let _y: f32 = 3.0;  // f32

    println!("Numeric operations!");
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4*30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    println!("The boolean type!");
    let _t = true;
    let _f: bool = false;

    println!("The character type!");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
