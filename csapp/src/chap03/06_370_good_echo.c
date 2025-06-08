//usr/bin/clang "$0" && exec ./a.out "$@"
#include <stdio.h>

#define BSIZE 32

void good_echo() {
    char buf[BSIZE];
    while (fgets(buf, BSIZE, stdin)) {
        if (fputs(buf, stdout) < 0) {
            break;
        }
    }
}

int main() {
    good_echo();
    return 0;
}
