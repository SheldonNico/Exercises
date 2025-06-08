#!/bin/sh
//usr/bin/env rustc $0 -o a.out && ./a.out && rm ./a.out ; exit
use std::iter::FromIterator;

fn fact(mut n: i64) -> i64 {
    let mut r = 1;
    while (n > 0) {
        r *= n;
        n -= 1;
    }
    return r;
}

fn main() {
    println!("{}", unsafe { std::mem::transmute::<_, i8>(0xf4u8) });
    println!("{:x}", 0x400431 - 12);
    println!("{}", 0xffffff73u32 as i32);
    println!("{:x}", 0x4005ed - 141);
    // println!("{:b}", (0xf4i8) );

    println!("14!     = {}", fact(14));
    println!("2**31-1 = {}", (2u64).pow(31)-1);

    println!("{:x}", -16i64);
    println!("{}", 0b10000);
    // println!("{:x}", 1.8f64);
    println!("{:0>32b}", 1077936128i32);
    println!("{}", 0b10000000100);
    println!("{}", 0x3ff);
    println!("{:0>8x}", 4294967295u32);
    println!("{:0>8x}", 2147483647);

    println!("{:0>8x}", -2147483648i32);
    println!("{:0>8x}", 0);

    println!("{}", 0x3c);
    println!("{}", 0x120);
    println!("{}", 0x10);
    println!("{:0>32b}", -16i32);

    let mut points = std::collections::HashSet::new();
    for n in 0..50 {
        for s1 in 500..100000 {
            let e1 = s1 - (s1 - (30+8*n) / 16 * 16 + 15) / 16 * 16 - n * 8;
            // eprintln!("{n}x{s1}: {e1}");
            points.insert(e1);
        }
    }
    let mut points = Vec::from_iter(points);
    points.sort();
    eprintln!("{}: {:?}", points.len(), points);
}
