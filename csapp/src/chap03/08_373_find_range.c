//usr/bin/gcc -S -O1 "$0" && /usr/bin/gcc "$0" && exec ./a.out "$@"
#include <stdio.h>

typedef enum {NEG, ZERO, POS, OTHER} range_t;

range_t find_range(float x) {
    int result;
    unsigned char cf, zf, pf;
    float y = 0.0;

    asm(
            "vucomiss %[x],%[y]\n\t"
            "setc %[cf]\n\t"
            "setz %[zf]\n\t"
            "setp %[pf]\n\t"
            : [cf] "=r" (cf), [zf] "=r" (zf), [pf] "=r" (pf)
            : [x] "x" (x), [y] "x" (y)
    );
    // printf("%f: %d %d %d\n", x, cf, zf, pf);

    if (pf) {
        return OTHER;
    } else if (cf) {
        return POS;
    } else if (zf) {
        return ZERO;
    } else {
        return NEG;
    }
}

range_t find_range_move(float x) {
    int result;
    char cf = 0, zf = 0, pf = 0;
    float y = 0.0;
    short one = 1;

    asm(
            "vucomiss %[x],%[y]\n\t"
            // "cmovpw %[one],%[pf]\n\t"
            // "cmovcw %[one],%[cf]\n\t"
            // "cmovzw %[one],%[zf]\n\t"
            "setc %[cf]\n\t"
            "setz %[zf]\n\t"
            "setp %[pf]\n\t"
            // "cmovp %[cf],%[zf]\n\t"
            : [cf] "=r" (cf), [zf] "=r" (zf), [pf] "=r" (pf)
            : [x] "x" (x), [y] "x" (y), [one] "r" (one)
    );
    // printf("%f: %d %d %d\n", x, cf, zf, pf);

    zf = cf | zf;
    return zf + cf + pf;

}

int main() {
    printf("%f\n", 0.0/0.0);
    printf("%f\n", -(0.0/ 0.0));
    printf("\n");

    printf("%d\n",find_range(1.0));
    printf("%d\n",find_range(15.0));
    printf("%d\n",find_range(0.0));
    printf("%d\n",find_range(-1.0));
    printf("%d\n",find_range(0.0/0.0));
    printf("%d\n",find_range(-(0.0/0.0)));
    printf("\n");

    printf("%d\n",find_range_move(1.0));
    printf("%d\n",find_range_move(15.0));
    printf("%d\n",find_range_move(0.0));
    printf("%d\n",find_range_move(-1.0));
    printf("%d\n",find_range_move(0.0/0.0));
    printf("%d\n",find_range_move(-(0.0/0.0)));
    printf("\n");
}
