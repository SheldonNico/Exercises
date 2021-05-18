use super::Timer;
use std::collections::HashMap;

fn knapsack_from<const N: usize>(idx: usize, curr_weight: usize, capacity: usize, weight: &[usize; N], value: &[usize; N]) -> usize {
    if idx >= N {
        return 0;
    }

    if curr_weight + weight[idx] <= capacity {
        let v1 = knapsack_from(idx+1, curr_weight + weight[idx], capacity, weight, value) + value[idx];
        let v2 = knapsack_from(idx+1, curr_weight, capacity, weight, value);
        return v1.max(v2)
    } else {
        return knapsack_from(idx+1, curr_weight, capacity, weight, value);
    }
}

pub fn knapsack_solve<const N: usize>(capacity: usize, weight: &[usize; N], value: &[usize; N]) -> usize {
    knapsack_from(0, 0, capacity, weight, value)
}

pub fn knapsack_solve_norec<const N: usize>(capacity: usize, weight: &[usize; N], value: &[usize; N]) -> usize {
    let mut max_value = vec![vec![0; capacity+1]; N+1];
    for item in 0..N+1 {
        for cap in 0..capacity+1 {
            if item == 0 || cap == 0 {
                max_value[item][cap] = 0;
            } else if weight[item-1] <= cap {
                let p1 = value[item-1] + max_value[item-1][cap-weight[item-1]];
                let p2 = max_value[item-1][cap];
                max_value[item][cap] = p1.max(p2);
            } else {
                max_value[item][cap] = max_value[item - 1][cap];
            }
        }
    }
    max_value[N][capacity]
}

fn abbreviation_from(input: &[char], output: &[char]) -> bool {
    if output.len() == 0 {
        for c in input.iter() {
            if c.is_uppercase() {
                return false;
            }
        }
        return true;
    }

    if input.len() == 0 {
        return false;
    }

    let a = input[0]; let b = output[0];

    if a.is_uppercase() {
        if a == b {
            return abbreviation_from(&input[1..], &output[1..]);
        } else {
            return false;
        }
    } else {
        if a.to_ascii_uppercase() == b.to_ascii_uppercase() {
            return abbreviation_from(&input[1..], output) || abbreviation_from(&input[1..], &output[1..]);
        } else {
            return abbreviation_from(&input[1..], output);
        }
    }
}

pub fn abbreviation(input: &str, output: &str) -> bool {
    let input: Vec<char> = input.chars().collect();
    let output: Vec<char> = output.chars().collect();
    abbreviation_from(&input, &output)
}

pub fn armstrong_number(mut num: usize) -> bool {
    let mut s = 0; let n = num;
    while num > 0 {
        let r = num % 10;
        s += r * r * r;
        num = num / 10;
    }
    s == n
}

pub fn bellman_fold(graph: Vec<(usize, usize, i32)>, vertex_num: usize, start: usize) {
    let mut dist = vec![std::i32::MAX; vertex_num];
    dist[start] = 0;

    for _ in 0..vertex_num {
        for &(u, v, w) in graph.iter() {
            if dist[u] != std::i32::MAX && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }

    for &(u, v, w) in graph.iter() {
        if dist[u] != std::i32::MAX && dist[u] + w < dist[v] {
            eprintln!("Graph contains negative weight cycle. Hence, shortest distance not guaranteed.");
        }
    }

    for (to, d) in dist.iter().enumerate() {
        println!("{} -> {}: {}", start, to, d);
    }
}

pub fn catalan_number(n: usize) -> usize {
    let mut init = 1;
    for i in 1..n+1 {
        init = init * (2*i-1) * 2 / (i + 1);
    }
    init
}

pub fn coin_change_solve(coins: &[i32], target: i32) -> i32 {
    assert!(target >= 0);
    let mut mem = vec![-1; target as usize + 1];
    mem[0]= 0;

    for target in 1..mem.len() {
        for &coin in coins.iter() {
            if target >= coin as usize {
                if mem[target - coin as usize] >= 0 {
                    let next = mem[target - coin as usize] + 1;
                    if mem[target] >= 0 {
                        mem[target] = mem[target].min(next);
                    } else {
                        mem[target] = next;
                    }
                }
            }
        }
    }

    mem[target as usize]
}

pub fn cut_rod_solve(prices: &[usize], limit: usize) -> usize {
    assert!(prices.len() > 0);

    let mut window = vec![0; prices.len()];
    for curr in 1..limit+1 {
        let mut res = 0;
        for o in 0..window.len() {
            if o < curr  {
                res = res.max(prices[o] + window[o]);
            }
        }
        window.rotate_right(1);
        window[0] = res;
        // println!("{:?}", window);
    }

    window[0]
}

fn p1547_min_cost_from(sx: i32, ed: i32, cuts: &Vec<i32>, stacked: &mut HashMap<(i32, i32), i32>) -> i32 {
    if let Some(v) = stacked.get(&(sx, ed)) { return *v; }

    let mut res = vec![];
    for &cut in cuts.iter() {
        if sx < cut && cut < ed {
            let v1 = p1547_min_cost_from(sx, cut, cuts, stacked);
            let v2 = p1547_min_cost_from(cut, ed, cuts, stacked);
            res.push(v1 + v2 + ed - sx)
        }
    }

    let r = res.into_iter().min().unwrap_or(0);
    stacked.insert((sx, ed), r);
    r
}

pub fn p1547_min_cost(n: i32, cuts: Vec<i32>) -> i32 {
    let mut stacked = HashMap::new();
    p1547_min_cost_from(0, n, &cuts, &mut stacked)
}

fn edit_distance_from2(word1: &[u8], word2: &[u8], limit: usize) -> usize {
    if word1 == word2 { return 0; }

    if word1.len() == 0 { return word2.len().min(limit); } // insert
    if word2.len() == 0 { return word1.len().min(limit); } // delete

    let wx = word1[0];
    let wy = word2[0];

    if limit > 0 {
        if wx == wy {
            // ?? why
            edit_distance_from2(&word1[1..], &word2[1..], limit)
        } else {
            let l1 = edit_distance_from2(&word1[1..], &word2[1..], limit-1); // replace
            let l2 = edit_distance_from2(word1,       &word2[1..], l1); // insert
            let l3 = edit_distance_from2(&word1[1..], word2,       l2);   // delete
            l3 + 1
        }
    } else {
        0
    }
}

fn edit_distance_from(word1: &[u8], word2: &[u8], mem: &mut HashMap<(usize, usize), usize>) -> usize {
    let w1 = word1.len(); let w2 = word2.len();

    let v = if mem.contains_key(&(w1, w2)) {
        *mem.get(&(w1, w2)).unwrap()
    } else if word1 == word2 {
        0
    } else if word1.len() == 0 {
        word2.len() // insert
    } else if word2.len() == 0 {
        word1.len() // delete
    } else {
        let wx = word1[0];
        let wy = word2[0];

        if wx == wy {
            // ?? why
            edit_distance_from(&word1[1..], &word2[1..], mem)
        } else {
            let l1 = edit_distance_from(&word1[1..], &word2[1..], mem); // replace
            let l2 = edit_distance_from(word1,       &word2[1..], mem); // insert
            let l3 = edit_distance_from(&word1[1..], word2,       mem); // delete
            l1.min(l2).min(l3) + 1
        }
    };

    mem.insert((w1, w2), v);

    v

}

pub fn edit_distance_solve2(from: &str, to: &str) -> usize {
    let limit = from.len().max(to.len());
    edit_distance_from2(from.as_bytes(), to.as_bytes(), limit)
}

pub fn edit_distance_solve(from: &str, to: &str) -> usize {
    let mut mem = HashMap::new();
    let r = edit_distance_from(from.as_bytes(), to.as_bytes(), &mut mem);
    r
}

// leetcode won't pass since it's too slow...
fn egg_drop(egg: usize, floor: usize) -> usize {
    if floor == 0 { return 0; }
    let mut sol = vec![vec![0; floor + 1]; egg + 1];

    for e in 0..egg+1 {
        sol[e][1] = 1;
        sol[e][0] = 0;
    }

    for f in 0..floor+1 {
        sol[1][f] = f;
    }

    for e in 2..egg+1 {
        for f in 2..floor+1 {
            // if e == 1 {
            //     sol[e][f] = f;
            // } else if f == 1 {
            //     sol[e][f] = 1;
            // } else {
                // egg >= 2 f >= 2

                sol[e][f] = std::usize::MAX;
                for curr in 1..f {
                    let out = 1 + sol[e][f-curr].max(sol[e-1][curr-1]);
                    sol[e][f] = sol[e][f].min(out);
                }
                // sol[e][f] = 1 + sol[e][f-1].max(sol[e-1][f-1]);

                // sol[e][f] = sol[e][f-1].min(1 + sol[e][1].max(sol[e-1][f-2]))
                // sol[e][f] = (sol[e-1][f-1] + 1).max(1); // .min(1 + sol[e][1].max(sol[e-1][f-2]))
            // }
        }
    }

    // println!("{:?}", sol);

    sol[egg][floor]
}

pub fn fibonacci(n: usize) -> usize {
    if n == 0 { return 0; }
    let mut a = 0; let mut b = 1;
    for _ in 0..n-1 {
        let tmp = b;
        b = a +b;
        a = tmp;
    }
    b
}

pub fn floyd_warshall() {
    todo!()
}

pub fn max_sub_array(nums: &Vec<i32>) -> i32 {
    if nums.len() == 0 { return 0 }
    let mut cont_sum = nums.clone();

    for idx in 1..nums.len() {
        cont_sum[idx] = nums[idx].max(nums[idx] + cont_sum[idx-1]);
    }
    cont_sum.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_sub_array_sample() {
        assert_eq!(max_sub_array(&vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
        assert_eq!(max_sub_array(&vec![1]), 1);
        assert_eq!(max_sub_array(&vec![-1]), -1);
        assert_eq!(max_sub_array(&vec![5,4,-1,7,8]), 23);
    }

    #[test]
    fn fibonacci_sample() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    fn egg_drop_sample() {
        assert_eq!(egg_drop(8, 1000), 10);
        assert_eq!(egg_drop(8, 10000), 14);

        assert_eq!(egg_drop(2, 2), 2);
        assert_eq!(egg_drop(1, 2), 2);
        assert_eq!(egg_drop(2, 6), 3);
        assert_eq!(egg_drop(3, 14), 4);
    }

    #[test]
    fn edit_distance_sample() {
        {
            let _t = Timer::new("with mem");
            assert_eq!(edit_distance_solve("intention", "execution"), 5);
            assert_eq!(edit_distance_solve("horse", "ros"), 3);
            assert_eq!(edit_distance_solve("dinitrophenylhydrazine", "acetylphenylhydrazine"), 6);
            assert_eq!(edit_distance_solve(
                    "pneumonoultramicroscopicsilicovolcanoconiosis",
                    "ultramicroscopically"
            ), 27);
        }

        {
            let _t = Timer::new("with eaily drop");
            assert_eq!(edit_distance_solve2("intention", "execution"), 5);
            assert_eq!(edit_distance_solve2("horse", "ros"), 3);
            assert_eq!(edit_distance_solve2("dinitrophenylhydrazine", "acetylphenylhydrazine"), 6);
            // too long
            // assert_eq!(edit_distance_solve2(
            //         "pneumonoultramicroscopicsilicovolcanoconiosis",
            //         "ultramicroscopically"
            // ), 27);
        }
    }

    #[test]
    fn p1547_min_cost_sample() {
        assert_eq!(p1547_min_cost(7, vec![1, 3, 4, 5]), 16);
        assert_eq!(p1547_min_cost(9, vec![5,6,1,4,2]), 22);
    }

    #[test]
    fn cut_rod_solve_sample() {
        assert_eq!(cut_rod_solve(&[2,9,17,23,45], 5), 45);
        assert_eq!(cut_rod_solve(&[
                1,  5,  8,  9,  10, 17, 17, 20, 24, 30,  // price array
                31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
                41, 42, 43, 44, 45, 46, 47, 48, 49, 50
        ], 30), 90);
        assert_eq!(cut_rod_solve(&[1, 2, 4, 6, 8, 45, 21, 9], 8), 47);
        assert_eq!(cut_rod_solve(&[3, 5, 8, 9, 10, 17, 17, 20], 8), 24);
        assert_eq!(cut_rod_solve(&[1, 5, 8, 9, 10, 17, 17, 20], 8), 22);
        assert_eq!(cut_rod_solve(&[1, 5, 8], 3), 8);
    }

    #[test]
    fn coin_change_sample() {
        let coins = vec![1, 2, 5];
        assert_eq!(coin_change_solve(&coins, 0), 0);
        assert_eq!(coin_change_solve(&coins, 11), 3);

        assert_eq!(coin_change_solve(&[1, 2, 5], 100), 20);
        assert_eq!(coin_change_solve(&[1, 2, 3, 4], 15), 4);
        assert_eq!(coin_change_solve(&[186,419,83,408], 6249), 20);
    }

    #[test]
    fn catalan_number_sample() {
        assert_eq!(catalan_number(0), 1);
        assert_eq!(catalan_number(1), 1);
        assert_eq!(catalan_number(2), 2);
        assert_eq!(catalan_number(3), 5);
        assert_eq!(catalan_number(4), 14);
        assert_eq!(catalan_number(5), 42);
    }

    #[test]
    fn knapsack_sample() {
        let weight = [10, 20, 30];
        let value = [60, 100, 120];
        let capacity = 50;
        let expected = 220;
        assert_eq!(knapsack_solve(capacity, &weight, &value), expected);
        assert_eq!(knapsack_solve_norec(capacity, &weight, &value), expected);

        let weight = [24, 10, 10, 7];
        let value = [24, 18, 18, 10];
        let capacity = 25;
        let expected = 36;
        assert_eq!(knapsack_solve(capacity, &weight, &value), expected);
        assert_eq!(knapsack_solve_norec(capacity, &weight, &value), expected);

        let weight = [
            382745, 799601, 909247, 729069, 467902,  44328,  34610, 698150, 823460, 903959, 853665, 551830, 610856,
            670702, 488960, 951111, 323046, 446298, 931161,  31385, 496951, 264724, 224916, 169684,
        ];
        let value = [
            825594, 1677009, 1676628, 1523970,  943972,   97426,   69666, 1296457, 1679693, 1902996, 1844992, 1049289,
            1252836, 1319836,  953277, 2067538,  675367,  853655, 1826027,   65731,  901489,  577243,  466257,  369261,
        ];
        let capacity = 6404180;
        let expected = 13549094;
        {
            let _t = Timer::new("with recursion");
            assert_eq!(knapsack_solve(capacity, &weight, &value), expected);
        }
        {
            let _t = Timer::new("without recursion");
            assert_eq!(knapsack_solve_norec(capacity, &weight, &value), expected);
        }
    }

    #[test]
    fn abbreviation_sample() {
        let s = "daBcd"; let t = "ABC";
        assert!(abbreviation(s, t));

        let s = "DRFNLZZVHLPZWIupjwdmqafmgkg"; let t = "DRFNLZZVHLPZWI";
        assert!(abbreviation(s, t));
    }

    #[test]
    fn armstrong_number_sample() {
        assert!(armstrong_number(153));
        assert!(armstrong_number(370));
        assert!(armstrong_number(371));
        assert!(armstrong_number(407));
        assert!(!armstrong_number(22));
    }

    #[test]
    fn bellman_fold_sample() {
        let graph= vec![(0, 1, -1), (0, 2, 4), (1, 2, 3), (1, 3, 2), (1, 4, 2), (3, 2, 5), (3, 1, 1), (4, 3, -3)];
        bellman_fold(graph, 5, 0);
    }
}
