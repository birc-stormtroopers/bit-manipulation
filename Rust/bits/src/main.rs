use num_traits::PrimInt;
use std::string::ToString;

trait BitWidth {
    fn width() -> usize;
}

#[cfg_attr(rustfmt, rustfmt_skip)]
mod bidwiths {
    use super::BitWidth;
    impl BitWidth for u8 { fn width() -> usize { return 8; } }
    impl BitWidth for u16 { fn width() -> usize { return 16; } }
    impl BitWidth for u32 { fn width() -> usize { return 32; } }
    impl BitWidth for u64 { fn width() -> usize { return 64; } }

    impl BitWidth for i8 { fn width() -> usize { return 8; } }
    impl BitWidth for i16 { fn width() -> usize { return 16; } }
    impl BitWidth for i32 { fn width() -> usize { return 32; } }
    impl BitWidth for i64 { fn width() -> usize { return 64; } }
}

fn bits<W: PrimInt + ToString + BitWidth>(x: W) -> String {
    let mut bits = Vec::new();
    for i in 0..W::width() {
        bits.push(((x >> i) & W::one()).to_string());
        if i % 8 == 7 {
            bits.push(" ".to_string());
        }
    }
    bits.pop(); // we added a space too much at the end
    bits.reverse(); // we write words in the opposite order
    return (bits).join("");
}

fn basic_operations() {
    let x: u16 = 0xf4e2; // [f: 1111, 4: 0010, e: 1110, 2: 0010]
    println!("Unsigned:");
    println!("x:                      {}", bits(x));
    println!("x shifted left by two:  {}", bits(x << 2));
    println!("x shifted right by two: {}", bits(x >> 2));
    println!("");

    println!("x:                      {}", bits(x));
    println!("x >> 2:                 {}", bits(x << 2));
    println!("x & (x >> 2):           {}", bits(x & (x >> 2)));
    println!("");

    println!("x:                      {}", bits(x));
    println!("x << 2:                 {}", bits(x << 2));
    println!("x | (x << 2):           {}", bits(x & (x << 2)));
    println!("");

    println!("x:                      {}", bits(x));
    println!("x << 2:                 {}", bits(x << 2));
    println!("x ^ (x << 2):           {}", bits(x ^ (x << 2)));
    println!("");

    println!("x:                      {}", bits(x));
    println!("!x:                     {}", bits(!x));
    println!("");

    #[allow(overflowing_literals)] // so we can cast the bit-pattern 0xf4e2 to i16
    let x: i16 = 0xf4e2 as i16; // [f: 1111, 4: 0010, e: 1110, 2: 0010]
    println!("Signed:");
    println!("x:                      {}", bits(x));
    println!("x shifted left by two:  {}", bits(x << 2));
    println!("x shifted right by two: {}", bits(x >> 2)); // arithmetic shift
}

fn main() {
    basic_operations();
}
