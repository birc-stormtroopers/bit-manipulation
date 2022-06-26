# Tips and tricks for manipulating bits

Despite popular conception, computer do not operate on ones and zeros. At least, it is a misleading idea. It is true that computers manipulate bits, but they do so in blocks of computer words, that are multi-bit binary numbers. If you want to manipulate numbers, the computer can readily do that, but if you want to manipulate individual bits, you have more work to do, and you have to do it by manipulating entire computer words.

This is still worthwhile to do in many cases. Bits are simple, and if you can manipulate all the bits in a word as single instructions, you can sometimes write faster code that you could otherwise do. If you can reduce artithmetic to simpler bit-wise operations, you can also gain some speed. These are, of course, micro-optimisations and not something you should focus on when developing algorithms, where larger-scale decisions are far more important, but if you need the extra little boost, you can often get it here. And sometimes, thinking in bits will actually give you better algorithms and data structures in the asymptotic sense as well, but that is beyond the scope of this little tutorial.

The purpose of this repository is just to show you have you can manipulate bits on a modern computer, and to show you some tricks that exploits this. I'll leave more complicated data structures and algorithms to other repos.

## Basic bit operations

Before we start, we should have an idea about how computers store information. At the basic level, you do not have the individual bit but some larger word size. If you use the common x86_64 architecture, for example, you can address data at a granularity of bytes, i.e. 8-bit numbers, but the CPU's registers are 64 bits, so that is typically the smallest data size you work with.

I am not going to use 64-bit words here, because they are unwieldy to draw and hard to read, so instead I will use 16-bit words. If you want 64-bit words, just extend them four times.

When we draw a word, we usually do it like this:

![Bits in a computer word](figs/bit-operations/bits-in-a-word.png)

It has 16 bits, and we number them $b_0,b_1,\ldots,b_{15}$, and we number the right-to-left. That is the opposite of how we would write something like a string, where we typically write left-to-right in this culture, but it matches the way we write numbers. In decimal, you would write a number like 12 to mean 1 tens and 2 ones, with the most significant digit first and the least significant digit last. In binary, we do the same. We interpret a binary number 
$x = (b_0,b_1,\ldots,b_{15})$
as the number
$$x= b_{15}\cdot 2^{15} + b_{14}\cdot 2^{14} + \cdots + b_{1}\cdot 2^1 + b_0 \cdot 2^0 =  \sum_{i=0}^{15}b_i\cdot 2^i$$

Most of the time, if we just work with 16 bits, it doesn't matter which order they are in, but if we start shifting them left and right, we need to know which direction that is, and because we have this number interpretation, the most significant bit is the left-most and the least significant bit is the right-most.

The basic operations you can do with individual bits are those you know from boolean logic. Here, we interpret bits as truth-values, with 1 being true and 0 being false, and the operations are

 * `NOT b`: true if b is false, false if b is true
 * `a AND b`: true if both a and b are true, false otherwise
 * `a OR b`: true if either of a or b are true, false otherwise
 * `a XOR b`: true if exactly one of a or b are true, false otherwise

The rules are the same when you use these operations on a computer word. There, you just apply the rule to all the operation in one or two words in parallel.

![Bit-wise operations on words](figs/bit-operations/bitwise-operations.png)

Other operations work on the entire word, but still bit-wise. The most important are shift operations, and I will only present those here, and leave others for later tricks.

When you shift words left or right, usually written as `x << k` or `x >> k` for shifting the word `x` left or right by `k` bits, you get exactly what it says on the tin: you shift all the bits in the word `k` places to the left or right.

The left shift is the simplest, because there are only one version of it. It will shift bits to the right, the bits it shifts out of the edge of the word are lost[^1] and the bits that are left at the right are set to zero, as if we had shifted zero bits in from some even lower index bits.

![Right shift](figs/bit-operations/shift-left.png)

With the way we interpret bit-patterns as binary numbers, a left shift corresponds to multiplying by a factor $2^k$ but modulo the size of your words. If the word is

$$x= b_{15}\cdot 2^{15} + b_{14}\cdot 2^{14} + b_{13}\cdot 2^{13} + \cdots + b_{3}\cdot 2^3 + b_{2}\cdot 2^2 + b_{1}\cdot 2^1 + b_0 \cdot 2^0 =  \sum_{i=0}^{15}b_i\cdot 2^i$$

then shifting, ignoring overflow (the bits that are dropped on the left) we get

$$x \ll 3 = b_{15}\cdot 2^{18} + b_{14}\cdot 2^{17} + b_{16}\cdot 2^{16} + \cdots + b_{3}\cdot 2^6 + b_{2}\cdot 2^5 + b_{1}\cdot 2^4 + b_0 \cdot 2^3 + 0\cdot 2^2 + 0\cdot 2^1 + 0\cdot 2^0 =  2^3\cdot\sum_{i=0}^{15}b_i\cdot 2^i$$

and chopping off the contributions that go beyond the available word-size, bits $2^{18}, 2^{17}, 2^{16}$, corresponds to taking the remainder with respect to 
$2^{16}$.

Shifting bits doesn't require complicated hardware, not compared to multiplication, so multiplying by factors of two is much faster done by left-shifting. Your compiler will automatically do that, if it knows it is a power of two it is multiplying with. Hardware will also sometimes figure it out, but there you are usually safer to translate to bit operations if you need it. But this is a micro-optimisation that is rarely worth it; figure out a way to let the compiler know what you are multiplying with instead, if you can.

Shifting right does the same thing, it moves the bits to the right, but there is a complication with what it shifts in from the left. The simple solution is to shift in zeros just as left-shift does, and that is one option. It is called *logical right-shift* (but not really because it is the "logical" choice, but because it considers the bits as independent boolean values).

![Logical shift](figs/bit-operations/logical-shift-right.png)

This operation is analogue to left-shift, and if you interpret the bits as a number the way we have above, which is an *unsigned* number, then it behaves like a division by a power of two. This is because the number interpretation is such that we multiply the magnitude by two every time we go one bit up and we divide by two every time we go one bit down. So dividing by a multiple of twos amounts to shifting the high bits down. If you extract the low bits instead, incidentally, you will get the remainder of that division. You can do that with a mask (a bit pattern) that you AND with. We will see how to create masks later.

![Logical shift and division](figs/bit-operations/logical-shift-and-division-remainder.png)

However, if you interpret the bit pattern as a signed integer, you might have negative numbers, and those are not encoded the same way. They are encoded in two's complement on all modern computers, and we look at those in the next section. Suffices to say here is that if you want to interpret a shift of such a number as a division, then the highest bit is what you must shift in on the left. So if the highest bit is zero, we get the same as logical shift, but if the highest bit is one, we get one-bits shifted in instead. This type of shift is called *arithmetic shift* because it adapts to the number interpretation of the bit pattern.

![Arithmetic shift](figs/bit-operations/arithmetic-shift-right.png)

There's a couple of issues if you use shifting in your code. One is that you do not always control whether you use logical or arithmetic shift. All languages I know of, that have unsigned integer types, will use logical shift on those. But if you have signed integers, you need to check with your language. Some languages have separate operators for logical and arithmetic shift, `>>>` for logical and `>>` for arithemtic shift in Java, for example. Or they will use arithmetic shift for signed types and logical for unsigned. Or, as in with the every complicated C programming language, leave it undefined--you might get one, you might get the other, and we are not going to tell you.

Another issue is the offset `k` we shift with. Since zeros (or maybe ones with arithmetic shift) are shifted in, you might think that you can shift by an arbitrary amount. Think again. The hardware instructions for shifting generally require a small number that can be encoded in machine code, and they don't necessarily accept shifts larger than the word size. (In C, it is of course undefined what happens if you shift by more than the word size; Rust is better, here it is a compile time error to even attempt). So keep `k` smaller than the word size if you want to live a long and happy programmer life. There are times where this is annoying, and you at least would want to shift a 32-bit word by 32 bits and just let the result be all zeros, for example, but then you have to program your way around that.

### Printing words (for educational purposes)

Before we go to the next section, let's see how we can perform these operations in a  programming language. I've chosen Rust because it is a reasonable low-level langauge, and the operations we have on bits here we probably have everywhere, but at the same time it is more strict in its definition compared to a language such as C, were some operations are left undefined by the language definition. What we can do in one language, though, we can usually do in all of them, except that the syntax for the operations can be different. (The bit-wise not in Rust is `!`, for example, but it is `~` in most languages I know (that do not use `~` as a unary minus)).

A word of warning if you are using Python, though: there you don't have fixed sized words, and that changes many of the operations. You cannot shift bits off the left end of a word, for example, and some operations are less straightforward. If you want to manipulate bits in Python, you need to read the documentation there.


I wrote a function for printing bits for my examples. I wrote one that can handle different word sizes (although I am not really going to use that here), so there is a bit of trait and generics hacking, but it should be simple enough to see what I am doing.

```rust
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
```

The main bit is the `bits()` function. There, I run through the bits from zero up to (but not including) the width of the word. To get a bit, I shift the word `i` to the right, which will place the bit I want as the right-most bit, which means that I can get that bit, and only that bit, if I mask with a word that only has that bit set. I get that from the trait with `W::one()`, but it is just the integer 1. With four bits, `abcd`, it would look something like that

```
Shift               Masked with 0001
abcd >> 0 = abcd    000d
abcd >> 1 = 0abc    000c
abcd >> 2 = 00ab    000b
abcd >> 3 = 000a    000a
```

I wrote this as if we are shifting 0 in from the left, i.e. as a logical shift, but it doesn't matter if it is zero or one, because I mask the top bits out anyway.

I get the bits in the reverse order compared to how I want to print them. I get the least-significant bit first and the most-significant bit last, so I see the bits `[d c b a]`. I want to print them as `[a b c d]`, so I reverse them. I add spaces between blocks of eight bits just to make the words easier to read.

The bit operators in Rust are:
 * NOT: `!`
 * OR: `|`
 * AND: `&`
 * XOR: `^`
 * SHIFT: `<<` and `>>`. The right shift is logical for unsigned integers and arithmetic for signed.

You can see the operations in action in this code snippet:

```rust
    let x: u16 = 0xf4e2; // [f: 1111, 4: 0010, e: 1110, 2: 0010]
    println!("Unsigned:");
    println!("x:                      {}", bits(x));
    println!("x shifted left by two:  {}", bits(x << 2));
    println!("x shifted right by two: {}", bits(x >> 2));
    println!("");

    println!("x:                      {}", bits(x));
    println!("x >> 2:                 {}", bits(x >> 2));
    println!("x & (x >> 2):           {}", bits(x & (x >> 2)));
    println!("");

    println!("x:                      {}", bits(x));
    println!("x << 2:                 {}", bits(x << 2));
    println!("x | (x << 2):           {}", bits(x | (x << 2)));
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
```

that should produce this output:

```
Unsigned:
x:                      11110100 11100010
x shifted left by two:  11010011 10001000
x shifted right by two: 00111101 00111000

x:                      11110100 11100010
x >> 2:                 00111101 00111000
x & (x >> 2):           00110100 00100000

x:                      11110100 11100010
x << 2:                 11010011 10001000
x | (x << 2):           11110111 11101010

x:                      11110100 11100010
x << 2:                 11010011 10001000
x ^ (x << 2):           00100111 01101010

x:                      11110100 11100010
!x:                     00001011 00011101

Signed:
x:                      11110100 11100010
x shifted left by two:  11010011 10001000
x shifted right by two: 11111101 00111000
```

As you can see, the last right-shift drags ones in from the left. This is because it is an arithmetic shift and there was a one in the most-significant bit before the shift. When we shifted the same bit-pattern earlier, but in an unsigned integer, we shifted zero bits in; that was logical shift.


## Two's complement arithmetic

**FIXME: continue here**

Ok, what's this with signed values, then, and why do we have arithmetic shift?




[^1]: There might be flags set in a register to tell you if any set bits were shifted out, but unless you are writing machine code, you do not have access to this, so from a high-level programming perspective the bits are lost.
