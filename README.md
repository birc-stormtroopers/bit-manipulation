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

The reason we have an arithmetic shift operator is because we have two's-complement integers. Dividing by powers of two is easy and efficiently done with right-shifting, but if we always shift in zero-bits, it would only work for unsigned values.

If we take a positive number and divide it by a power of two, we get what amounts to a shift to the rigth. (Of course, we usually think about it the other way around; we use shifts to divide).

```
64               = 01000000
64 / 4 = 64 >> 2 = 00010000 = 16
```

But if we take a negative number, and implement division of a power of two with a shift, we won't get the right answer:

```
-64                = 11000000
-64 / 4 = -64 >> 2 = 00110000 = 48
```

We *almost* get the right answer; the answer we get is the positive number the matches the negative number we wanted on the modulus circle, but it isn't the number we wanted.

If instead we use arithemetic shift, we maintain the sign bit on the number, and draggint ones in from the left ensures us that we get the bit-pattern for the right negative number.

```
-64                = 11000000
-64 / 4 = -64 >> 2 = 11110000 = -16
```

To see why this works, consider the two's-complement interpretation of a bit pattern once more:

$$x = -b_{w-1}2^{w-1} + \sum_{i=0}^{w-2} b_i\cdot 2^i$$

If we have a negative number (you can check for positive numbers yourself), then the first bit is set, so it contributes a value of $-2^{w-1}$ and the rest sum to some positive contribution.

$$x = -2^{w-1} + \sum_{i=0}^{w-2} b_i\cdot 2^i$$

If we just treat this as basic arithmetic, we can divide by two through all the terms, and we will see that this amounts to an arithmetic shift. If one shift divides by two, then doing it $k$ times will amount to shifting
$k$ times, and if one time amounts to dividing by two, then
$k$ times amounts to dividing by
$2^k$.

Dividing all terms by two we get:

$$x/2 = -2^{w-2} + \sum_{i=0}^{w-2} b_i\cdot 2^{i-1}
      = -2^{w-2} + \sum_{i=0}^{w-3} b_{i+1}\cdot 2^i + b_0/2$$

and remember that a negative number such as $-2^{w-2}$ is represented in two's-complement as

$$-2^{w-2} = -2^{w-1} + 2^{w-2}$$

That means that dividing by two gives us the equation

$$x/2 = -2^{w-1} + 2^{w-2} + \sum_{i=0}^{w-3} b_{i+1}\cdot 2^i + b_0/2.$$

We now have the two highest bit set, and all the other bits contribute half as much as before, i.e., they are shifted right by one.

If the lowest bit, $b_0$, is zero, this is a division by two. But what happens if we cannot divide
$x$ exactly? What if
$b_0=1$?
We cannot represent fractions with this representation, so we are doing integer division, but is that what we are getting?

When we shift, the low bit(s) that are shifted off the edge of the word are thrown away, so in the expression above $b_0/2$ means zero. If we had a positive number, throwing away the half-bit would round the result down towards zero, which is what integer division does.

But if we throw away the positive contribution $b_0/2$ from the sum, we are not rounding up towards zero; instead we are rounding down.

$$-2^{w-1} + 2^{w-2} + \sum_{i=0}^{w-3} b_{i+1}\cdot 2^i < -2^{w-1} + 2^{w-2} + \sum_{i=0}^{w-3} b_{i+1}\cdot 2^i + b_0/2$$

so we are rounding down towords minus infinity. In this sense, arithmetic shift is *not* like division. It is only division if there is no remainder; if there is a remainder, it rounds in the wrong direction.

Another clear example of this is dividing -1 by two. We would expect -1/2 = 0 for integer division, but if you shift -1, the bit pattern that consists of all ones, an arithmetic shift would give us -1 back.

```
  11111111 >> 1 = 11111111
```

When you shift right, you still get multiplication by powers of two. Consider the two's-complement interpretation of a number, and multiply the number by two:

$$2\times\left(-b_{w-1}\cdot 2^{w-1} + \sum_{i=0}^{w-2} b_i\cdot 2^i\right)
=-b_{w-1}\cdot 2^w + \sum_{i=0}^{w-2} b_i\cdot 2^{i+1}$$

If we have a positive number $b_{w-1}$ is zero, and we have increased each bit's contribution by a factor two, amounting to shifting them one position to the right.

If we have a negative number we have

$$-b_{w-1}\cdot 2^w + \sum_{i=0}^{w-2} b_i\cdot 2^{i+1}
= -2^w + b_{w-2}\cdot 2^{w-1} + \sum_{i=1}^{w-2} b_{i-1}\cdot 2^i + 0\cdot 2^0$$

which, since $-2^w = -2^{w-1} - 2^{w-2}$ is

$$-2^{w-1} + (b_{w-2} - 1)\cdot 2^{w-1} + \sum_{i=1}^{w-2} b_{i-1}\cdot 2^i + 0\cdot 2^0.$$

I know we don't have two bit-locations for $w-1$, but we don't have one for
$w$ either, so at this point we don't have a valid bit pattern, but we are just doing arithmetic on numbers.

If the new highest bit, $b_{w-2}$, is a one, then the second term cancels and we have the number

$$-2^{w-1} + \sum_{i=1}^{w-2} b_{i-1}\cdot 2^i + 0\cdot 2^0.$$

where the remaining bits all have a magnitude that is a factor of two higher, i.e., they are shifted left, and we have shifted a zero into the least significant position. In other words, if we shift a one into the left-most bit, a shift and a multiplication by two is the same.

```
 -1 = 1111
 -1 << 1 = 1110 = -2
 -2 = 1110
 -2 << 1 = 1100 = -4
 -3 = 1101
 -3 << 1 = 1010 = -6
 -4 = 1100
 -4 << 1 = 1000 = -8
```

If the new left-most bit is zero, on the other hand, we have

$$-2^{w-1} + (b_{w-2} - 1)\cdot 2^{w-1} + \sum_{i=1}^{w-2} b_{i-1}\cdot 2^i + 0\cdot 2^0
= -2^{w-1} - 2^{w-1} + \sum_{i=1}^{w-2} b_{i-1}\cdot 2^i + 0\cdot 2^0
= -2^{w} + \sum_{i=1}^{w-2} b_{i-1}\cdot 2^i + 0\cdot 2^0$$

where we can't represent $-2^{w}$ in a
$w$-bit word, so it is dropped, leaving

$$\sum_{i=1}^{w-2} b_{i-1}\cdot 2^i + 0\cdot 2^0$$

We have an overflow, where we are now missing $-2^w$. So we get a number that, if we subtracted
$2^w$ from it, would be the right result.

```
 -5 = 1011
 -5 << 1 = 0110 = 6 (-2**4 = -10)
 -6 = 1010
 -6 << 1 = 0100 = 4 (-2**4 = -12)
 -7 = 1001
 -7 << 1 = 0010 = 2 (-2**4 = -14)
 -8 = 1000
 -8 << 1 = 0000 = 0 (-2**4 = -16)
```

Overflows are not unique to two's-complement numbers. If you shift bits off the end, you move past the range of numbers you can represent, and you are doing arithemtics in the ring $\mod 2^w$ instead of the integers. That is just a consequence of working with fixed-sized words. At least the signed and unsigned integers work roughly the same when it comes to multiplication of powers of two and shifting bits to the left...

If this all seems a bit overwheling by now, have no fear. You rarely have to think too much about two's-complement arithmetic to exploit bit-manipulation for tricks. The only thing you *really* need to know about two's-complement is the equation $-x = \neg x + 1$. That comes in handy from time to time, especially if you want to translate boolean values into bit masks. This is because boolean values are often represented as the numbers 0 for false and 1 for true, or the bit-patterns `000....000` and `000...001`, but often you would rather have bit masks `000...000` and `111...111` (all zeros or all ones). That is, if you have a boolean value `b` but you want a mask `m` such that

```
 b = 0000 -> m = 0000
 b = 0001 -> m = 1111
```

You can do that in several ways. If you have arithmetic shift, you could shift `b` `w-1` to the left and then `w-1` to the right:

```
0000 << 3 = 0000, 0000 >> 3 = 0000
0001 << 3 = 1000, 1000 >> 3 = 1111
```

(but this only works with arithmetic shift and not logical shift).

You could shift and OR a couple of time (doubling the shift each time, so a $\log w$ number of times):

```
0000 | (0000 << 1) = 0000
0000 | (0000 << 2) = 0000

0001 | (0001 << 1) = 0011
0011 | (0011 << 2) = 1111
```

but by far the easiest way, and only one hardware instruction, is to change the sign of `b`:

```
b = 0000; -b = 0000 (~0000 + 1 = 1111 + 1 = 0000 (with overflow))
b = 0001; -b = 1111 (~0001 + 1 = 1110 + 1 = 1111)
```



## A bag of bit-tricks

That was the basic theory. The most fundamental bit-operations, and how we treat bit-patterns as numbers, both signed and unsigned, on modern computers. From here on, it is just a long list of various tricks you can use for manipulating computer words with bit operations. If you think of more, please make a pull request. It is always fun to have a large catalogue of clever ideas.

### Getting and setting bits

Perhaps the most fundamental task we could think of is getting an individual bit from a word. To do that, shift a one in under the bit and mask with `&`. If you want bit `i`, then you do `x & (1 << i)`. It gives you a new word where bit `i` has the same value as `x[i]`, and all remaining bits zero.

```
x            = 01111101
1 << 0       = 00000001
x & (1 << 0) = 00000001

x            = 01111101
1 << 1       = 00000010
x & (1 << 1) = 00000000

x            = 01111101
1 << 2       = 00000100
x & (1 << 2) = 00000100

x            = 01111101
1 << 3       = 00001000
x & (1 << 3) = 00001000
```

In Rust it could look like this (for eight-bit words):

```rust
fn get(x: u8, i: u8) -> u8 {
    x & (1 << i)
}
```

If you want to interpret the result as a boolean value, there are two issues. The first is how hardware generally interpret bit-patterns as truth-values. Most hardware instructions consider the bit pattern with all zeros to be false and anything else to be true. Many programming langauges do the same, considering anything "zero-like" (empty lists or strings, the number zero, etc.) to mean false and anything else to mean true. This is sometimes called [truthiness](https://en.wikipedia.org/wiki/Truthiness) after The Colbert Report. If you in such a language, an integer can already be used as a truth(y) value, and you don't have to do anything more to interpret the bit pattern you get from `x & (1 << i)`.

Some languages are more strict about their type system, though, and do not consider any type of integer a boolean value. If so, you can easily translate the integer the bit manipulations are making into a boolean value; just compare the result with zero. If the bit pattern `x & (1 << i)` is zero (which we should interpret as false), then `x & (1 << i) != 0` is false, while if `x & (1 << i)` is non-zero, `x & (1 << i) != 0` is true. Comparing with zero has the effect of telling the type system that any non-zero value should be `true` and only zero should be `false`.

```rust
fn get_bool(x: u8, i: u8) -> bool {
    (x & (1 << i)) != 0
}
```

The extra comparsion is not necessarily anything you pay for. The compiler can figure out that you just want a truth-value out of an integer, and it can usually make something useful out of that. [It can actually be slightly more efficient](https://godbolt.org/z/7WhbPToT8), since we ask for simpler output, so in the linked-to example, the assembly code for the boolean version directly compares the bit and sets the result to `00000000` or `00000001` based on the comparison.

If you wanted to translate the bit lookup into zero or one, `00000000` or `00000001`, you could also have done this:

```rust
fn get_zero_one(x: u8, i: u8) -> u8 {
    (x >> i) & 1
}
```

If your language represents true and false as zero and one (when working with bools and not some truthiness) casting a boolean value to an integer would also give you zero or one, so `get_zero_one(x, i)` will give you the same as `get_bool(x, i) as u8`.

Now let's say you have a word `x` and you want to set the i'th bit to one, that is, you want a new word `y` where `y[i] = 1` and `y[j] = x[j]` everywhere else.

To do that, shift a one-bit up to position `i` with `1 << i` and then OR it with `x`.

```
x            = 01111101
1 << 0       = 00000001
x | (1 << 0) = 01111101

x            = 01111101
1 << 1       = 00000010
x | (1 << 1) = 01111111

x            = 01111101
1 << 2       = 00000100
x | (1 << 2) = 01111101

x            = 01111101
1 << 3       = 00001000
x | (1 << 3) = 01111101
```

In Rust:

```rust
fn set_bit(x: u8, i: u8) -> u8 {
    x | (1 << i)
}
```

If you instead want to clear the bit, so `y[i] = 0` and `y[j] = x[j]` everywhere else, you can shift a one up to position `i` with `1 << i` once more, but now flip all the bits to get ones everywhere *except* position `i`: `!(1 << i)`. If you AND this with `x`, position `i` is set to zero, because whatever `x[i]` is we are AND'ing with zero, and all the other bits remaint he same, because we AND `x[j]` with 1.

```
x             = 01111101
!(1 << 0)     = 11111110
x & !(1 << 0) = 01111100

x             = 01111101
!(1 << 1)     = 11111101
x & !(1 << 1) = 01111101

x             = 01111101
!(1 << 2)     = 11111011
x & !(1 << 2) = 01111001

x             = 01111101
!(1 << 3)     = 11110111
x & !(1 << 3) = 01110101
```

In Rust, for eight-bit words:

```rust
fn clear_bit(x: u8, i: u8) -> u8 {
    x & !(1 << i)
}
```


### Bit-masks

Getting and setting individual bits is generally useful, but sometimes we want to manipulate larger chunks of contiguous bits. For that, we also use shift, OR, and AND, but just with larger blocks, called *masks*.

Before we start constructing masks, there is an observation we need to make. If you have a bit pattern `x` and you subtract 1 from it, the rightmost one-bit (if there is one) gets flipped to zero while the bits lower than that are flipped to one, and if there isn't any set bits, then `x - 1 = -x` (because `0 - 1 = -1`) which are all set bits.

```
x         = 00000000
x - 1     = 11111111

x         = 00001101
x - 1     = 00001100

x         = 00001110
x - 1     = 00001101

x         = 00001111
x - 1     = 00001110

x         = 00010000
x - 1     = 00001111
```

This is just a consequence of how subtraction with borrowing works; we borrow from the right-most 1 all the way down to position zero in order to subtract one. The case where `x` is the same thing, but in arithmetic modulo $2^w$.

Depending on how anal the type checker in your langauge is, you might have to cast between signed and unsigned to subtract with overflow, which happens for `x = 0`, but in Rust it looks like this:

```rust
fn minus_one(x: u8) -> u8 {
    (x as i8 - 1) as u8
}
```

It is only necessary for `x = 0` where `x - 1` gives an overflow; if we knew `x > 0` we could stay in `u8` the entire time.

We can use this to get a contiguous range of set bits that we can use as a mask. If you want to extract the last `k` bits from a word, you want to set those `k` bits to one and everything else to zero. So make a word with one bit at position `k` (since we index from zero, that is one bit to the left of where you want your mask to start) and then subtract one from it. The one-bit word you can make by shifting one left by `k`.

```
1 << 0 = 00000001
mask   = 00000000

1 << 2 = 00000100
mask   = 00000011

1 << 3 = 00001000
mask   = 00000111

1 << 4 = 00010000
mask   = 00001111

1 << 5 = 00100000
mask   = 00011111
```

If you want a mask that doesn't start at index zero, you can start by making a mask of the right width and then shift it up to where you want it.

```rust
fn mask(low: u8, high: u8) -> u8 {
    let mask_width = high - low;
    let low_mask = (1 << mask_width) - 1;
    low_mask << low
}
```

```
Mask [0,0)
low_mask = (1 << 0) - 1  = 00000000
mask     = low_mask << 0 = 00000000

Mask [0,1)
low_mask = (1 << 1) - 1  = 00000001
mask     = low_mask << 0 = 00000001

Mask [2,7)
low_mask = (1 << 5) - 1  = 00011111
mask     = low_mask << 2 = 01111100

Mask [3,5)
low_mask = (1 << 2) - 1  = 00000011
mask     = low_mask << 3 = 00011000
```

The latter is something you sometimes use to set specific bits in a word while leaving the rest unchanged, but mostly you use masks that start at zero and use shift to move the bits in the word down there.

You can use masks to pack data into smaller words that your hardware readily gives you access to. For example, no hardware I am aware lets me address in units smaller than a byte (8-bit words), but since there are only four different nucleotides in DNA (and ignoring that we have to represent uncertainty and such), I should be able to represent each nucleotide in two bits. Well, I can. I can, for example, pack four two-bit words into one eight-bit word and get the packed data out again:

```rust
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
```

**FIXME**

### The right-most set bit

**FIXME**

#### Get the right-most bit

If you have a word `x`, you might want to have only the right-most bit in `x`. Assuming `x` is not zero, this expression `y = x & -x` will set `y` to the word that consists of the right-most bit in `x` and only the right-most bit. If `x` is zero, `y` is zero as well.

```
x      = 00101100

~x     = 11010011
+1     = 00000001
-x     = 11010100

x      = 00101100
-x     = 11010100
x & -x = 00000100
```

In Rust, it could look something like this:

```rust
fn get_rightmost(x: u8) {
    x & -(x as i8) as u8
}
```

The casting stuff (`as i8`, `as u8`) is necessary because you cannot change the sign of an unsigned number in Rust, but you could also just work with signed numbers.

```rust
fn get_rightmost(x: i8) {
    x & -x
}
```


#### Is a number a power of two?

If `x` is a power of two, it can have at most one bit set. (Zero is a power of two, and then `x` has zero bits set, but otherwise $x=2^i$ means that `x` only has bit `i` set).



**FIXME: more below***















[^1]: There might be flags set in a register to tell you if any set bits were shifted out, but unless you are writing machine code, you do not have access to this, so from a high-level programming perspective the bits are lost.

[^2]: You have two zeros with floating point numbers, but the instructions that care for zero are not looking at floating point numbers so it isn't an issue there).

[^3]: There is also a [one's-complement representation](https://en.wikipedia.org/wiki/Ones'_complement) but no one uses it, to the best of my knowledge.

[^4]: There is a [Wrapping](https://doc.rust-lang.org/stable/std/num/struct.Wrapping.html) type if you want wrapping ($x \mod 2^\mathrm{ws}$) behaviour. There is also a compiler flag, `#![allow(arithmetic_overflow)]`, but explicitly using the wrapping type makes it explicit in the code that this is the behaviour that you want.
