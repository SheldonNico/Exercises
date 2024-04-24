use crate::util::ListNode;
use std::collections::HashMap;

pub fn p0055_can_jump(nums: Vec<i32>) -> bool {
    let mut hist = HashMap::new();
    let nums: Vec<usize> = nums
        .into_iter()
        .filter(|n| *n >= 0)
        .map(|n| n as usize)
        .collect();
    p0055_can_jump_rec(0, nums.len() - 1, &nums, &mut hist)
}

fn p0055_can_jump_rec(
    start: usize,
    stop: usize,
    nums: &Vec<usize>,
    mut hist: &mut HashMap<(usize, usize), bool>,
) -> bool {
    println!("{} {}", start, stop);
    if !hist.contains_key(&(start, stop)) {
        let mut connect = false;

        if (start == stop) || (stop - start <= nums[start]) {
            connect = true;
        } else if start < nums.len() {
            for step in (1..nums[start] + 1).rev() {
                if p0055_can_jump_rec(start + step, stop, &nums, &mut hist) {
                    connect = true;
                    break;
                }
            }
        }
        hist.insert((start, stop), connect);
    }
    hist[&(start, stop)]
}

pub fn p0061_rotate_right(head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
    fn get_length(head: &Option<Box<ListNode>>) -> usize {
        if head.is_none() {
            return 0;
        }
        return 1 + get_length(&head.as_ref().unwrap().next);
    }
    fn append(slf: &mut Option<Box<ListNode>>, val: i32) {
        match slf {
            None => {
                *slf = Some(Box::new(ListNode { val, next: None }));
            }
            Some(p) => {
                let ListNode { ref mut next, .. } = **p;
                append(next, val);
            }
        }
    }

    fn extend(left: &mut Option<Box<ListNode>>, right: Option<Box<ListNode>>) {
        if let Some(p) = right {
            let ListNode { next, val } = *p;
            append(left, val);
            extend(left, next);
        }
    }
    fn rotate_left(
        mut head: Option<Box<ListNode>>,
        k: i32,
        mut left: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if k < 0 {
            panic!("not enough node to rotate.");
        }
        if k == 0 {
            extend(&mut head, left);
            return head;
        }

        let ListNode { val, next } = *head.unwrap();
        append(&mut left, val);

        rotate_left(next, k - 1, left)
    }

    if get_length(&head) == 0 {
        return None;
    }

    k = k % get_length(&head) as i32;
    k = get_length(&head) as i32 - k;
    rotate_left(head, k, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_p0061() {
        assert_eq!(
            p0061_rotate_right(Some(Box::new(listnode!(1, 2, 3, 4, 5))), 2),
            Some(Box::new(listnode!(4, 5, 1, 2, 3)))
        );
        assert_eq!(
            p0061_rotate_right(Some(Box::new(listnode!(1, 2, 3, 4, 5))), 7),
            Some(Box::new(listnode!(4, 5, 1, 2, 3)))
        );
        assert_eq!(p0061_rotate_right(None, 0), None);
        assert_eq!(p0061_rotate_right(None, 9), None);
    }
}
