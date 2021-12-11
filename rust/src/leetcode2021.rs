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

#[cfg(test)]
mod tests {
    use super::*;

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
            // ("bcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcde", "cebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebd", false),
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
