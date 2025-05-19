//usr/bin/clang "$0" && exec ./a.out "$@"
#include <stdio.h>

int main() {
    int value = 200 * 300 * 400 * 500;
    printf("overflow: %d\n", value);
    return 0;
}

