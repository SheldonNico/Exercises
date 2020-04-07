use std::collections::HashMap;
use crate::util::{ListNode};


pub fn p055_can_jump(nums: Vec<i32>) -> bool {
    let mut hist = HashMap::new();
    let nums: Vec<usize> = nums.into_iter().filter(|n| *n >= 0).map(|n| n as usize).collect();
    p055_can_jump_rec(0, nums.len()-1, &nums, &mut hist)
}

fn p055_can_jump_rec(start: usize, stop: usize, nums: &Vec<usize>, mut hist: &mut HashMap<(usize, usize), bool>) -> bool {
    println!("{} {}", start, stop);
    if !hist.contains_key(&(start, stop)) {
        let mut connect = false;

        if (start == stop) || (stop - start <= nums[start]) {
            connect = true;
        } else if start < nums.len() {
            for step in (1..nums[start]+1).rev() {
                if p055_can_jump_rec(start+step, stop, &nums, &mut hist) {
                    connect = true; break;
                }
            }
        }
        hist.insert((start, stop), connect);
    }
    hist[&(start, stop)]
}


pub fn p061_rotate_right(head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
    head


/*
 *    if k <= 0 {
 *        head
 *    } else {
 *        match head {
 *            Some( ListNode { val } )
 *        }
 *
 *        let mut len = 0;
 *        while rest.is_ok() {
 *            rest = rest.get();
 *            len += 1;
 *        }
 *
 *        k = k % len;
 *        let right = head;
 *        let mut count = 0;
 *        while count < len - k {
 *            let rest = rest.get();
 *            count += 1;
 *        }
 *
 *        rest.next = None;
 *
 *
 *
 *
 *
 *    }
 */
}

pub fn p1223_die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
    fn mod_add(a: i32, b: i32) -> i32 {
        let modnum: i32 = i32::pow(10, 9)+7;
        (a + b) % modnum
    }
    if n <= 0 { return 0; }

    let n = n as usize;
    let mut ways: Vec<Vec<i32>> = vec![vec![0; 6]; n+1];
    for a in 0..6 {
        for len in 1..=std::cmp::min(n, roll_max[a] as usize) {
            ways[len][a] += 1;
        }
    }

    for len in 1..n {
        for prv in 0..6 {
            for nxt in 0..6 {
                if prv == nxt { continue; }
                for cnt in 1..=std::cmp::min(n, roll_max[nxt] as usize) {
                    if cnt + len > n { break; }
                    ways[len+cnt][nxt] = mod_add(ways[len+cnt][nxt], ways[len][prv]);
                }
            }
        }
    }

    let mut answer = 0;
    for a in 0..6 {
        answer = mod_add(answer, ways[n][a])
    }

    answer
}
