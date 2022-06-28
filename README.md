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

$$x= b_{15}\cdot 2^{15} + b_{14}\cdot 2^{14} + \cdots + b_{1}\cdot 2^1 + b_0 \cdot 2^0 = \sum_{i=0}^{15}b_i\cdot 2^i$$

then shifting, ignoring overflow (the bits that are dropped on the left) we get

$$x \ll 3  = b_{15}\cdot 2^{18} + b_{14}\cdot 2^{17} + \cdots + b_{1}\cdot 2^4 + b_0 \cdot 2^3 = \sum_{i=0}^{15}b_i\cdot 2^{i+3} = 2^3\cdot\sum_{i=0}^{15}b_i\cdot 2^i = 2^3 x$$

and chopping off the contributions that go beyond the available word-size, bits $2^{18}, 2^{17}, 2^{16}$, corresponds to taking the remainder with respect to 
$2^{16}$.

Shifting bits doesn't require complicated hardware, not compared to multiplication, so multiplying by factors of two is much faster done by left-shifting. Your compiler will automatically do that, if it knows it is a power of two it is multiplying with. Hardware will also sometimes figure it out, but there you are usually safer to translate to bit operations if you need it. But this is a micro-optimisation that is rarely worth it; figure out a way to let the compiler know what you are multiplying with instead, if you can.

Shifting right does the same thing, it moves the bits to the right, but there is a complication with what it shifts in from the left. The simple solution is to shift in zeros just as left-shift does, and that is one option. It is called *logical right-shift* (but not really because it is the "logical" choice, but because it considers the bits as independent boolean values).

![Logical shift](figs/bit-operations/logical-shift-right.png)

This operation is analogue to left-shift, and if you interpret the bits as a number the way we have above, which is an *unsigned* number, then it behaves like a division by a power of two. This is because the number interpretation is such that we multiply the magnitude by two every time we go one bit up and we divide by two every time we go one bit down. So dividing by a multiple of twos amounts to shifting the high bits down. If you extract the low bits instead, incidentally, you will get the remainder of that division. You can do that with a mask (a bit pattern) that you AND with. We will see how to create masks later.

![Logical shift and division](figs/bit-operations/logical-shift-and-division-remainder.png)

However, if you interpret the bit pattern as a signed integer, you might have negative numbers, and those are not encoded the same way. They are encoded in two's complement on all modern computers, and we look at that shortly. Suffices to say here is that if you want to interpret a shift of such a number as a division, then the highest bit is what you must shift in on the left. So if the highest bit is zero, we get the same as logical shift, but if the highest bit is one, we get one-bits shifted in instead. This type of shift is called *arithmetic shift* because it adapts to the number interpretation of the bit pattern.

![Arithmetic shift](figs/bit-operations/arithmetic-shift-right.png)

There's a couple of issues if you use shifting in your code. One is that you do not always control whether you use logical or arithmetic shift. All languages I know of, that have unsigned integer types, will use logical shift on those. But if you have signed integers, you need to check with your language. Some languages have separate operators for logical and arithmetic shift, `>>>` for logical and `>>` for arithemtic shift in Java, for example. Or they will use arithmetic shift for signed types and logical for unsigned. Or, as in with the every complicated C programming language, leave it undefined--you might get one, you might get the other, and we are not going to tell you.

Another issue is the offset `k` we shift with. Since zeros (or maybe ones with arithmetic shift) are shifted in, you might think that you can shift by an arbitrary amount. Think again. The hardware instructions for shifting generally require a small number that can be encoded in machine code, and they don't necessarily accept shifts larger than the word size. (In C, it is of course undefined what happens if you shift by more than the word size; Rust is better, here it is a compile time error to even attempt). So keep `k` smaller than the word size if you want to live a long and happy programmer life. There are times where this is annoying, and you at least would want to shift a 32-bit word by 32 bits and just let the result be all zeros, for example, but then you have to program your way around that.

### Using the operators

Before we go to the next section, let's see how we can perform these operations in a  programming language. I've chosen Rust because it is a reasonable low-level langauge, and the operations we have on bits here we probably have everywhere, but at the same time it is more strict in its definition compared to a language such as C, were some operations are left undefined by the language standard. What we can do in one language, though, we can usually do in all of them, except that the syntax for the operations can be different. (The bit-wise negation, `NOT` above, is `!` in Rust, for example, but it is `^` in Go and `~` in most other languages I know (that do not use `~` as a unary minus)).

A word of warning if you are using Python, though: there you don't have fixed sized words, and that changes many of the operations. You cannot shift bits off the left end of a word, for example, and some operations are less straightforward. If you want to manipulate bits in Python, you need to read the documentation there.


The bit operators in Rust are:
 * NOT: `!`
 * OR: `|`
 * AND: `&`
 * XOR: `^`
 * SHIFT: `<<` and `>>`. The right shift is logical for unsigned integers and arithmetic for signed.

You can see the operations in action in these code snippets:

```rust
    let x: u16 = 0xf4e2; // [f: 1111, 4: 0010, e: 1110, 2: 0010]
    println!("Unsigned:");
    println!("x:                      {:016b}", x);
    println!("!x:                     {:016b}", !x);
    println!("");
```

```
Unsigned:
x:                      1111010011100010
!x:                     0000101100011101
```


```rust
    println!("x:                      {:016b}", x);
    println!("x << 2:                 {:016b}", x << 2);
    println!("x | (x << 2):           {:016b}", x | (x << 2));
    println!("");
```

```
x:                      1111010011100010
x << 2:                 1101001110001000
x | (x << 2):           1111011111101010
```

```rust
    println!("x:                      {:016b}", x);
    println!("x << 2:                 {:016b}", x << 2);
    println!("x ^ (x << 2):           {:016b}", x ^ (x << 2));
    println!("");
```

```
x:                      1111010011100010
x << 2:                 1101001110001000
x ^ (x << 2):           0010011101101010
```

```rust
    println!("x:                      {:016b}", x);
    println!("x >> 2:                 {:016b}", x >> 2);
    println!("x & (x >> 2):           {:016b}", x & (x >> 2));
    println!("");
```

```
x:                      1111010011100010
x >> 2:                 0011110100111000
x & (x >> 2):           0011010000100000
```

Notice that the right-shift pulls zeros in from the left. That will change if we use a signed integer instead:


```rust
    #[allow(overflowing_literals)] // so we can cast the bit-pattern 0xf4e2 to i16
    let x: i16 = 0xf4e2 as i16; // [f: 1111, 4: 0010, e: 1110, 2: 0010]
    println!("Signed:");
    println!("x:                      {:016b}", x);      // same bit-pattern as before
    println!("x << 2:                 {:016b}", x << 2); // left-shift the same
    println!("x >> 2:                 {:016b}", x >> 2); // arithmetic shift
```

```
Signed:
x:                      1111010011100010
x << 2:                 1101001110001000
x >> 2:                 1111110100111000
```

As you can see, this right-shift drags ones in from the left. This is because it is an arithmetic shift and there was a one in the most-significant bit before the shift. When we shifted the same bit-pattern earlier, but in an unsigned integer, we shifted zero bits in; that was logical shift.

## Unsigned arithmetic

With the interpretation of binary numbers we have above, where we consider the bits as coefficients for increasing powers of two, arithmetic works as you would expect it from your elementary school math lessons, except that there are only a finite number of bits to work with.

### Addition

If you add two numbers, you add them from least-signficant bit to most-significant bit, with carries where necessary. For two 8-bit words, it could look like this:

```
       57 =   00111001
    + 111 = + 01101111

    carry:         1
       57:   00111001
      111: + 01101111
                    0
    
    carry:        1
       57:   00111001
      111: + 01101111
                   00

    carry:       1
       57:   00111001
      111: + 01101111
                  000

    carry:     1
       57:   00111001
      111: + 01101111
                01000

    carry:    1
       57:   00111001
      111: + 01101111
               101000

    carry:   1
       57:   00111001
      111: + 01101111
              0101000

    carry:  0
       57:   00111001
      111: + 01101111
             10101000

       168 = 10101000
```

If you run out of bits, though, say you want to add 128 + 128 in 8 bits, you can't represent the result.

```
      128:   10000000
      128: + 10000000
      =     100000000 <- requires 9 bits
```

What happens then depend on your programming language and/or the hardware you work on. For unsigned values, the typical behaviour is that any extra bits are just droped, which means that 128 + 128 equals zero in unsigned 8-bit words. This has the straightforward interpretation that you are doing arithmetic in the ring $x\mod 2^{\mathrm{ws}}$ when you work with words of size `ws`. 

$$128 + 128 \mod 2^8 = 256 \mod 2^8 = 0 \mod 2^8$$

Your CPU will have registers to indicate that you had an overflow, but they are not available in high-level langauges (not even in low-level languages such as C). Sometimes, this is the behaviour you want, but more often it is an error. Typically, though, it is not something you will be informed about by your programming envirnonment, and you need to check it explicitly. In Rust, overflow is a runtime error *when you compile in debug mode*,[^4] but it is silently ignored in development mode. In C, unsigned arithmetic is always to ignore overflow when variables are signed, but left undefined (just one tad worse than silently ignoring errors) when you use signed integers. Generally, overflow is something you have to worry about if you do arithmetic and there is a risk of them occurring, because you are unlikely to be told about them. What happens, is up to your langauge and system.

### Subtraction

Subtraction also works the way you would expect, just with a borrow instead of a carry.

```
     102 =   01100110
   -  67 = - 01000011
      35 =   00100011

   borrow:          2
      102:   01100100
       67:   01000011
                    1

   borrow:         2  
      102:   01100000
       67:   01000011
                   11

   borrow:           
      102:   01100000
       67:   01000011
                  011

   borrow:           
      102:   01100000
       67:   01000011
                 0011

   borrow:           
      102:   01100000
       67:   01000011
                00011

   borrow:           
      102:   01100000
       67:   01000011
               100011

   borrow:           
      102:   01100000
       67:   01000011
              0100011

   borrow:           
      102:   01100000
       67:   01000011
             00100011
```

What happens if we subtract a larger number from a smaller, though? We should get a negative number, (with 67 - 102 we would expect -35), but we don't *have* negative numbers with the current representation, and we won't have them until the next section. Let's try to just subtract like before, but when we need to borrow and there aren't any numbers left, we will borrow from off the left edge of the numbers. That is where extra bits went with overflow before, so let's try to get them back from the same place here:

```
   borrow:          
       67:   01000011
      102:   01100100
                    1

   borrow:          
       67:   01000011
      102:   01100100
                   11

   borrow:        2  
       67:   00111011
      102:   01100100
                  111

   borrow:          
       67:   00111011
      102:   01100100
                 1111

   borrow:          
       67:   00111011
      102:   01100100
                11111

   borrow:          
       67:   00111011
      102:   01100100
               011111

   borrow:    2    <- you borrowed from off the edge
       67:   10111011
      102:   01100100
              1011111

   borrow:
       67:   10111011
      102:   01100100
    = 221:   11011111
```

The number we get is 221, which looks odd, but

$$221 \mod 2^8 = -35 \mod 2^8$$

so we get the same modulus arithmetic as before.

What actually happens will again depend on your language and system. When we need to borrow from off the edge of the word we have an overflow, and that can be silently ignored, be an error, or be any other number of things. You need to check the documentation for your langauge to know exactly what would happen.

## Multiplication and division

I won't go into much detail with how multiplication and division work, because it is complicated on the hardware, but the effect is what you would be used to here as well. If you multiply two words, `w * v`, it will amount to adding `w` to `w` `v` times

```
    w  = 67  = 01000011
             + 01000011
   2w = 134  = 10000110
             + 01000011
   3w = 201 =  11001001
```

If you can do long division with decimal numbers, you can do the same with binary numbers

```

    201 / 3

 201 = 11001001
   3 = 00000011

      01
 11 ) 11001001
      11
      --
       00


      010
 11 ) 11001001
      11
      --
       00
       00
       --
        00


      0100
 11 ) 11001001
      11
      --
       00
       00
       --
        00
        00
        --
         01


      01000
 11 ) 11001001
      11
      --
       00
       00
       --
        00
        00
        --
         01
         00
         --
          10


      010000
 11 ) 11001001
      11
      --
       00
       00
       --
        00
        00
        --
         01
         00
         --
          10
          10
          --
          100


      0100001
 11 ) 11001001
      11
      --
       00
       00
       --
        00
        00
        --
         01
         00
         --
          10
          10
          --
          100
           11
           --
            11

      01000011
 11 ) 11001001
      11
      --
       00
       00
       --
        00
        00
        --
         01
         00
         --
          10
          10
          --
          100
           11
           --
            11
            11
            --
             0


    11001001 / 00000011 = 01000011
         201 /        3 =       67
```

Multiplication can give you overflow, and division is integer division, so if there is a remainder, you need another operator to get that (it is typically `%` instead of `/`, but it depends on your language).



## Two's complement arithmetic

The interpretation of bit-patterns as numbers we saw earlier

$$x = \sum_{i=0}^{15}b_i\cdot 2^i$$

only works for non-negative numbers. It only tells us a magnitude (the absolute value) of the number. To also allow for negative numbers, [we need to add something](https://en.wikipedia.org/wiki/Signed_number_representations). One possibility is to set asside one of the bits, a so-called *sign bit*, to indicate whether the number should be considered positive or negative. Floating point numbers do this. This has some drawbacks, most noticeable that you get two zeros, which complicates many computer instructures that rely on checks for zero.[^2] The hardware manipulation of numbers with a sign bit is also more complicated, since the sign bit determines what something like `x + y` should be; if one or both of the number are negative, the hardware logic should treat the addition differently from if they are both positive.

All modern hardware now use the [two's-complement representation](https://en.wikipedia.org/wiki/Two's_complement).[^3] There, the highest bit has a different interpretation: it contributes a value that is minus two to the word-size minus one, so for a 16-bit word in two's complement, the bits are interpreted as

$$x = \sum_{i=0}^{14}b_i\cdot 2^i - b_{15}2^{15}$$

With this interpretation, the high bit indicates whether we should interpret a bit-pattern as a positive or negative number, just as if it were a sign-bit, but it doesn't function exactly like a sign bit. There is still only one zero, which we get if all the bits are zero. If we set all except the high bit to zero, we would not get zero but a negative number, $-b_{15}2^{15}$.

For the example below, I use 4-bit words (because 8-bit would be too long), and I run through all the bit patterns twice, showing the unsigned interpretation on the left and the two's-complement signed pattern on the right.

```
Unsigned 4-bit    Signed 4-bit
                  * -2**3  Rest  = 
0000 =  0         0        000   =  0 - 0*8 =  0
0001 =  1         0        001   =  1 - 0*8 =  1
0010 =  2         0        010   =  2 - 0*8 =  2
0011 =  3         0        011   =  3 - 0*8 =  3
0100 =  4         0        100   =  4 - 0*8 =  4
0101 =  5         0        101   =  5 - 0*8 =  5
0110 =  6         0        110   =  6 - 0*8 =  6
0111 =  7         0        111   =  7 - 0*8 =  7
1000 =  8         1        000   =  0 - 1*8 = -8
1001 =  9         1        001   =  1 - 1*8 = -7
1010 = 10         1        010   =  2 - 1*8 = -6
1011 = 11         1        011   =  3 - 1*8 = -5
1100 = 12         1        100   =  4 - 1*8 = -4
1101 = 13         1        101   =  5 - 1*8 = -3
1110 = 14         1        110   =  6 - 1*8 = -2
1111 = 15         1        111   =  7 - 1*8 = -1
0000 =  0         0        000   =  0 - 0*8 =  0
0001 =  1         0        001   =  1 - 0*8 =  1
0010 =  2         0        010   =  2 - 0*8 =  2
0011 =  3         0        011   =  3 - 0*8 =  3
0100 =  4         0        100   =  4 - 0*8 =  4
0101 =  5         0        101   =  5 - 0*8 =  5
0110 =  6         0        110   =  6 - 0*8 =  6
0111 =  7         0        111   =  7 - 0*8 =  7
1000 =  8         1        000   =  0 - 1*8 = -8
1001 =  9         1        001   =  1 - 1*8 = -7
1010 = 10         1        010   =  2 - 1*8 = -6
1011 = 11         1        011   =  3 - 1*8 = -5
1100 = 12         1        100   =  4 - 1*8 = -4
1101 = 13         1        101   =  5 - 1*8 = -3
1110 = 14         1        110   =  6 - 1*8 = -2
1111 = 15         1        111   =  7 - 1*8 = -1
```

The reason I wrote all the numbers twice is that I wanted you to think about the arithmetic with the overflow behaviour, i.e., arithmetic modulus $2^\mathrm{ws}$. If we consider the unsigned numbers, this means that we are moving around a circle of positive numbers. Adding two numbers `x + y` means moving from the number `x` and `y` steps clockwise, subtracting `x - y` means moving from `x` by `y` steps counter-clockwise.

![Unsigned 4-bit words, modulus arithmetic](figs/twos-complement/u4.png)

If we reinterpret the modulus circle with two's complement, the addition and subtraction works the same way: addition means moving a number of steps clockwise and subtraction moving a number of steps counter-clockwise.

![Signed 4-bit words, modulus arithmetic](figs/twos-complement/i4.png)

With modulus arithmetic, it is just a question of which number we use to represent the modulus class. With unsigned numbers, we use the numbers `0, 1, 2, ..., 15`, but with two's complement, we use negative numbers for the latter half of the classes.

![Equivalent modulus arithmetic](figs/twos-complement/modulo-line.png)

Notice that this means that we have one more negative number than positive. On the figures, I've painted zero green, so zero is lumped together with the positive numbers. There are an equal number of red and green, but one of the green is zero while the rest are positive numbers, while all of the red are negative numbers.

![Positive and negative numbers, paired up](figs/twos-complement/pairing-numbers.png)

In other words, the smallest possible negative number in a finite number of bits, when we interpret them as two's-complement numbers, does not have a corresponding positive number.

You might now wonder if there is a simple way to translate the bit-pattern of a positive number $x$ into the corresponding
negative number, $-x$. There is: you negate the bits
in $x$
to get $\neg x$, and then you add one

$$-x = \neg x + 1.$$

![Make a positive number negative](figs/twos-complement/make-negative.png)

This doesn't just work for changing positive numbers into negative but also for changing negative numbers into positive

![Changing sign](figs/twos-complement/change-sign-circle.png)

with the caveat that if you try to change the smallest negative number into the corresponding positive number, which as you now know doesn't exit, you end up with the smallest negative number again.

![Negative MIN_INT](figs/twos-complement/minus-smallest.png)

There are several nice properties about this representation. One of them is that we don't need special logical to add positive and negative numbers. If we treat all numbers as unsigned and add them with overflow, then we also handle signed addition if we instead interpret the numbers that way.

```
Signed:
  100 = 01100100
+ 130 = 10000010
----------------
  230 = 11100110
================

Unsigned:
   100 = 01100100
+ -126 = 10000010
-----------------
   -26 = 11100110
=================
```

The bit addition is the same in the two examples, we just interpret the first as arithmetic in unsigned binary numbers and the second as arithmetic in two's-complement.

The second example also shows another nice property of two's-complement: if you want to do subtraction, $x - y$, then you can just change the sign of
$y$ and do addition,
$x - y = x + (-y)$. This shouldn't be surprising from your knowledge of arithmetic, but it works perfectly with the bit patterns as well, if you use two's complement: first you change the sign and then you add as if there were no such thing as signed integers.


**FIXME: continue here**

Ok, what's this with signed values, then, and why do we have arithmetic shift?




[^1]: There might be flags set in a register to tell you if any set bits were shifted out, but unless you are writing machine code, you do not have access to this, so from a high-level programming perspective the bits are lost.

[^2]: You have two zeros with floating point numbers, but the instructions that care for zero are not looking at floating point numbers so it isn't an issue there).

[^3]: There is also a [one's-complement representation](https://en.wikipedia.org/wiki/Ones'_complement) but no one uses it, to the best of my knowledge.

[^4]: There is a [Wrapping](https://doc.rust-lang.org/stable/std/num/struct.Wrapping.html) type if you want wrapping ($x \mod 2^\mathrm{ws}$) behaviour. There is also a compiler flag, `#![allow(arithmetic_overflow)]`, but explicitly using the wrapping type makes it explicit in the code that this is the behaviour that you want.
