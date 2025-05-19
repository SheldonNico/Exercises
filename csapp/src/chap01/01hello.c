//usr/bin/clang -m32 "$0" && exec ./a.out "$@"
#include <stdio.h>

int main() {
    printf("hello, world\n");
    printf("size of long: %u\n", sizeof(long));
    printf("%ld", 1L << sizeof(long)<<3 - 1 );
    return 0;
}
