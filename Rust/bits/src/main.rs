fn basic_operations() {
    let x: u16 = 0xf4e2; // [f: 1111, 4: 0010, e: 1110, 2: 0010]
    println!("Unsigned:");
    println!("x:                      {:016b}", x);
    println!("!x:                     {:016b}", !x);
    println!("");

    println!("x:                      {:016b}", x);
    println!("x >> 2:                 {:016b}", x >> 2);
    println!("x & (x >> 2):           {:016b}", x & (x >> 2));
    println!("");

    println!("x:                      {:016b}", x);
    println!("x << 2:                 {:016b}", x << 2);
    println!("x | (x << 2):           {:016b}", x | (x << 2));
    println!("");

    println!("x:                      {:016b}", x);
    println!("x << 2:                 {:016b}", x << 2);
    println!("x ^ (x << 2):           {:016b}", x ^ (x << 2));
    println!("");

    #[allow(overflowing_literals)] // so we can cast the bit-pattern 0xf4e2 to i16
    let x: i16 = 0xf4e2 as i16; // [f: 1111, 4: 0010, e: 1110, 2: 0010]
    println!("Signed:");
    println!("x:                      {:016b}", x);
    println!("x << 2:                 {:016b}", x << 2);
    println!("x >> 2:                 {:016b}", x >> 2); // arithmetic shift
    println!("");
}

fn unsigned_arithmethic() {
    let x: u8 = 57;
    let y: u8 = 111;
    println!("{:0} =  {:08b}", x, x);
    println!("{:0} = {:08b}", y, y);
    println!("{:0} = {:08b}", x + y, x + y);
    println!("");

    let x: u8 = 102;
    let y: u8 = 67;
    println!("{:0} = {:08b}", x, x);
    println!("{:0} =  {:08b}", y, y);
    println!("{:0} =  {:08b}", x - y, x - y);
    println!("");

    let x: i8 = 67;
    let y: i8 = 102;
    println!("{:0} = {:08b}", x, x);
    println!("{:0} =  {:08b}", y, y);
    println!("{:0} =  {:08b}", x - y, x - y);
    println!("{:0} =  {:08b}", (x - y) as u8, (x - y) as u8);
    println!("");

    let x: u8 = 67;
    println!("{:0} = {:08b}", x, x);
    println!("{:0} =  {:08b}", 2 * x, 2 * x);
    println!("{:0} =  {:08b}", 3 * x, 3 * x);
    println!("");
}

fn main() {
    basic_operations();
    unsigned_arithmethic();
}
