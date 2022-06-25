#include <stdio.h>
#include <stdlib.h>

char global_buf[18];
char *bits(uint16_t x, char *buf)
{
    buf = buf ? buf : global_buf;
    for (int i = 0, j = 0; i < 16; i++, j++)
    {
        // Shift 1 ws-1 up and mask the bit out from there.
        // We don't need the bit at the bottom when we just
        // use it is a truth value.
        buf[j] = (x & (1 << (15 - i))) ? '1' : '0';
        if (i == 7) // put a space in the middle for ease of reading
            buf[++j] = ' ';
    }
    return buf;
}

int main(void)
{
    uint16_t x = 0xf4e2; // [f: 1111, 4: 0010, e: 1110, 2: 0010]

    printf("Unsigned:\n");
    printf("x:                      %s\n\n", bits(x, 0));
    printf("x shifted left by two:  %s\n", bits(x << 2, 0));
    printf("x shifted right by two: %s\n", bits(x >> 2, 0));
    printf("\n");

    printf("x:                      %s\n", bits(x, 0));
    printf("x >> 2:                 %s\n", bits(x >> 2, 0));
    printf("x & (x >> 2):           %s\n", bits(x & (x >> 2), 0));
    printf("\n");

    printf("x:                      %s\n", bits(x, 0));
    printf("x << 2:                 %s\n", bits(x << 2, 0));
    printf("x | (x << 2):           %s\n", bits(x | (x << 2), 0));
    printf("\n");

    printf("x:                      %s\n", bits(x, 0));
    printf("x << 2:                 %s\n", bits(x << 2, 0));
    printf("x ^ (x << 2):           %s\n", bits(x ^ (x << 2), 0));
    printf("\n");

    printf("x:                      %s\n", bits(x, 0));
    printf("~x:                     %s\n", bits(~x, 0));
    printf("\n");

    int16_t y = 0xf4e2; // [f: 1111, 4: 0010, e: 1110, 2: 0010]
    printf("Signed:\n");
    printf("y:                      %s\n", bits(y, 0));
    printf("y shifted left by two:  %s\n", bits(y << 2, 0));
    // this may or may not be arithmetic shift.
    printf("y shifted right by two: %s\n", bits(y >> 2, 0));

    return 0;
}
