use std::collections::{HashSet, HashMap};

pub fn p0080_remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut left = 2; let mut right = nums.len();

    while left < right {
        if nums[left-1] == nums[left] && nums[left-2] == nums[left] {
            let mut len = 1;
            while left+len < nums.len() && nums[left+len] == nums[left] {
                len += 1;
            }

            nums[left..right].rotate_left(len);
            right -= len;
        } else {
            left += 1;
        }
    }

    right as i32
}

pub fn p0081_search(nums: &[i32], target: i32) -> bool {
    let left = 0; let right = nums.len();
    if right == 0 { return false; }
    if right == 1 { return nums[0] == target; }
    if nums[left] < nums[right-1] {
        if nums[left] <= target && nums[right-1] >= target {
            return nums.binary_search(&target).is_ok();
        } else {
            return false;
        }
    }

    let mid = (left + right) / 2;
    return p0081_search(&nums[left..mid], target) || p0081_search(&nums[mid..right], target)
}

pub fn p0081_search_faster(nums: &[i32], target: i32) -> bool {
    if nums.len() == 0 { return false; }
    let mut left = 0; let mut right = nums.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if nums[mid] == target {
            return true;
        }
        if left == right { return false }

        if nums[left] == nums[mid] && nums[right] == nums[mid] {
            left += 1;
            right -= 1;
        } else if nums[left] <= nums[mid] {
            if nums[left] <= target && target < nums[mid] {
                right = mid-1;
            } else {
                left = mid+1;
            }
        } else {
            if nums[mid] < target && target <= nums[right] {
                left = mid+1;
            } else {
                right = mid-1;
            }
        }
    }

    return false;
}

pub fn p0084_largest_rectangle_area(heights: &[usize]) -> usize {
    let mut leftmost: Vec<usize> = vec![];
    let mut stacked = 0;

    for idx in 0..heights.len() {
        let height = heights[idx];
        if leftmost.len() > height {
            for (shift, left) in leftmost.drain((height+1)..).enumerate() {
                let area = (idx - left) * (shift + height +1);
                if area > stacked {
                    stacked = area;
                }
            }
        } else {
            leftmost.append(&mut vec![idx; height+1-leftmost.len()]);
        }
    }

    let width = heights.len();
    for (height, idx) in leftmost.into_iter().enumerate() {
        let area = (width - idx) * height;
        if area > stacked {
            stacked = area;
        }
    }

    stacked
}

pub fn p0085_maximal_rectangle(matrix: &Vec<Vec<char>>) -> usize {
    let nrows = matrix.len();
    if nrows == 0 { return 0; }
    let ncols = matrix[0].len();
    if ncols == 0 { return 0; }

    let mut heights = vec![0; ncols];
    let mut area = 0;

    for row in  0..nrows {
        for col in 0..ncols {
            if matrix[row][col] == '1' {
                heights[col] += 1;
            } else {
                heights[col] = 0;
            }
        }
        let next = p0084_largest_rectangle_area(&heights);
        if next > area {
            area = next;
        }
    }

    area
}

fn p0087_is_scramble_rec(
    sd0: usize, sd1: usize, len: usize, mem: &mut HashMap<(usize, usize, usize), bool>,
    s1: &[char], s2: &[char]
) -> bool {
    // println!("{:?} {:?}", &s1[sd0..sd0+len], &s2[sd1..sd1+len]);
    if let Some(v) = mem.get(&(sd0, sd1, len)) {
        return *v;
    }

    let mut rtn = false;
    if s1[sd0..sd0+len] == s2[sd1..sd1+len] {
        rtn = true;
    } else {
        let mut left1: HashSet<_> = Default::default();
        let mut left2: HashSet<_> = Default::default();
        let mut right: HashSet<_> = Default::default();

        for idx in 0..len-1 {
            left1.insert(s1[sd0+idx]);
            left2.insert(s1[sd0+len-1-idx]);
            right.insert(s2[sd1+idx]);

            if left1 == right {
                if p0087_is_scramble_rec(sd0, sd1, idx+1, mem, s1, s2) &&
                    p0087_is_scramble_rec(sd0+idx+1, sd1+idx+1, len-idx-1, mem, s1, s2)
                {
                    rtn = true;
                    break;
                }
            }
            if left2 == right {
                if p0087_is_scramble_rec(sd0, sd1+idx+1, len-idx-1, mem, s1, s2) &&
                    p0087_is_scramble_rec(sd0+len-1-idx, sd1, idx+1, mem, s1, s2)
                {
                    rtn = true;
                    break;
                }
            }
        }

    }
    mem.insert((sd0, sd1, len), rtn);
    rtn

}

pub fn p0087_is_scramble(s1: &[char], s2: &[char]) -> bool {
    if s1.len() != s2.len() { return false; }

    let mut mem: HashMap<(usize, usize, usize), bool> = HashMap::new();

    p0087_is_scramble_rec(0, 0, s1.len(), &mut mem, s1, s2)
}

pub fn p0088_merge(nums1: &mut Vec<i32>, m: usize, nums2: &mut Vec<i32>, n: usize) {
    let len = nums1.len();
    assert!(n <= nums2.len());
    assert!(m+n == nums1.len());

    let mut left = 0; let mut right = 0;
    while left+right < m+n {
        // println!("{:?} {} {:?}", nums1, left, right);
        if right < n && left < m {
            if nums1[left+right] > nums2[right] {
                nums1[len-1] = nums2[right];
                nums1[left+right..].rotate_right(1);
                right += 1;
            } else {
                left += 1;
            }
        } else if right < n {
            nums1[left+right] = nums2[right];
            right += 1;
        } else {
            break;
        }
    }
}

pub fn p0089_gray_code_fast(n: i32) -> Vec<i32> {
    if n == 0 { return vec![] }
    let mut out = vec![0, 1];

    for idx in 0..n-1 {
        for inner in out.clone().into_iter().rev() {
            out.push(inner | (1 << ((idx+1) as usize)));
        }
    }
    out
}

pub fn p0089_gray_code_iter(n: usize) -> Vec<Vec<usize>> {
    if n == 0 { return vec![] }
    let mut out = vec![vec![0], vec![1]];
    for _ in 0..n-1 {
        let new = std::mem::replace(&mut out, vec![]);

        for mut inner in new.clone().into_iter() {
            inner.insert(0, 0);
            out.push(inner);
        }

        for mut inner in new.into_iter().rev() {
            inner.insert(0, 1);
            out.push(inner);
        }
    }
    out
}

pub fn p0089_gray_code_rec<'r>(n: usize, mem: &'r mut HashMap<usize, Vec<Vec<usize>>>) -> &'r Vec<Vec<usize>> {
    if !mem.contains_key(&n) {
        let rtn = match n {
            0 => vec![],
            1 => vec![vec![0], vec![1]],
            _ => {
                let mut out = vec![];
                let last = p0089_gray_code_rec(n-1, mem);

                for mut inner in last.clone().into_iter() {
                    inner.insert(0, 0);
                    out.push(inner);
                }

                for mut inner in last.clone().into_iter().rev() {
                    inner.insert(0, 1);
                    out.push(inner);
                }
                out
            }
        };

        mem.insert(n, rtn);
    }

    mem.get(&n).unwrap()
}

pub fn p0089_gray_code(n: i32) -> Vec<i32> {
    let mut mem: HashMap<_, _> = HashMap::new();
    p0089_gray_code_rec(n as usize, &mut mem).into_iter().map(|line| {
        let mut num = 0;
        for n in line.into_iter() {
            num = num*2 + n;
        }
        num as i32
    }).collect()
}

pub fn p0090_subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut out: HashSet<Vec<i32>> = vec![vec![]].into_iter().collect();
    for num in nums.into_iter() {
        for mut old in out.clone().into_iter() {
            old.push(num);
            // println!(">>> {:?}", old);
            out.insert(old);
        }
    }
    // println!("{:?}", out);
    out.into_iter().collect()
}

pub fn p0091_num_decodings_rec(s: &[char], st: usize, mem: &mut HashMap<usize, usize>) -> usize {
    if let Some(v) = mem.get(&st) { return *v; }
    if st < s.len() && s[st] == '0' { return 0; }

    let rtn = if st + 1 < s.len() {
        if s[st] == '1' || (s[st] == '2' && s[st+1] <= '6') {
            p0091_num_decodings_rec(s, st+1, mem) + p0091_num_decodings_rec(s, st+2, mem)
        } else {
            p0091_num_decodings_rec(s, st+1, mem)
        }
    } else if st < s.len() {
        p0091_num_decodings_rec(s, st+1, mem)
    } else {
        1
    };
    mem.insert(st, rtn);
    rtn
}

pub fn p0091_num_decodings(s: &[char]) -> i32 {
    let mut mem = Default::default();
    if s.len() == 0 { return 1; }
    p0091_num_decodings_rec(s, 0, &mut mem) as _
}

pub fn p0093_restore_ip_addresses(s: String) -> Vec<String> {
    let mut out: Vec<Vec<String>> = vec![vec!["".to_owned()]];
    for c in s.chars().into_iter() {
        for mut addr in std::mem::replace(&mut out, Default::default()).into_iter() {
            let mut addr_ = addr.clone();
            let len = addr.len();

            let last = addr.last_mut().unwrap();

            if let Some(_) = p0093_get_digits(last) {
                addr_.push(c.to_string());
                out.push(addr_);
            }

            if last.len() < 3 && len <= 4 {
                last.push(c);
                out.push(addr);
            }
        }
    }

    let out = out.into_iter().filter(|words| {
        words.len() == 4 && p0093_get_digits(words.last().unwrap()).is_some()
    }).map(|words| words.join(".")).collect();
    // println!("{:?}", out);
    out
}

fn p0093_get_digits(digits: &str) -> Option<u8> {
    if digits.len() == 0 { return None; }
    if digits.starts_with("0") && digits.len() > 1 { return None; }
    digits.parse::<u8>().ok()
}

fn p0097_is_interleave_rec(
    s1: &[char], s2: &[char], s3: &[char], n1: usize, n2: usize, mem: &mut HashMap<(usize, usize), bool>
) -> bool {
    if n1 == s1.len() { return s2[n2..] == s3[n1+n2..]; }
    if n2 == s2.len() { return s1[n1..] == s3[n1+n2..]; }

    if let Some(v) = mem.get(&(n1, n2)) { return *v; }

    let rtn = if s1[n1] == s3[n1+n2] && p0097_is_interleave_rec(
        s1, s2, s3, n1+1, n2, mem
    ) {
        true
    } else if s2[n2] == s3[n1+n2] && p0097_is_interleave_rec(
        s1, s2, s3, n1, n2+1, mem
    ) {
        true
    } else {
        false
    };
    mem.insert((n1, n2), rtn);

    rtn
}

pub fn p0097_is_interleave(s1: &[char], s2: &[char], s3: &[char]) -> bool {
    if s1.len() + s2.len() != s3.len() { return false; }
    let mut mem = Default::default();
    p0097_is_interleave_rec(s1, s2, s3, 0, 0, &mut mem)
}

pub fn p0118_generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut out = vec![vec![1]];
    for _ in 1..num_rows {
        let last = out.last().unwrap();
        let mut new = vec![1; last.len()+1];
        for idx in 1..new.len()-1 {
            new[idx] = last[idx-1] + last[idx];
        }
        out.push(new);
    }
    out
}

pub fn p0119_get_row(row_index: i32) -> Vec<i32> {
    let mut out = vec![1];
    for _ in 1..row_index+1 {
        let mut new = vec![1; out.len()+1];
        for idx in 1..new.len()-1 {
            new[idx] = out[idx-1]+out[idx];
        }
        out = new;
    }
    out
}


pub fn p0120_minimum_total_rec(triangle: &Vec<Vec<i32>>, row: usize, col: usize, mem: &mut HashMap<(usize, usize), i32>) -> i32 {
    if let Some(v) = mem.get(&(row, col)) { return *v; }
    if row == triangle.len()-1 { return triangle[row][col]; }

    let left = p0120_minimum_total_rec(triangle, row+1, col, mem);
    let right = p0120_minimum_total_rec(triangle, row+1, col+1, mem);
    let rtn = triangle[row][col] + left.min(right);
    mem.insert((row, col), rtn);

    rtn
}

pub fn p0120_minimum_total(triangle: &Vec<Vec<i32>>) -> i32 {
    let mut mem = Default::default();
    p0120_minimum_total_rec(triangle, 0, 0, &mut mem)
}

pub fn p0122_max_profit_faster(prices: &[i32]) -> i32 {
    let mut out = 0;
    for idx in 1..prices.len() {
        out += (prices[idx] - prices[idx-1]).max(0)
    }
    out
}

pub fn p0122_max_profit(prices: &[i32]) -> i32 {
    let len = prices.len();
    if len == 0 { return 0; }
    let mut profit = vec![0; prices.len()];
    for idx in (0..len-1).rev() {
        let mut max = profit[idx+1]; // do not use init point
        for idy in (idx+1)..len {
            let mut curr = (prices[idy]-prices[idx]).max(0);
            if idy+1 < len {
                curr += profit[idy+1];
            }
            if max < curr {
                max = curr;
            }
        }

        profit[idx] = max;
    }
    // println!("{:?}", profit);

    profit[0]
}

pub fn p0123_max_profit(prices: &[i32]) -> i32 {
    let mut profits = vec![];
    let mut diff = 0;

    for idx in 1..prices.len() {
        let curr = prices[idx] - prices[idx-1];
        if (curr >= 0 && diff >= 0) || (curr <= 0 && diff <= 0) {
            diff += curr;
        } else {
            profits.push(diff);
            diff = curr;
        }
    }
    if diff != 0 { profits.push(diff); }

    let len = profits.len();
    if len == 0 { return 0; }

    let mut profit_one = vec![0; len];
    let mut profit_two = vec![0; len];

    profit_one[len-1] = profits[len-1].max(0);
    profit_two[len-1] = profits[len-1].max(0);
    // 从 idx 必须要买入的累计最大值
    let mut cumsum_one = 0;
    let mut cumsum_two = 0;
    for idx in (0..len-1).rev() {
        cumsum_one = profits[idx] + cumsum_one.max(0);
        profit_one[idx] = profit_one[idx+1].max(cumsum_one).max(0);
        cumsum_two = profits[idx] + cumsum_two.max(profit_one[idx+1]);
        profit_two[idx] = 0.max(profit_two[idx+1]).max(cumsum_two).max(cumsum_one);
    }
    // println!("{:?}=> {:?} {:?}", profits, profit_one, profit_two);

    profit_two[0]
}
pub fn p0125_is_palindrome_fast(s: &str) -> bool {
    let iter = s.chars().filter(char::is_ascii_alphanumeric).map(|c| char::to_ascii_lowercase(&c));
    iter.clone().eq(iter.rev())
}

pub fn p0125_is_palindrome(s: &[char]) -> bool {
    if s.len() == 0 { return true; }
    let mut left = 0; let mut right = s.len()-1;
    while left < right {
        while left < right && !s[left].is_ascii_alphanumeric() {
            left += 1;
        }
        while right > left && !s[right].is_ascii_alphanumeric() {
            right -= 1;
        }
        if right <= left { break; }
        if s[left].to_ascii_lowercase() != s[right].to_ascii_lowercase() { return false; }

        left += 1; right -= 1;
    }

    true
}

fn p0127_compare(s1: &[char], s2: &[char]) -> usize {
    let mut count = 0;
    for (c1, c2) in s1.iter().zip(s2.iter()) {
        if c1 != c2 {
            count += 1;
        }
    }
    count
}

pub fn p0127_ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let begin_word: Vec<_> = begin_word.chars().collect();
    let end_word: Vec<_> = end_word.chars().collect();
    let mut word_vec = vec![begin_word.clone()];
    let start = 0; let mut end = 0;
    for (idx, word) in word_list.into_iter().enumerate() {
        let word: Vec<char> = word.chars().collect();
        if word == end_word {
            end = idx+1;
        }
        word_vec.push(word);
    }

    if word_vec[end] != end_word { return 0; }

    let mut grid: Vec<Vec<usize>> = vec![];
    for wid in 0..word_vec.len() {
        let mut to = vec![];
        for nid in 0..word_vec.len() {
            if p0127_compare(&word_vec[wid], &word_vec[nid]) == 1 {
                to.push(nid);
            }
        }
        grid.push(to);
    }

    let mut start: HashSet<usize> = vec![start].into_iter().collect();
    let mut count = 1;
    while start.len() < word_vec.len() {
        count += 1;
        // println!("{:?} {}", start, end);
        let mut new = vec![];
        for wid in start.iter() {
            for nid in grid[*wid].iter() {
                if *nid == end { return count; }
                if !start.contains(nid) {
                    new.push(*nid);
                }
            }
        }
        if new.len() == 0 { return 0; }
        for nid in new.into_iter() {
            start.insert(nid);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0127() {
        for (idx, (begin_word, end_word, word_list, expected)) in vec![
            ("hit", "cog", vec!["hot","dot","dog","lot","log","cog"], 5),
            ("hit", "cog", vec!["hot","dot","dog","lot","log"], 0),
        ].into_iter().enumerate() {
            assert_eq!(
                p0127_ladder_length(
                    begin_word.to_owned(),
                    end_word.to_owned(),
                    word_list.into_iter().map(|s| s.to_owned()).collect()
                ),
                expected,
                "Test #{} failed...",
                idx
            );
        }

    }

    #[test]
    fn t0125() {
        for (idx, (st, expected)) in vec![
            ("A man, a plan, a canal: Panama", true),
            (" ", true),
            ("race a car", false),
            ("0P", false),
        ].into_iter().enumerate() {
            assert_eq!(
                p0125_is_palindrome_fast(st),
                expected,
                "Test #{} failed...",
                idx
            );
            assert_eq!(
                p0125_is_palindrome(&st.chars().collect::<Vec<_>>()),
                expected,
                "Test #{} failed...",
                idx
            );
        }
    }

    #[test]
    fn t0123() {
        for (idx, (prices, expected)) in vec![
            (vec![3,3,5,0,0,3,1,4], 6),
            (vec![2,1,2,0,1], 2),
            (vec![1,2,3,4,5], 4),
            (vec![7,6,4,3,1], 0),
            (vec![1], 0),
        ].into_iter().enumerate() {
            assert_eq!(
                p0123_max_profit(&prices),
                expected,
                "Test #{} failed...",
                idx
            );
        }
    }

    #[test]
    fn t0122() {
        for (idx, (prices, expected)) in vec![
            (vec![7,1,5,3,6,4], 7),
            (vec![1,2,3,4,5], 4),
            (vec![7,6,4,3,1], 0),
        ].into_iter().enumerate() {
            assert_eq!(
                p0122_max_profit_faster(&prices),
                expected,
                "Test #{} failed...",
                idx
            );
            assert_eq!(
                p0122_max_profit(&prices),
                expected,
                "Test #{} failed...",
                idx
            );
        }
    }

    #[test]
    fn t0120() {
        for (idx, (tri, expected)) in vec![
            (vec![vec![2], vec![3,4], vec![6,5,7], vec![4,1,8,3]], 11),
            (vec![vec![-10]], -10),
        ].into_iter().enumerate() {
            assert_eq!(
                p0120_minimum_total(&tri),
                expected,
                "Test #{} failed...",
                idx
            );
        }

    }

    #[test]
    fn t0119() {
        for (idx, (n, expected)) in vec![
            (0, vec![1]),
            (1, vec![1, 1]),
            (3, vec![1, 3, 3, 1]),
        ].into_iter().enumerate() {
            assert_eq!(
                p0119_get_row(n),
                expected,
                "Test #{} failed...",
                idx
            );
        }
    }

    #[test]
    fn t0118() {
        for (idx, (n, expected)) in vec![
            (1, vec![vec![1]]),
            (5, vec![vec![1], vec![1,1], vec![1,2,1], vec![1,3,3,1], vec![1,4,6,4,1]]),
        ].into_iter().enumerate() {
            assert_eq!(
                p0118_generate(n),
                expected,
                "Test #{} failed...",
                idx
            );
        }
    }

    #[test]
    fn t0097() {
        for (idx, (s1, s2, s3, expected)) in vec![
            ("aabcc", "dbbca", "aadbbcbcac", true),
            ("aabcc", "dbbca", "aadbbbaccc", false),
            ("", "", "", true),
        ].into_iter().enumerate() {
            assert_eq!(
                p0097_is_interleave(&s1.chars().collect::<Vec<_>>(), &s2.chars().collect::<Vec<_>>(), &s3.chars().collect::<Vec<_>>()),
                expected,
                "Test #{} failed...",
                idx
            );
        }
    }

    #[test]
    fn t0093() {
        for (idx, (ip, expected)) in vec![
            ("25525511135", 2),
            ("0000", 1),
            ("1111", 1),
            ("010010", 2),
            ("101023", 5),
        ].into_iter().enumerate() {
            assert_eq!(
                p0093_restore_ip_addresses(ip.to_owned()).len(),
                expected,
                "Test #{} failed...",
                idx
            );
        }
    }

    #[test]
    fn t0091() {
        for (idx, (mut st, expected)) in vec![
            ("12", 2),
            ("226", 3),
            ("0", 0),
        ].into_iter().enumerate() {
            assert_eq!(
                p0091_num_decodings(&st.chars().collect::<Vec<_>>()),
                expected,
                "Test #{} failed...",
                idx
            );
        }

    }

    #[test]
    fn t0090() {
        for (idx, (mut nums, expected)) in vec![
            (vec![1,2,2], 6),
            (vec![1], 2),
            (vec![4, 4, 4, 1, 4], 10),
        ].into_iter().enumerate() {
            assert_eq!(
                p0090_subsets_with_dup(nums).len(),
                expected,
                "Test #{} failed...",
                idx
            );
        }
    }

    #[test]
    fn t0088() {
        for (idx, (mut nums1, m, mut nums2, n, expected)) in vec![
            (vec![1,2,3,0,0,0], 3, vec![2,5,6], 3, vec![1,2,2,3,5,6]),
            (vec![1], 1, vec![], 0, vec![1]),
            (vec![0], 0, vec![1], 1, vec![1])
        ].into_iter().enumerate() {
            p0088_merge(&mut nums1, m, &mut nums2, n);
            assert_eq!(
                nums1,
                expected,
                "Test #{} failed...",
                idx
            );
        }
    }

    #[test]
    fn t0087() {
        for (idx, (s1, s2, expected)) in vec![
            ("abc", "bca", true),
            ("abb", "bba", true),
            ("abcde", "caebd", false),
            ("great", "rgeat", true),
            ("a", "a", true),
            ("abcdg", "caebd", false),
            ("ccabcbabcbabbbbcbb","bbbbabccccbbbabcba", false),
            ("eebaacbcbcadaaedceaaacadccd", "eadcaacabaddaceacbceaabeccd", false),
        ].into_iter().enumerate() {
            assert_eq!(
                p0087_is_scramble(&s1.chars().collect::<Vec<_>>(), &s2.chars().collect::<Vec<_>>()),
                expected,
                "Test #{} failed...",
                idx
            );
        }
    }

    #[test]
    fn t0080() {
        for (mut nums, expected) in vec![
            (vec![1, 1, 1, 2, 2, 3], vec![1, 1, 2, 2, 3]),
            (vec![0,0,1,1,1,1,2,3,3], vec![0,0,1,1,2,3,3]),
        ].into_iter() {
            let k = p0080_remove_duplicates(&mut nums) as usize;
            assert_eq!(nums[..k], expected);
        }
    }

    #[test]
    fn t0081() {
        for (mut nums, target, expected) in vec![
            (vec![1], 0, false),
            (vec![2,5,6,0,0,1,2], 0, true),
            (vec![2,5,6,0,0,1,2], 3, false),
        ].into_iter() {
            assert_eq!(p0081_search_faster(&nums, target), expected);
            assert_eq!(p0081_search(&nums, target), expected);
        }
    }

    #[test]
    fn t0084() {
        for (mut nums, expected) in vec![
            (vec![2, 1, 5, 6, 2, 3], 10),
            (vec![2, 4], 4),
        ].into_iter() {
            assert_eq!(p0084_largest_rectangle_area(&nums), expected);
        }
    }

    #[test]
    fn t0085() {
        for (matrix, expected) in vec![
            // ([["1","0","1","0","0"], ["1","0","1","1","1"], ["1","1","1","1","1"], ["1","0","0","1","0"]], 6),
            // ([["0"]], 0),
            ([["1"]], 1),
        ].into_iter() {
            let matrix = matrix.into_iter().map(|row| {
                row.into_iter().map(|s| s.chars().next().unwrap()).collect()
            }).collect();
            assert_eq!(p0085_maximal_rectangle(&matrix), expected);
        }
    }
}
