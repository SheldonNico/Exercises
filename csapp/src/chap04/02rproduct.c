//usr/bin/gcc -S -O1 "$0" && exit
long rproduct(long *start, long count) {
    if (count <= 1)
        return 1;
    return (*start) * rproduct(start+1, count-1);
}
