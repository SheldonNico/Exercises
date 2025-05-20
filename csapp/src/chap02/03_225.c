//usr/bin/clang "$0" && exec ./a.out "$@"
#include <stdio.h>
float sum_elements(float a[], unsigned length) {
    int i;
    float result = 0.0;
    for (i = 0; i <= length -1; i++) {
        result += a[i];
    }
    return result;
}

int main() {
    float a1[] = {0.0};
    printf("%f\n", sum_elements(a1, 1));
    // printf("%f\n", sum_elements(a1, 0)); // will causing segmentfault
    printf("%u\n", 0u - 1);
    printf("%u\n", 0u - 4294967286u);
    printf("%u\n", 0u - 2122222221u);
    printf("%u\n", !!1223);

    float dd = -123.9;
    int k = dd;
    printf("%d\n", k);
    return 0;
}
