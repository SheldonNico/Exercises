#![allow(dead_code, unused_variables)]
#![feature(map_first_last)]
#![feature(hash_drain_filter)]
pub mod util;
mod snippet;

pub mod leetcode2020;
pub mod leetcode2021;

pub mod codewar;
pub mod advcode2020;
pub mod advcode2021;
pub mod advcode2022;

pub mod topic1_array;
pub mod topic2_graph;
pub mod cpp_algorithms;

pub fn main() {
    advcode2022::p19();
}
