#![allow(dead_code)]
use {
    std::fs,
    std::io::{BufReader, BufRead}
};

fn day01() {
    let file = BufReader::new(fs::File::open("./assets/day01.txt").unwrap());

    let mut sum: u32 = 0;
    for num in file.lines() {
        let num: u32 = num.unwrap().parse().expect("Fail to parse number");
        let mut res = num / 3;
        if res >= 2 {
            res -= 2;
        }
        sum += res;
    }
    println!("Final answer to day01: {}", sum);
}

fn main() {
}
