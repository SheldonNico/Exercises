//usr/bin/gcc -S -O1 "$0" && /usr/bin/gcc "$0" && exec ./a.out "$@"
//usr/bin/gcc -S -O1 "$0" && exit
#include <stdio.h>
int tmult_ok_asm(long x, long y, long *dest) {
    long p = x*y;
    *dest = p;
    return p > 0;
}

int umult_ok_asm(unsigned long x, unsigned long y, unsigned long *dest) {
    unsigned long p = x*y;
    *dest = p;
    return p > 0;
}

int tmult_ok1(long x, long y, long *dest) {
    long result = 0;
    *dest = x * y;
    asm("setae %al");
    return result;
}

int tmult_ok2(long x, long y, long *dest) {
    int result = 0;
    *dest = x * y;
    asm(
            "setae %%bl # set low-order byte \n\t"
            "movzbl %%bl,%[val]"
            : [val] "=r" (result)
            :
            : "%bl"
    );
    return result;
}

int tmult_ok3(long x, long y, long *dest) {
    unsigned char result;
    *dest = x * y;
    asm(
            "setae %[res]"
            : [res] "=r" (result)
    );
    return (int) result;
}

int umult_ok1(unsigned long x, unsigned long y, unsigned long *dest) {
    unsigned char bresult;
    asm(
            "movq %[x],%%rax\n\t"
            "mulq %[y]\n\t"
            "movq %%rax,%[p]\n\t"
            "setae %[b]"
            : [p] "=m" (*dest), [b] "=r" (bresult)
            : [x] "r" (x), [y] "r" (y)
            : "%rax","%rdx"
    );

    return (int)bresult;
}

int tmult_ok4(long x, long y, long *dest) {
    unsigned char bresult;
    asm(
            "imulq %[x],%[y]\n\t"
            "movq %[y],%[p]\n\t"
            "setae %[b]"
            : [p] "=m" (*dest), [b] "=r" (bresult), [y] "+r" (y)
            : [x] "r" (x)
    );

    return (int) bresult;
}

void umult_full(unsigned long x, unsigned long y, unsigned long *dest) {
    unsigned long* r0 = dest;
    unsigned long* r1 = dest+1;
    asm(
            "movq %[x],%%rax\n\t"
            "mulq %[y]\n\t"
            "movq %%rax,%[r0]\n\t"
            "movq %%rdx,%[r1]"
            : [r0] "=m" (*r0), [r1] "=m" (*r1)
            : [x] "r" (x), [y] "r" (y)
            : "%rax","%rdx"
    );
}

void bad_incr(int *ptr) {
    (*ptr)++;
}

void lock_incr(int *ptr) {
    asm(
            "lock\n\t"
            "addl $1,%[p]"
            : [p] "+m" (*ptr)
    );
}

// NOTE: label should not be hardcoded here
int odd_parity(unsigned long x) {
    unsigned char bflag = 0;
    unsigned char bflag8;
    while (x) {
        bflag8 = 1;
        asm(
                "testq %[x],%[x]\n\t"
                "jp .L0\n\t"
                "movb $0,%[b8]\n\t"
                ".L0:\n\t"
                : [b8] "+r" (bflag8)
                : [x] "r" (x)
        );
        bflag ^= bflag8;
        x >>= 8;
    }

    return (int)bflag;
}

double dmin(double x, double y) {
    double result;
    asm(
            "vminsd %[x],%[y],%[r]\n\t"
            : [r] "=x" (result)
            : [x] "x" (x), [y] "x" (y) 
    );
    return result;
}

double dsqrt(double x) {
    double result;
    asm(
            "sqrtsd %[x],%[r]"
            : [r] "=x" (result)
            : [x] "x" (x)
    );
    return result;
}

int main() {
    unsigned long dest[2];
    umult_full(1100000000000000, 1212333333333331, dest);
    printf("%lu vs %lu\n", dest[0], dest[1]);

    printf("%d\n", odd_parity(1));
    printf("%d\n", odd_parity(3));
    printf("%d\n", odd_parity(4));
    printf("%d\n", odd_parity(5));

    printf("%f\n", dmin(5.0, 4.0));
    printf("%f\n", dmin(-5.0, 4.0));

    printf("%f\n", dsqrt(9.0));
    printf("%f\n", dsqrt(16.0));
}
