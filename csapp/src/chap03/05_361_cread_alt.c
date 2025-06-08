//usr/bin/clang "$0" && exec ./a.out "$@"

long cread(long *xp) {
    return (xp ? *xp : 0);
}

long cread_alt(long *xp) {
    if (xp != 0) {
        return *xp;
    } else {
        return 0;
    }
}
