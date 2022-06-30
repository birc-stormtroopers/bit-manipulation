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

fn set_bit(x: u8, i: u8) -> u8 {
    println!("x            = {:08b}", x);
    println!("1 << {}       = {:08b}", i, 1 << i);
    println!("x | (1 << {}) = {:08b}", i, x | (1 << i));
    println!("");
    x | (1 << i)
}

fn clear_bit(x: u8, i: u8) -> u8 {
    println!("x             = {:08b}", x);
    println!("!(1 << {})     = {:08b}", i, !(1 << i) as u8);
    println!("x & !(1 << {}) = {:08b}", i, x & !(1 << i));
    println!("");
    x & !(1 << i)
}

fn mask(low: u8, high: u8) -> u8 {
    let mask_width = high - low;
    let low_mask = (1 << mask_width) - 1;
    let mask = low_mask << low;
    println!("Mask [{},{})", low, high);
    println!("low_mask = (1 << {}) - 1  = {:08b}", mask_width, low_mask);
    println!("mask     = low_mask << {} = {:08b}", low, mask);
    println!("");
    mask
}

fn pack_dna(x: u8, y: u8, z: u8, w: u8) -> u8 {
    let res = x as u8;
    let res = (res << 2) | y;
    let res = (res << 2) | z;
    let res = (res << 2) | w;
    res
}

fn unpack_dna(dna: u8) -> (u8, u8, u8, u8) {
    let mask = (1 << 2) - 1; // 0x03
    let w = (dna >> 0) & mask;
    let z = (dna >> 2) & mask;
    let y = (dna >> 4) & mask;
    let x = (dna >> 6) & mask;
    (x, y, z, w)
}

fn popcount(x: u8) -> u8 {
    let mut count = 0;
    let mut y = x;
    while y > 0 {
        count += 1;
        y = y & (y - 1);
    }
    return count;
}

fn twopow(x: u8) -> bool {
    return x != 0 && x & (x - 1) == 0;
}

fn twopow2(x: u8) -> bool {
    return x != 0 && x == (x & -(x as i8) as u8);
}

fn log2(x: u8) -> Option<(u8, u8)> {
    if x == 0 {
        return None;
    }

    let w = u8::BITS as u8; // word size in u8
    let lz = x.leading_zeros() as u8;
    let rem = ((x & (x - 1)) != 0) as u8;
    let round_down = w - 1 - lz;
    let round_up = round_down + rem;

    Some((round_down, round_up))
}

fn log2_test() {
    println!("");
    println!("Testing log calculations");
    for i in 0..10 {
        match log2(i) {
            None => {
                println!("Undefined");
            }
            Some((n, m)) => {
                println!("{} -> {},{}", i, n, m);
            }
        }
    }
    println!("");
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

    set_bit(128 - 3, 0);
    set_bit(128 - 3, 1);
    set_bit(128 - 3, 2);
    set_bit(128 - 3, 3);

    clear_bit(128 - 3, 0);
    clear_bit(128 - 3, 1);
    clear_bit(128 - 3, 2);
    clear_bit(128 - 3, 3);

    mask(0, 0);
    mask(0, 1);
    mask(2, 7);
    mask(3, 5);

    let dna = pack_dna(0, 1, 2, 3);
    println!("dna = {:08b}", dna);
    let (x, y, z, w) = unpack_dna(dna);
    println!("{} {} {} {}", x, y, z, w);

    for i in 0..8 {
        println!("popcount({}) = {}", i, popcount(i));
        println!("twopow({}) = {}", i, twopow(i));
        println!("twopow2({}) = {}", i, twopow2(i));
    }

    log2_test();

    for i in 0i8..10i8 {
        println!(
            "Trailing zeros in {} [{:08b}]: {}",
            i,
            i,
            i.trailing_zeros()
        );
    }
}
