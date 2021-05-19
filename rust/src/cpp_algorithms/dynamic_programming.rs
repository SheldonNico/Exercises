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

fn longest_common_subsequence_from(text1: &[u8], text2: &[u8], mem: &mut HashMap<(usize, usize), usize>) -> usize {
    if text1.len() == 0 || text2.len() == 0 { return 0; }
    let k1 = text1.len(); let k2 = text2.len();
    if let Some(v) = mem.get(&(k1, k2)) { return *v; }

    let w1 = text1[0];
    let w2 = text2[0];

    let v = if w1 == w2 {
        1 + longest_common_subsequence_from(&text1[1..], &text2[1..], mem)
    } else {
        let v1 = longest_common_subsequence_from(&text1[1..], text2, mem);
        let v2 = longest_common_subsequence_from(text1, &text2[1..], mem);
        v1.max(v2)
    };
    mem.insert((k1, k2), v);
    v
}

pub fn longest_common_subsequence(text1: &str, text2: &str) -> usize {
    let mut mem = HashMap::new();
    longest_common_subsequence_from(text1.as_bytes(), text2.as_bytes(), &mut mem)
}

pub fn longest_common_subsequence_norec(text1: &str, text2: &str) -> usize {
    let text1 = text1.as_bytes(); let text2 = text2.as_bytes();

    let mut sol = vec![vec![0; text2.len()+1]; text1.len()+1];
    for i in 1..text1.len() + 1 {
        for j in 1..text2.len() + 1 {
            if text1[i-1] == text2[j-1] {
                sol[i][j] = 1 + sol[i-1][j-1];
            } else {
                sol[i][j] = sol[i-1][j].max(sol[i][j-1])
            }
        }
    }
    sol[text1.len()][text2.len()]
}

pub fn longest_increasing_subsequence(nums: &[i32]) -> i32 {
    assert!(nums.len() > 0);

    let mut sol = vec![1; nums.len()];
    for idx in 1..nums.len() {
        for s in 0..idx {
            if nums[s] < nums[idx] && sol[idx] < sol[s] + 1{
                sol[idx] = sol[s] + 1;
            }
        }
    }

    sol.into_iter().max().unwrap()
}

pub fn matrix_chain_multiplication_from(values: &[usize], sol: &mut Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    if j <= i + 1 { return 0; }
    if sol[i][j] == 0 {
        let mut min = std::usize::MAX;
        for s in i+1..j {
            let mut cost = matrix_chain_multiplication_from(values, sol, i, s);
            cost += matrix_chain_multiplication_from(values, sol, s, j);
            cost += values[i] * values[s] * values[j];
            if min > cost { min = cost; }
        }
        sol[i][j] = min;
    }

    sol[i][j]
}

pub fn matrix_chain_multiplication(values: &[usize]) -> usize {
    assert!(values.len() >= 3);
    let mut sol = vec![vec![0; values.len()]; values.len()];
    matrix_chain_multiplication_from(values, &mut sol, 0, values.len()-1)
}

pub fn palindrome_partitioning_solve(s: &str) -> usize {
    let s = s.as_bytes();
    let mut is_palindrome = vec![vec![false; s.len()]; s.len()];
    let mut cuts = vec![vec![0; s.len()]; s.len()];

    // NOTE: the [i][j] means [i, j], closed on both side
    for i in 0..s.len() {
        is_palindrome[i][i] = true;
        cuts[i][i] = 0;
    }

    for len in 2..s.len()+1 {
        for start_index in 0..s.len() - len + 1 {
            let end_index = start_index + len - 1;

            if len == 2 {
                is_palindrome[start_index][end_index] = s[start_index] == s[end_index];
            } else {
                is_palindrome[start_index][end_index] = s[start_index] == s[end_index] && is_palindrome[start_index+1][end_index-1];
            }

            if is_palindrome[start_index][end_index] {
                cuts[start_index][end_index] = 0;
            } else {
                cuts[start_index][end_index] = std::usize::MAX;
                for s in start_index..end_index {
                    let n = cuts[start_index][s] + cuts[s+1][end_index] + 1;
                    if n < cuts[start_index][end_index] {
                        cuts[start_index][end_index] = n;
                    }
                }
            }
        }
    }

    cuts[0][s.len()-1]
}

pub fn shortest_common_supersequence_tricky(str1: &str, str2: &str) -> String {
    let str1: Vec<_> = str1.chars().collect(); let str2: Vec<_> = str2.chars().collect();

    let mut lookup = vec![vec![0; str2.len()+1]; str1.len()+1];
    for i in 1..str1.len()+1 {
        for j in 1..str2.len()+1 {
            if str1[i-1] == str2[j-1] {
                lookup[i][j] = 1 + lookup[i-1][j-1]
            } else {
                lookup[i][j] = lookup[i-1][j].max(lookup[i][j-1]);
            }
        }
    }

    let mut i = str1.len(); let mut j = str2.len(); let mut s = "".to_string();
    while i > 0 && j > 0 {
        if str1[i-1] == str2[j-1] {
            s.push(str1[i-1]);
            i -= 1;
            j -= 1;
        } else {
            if lookup[i-1][j] > lookup[i][j-1] {
                s.push(str1[i-1]);
                i -= 1;
            } else {
                s.push(str2[j-1]);
                j -= 1;
            }
        }
    }

    while i > 0 {
        s.push(str1[i-1]);
        i -= 1;
    }
    while j > 0 {
        s.push(str2[j-1]);
        j -= 1;
    }

    s.chars().rev().collect()
}

pub fn shortest_common_supersequence_solve(str1: &str, str2: &str) -> String {
    let str1: Vec<_> = str1.chars().collect(); let str2: Vec<_> = str2.chars().collect();

    // let mut shortest = vec![vec![vec![]; str2.len()+1]; str1.len()+1];

    // for i in 0..str1.len()+1 {
    //     shortest[i][0] = str1[..i].into();
    // }
    // for j in 0..str2.len()+1 {
    //     shortest[0][j] = str2[..j].into();
    // }

    let mut last_row: Vec<Vec<char>> = vec![]; let mut curr_row = vec![];
    for j in 0..str2.len() + 1 {
        last_row.push(str2[..j].into());
        curr_row.push(vec![]);
    }

    for i in 1..str1.len()+1 {
        curr_row[0] = str1[..i].into();
        for j in 1..str2.len()+1 {
            if str1[i-1] == str2[j-1] {
                // std::mem::swap(&mut curr_row[j], &mut last_row[j-1]);
                curr_row[j] = last_row[j-1].clone();
                curr_row[j].push(str1[i-1]);
                // println!(">>>>> {:?} {} {} {} {}", curr_row[j], i, j, str1[i-1], str2[j-1]);

                // shortest[i][j] = shortest[i-1][j-1].clone();
                // shortest[i][j].push(str1[i-1]);
            } else {
                if last_row[j].len() >= curr_row[j-1].len() {
                    curr_row[j] = curr_row[j-1].clone(); // previous value in curr_row can not be changed
                    curr_row[j].push(str2[j-1]);
                } else {
                    curr_row[j] = last_row[j].clone();
                    curr_row[j].push(str1[i-1]);
                }

                // if shortest[i-1][j].len() >= shortest[i][j-1].len() {
                //     shortest[i][j] = shortest[i][j-1].clone();
                //     shortest[i][j].push(str2[j-1]);
                // } else {
                //     shortest[i][j] = shortest[i-1][j].clone();
                //     shortest[i][j].push(str1[j-1]);
                // }
            }
        }
        std::mem::swap(&mut last_row, &mut curr_row);
    }

    last_row[str2.len()].iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_common_supersequence_sample() {
        assert_eq!(shortest_common_supersequence_solve("abac", "cab"), "cabac");
        assert_eq!(shortest_common_supersequence_solve("ABCXYZ", "ABZ"), "ABCXYZ");
        assert_eq!(shortest_common_supersequence_solve("ABZ", "ABCXYZ"), "ABCXYZ");
        assert_eq!(shortest_common_supersequence_solve("AGGTAB", "GXTXAYB"), "AGGXTXAYB");
        assert_eq!(shortest_common_supersequence_solve("X", "Y"), "XY");

        assert_eq!(shortest_common_supersequence_tricky("abac", "cab"), "cabac");
        assert_eq!(shortest_common_supersequence_tricky("ABCXYZ", "ABZ"), "ABCXYZ");
        assert_eq!(shortest_common_supersequence_tricky("ABZ", "ABCXYZ"), "ABCXYZ");
        assert_eq!(shortest_common_supersequence_tricky("AGGTAB", "GXTXAYB"), "AGGXTXAYB");
        assert_eq!(shortest_common_supersequence_tricky("X", "Y"), "XY");
    }


    #[test]
    fn palindrome_partitioning_sample() {
        assert_eq!(palindrome_partitioning_solve("nitik"), 2);
        assert_eq!(palindrome_partitioning_solve("ababbbabbababa"), 3);
        assert_eq!(palindrome_partitioning_solve("abdc"), 3);
    }
    // "a babbbab babab a"

    #[test]
    fn matrix_chain_multiplication_sample() {
        assert_eq!(matrix_chain_multiplication(&[10, 30, 5, 60]), 4500);
        assert_eq!(matrix_chain_multiplication(&[40, 20, 30, 10, 30]), 26000);
        assert_eq!(matrix_chain_multiplication(&[10, 20, 30, 40, 30]), 30000);
        assert_eq!(matrix_chain_multiplication(&[10, 20, 30]), 6000);
    }

    #[test]
    fn longest_increasing_subsequence_sample() {
        assert_eq!(longest_increasing_subsequence(&vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(longest_increasing_subsequence(&vec![7,7,7,7,7,7,7]), 1);
    }

    #[test]
    fn longest_common_subsequence_sample() {
        assert_eq!(longest_common_subsequence("abcd", "acd"), 3);
        assert_eq!(longest_common_subsequence_norec("abcd", "efg"), 0);
    }

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
