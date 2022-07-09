use core::ops::*;

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

fn twopow(x: u8) -> bool {
    return x != 0 && x & (x - 1) == 0;
}

fn twopow2(x: u8) -> bool {
    return x != 0 && x == (x & -(x as i8) as u8);
}

fn log2_down(x: u32) -> Option<u32> {
    // Returns None if x is zero, since that triggers an overflow in the
    // subtraction
    (u32::BITS - 1).checked_sub(x.leading_zeros())
}

fn log2_up(x: u32) -> Option<u32> {
    let k = log2_down(x)?; // Returns None if we can't compute log2_down()
    Some(k + (k != x.trailing_zeros()) as u32)
}

fn log2_test() {
    println!("");
    println!("Testing log calculations");
    for i in 0..10 {
        println!("{} -> {:?}, {:?}", i, log2_down(i), log2_up(i));
    }
    println!("");
}

fn leftmost<W>(x: W) -> W
where
    W: Shr<u8, Output = W> + BitOr<Output = W> + BitOrAssign + BitXor<Output = W> + Copy,
{
    let w = (std::mem::size_of_val(&x) * u8::BITS as usize) as u8;
    let mut x = x;
    let mut shift = 1u8;
    while shift < w {
        x |= x >> shift;
        shift *= 2;
    }
    x ^ (x >> 1)
}

fn next_set(x: u32) -> Option<u32> {
    // x               xxx0 1110
    // rightmost:      0000 0010 <- rightmost 1-bit
    // carried:        xxx1 0000 <- removed 1-string and put carry
    // x ^ carried:    0001 1110 <- 1-string + carry
    // ones:           0000 0011 <- carry will add a bit, we remove another
    // carried | ones  xxx1 0011 <- one bit went up, the others down
    let rightmost = x & neg(x);
    let carried = x.checked_add(rightmost)?; // returns None if overflow
    let ones = (x ^ carried).checked_shr(x.trailing_zeros() + 2)?;
    Some(carried | ones)
}

// unary minus on unsigned not allowed in Rust, so this fakes it through a cast
#[inline]
fn neg(x: u32) -> u32 {
    -(x as i32) as u32
}
fn ashift(x: u32, k: u32) -> u32 {
    let sign_extension = neg(x >> 31); // all zeros or all ones depending on leftmost bit
    let sign_extension = sign_extension << (32 - k); // except lower k
    (x >> k) | sign_extension
}

/*
fn popcount(x: u32) -> u32 {
    let mut count = 0;
    let mut y = x;
    while y > 0 {
        count += 1;
        y = y & (y - 1);
    }
    return count;
}
*/

/*
fn popcount(x: u32) -> u32 {
    let mut count = 0;
    for i in 0..32 {
        count += (x >> i) & 1
    }
    return count;
}
*/

fn popcount32(x: u32) -> u32 {
    let m1 = 0x55555555;
    let m2 = 0x33333333;
    let m4 = 0x0f0f0f0f;
    let y = 0x01010101;

    let mut x = x;
    x -= (x >> 1) & m1;
    x = (x & m2) + ((x >> 2) & m2);
    x = (x + (x >> 4)) & m4;

    // the multiplication will overflow
    x.wrapping_mul(y) >> (u32::BITS - 8)
}

/*
fn popcount64(x: u64) -> u64 {
    let m1 = 0x5555555555555555;
    let m2 = 0x3333333333333333;
    let m4 = 0x0f0f0f0f0f0f0f0f;
    let m8 = 0x00ff00ff00ff00ff;
    let m16 = 0x0000ffff0000ffff;
    let m32 = 0x00000000ffffffff;
    let x = (x & m1) + ((x >> 1) & m1);
    let x = (x & m2) + ((x >> 2) & m2);
    let x = (x & m4) + ((x >> 4) & m4);
    let x = (x & m8) + ((x >> 8) & m8);
    let x = (x & m16) + ((x >> 16) & m16);
    let x = (x & m32) + ((x >> 32) & m32);
    x
}

fn popcount64(x: u64) -> u64 {
    let m1 = 0x5555555555555555;
    let m2 = 0x3333333333333333;
    let m4 = 0x0f0f0f0f0f0f0f0f;
    let y = 0x0101010101010101;

    let mut x = x;
    x -= (x >> 1) & m1;
    x = (x & m2) + ((x >> 2) & m2);
    x = (x + (x >> 4)) & m4;

    // the multiplication will overflow
    u64::wrapping_mul(x, y) >> (u64::BITS - 8)
}
*/

/*
fn rank_mask(i: u32) -> u32 {
    // We allow indexing on bits 0, 1, ..., 32 (inclusive).
    // We can't shift 32, so we use mod to wrap around 32, so both
    // zero and 32 will be shifted by zero, but then we use a mask
    // that is zero for zero and all ones for 32.
    let shift_by = (u32::BITS - i) % 32;
    let mask = -((i != 0) as i32) as u32;
    mask >> shift_by
}
*/

/*
fn rank_mask(i: u32) -> u32 {
    // Shift a bit up to position i-1 and get that bit and
    // the bits to the right of it. Use % to avoid overflow.
    let shift = (i as i32 - 1) as u32 % u32::BITS;
    let bit = 1 << shift; // first bit we want
    let mask = bit | (bit - 1); // plus those to the right

    // A mask that is all zeros if i is zero and all ones
    // if i is non-zero
    let zero_mask = -((i != 0) as i32) as u32;

    mask & zero_mask
}
*/

fn rank_mask(i: u32) -> u32 {
    let mask = -((i != 0) as i32) as u32;
    mask.wrapping_shr(u32::BITS - i)
}

fn rank(w: u32, i: u32) -> u32 {
    println!("mask: {:032b}", rank_mask(i));
    (w & rank_mask(i)).count_ones()
}

/*
fn swap(x: &mut u32, y: &mut u32) {
    *x ^= *y;
    *y ^= *x;
    *x ^= *y;
}
*/

fn swap(x: &mut u32, y: &mut u32) {
    let a = *x ^ *y;
    let b = *y ^ a;
    let c = a ^ b;
    *y = b;
    *x = c;
}

// XOR up to and including n
fn xor_to_n_naive(n: u32) -> u32 {
    let mut w = 0;
    for a in 0..=n {
        w ^= a;
    }
    w
}

// XOR up to and including n
fn xor_to_n(n: u32) -> u32 {
    match n & 0b11 {
        0b00 => n,
        0b01 => 1,
        0b10 => n + 1,
        0b11 => 0,
        _ => panic!("Can't happen!"),
    }
}

fn find_missing(x: &[u32]) -> u32 {
    let mut w = xor_to_n(x.len() as u32);
    for a in x {
        w ^= a;
    }
    w
}

fn find_non_dup(x: &[u32]) -> u32 {
    x.iter().fold(0, |acc, w| acc ^ w)
}

fn branch_select(b: bool, x: u32, y: u32) -> u32 {
    if b {
        x
    } else {
        y
    }
}

fn branchless_select(b: bool, x: u32, y: u32) -> u32 {
    y ^ ((x ^ y) & -(b as i32) as u32)
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
        println!("popcount({}) = {}", i, popcount32(i as u32));
        println!("twopow({}) = {}", i, twopow(i));
        println!("twopow2({}) = {}", i, twopow2(i));
    }

    for i in 0i8..10i8 {
        println!(
            "Trailing zeros in {} [{:08b}]: {}",
            i,
            i,
            i.trailing_zeros()
        );
    }

    for i in 0..10 {
        println!("leftmost of {:08b} is {:08b}", i, leftmost(i));
    }

    println!("{} [{:08b}] -> {:?}", 3, 3, next_set(3));
    println!("{} [{:08b}] -> {:?}", 5, 5, next_set(5));
    println!("{} [{:08b}] -> {:?}", 6, 6, next_set(6));
    let ones = (-1i32) as u32;
    println!("{} [{:08b}] -> {:?}", ones, ones, next_set(ones));
    println!("{} [{:08b}] -> {:?}", 0, 0, next_set(0));

    log2_test();

    for i in -55i32..-50i32 {
        println!("{}", i);
        println!("{:08b} >> 2 = {:08b}", i, i >> 1);
        println!("{:08b} >> 2 = {:08b}", i, ashift(i as u32, 1));
    }
    println!("");
    for i in 50i32..55i32 {
        println!("{}", i);
        println!("{:08b} >> 2 = {:08b}", i, i >> 1);
        println!("{:08b} >> 2 = {:08b}", i, ashift(i as u32, 1));
    }

    let w: u32 = 0xdeadbeef;
    println!("{:032b}", w);
    for i in 0..=32 {
        // we can index up to 32
        println!(
            "{:032b} {:032b} {}",
            rank_mask(i),
            w & rank_mask(i),
            rank(w, i)
        );
    }

    for i in 0xdeadbeef..0xdeadbeff {
        println!(
            "popcount({:08b}) = {} [{}]",
            i,
            popcount32(i),
            i.count_ones()
        );
    }

    let mut x = 42;
    let mut y = 13;
    println!("before swap: {} {}", x, y);
    swap(&mut x, &mut y);
    println!("after swap: {} {}", x, y);

    for i in 0..120 {
        assert!(xor_to_n(i) == xor_to_n_naive(i));
    }

    println!("missing {}", find_missing(&vec![1, 0, 2, 4]));
    println!("non-dup {}", find_non_dup(&vec![1, 0, 1, 0, 2]));

    println!(
        "select: true 13 42: {} {}",
        branch_select(true, 13, 42),
        branchless_select(true, 13, 42)
    );
    println!(
        "select: false 13 42: {} {}",
        branch_select(false, 13, 42),
        branchless_select(false, 13, 42)
    );
}
