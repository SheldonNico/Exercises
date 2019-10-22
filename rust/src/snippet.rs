use std::collections::HashMap;
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

fn p056_merge_(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut out = vec![];
    for (mut st, mut et) in intervals.into_iter() {
        let old = std::mem::replace(&mut out, vec![]);
        for (ost, oet) in old.into_iter() {
            if ost > et || oet < st {
                out.push( (ost, oet) );
            } else {
                st = std::cmp::min(ost, st);
                et = std::cmp::max(oet, et);
            }
        }

        out.push( (st, et) );
        //println!("{:?}", out);
    }

    out
}

pub fn p056_merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals_checked = vec![];
    for interv in intervals.into_iter() {
        assert!(interv.len() == 2, "format not right");
        intervals_checked.push( (interv[0], interv[1]) );
    }

    let mut out = vec![];
    for (st, et) in p056_merge_(intervals_checked).into_iter() {
        out.push(vec![st, et]);
    }
    out
}

pub fn p057_insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    intervals.push(new_interval);
    let mut out = p056_merge(intervals);
    out.sort();
    out
}

