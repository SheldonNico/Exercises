pub fn p1295_find_numbers(nums: Vec<i32>) -> i32 {
    fn has_even_digits(mut num: i32) -> bool {
        let mut count = 1;
        while num > 9 {
            num = num / 10;
            count += 1;
        }

        count % 2 == 0
    }

    nums.iter().filter(|v| has_even_digits(**v)).count() as i32
}

pub fn p1313_decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut out = vec![];
    for chunk in nums.chunks_exact(2) {
        let freq = chunk[0] as usize; let num = chunk[1];
        out.extend(vec![num; freq]);
    }
    out
}

pub fn p1365_smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut nums_sort = nums.clone();
    nums_sort.sort();

    nums.iter().map(|num| {
        let mut idx = nums_sort.binary_search(num).unwrap();
        while idx > 0 && nums_sort[idx-1] == nums_sort[idx] {
            idx -= 1;
        }
        idx as i32
    }).collect()
}

// dp
pub fn p0315_count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let mut out = vec![]; let mut curr = vec![];
    for num in nums.into_iter().rev() {
        let idx = match curr.binary_search(&num) {
            Ok(mut pos) => {
                while pos > 0 && curr[pos-1] == curr[pos] {
                    pos -= 1
                }
                pos
            },
            Err(pos) => {
                pos
            }
        };

        out.push(idx as i32);
        curr.insert(idx, num);
    }
    out.into_iter().rev().collect()
}

pub fn p1389_create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut out = vec![];
    for (num, idx) in nums.into_iter().zip(index.into_iter()) {
        out.insert(idx as usize, num);
    }
    out
}

pub fn p1266_min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    if points.len() == 0 { return 0; }
    let (mut sx, mut sy) = (points[0][0], points[0][1]);
    let mut cum = 0; let mut mem = Default::default();

    fn distance(point: (i32, i32), mem: &mut std::collections::HashMap<(i32, i32), i32>) -> i32 {
        if mem.contains_key(&point) { return mem[&point]; }
        let (x, y) = point;
        //if x == y { return x; }
        //if x == 0 { return y; }
        //if y == 0 { return x; }
        //if x < y  { return y; }
        //if x > y  { return x; }
        return x.max(y)

        //let mut d = i32::max_value();
        //if x > 0          { d = d.min(distance(((x-1).abs(), y          ), mem)); }
        //if y > 0          { d = d.min(distance(( x         , (y-1).abs()), mem)); }
        //if y > 0 && x > 0 { d = d.min(distance(((x-1).abs(), (y-1).abs()), mem)); }

        //mem.insert(point, d+1);
        //d + 1
    }

    for point in points.into_iter().skip(1) {
        let next = ((sx-point[0]).abs(), (sy-point[1]).abs());
        cum += distance(next, &mut mem);
        sx = point[0]; sy = point[1];
    }

    cum
}

pub fn p1299_replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut out = Vec::with_capacity(arr.len()); let mut curr = -1;

    for num in arr.into_iter().rev() {
        out.push(curr);
        curr = curr.max(num);
    }
    out.reverse();
    out
}

pub fn p1051_height_checker(heights: Vec<i32>) -> i32 {
    let mut target = heights.clone(); target.sort(); let mut moved = 0;
    for (n, t) in heights.iter().zip(target.iter()) {
        if n != t {
            moved += 1;
        }
    }
    moved
}

pub fn p0189_rotate(nums: &mut Vec<i32>, k: i32) {
    let k = (k as usize) % nums.len();
    nums.rotate_right(k);
}

pub fn p1252_odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
    let mut mat = vec![vec![0; m as usize]; n as usize];
    for point in indices.iter() {
        for i in 0..(m as usize) {
            mat[point[0] as usize][i] += 1;
        }
        for i in 0..(n as usize) {
            mat[i][point[1] as usize] += 1;
        }
    }
    let mut count = 0;
    for arr in mat.iter() {
        for v in arr.iter() {
            if v % 2 != 0 {
                count += 1;
            }
        }
    }

    count
}

pub fn p0229_majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut count = std::collections::HashMap::new();
    for num in nums.iter() {
        *(count.entry(*num).or_insert(0)) += 1;
    }

    let len = nums.len() / 3;
    let mut out : Vec<_> = count.into_iter().filter(|(_, v)| v > &len).map(|(k, _)| k).collect();
    out.sort();
    out
}

pub fn p1304_sum_zero(n: i32) -> Vec<i32> {
    let mut out = vec![]; let mut count = 0;
    if n == 0 { return vec![]; }
    while out.len() < n as usize - 1 {
        if count > 0 {
            count = -count;
        } else {
            count = -count + 1;
        }
        out.push(count);
    }
    if count > 0 {
        out.push(-count);
    } else {
        out.push(0);
    }
    out
}

pub fn p0126_find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    fn compare(s1: &str, s2: &str) -> bool {
        if s1.len() != s2.len() { return false; }

        let mut count = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 { count += 1; }
            if count > 1 { return false; }
        }
        return count == 1;
    }

    assert_ne!(begin_word, end_word);
    let mut curr : std::collections::HashMap<&str, Vec<Vec<usize>>> = Default::default();
    let mut nowalked = std::collections::HashSet::new();

    let mut count = 0;
    for (idx, word) in word_list.iter().enumerate() {
        if *word == end_word {
            count += 1;
        }
        if compare(word, &begin_word) {
            curr.entry(word).or_default().push(vec![idx]);
            if *word == end_word {
                return vec![vec![begin_word, end_word]];
            }
        } else {
            nowalked.insert(idx);
        }
    }
    if count == 0 { return vec![] }

    for _ in 0..nowalked.len() {
        if curr.len() == 0 { return vec![]; }
        let old = std::mem::replace(&mut curr, Default::default());
        let mut walked = std::collections::HashSet::new();
        for (curr_str, stacks) in old.into_iter() {
            for idx in nowalked.iter() {
                let next_str = &word_list[*idx];
                if compare(next_str, curr_str) {
                    walked.insert(*idx);
                    let ptr = curr.entry(next_str).or_default();
                    for mut stack in stacks.clone().into_iter() {
                        stack.push(*idx);
                        ptr.push(stack);
                    }
                }
            }
        }

        if curr.contains_key(&end_word.as_ref()) {
            let mut out = vec![];
            for stacks in curr.get(&end_word.as_ref()).into_iter() {
                for stack in stacks {
                    let mut trace = vec![begin_word.clone()];
                    for sid in stack.iter() {
                        trace.push(word_list[*sid].clone());
                    }
                    out.push(trace);
                }
            }
            return out;
        }

        nowalked = nowalked.difference(&walked).map(Clone::clone).collect();
    }

    vec![]
}

pub fn p0121_max_profit(prices: Vec<i32>) -> i32 {
    let mut max = 0;
    for idx in 0..prices.len() {
        for jdx in idx..prices.len() {
            max = max.max(prices[jdx] - prices[idx]);
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1295() {
        assert_eq!(p1295_find_numbers(vec![12,345,2,6,7896]), 2);
    }

    #[test]
    fn test_p0315() {
        assert_eq!(p0315_count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
    }

    #[test]
    fn test_p1266() {
        assert_eq!(p1266_min_time_to_visit_all_points(vec![vec![0,0], vec![5,0]]), 5);
        assert_eq!(p1266_min_time_to_visit_all_points(vec![vec![3,2], vec![-2,2]]), 5);
        assert_eq!(p1266_min_time_to_visit_all_points(vec![vec![1,1], vec![3,4], vec![-1,0]]), 7);
        assert_eq!(p1266_min_time_to_visit_all_points(
                vec![vec![127,915], vec![-137,18], vec![995,782], vec![697,-219], vec![-225,515], vec![-507,-155], vec![-350,756], vec![-170,242], vec![-738,359], vec![596,-642], vec![-491,175], vec![-41,813], vec![-339,135], vec![55,-381], vec![459,360], vec![-252,547], vec![-228,718], vec![538,198], vec![115,677], vec![190,-883], vec![-88,42], vec![358,-840], vec![-140,76], vec![-395,770], vec![-273,-186], vec![-227,-262], vec![822,-74], vec![-126,432], vec![492,695], vec![434,-799], vec![-269,614], vec![348,-666], vec![741,875], vec![909,490], vec![601,840], vec![-563,942], vec![849,523], vec![809,-263], vec![809,454], vec![482,26], vec![-295,636], vec![-162,-384], vec![813,-524], vec![-863,-58], vec![798,-351], vec![-123,78], vec![-410,-777], vec![-928,-938], vec![-78,792], vec![289,-434], vec![-513,37], vec![-615,377], vec![655,593], vec![491,182], vec![-594,-536], vec![367,-130], vec![-888,-491], vec![958,-566], vec![466,967], vec![280,166], vec![-856,-233], vec![-304,194], vec![-479,722], vec![877,610], vec![-599,-68], vec![-430,-249]]
                ), 60222);
        assert_eq!(p1266_min_time_to_visit_all_points(vec![vec![3,2], vec![6,4]]), 3);
    }

    #[test]
    fn test_p1051() {
        assert_eq!(p1051_height_checker(vec![1,1,4,2,1,3]), 3);
        assert_eq!(p1051_height_checker(vec![5,1,2,3,4]), 5);
        assert_eq!(p1051_height_checker(vec![1,2,3,4,5]), 0);
    }

    #[test]
    fn test_p1252() {
        assert_eq!(p1252_odd_cells(2, 3, vec![vec![0,1], vec![1,1]]), 6);
        assert_eq!(p1252_odd_cells(2, 2, vec![vec![1,1], vec![0,0]]), 0);
    }

    #[test]
    fn test_p0229() {
        assert_eq!(p0229_majority_element(vec![1,1,1,3,3,2,2,2]), vec![1, 2]);
        assert_eq!(p0229_majority_element(vec![3,2,3]), vec![3]);
    }

    #[test]
    fn test_p0126() {
        fn test(s: &str, e: &str, list: Vec<&str>, target: Vec<Vec<&str>>) {
            let out = p0126_find_ladders(s.to_string(), e.to_string(), list.into_iter().map(|s| s.to_string()).collect());
            let mut out: Vec<String> = out.into_iter().map(|sl| sl.into_iter().collect()).collect();
            let mut target: Vec<String> = target.into_iter().map(|sl| sl.into_iter().collect()).collect();
            out.sort(); target.sort();
            assert_eq!(out, target);
        }

        test(
            "hit", "cog",
            vec!["hot","dot","dog","lot","log","cog"],
            vec![ vec!["hit","hot","dot","dog","cog"],
                  vec!["hit","hot","lot","log","cog"]
            ]);


    }

    #[test]
    fn test_p0121() {
        assert_eq!(p0121_max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(p0121_max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
