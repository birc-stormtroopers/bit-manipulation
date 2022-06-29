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

#[allow(arithmetic_overflow)]
fn twos_complement() {
    let x: i8 = 9;
    println!("{:0}  =  {:08b}", x, x);
    println!("{:0} = {:08b}", !x, !x);
    println!("{:0}  = {:08b}", -x, -x);
    println!("");

    let x: u8 = 100;
    let y: u8 = 130;
    println!("{:0} = {:08b}", x, x);
    println!("{:0} = {:08b}", y, y);
    println!("{:0} = {:08b}", x + y, x + y);
    println!("");

    let x: i8 = 100;
    #[allow(overflowing_literals)]
    let y: i8 = 130 as i8;
    println!(" {:0} = {:08b}", x, x);
    println!("{:0} = {:08b}", y, y);
    println!(" {:0} = {:08b}", x + y, x + y);
    println!("");

    let x: i8 = 64;
    println!("{:0}               = {:08b}", x, x);
    println!("{:0} / 4 = {:0} >> 2 = {:08b} = {:0}", x, x, x >> 2, x >> 2);
    println!("");
    let x: i8 = -64;
    println!("{:0}                = {:08b}", x, x);
    println!("{:0} / 4 = {:0} >> 2 = {:08b} = {:0}", x, x, x >> 2, x >> 2);
    println!("");
    let x: i8 = -64;
    println!("{:0}                = {:08b}", x, x);
    println!(
        "{:0} / 4 = {:0} >> 2 = {:08b} = {:0}",
        x,
        x,
        (x as u8) >> 2,
        (x as u8) >> 2
    );
    println!("");
}

fn get_rightmost(x: i8) {
    println!("x      = {:08b}", x);
    println!("");
    println!("!x     = {:08b}", !x);
    println!("+1     = {:08b}", 1);
    println!("-x     = {:08b}", -x);
    println!("");
    println!("x & -x = {:08b}", x & -x);
    println!("");
}

fn get(x: u8, i: u8) -> u8 {
    println!("x            = {:08b}", x);
    println!("1 << {}       = {:08b}", i, 1 << i);
    println!("x & (1 << {}) = {:08b}", i, x & (1 << i));
    println!("");
    x & (1 << i)
}

fn get_bool(x: u8, i: u8) -> bool {
    (x & (1 << i)) != 0
}

fn get_zero_one(x: u8, i: u8) -> u8 {
    (x >> i) & 1
}

fn main() {
    basic_operations();
    unsigned_arithmethic();
    twos_complement();
    get_rightmost(98);

    get(128 - 3, 1);
    get(128 - 3, 2);
    get(128 - 3, 3);
    println!(
        "{} {} {}",
        get_bool(128 - 3, 1),
        get_bool(128 - 3, 2),
        get_bool(128 - 3, 3)
    );
    println!(
        "{} {} {}",
        get_zero_one(128 - 3, 1),
        get_zero_one(128 - 3, 2),
        get_zero_one(128 - 3, 3)
    );
}
