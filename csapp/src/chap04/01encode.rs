#!/bin/sh
//usr/bin/env rustc $0 -o a.out && ./a.out && rm ./a.out ; exit
fn main() {
    println!("{:0>16x}", 0x100i64 + 12);
}
