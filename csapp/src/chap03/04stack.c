//usr/bin/clang "$0" && exec ./a.out "$@"
#include "stdio.h"

int main() {
    long local;
    printf("local @ %p\n", &local);
    return 0;
}
