#![allow(dead_code)]
use {
    std::collections::{HashMap, LinkedList, BTreeSet, VecDeque, HashSet, BTreeMap},
    std::cmp::{Eq},
    std::str::Chars,
    std::boxed::Box,
    std::fmt,
};

pub use crate::util::{ListNode};
pub use crate::snippet::*;

pub fn p001_two_sum(nums: &Vec<isize>, target: isize) -> (usize, usize) {
    let mut founded: HashMap<isize, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        if founded.contains_key(num){
            return (founded[num], i)
        } else {
            founded.insert( target - num, i);
        }
    }
    panic!("not possible to find a solution")
}

pub fn p002_add_two_numbers(num1: &LinkedList<usize>, num2: &LinkedList<usize>) -> LinkedList<usize> {
    let mut num1 = num1.iter();
    let mut num2 = num2.iter();
    let mut n1; let mut n2 = 0; let mut shift = 0;

    let mut res = LinkedList::new();

    loop {
        if let Some(n) = num1.next() {
            n1 = *n;
            if let Some(n) = num2.next() {
                n2 = *n;
            }
        } else if let Some(n) = num2.next() {
            n2 = *n;
            n1 = 0;
        } else {
            break;
        }

        if (n1+n2+shift) > 9 {
            res.push_back(n1+n2+shift-10);
            shift = 1;
        } else {
            res.push_back(n1+n2+shift);
            shift = 0;
        }
    }
    if shift > 0 {
        res.push_back(shift);
    }


    res
}

pub fn p003_longest_substring(chars: &str) -> String {
    let mut maxlen = 0;
    let mut maxstr = String::new();
    for (i, _c) in chars.chars().enumerate() {
        let mut len = 0;
        let mut short = String::new();
        for cc in chars[i..].chars() {
            if short.contains(cc) {
                break;
            }
            short.push(cc);
            len += 1;
        }
        if len > maxlen {
            maxlen = len;
            maxstr = short;
        }
    }

    maxstr
}

pub fn p004_median_of_two_sorted_arrays(arr1: &Vec<i32>, arr2: &Vec<i32>) -> f64 {
    assert!(arr1.len() > 0 || arr2.len() > 0, "argument must be non empty");

    let middle = (arr1.len()+ arr2.len()) / 2;

    let mut index = 0;
    let mut first = 0; let mut second= 0;
    let mut bi = 0;
    for a in arr1.iter() {
        for b in arr2[bi..].iter() {
            if b > a { break; }
            bi += 1;
            index += 1;

            if index == middle {
                first = *b;
            } else if index == middle+1 {
                second = *b;
                break;
            }

        }
        index += 1;

        if index == middle {
            first = *a;
        } else if index == middle+1 {
            second = *a;
            break;
        }
    }

    for b in arr2[bi..].iter() {
        index += 1;

        if index == middle {
            first = *b;
        } else if index == middle+1 {
            second = *b;
            break;
        }

    }


    if (arr1.len()+ arr2.len()) % 2 == 0 {
        (second + first) as f64 / 2.0
    } else {
        second as f64
    }
}

pub fn p010_regular_expression_matching(s: String, p: String) -> bool {
    fn match_regex(s: &[char], p: &[char], curr: char, number: usize) -> bool {
        //println!("{:?} {:?} <-> {} {}", s, p, curr, number);

        if s.len() < number { return false; }

        let condition = s[..number].iter().all(|c| {
            match curr {
                '.' => true,
                '*' => panic!("* is not suitable as matcher"),
                _   => *c == curr,
            }
        });

        if condition {
            match p.len() {
                0 => { s[number..].len() == 0 },
                1 => match_regex(&s[number..], &p[1..], p[0], 1),
                _ => {
                    if p[1] == '*' {
                        let maxlen = s[number..].iter().take_while(|c| (**c == p[0]) || (p[0] == '.') ).count();

                        for i in 0..=maxlen {
                            if match_regex(&s[number..], &p[2..], p[0], maxlen-i) {
                                return true;
                            }
                        }
                        false

                    } else {
                        match_regex(&s[number..], &p[1..], p[0], 1)
                    }

                }
            }
        } else {
            false
        }
    }

    match_regex( &s.chars().collect::<Vec<char>>(), &p.chars().collect::<Vec<char>>(), '.', 0 )
}

pub fn p012_int_to_roman(num: i32) -> String {
    let mut out = String::new();

    fn split(num: i32, max: i32, min: i32, five: char, one: char, res: &mut String) {
        if num >= max {
            let count = num / max;
            for _ in 0..count { res.push(five); }
            res.push_str( &p012_int_to_roman( num - max*count ) );
        } else {
            res.push(one); res.push(five);
            res.push_str( &p012_int_to_roman( num-max+min) );
        }
    }

    match num {
        900..=3999 => {
            split(num, 1000, 100, 'M', 'C', &mut out);
        },
        400..=899   => {
            split(num, 500, 100, 'D', 'C', &mut out);
        },
        90..=399 => {
            split(num, 100, 10, 'C', 'X', &mut out);
        },
        40..=89 => {
            split(num, 50, 10, 'L', 'X', &mut out);
        },
        9..=39 => {
            split(num, 10, 1, 'X', 'I', &mut out);
        },
        4..=8 => {
            split(num, 5, 1, 'V', 'I', &mut out);
        }
        1..=3 => {
            split(num, 1, 1, 'I', 'I', &mut out);
        }
        0     => {},
        _           => panic!("out of function range."),
    }

    out
}

pub fn p013_roman_to_int(s: String) -> i32 {
    let mut out = 0; let mut curr = 0; let mut curr_char = 0;

    let mut split = |c: i32| {
        if c < curr_char {
            out += curr;
        } else if c == curr_char {
            curr += c; return;
        } else {
            out -= curr;
        }
        curr = c; curr_char = c;
    };

    for c in s.chars() {
        match c {
            'M' => { split(1000); },
            'D' => { split(500); },
            'C' => { split(100); },
            'L' => { split(50); },
            'X' => { split(10); },
            'V' => { split(5); },
            'I' => { split(1); },
            _   => panic!("char not recognized!"),
        }
    }

    out + curr
}

pub fn p014_longest_common_prefix(strs: Vec<String>) -> String {
    let mut out = String::new(); if strs.len() == 0 { return out; }
    let mut strs: Vec<_> = strs.iter().map(|s| s.chars()).collect();

    'outer: loop {
        let mut c = None;
        for st in strs.iter_mut() {
            if let Some(cc) = st.next() {
                match c {
                    None => {c = Some(cc); },
                    Some(co) if co == cc => { continue; },
                    _ => { break 'outer; },
                }

            } else {
                break 'outer;
            }
        }
        out.push(c.unwrap());
    }

    out
}

pub fn p015_three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut out: Vec<_> = vec![];
    let mut record = BTreeSet::new();
    let mut nums = nums; nums.sort();

    let len = nums.len(); if len < 3 {return out;};
    let min = nums.first().unwrap().clone(); let max = nums.last().unwrap().clone();

    'loopi: for i in 0..len-2 {
        if nums[i] < -2*max { continue 'loopi; }
        if nums[i] > -2*min { break 'loopi; }
        if nums[i] > 0  { break 'loopi; }

        'loopj: for j in i+1..len-1 {
            let rest = -nums[i]-nums[j];
            if rest < nums[j]  { break 'loopj; }

            if record.contains( &(nums[i], nums[j]) ) { continue; }
            record.insert( (nums[i], nums[j]) );

            if nums[j+1..].iter().take_while(|&&v| v <= rest).filter(|&&v| v == rest).count() > 0 {
                out.push( vec![ nums[i], nums[j], rest ] );
            }
        }
    }

    out
}

pub fn p016_three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums; nums.sort();
    let len = nums.len(); assert!(len >= 3, "input must be at least 3 numbers");
    let mut start; let mut end;

    let mut sum = nums[0]+nums[1]+nums.last().unwrap(); let mut delta = (sum-target).abs();

    for i in 0..len-2 {
        start = i+1; end = len-1;
        while start < end {
            let sum_ = nums[i] + nums[start] + nums[end];
            let delta_ = target - sum_;

            //println!("{} {} {} {} {}", nums[i], nums[start], nums[end], sum_, delta_);

            if delta_ == 0 { return sum_; }
            if delta_.abs() < delta {
                sum = sum_; delta = delta_.abs();
            }

            if delta_ > 0 {
                start += 1;
            } else {
                end -= 1;
            }
        }
    }

    sum
}

pub fn match_one(c: char, digits: &mut Chars, result: Vec<String>) -> Vec<String> {
    let digit_map= |c| match c {
        '2' => "abc", '3' => "def", '4' => "ghi",
        '5' => "jkl", '6' => "mno", '7' => "pqrs",
        '8' => "tuv", '9' => "wxyz", _   => "",
    };
    //println!(">>>> {} {:?}", c, result);

    let mut out = Vec::new();
    for poss in digit_map(c).chars() {
        for st in result.iter() {
            let mut o = st.clone(); o.push(poss);
            out.push( o );
        }
    }

    if let Some(c) = digits.next() {
        match_one(c, digits, out)
    } else {
        return out
    }
}

pub fn p017_letter_combinations(digits: String) -> Vec<String> {
    let mut digits = digits.chars();
    if let Some(c) = &digits.next() {
        match_one(*c, &mut digits, vec!["".to_string()])
    } else {
        vec![]
    }

}

pub fn p018_four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums; nums.sort();
    let mut out = vec![];
    let len = nums.len(); if len < 4 { return out; }
    let max = nums.last().unwrap();
    let mut record = BTreeSet::new();

    for i in 0..len-3 {
        if nums[i] > target / 4 { break; }
        if nums[i] < target - 3*max { continue; }
        for j in i+1..len-2 {
            if nums[j] > (target - nums[i]) / 3 { break; }
            if nums[j] < (target - nums[i] - 2*max) { continue; }
            for k in j+1..len-1 {
                if nums[k] > (target - nums[i] - nums[j]) / 2 { break; }
                if nums[k] < (target - nums[i] - nums[j] - max) { continue; }
                for l in k+1..len {
                    if nums[l] > target-nums[i]-nums[j]-nums[k] {
                        break;
                    }

                    if record.contains( &(nums[i], nums[j], nums[k]) ) { continue; }

                    if nums[l] == target-nums[i]-nums[j]-nums[k] {
                        out.push( vec![ nums[i], nums[j], nums[k], nums[l] ] );
                        record.insert( (nums[i], nums[j], nums[k]) );
                    }

                }
            }
        }
    }

    out
}

fn length(li: &Box<ListNode>) -> usize {
    match  &li.next {
        Some(lii) => 1 + length(&lii),
        None      => 1,
    }
}
pub fn p019_remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if let Some(h) = &head {
        let len = length(h);
        if len < n as usize { return None; }
        remove_nth_from_begin(head, len-(n as usize))
    } else {
        None
    }
}

fn remove_nth_from_begin(head: Option<Box<ListNode>>, n: usize) -> Option<Box<ListNode>> {
    if n > 0 {
        head.map( |mut node| {
            node.next = remove_nth_from_begin(node.next, n-1);
            node
        } )
    } else {
        head.and_then( |node| node.next )
    }
}

pub fn p020_is_valid(s: String) -> bool {
    let parens: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}')].iter().cloned().collect();

    let mut stack = VecDeque::new();
    for c in s.chars().filter(|p| parens.iter().any(|(k, v)| (k == p) || (v == p))) {
        if parens.contains_key(&c) {
            stack.push_back((c, 1));
        } else {
            if let Some( (last_char, last_count) ) = stack.pop_back() {
                if parens.get(&last_char).unwrap() == &c {
                    if last_count > 1 {
                        stack.push_back( (last_char, last_count-1) );
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    stack.len() == 0
}

pub fn p021_merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
/*
 * life time sucks here! ...
 *    l1.and_then(
 *        |node1| l2.and_then(
 *            |node2| {
 *                let mut newnode;
 *                if node1.val < node2.val {
 *                    newnode = ListNode::new(node1.val);
 *                    newnode.next = p021_merge_two_lists(node1.next, l2);
 *                } else {
 *                    newnode = ListNode::new(node2.val);
 *                    newnode.next = p021_merge_two_lists(node2.next, l1);
 *                }
 *
 *                Some(Box::new( newnode ))
 *            }
 *        )
 *    )
 */

    match (l1, l2) {
        (Some(n1), Some(n2)) => {
            let mut newnode: ListNode;
            if n1.val < n2.val {
                newnode = ListNode::new(n1.val);
                newnode.next = p021_merge_two_lists(n1.next, Some(n2));
            } else {
                newnode = ListNode::new(n2.val);
                newnode.next = p021_merge_two_lists(n2.next, Some(n1));
            }
            Some( Box::new( newnode ))
        },
        (None, Some(n2)) => Some(n2),
        (Some(n1), None) => Some(n1),
        (None, None) => None,
    }
}

pub fn p022_generate_parenthesis(n: i32) -> Vec<String> {
    fn dfs(n: i32, res: Vec<String>) -> Vec<String> {
        if n > 0 {
            dfs(n-1, res.into_iter().flat_map(|mut hist| {
                let left_size = hist.chars().filter(|c| c == &'(').count();
                let right_size = hist.chars().filter(|c| c == &')').count();

                let mut expand = vec![ ];
                for i in right_size..left_size {
                    let mut close = hist.clone();
                    for _ in 0..(left_size-i) {
                        close.push(')');
                    }
                    close.push('(');
                    expand.push( close );
                }

                hist.push('(');
                expand.push( hist );

                expand
            }).collect())
        } else {
            res.into_iter().map(|mut s| {
                let left_size = s.chars().filter(|c| c == &'(').count();
                let right_size = s.chars().filter(|c| c == &')').count();
                for _ in 0..(left_size-right_size) {
                    s.push(')');
                }
                s
            }).collect()
        }
    };

    dfs(n, vec!["".to_string()])
}

/// TODO here: lifetime with pointer sucks
pub fn p023_merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    use std::mem;
    println!(">>>>>>>>>>>>>>>");

    let mut lists: Vec<_> = lists.into_iter().filter(|v| v.is_some()).collect();
    if lists.len() == 0 { return None; }
    if lists.len() == 1 { return lists.pop().unwrap(); }

    let mut first = lists.pop().unwrap().unwrap(); let mut second = lists.pop().unwrap().unwrap();
    if first.val > second.val {
        mem::swap(&mut first, &mut second);
    }

    let mut hist = Vec::new();
    while lists.len() > 0 {
        let mut tmp = lists.pop().unwrap().unwrap();
        println!("{:?}", tmp);

        if (tmp.val > first.val) && (tmp.val < second.val) {
            mem::swap( &mut tmp, &mut second );
        } else if tmp.val < first.val {
            mem::swap(&mut tmp, &mut second);
            mem::swap(&mut second, &mut first);
        }
        hist.push( Some( tmp ) );
    }

    let mut curr = Vec::new();
    while first.val <= second.val {
        curr.push(first.val);
        if first.next.is_none() { break; }
        first = first.next.unwrap();
    }
    hist.push( Some(first) ); hist.push( Some(second) );

    println!("{:?}", curr);

    let mut out = p023_merge_k_lists( hist );
    for v in curr.into_iter().rev() {
        let mut tmp = ListNode::new(v); tmp.next = out;
        out = Some( Box::new(tmp) );
    }

    out
}

pub fn p024_swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        Some(boxl) => {
            match *boxl {
                ListNode { val: v, next: Some(boxll) } => {
                    let ListNode { val: vv, next: rest } = *boxll;
                    let mut out1 = ListNode::new(vv);
                    let mut out2 = ListNode::new(v);
                    out2.next = p024_swap_pairs(rest);
                    out1.next = Some(Box::new( out2 ));
                    Some(Box::new(out1))
                },
                ListNode { val: v, next: None } => {
                    let out1 = ListNode::new(v);
                    Some(Box::new(out1))
                }
            }
        }
        _ => None,
    }
}

pub fn p026_remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

pub fn p027_remove_element<T: Eq>(nums: &mut Vec<T>, val: T) -> i32 {
    nums.retain(|v| *v != val);
    nums.len() as i32
}

pub fn p028_str_str(haystack: String, needle: String) -> i32 {
    //if haystack.len() == 0 && needle.len() == 0 { return 0; }
    if haystack.len() < needle.len() { return -1; }

    let mut find = -1;
    let needle_len = needle.len();
    for i in 0..=haystack.len() - needle_len {
        if &haystack[i..i+needle_len] == &needle {
            find = i as i32;
            break;
        }
    }
    find
}

pub fn p029_divide(dividend: i32, divisor: i32) -> i32 {
    static UP: i32 = 2147483647;
    static LOW: i32 = -2147483648;
    static MID: i32 = UP / 2;

    if dividend == LOW {
        if divisor == -1 {
            UP
        } else if divisor < 0 {
            1 + p029_divide(dividend - divisor, divisor)
        } else {
            -1 + p029_divide(dividend + divisor, divisor)
        }
    } else if divisor == LOW {
        0
    } else {
        let sign = if (dividend > 0 && divisor < 0) || (dividend < 0 && divisor > 0) { -1 } else { 1 };

        let mut dividend = dividend.abs();
        let divisor = divisor.abs();

        let mut out: i32 = 0;

        let mut divisor_vec = vec![(divisor, sign)];

        while dividend >= divisor {
            if let Some( (divisor_curr, sign_curr) ) = divisor_vec.pop() {
                if dividend >= divisor_curr {
                    out += sign_curr;
                    dividend = dividend - divisor_curr;

                    divisor_vec.push( (divisor_curr, sign_curr) );
                    if divisor_curr <= MID {
                        divisor_vec.push( (divisor_curr + divisor_curr, sign_curr + sign_curr) );
                    }
                } else {
                }
            } else {
                break;
            }

        }

        out
    }
}

pub fn p030_find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let mut word_map: HashMap<String, i32> = HashMap::new();
    let mut word_len = 0;
    let size = words.len();
    for w in words.into_iter() {
        word_len = std::cmp::max(word_len, w.len());
        word_map.entry(w).and_modify(|v| {*v += 1}).or_insert(0);
    }

    // make mutable in-mutable
    let word_len = word_len;
    let word_map = word_map;
    let mut result = vec![];
    if size * word_len > s.len() {
        return result;
    }

    for i in 0..word_len {
        let mut left = i;
        let mut count = 0;
        let mut tmp: HashMap<String, i32> = HashMap::new();

        for j in (i..s.len()-word_len + 1).step_by(word_len) {
            let substr = &s[j..j+word_len];
            if let Some(v) = word_map.get(substr) {
                tmp.entry( substr.to_string() ).and_modify(|v| { *v += 1 }).or_insert(0);

                // here is tricky: rust can not have two reference to the ame obj(HashMap)
                // so I have to get value at every refreence
                if *tmp.get(substr).unwrap() <= *v {
                    count += 1;
                } else {
                    while *tmp.get(substr).unwrap() > *v {
                        let substr1 = &s[left..left+word_len];
                        let vref1 = tmp.get_mut(substr1).unwrap();

                        *vref1 -= 1;
                        //if *vref1 < *word_map.get(substr1).unwrap() {
                        if count > 0 {
                            count -= 1;
                        }
                        left += word_len;

                    }
                }

                if count == size {
                    result.push(left as i32);
                    tmp.entry( s[left..left+word_len].to_string() ).and_modify(|v| { *v -= 1 });

                    left += word_len;
                    count -= 1;
                }
            } else {
                tmp.clear();
                count = 0;
                left = j + word_len;
            }
        }
    }

    return result;
}

pub fn _p030_find_substring(s: String, words: Vec<String>) -> Vec<i32> {

    let mut out: Vec<i32> = vec![];
    let mut words = words;

    //let mut words_ref: Vec<&str> = Vec::new();
    let mut size = 0;
    for w in words.iter() {
        size = std::cmp::max(size, w.len());
        //words_ref.push(w);
    }

    if s.len() >= size && words.len() > 0 {
        for i in 0..=s.len()-size {
            if p030_find_substring_helper(&s[i..], &mut words) {
                out.push( i as i32 );
            }
        }
    }

    out
}

// must use tail recursion or loop here??
// is there any tail recurrsion one.
pub fn p030_find_substring_helper(s: &str, words: &mut Vec<String>) -> bool {
    if words.len() == 0 {
        return true;
    } else {
        for i in 0..words.len() {
            let w = words.drain(i..i+1).next().unwrap();
            if w.len() <= s.len() && w == s[..w.len()] {
                if p030_find_substring_helper(&s[w.len()..], words) {
                    // recovery origin string
                    words.insert(i, w);
                    return true;
                }
            }
            words.insert(i, w);
        }
    }
    return false;

}

pub fn p031_next_permutation(nums: &mut Vec<i32>) {
    let nums_len = nums.len();
    if nums_len < 1 { return; }

    let mut left = nums.last().unwrap();
    let mut ind_split = nums_len - 1;
    for (i, v) in nums.iter().enumerate().rev().skip(1) {
        //println!("{} {}: {}", i, v, left);
        if v >= left {
            ind_split = i;
            left = v;
        } else {
            break;
        }
    }

    if ind_split > 0 {
        let left = nums[ind_split - 1];
        let mut ind_swap = ind_split;

        for i in (ind_split..nums_len).rev() {
            if nums[i] > left {
                ind_swap = i;
                break;
            }
        }

        nums.swap(ind_split - 1, ind_swap);
    }
    nums[ind_split..].reverse();
}

pub fn p032_longest_valid_parentheses(s: String) -> i32 {
    let mut left = vec![];
    let mut stack = 0;
    let mut maxlen = 0;
    let mut points = vec![];

    for (id, paren) in s.chars().enumerate() {
        match paren {
            '(' => {
                stack += 1;
                left.push(id);
            },
            ')' => {
                stack -= 1;

                if stack < 0 {
                    stack = 0;
                    points.clear();
                    left.clear();
                } else {
                    let mut st = left.pop().unwrap();
                    // important: merge exist closing paren
                    while let Some( (lastst, lastet) ) = points.pop() {
                        if lastst >= st {
                        } else if lastet + 1 == st {
                            st = lastst;
                        } else {
                            points.push( (lastst, lastet) ); // if not matched, push back, it may be closed in future.
                            break;
                        }
                    }

                    points.push( (st, id) );
                    maxlen = std::cmp::max(id - st + 1, maxlen);
                }
            },
            _   => {}
        }
    }

    maxlen as i32
}

pub fn p033_search(nums: Vec<i32>, target: i32) -> i32 {
    let nums_len = nums.len();
    if nums_len == 0 { return -1; }

    let mut st = 0; let mut ed = nums_len - 1;
    if nums[st] < nums[ed] || nums_len == 1 {
        ed += 1;
    } else {
        while nums[st] >= nums[ed] {
            if ed - st == 1 {
                //st = ed;
                break;
            }
            let mid = (st + ed) / 2;
            if nums[st] <= nums[mid] {
                st = mid;
            } else {
                ed = mid;
            }
        }

    }
    let mid = ed;

    if target >= nums[0] {
        match nums[..mid].binary_search(&target) {
            Ok(k) => k as i32,
            _     => -1,
        }
    } else {
        if mid < nums_len {
            match nums[mid..nums_len].binary_search(&target) {
                Ok(k) => (mid + k) as i32,
                _     => -1
            }
        } else {
            -1
        }
    }
}


pub fn p034_search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let st; let ed;
    match (&nums).binary_search(&target) {
        Ok(start) => {
            let right = nums[start..].iter().take_while(|&&v| v == target).count();
            let left = nums[..start].iter().rev().take_while(|&&v| v == target).count();
            st = (start - left) as i32;
            ed = (start + right) as i32 - 1;
        },
        _ => {
            st = -1; ed = -1;
        }
    }

    vec![st, ed]
}

pub fn p035_search_insert(nums: Vec<i32>, target: i32) -> i32 {
    // emmm.... std library has this function
    match nums.binary_search(&target) {
        Ok(k) => k as i32,
        Err(v) => v as i32,
    }
}

#[derive(Debug)]
enum SudokuElem {
    Elem(char),
    Choice(HashSet::<char>),
}
impl fmt::Display for SudokuElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SudokuElem::Elem(c) => write!(f, "{}", c),
            SudokuElem::Choice(cs) => write!(f, "{:?}", cs),
        }
    }
}


impl Default for SudokuElem {
    fn default() -> Self {
        let data: HashSet<char> = "123456789".chars().collect();
        SudokuElem::Choice( data )
    }
}

struct Sudoku(Vec<SudokuElem>);
impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for vs in self.0.chunks(9) {
            for elem in vs.iter() {
                let out = match elem {
                    SudokuElem::Elem(c) => c,
                    _                   => &'.',
                };
                write!(f, "{}", out)?;
            }
            write!(f, "{}", '\n')?
        }
        Ok(())
    }
}

impl Sudoku {
    pub fn from_vector(board: Vec<Vec< char >>) -> Self {
        let mut puzzle = Vec::new();
        for col in board.into_iter() {
            for c in col.into_iter() {
                match c {
                    '1'..='9' => puzzle.push( SudokuElem::Elem(c) ),
                    _         => puzzle.push( SudokuElem::default() ),
                }
            }
        }
        assert!(puzzle.len() == 9*9);
        Sudoku( puzzle )
    }

    pub fn to_vector(&self) -> Vec<Vec<char>> {
        let mut res = vec![];
        for vs in self.0.chunks(9) {
            let col: Vec<char> = vs.iter().map(
                |v| match v {
                    SudokuElem::Elem(c) => c.clone(),
                    _                   => '.',
                }
            ).collect();
            res.push(col);
        }
        res
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..9 {
            // by row
            let idxc: Vec<usize> = (0..9).map(|j| i*9+j).collect();
            // by col
            let idxr: Vec<usize> = (0..9).map(|j| i+j*9).collect();
            // by block
            let idxb: Vec<usize> = (0..9).map(|j| {
                (i % 3) * 3 + (i / 3) * 27 + (j % 3) + 9 * (j / 3)
            }).collect();

            for idx in [idxc, idxr, idxb].into_iter() {
                let mut sets = HashSet::new();
                for vind in idx.iter() {
                    match &self.0[*vind] {
                        SudokuElem::Elem(v) => {
                            if sets.contains(v) {
                                return false;
                            }
                            sets.insert( v );
                        },
                        _                  => {},
                    }
                }

            }
        }
        return true;
    }

    pub fn upedate(&mut self) -> bool {
        let mut updatable = false;


        for i in 0..9 {
            // by row
            let idxc: Vec<usize> = (0..9).map(|j| i*9+j).collect();
            // by col
            let idxr: Vec<usize> = (0..9).map(|j| i+j*9).collect();
            // by block
            let idxb: Vec<usize> = (0..9).map(|j| {
                (i % 3) * 3 + (i / 3) * 27 + (j % 3) + 9 * (j / 3)
            }).collect();

            for idx in [idxc, idxr, idxb].into_iter() {
                let mut determined: HashSet<char> = HashSet::new();
                let mut undetermined: HashMap<char, Vec<usize>> = HashMap::new();
                for vind in idx.iter() {
                    match &self.0[*vind] {
                        SudokuElem::Elem(v) => {
                            determined.insert( *v );
                        },
                        SudokuElem::Choice( cs ) => {
                            for c in cs.iter() {
                                undetermined.entry(*c)
                                    .and_modify(|room| room.push(*vind))
                                    .or_insert(vec![*vind]);
                            }
                        }
                    }
                }


                for col in self.0.chunks(9) {
                    println!("{:?}", col);
                }
                println!("{:?} {:?}", undetermined, determined);
                for (c, room) in undetermined.into_iter() {
                    if !determined.contains(&c) {
                        if room.len() == 1 {
                            let vind = room[0];
                            self.0[vind] = SudokuElem::Elem(c);
                            println!("insert {}@{}", c, vind);
                            determined.insert( c );
                            updatable = true;
                        }
                    }
                }

                for vind in idx.iter() {
                    let vref = &mut self.0[*vind];
                    match vref {
                        SudokuElem::Elem(_) => { },
                        SudokuElem::Choice( cs ) => {
                            for c in determined.iter() {
                                if cs.remove(c) {
                                    updatable = true;
                                }
                            }

                            if cs.len() == 1 {
                                *vref = SudokuElem::Elem(*cs.iter().nth(0).unwrap())
                            }
                        },
                    }
                }


                println!("{}\n", self);
                if !self.is_valid() {
                    panic!("Debug reason");
                }


            }
        }

        return updatable;
    }

    pub fn solve(&mut self) {

        while self.upedate() { }
    }
}

pub fn p036_is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let puzzle = Sudoku::from_vector(board);
    puzzle.is_valid()
}

pub fn p037_solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut puzzle = Sudoku::from_vector(board.clone());
    puzzle.solve();

    let sol = puzzle.to_vector();
    for (i, col) in sol.into_iter().enumerate() {
        for (j, v) in col.into_iter().enumerate() {
            board[i][j] = v;
        }
    }
}

pub fn p038_count_and_say(n: i32) -> String {
    let mut last = vec![1];
    for _ in 0..n-1 {
        let mut num = last[0];
        let mut count = 1;
        let mut counts = vec![];

        for &v in last.iter().skip(1) {
            if num == v {
                count += 1;
            } else {
                counts.push( count );
                counts.push( num );
                num = v; count = 1;
            }
        }
        counts.push( count );
        counts.push( num );
        last = counts;
    }

    last.into_iter().map( |v| format!("{}", v) ).collect::<Vec<String>>().join("")
}


// even without tail recurrsion, this code can pass leetcode test.
pub fn combination_sum_(nums: &[i32], target: i32, curr: Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if nums.len() == 0 || target <= 0 {
        if target == 0 {
            result.push(curr.clone());
        }
    } else {
        let num = nums[0];
        let mut target = target;
        let mut curr = curr;
        while target >= 0 {
            combination_sum_(&nums[1..], target, curr.clone(), result);
            curr.push(num);
            target -= num;
        }
    }
}

pub fn p039_combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates; // shit template
    candidates.sort();
    candidates.dedup();
    let candidates = candidates;

    assert!(candidates[0] >= 0);

    let mut result = vec![];
    combination_sum_(&candidates, target, vec![], &mut result);
    result
}

pub fn combination_sum_nodup_(nums: &[i32], target: i32, curr: Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if nums.len() == 0 || target <= 0 {
        if target == 0 {
            result.push(curr.clone());
        }
    } else {
        let num = nums[0];
        let count = nums.iter().take_while(|&&v| v == num).count();

        let mut target = target;
        let mut curr = curr;

        combination_sum_nodup_(&nums[count..], target, curr.clone(), result);
        for _ in 0..count {
            curr.push(num);
            target -= num;
            combination_sum_nodup_(&nums[count..], target, curr.clone(), result);
        }
    }
}
pub fn p040_combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates; // shit template
    candidates.sort();
    let candidates = candidates;

    assert!(candidates[0] >= 0);

    let mut result = vec![];
    combination_sum_nodup_(&candidates, target, vec![], &mut result);
    result
}

// works but not satisify O(n) time usage.
pub fn p041_first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut minimum = 1;
    let mut nums = nums;
    nums.sort();
    for &n in nums.iter() {
        if n > 0 {
            if minimum < n {
                return minimum;
            } else if minimum == n {
                minimum += 1;
            }
        }
    }

    return minimum;
}

pub fn p042_trap(height: Vec<i32>) -> i32 {
    let mut area: i32 = 0;
    let bar_len = height.len();
    let mut start = 0;

    if bar_len <= 2 { return 0; }

    while start < bar_len - 1 {
        let h = height[start];

        let mut maxright = 0;
        let mut stop = 0;

        for (idx, &right) in height[start+1..].iter().enumerate() {
            if right >= h {
                stop = idx;
                break;
            } else {
                if right >= maxright {
                    maxright = right;
                    stop = idx;
                }
            }
        }
        stop = start + stop + 1;


        let bars: i32 = height[start+1..stop].iter().sum();
        area += ((stop - start - 1) as i32) * std::cmp::min(height[stop], h) - bars;
        start = stop;
    }

    area
}

pub fn p043_multiply(num1: String, num2: String) -> String {
    let mut res: Vec<u32> = vec![];
    for (dl, nl) in num1.chars().rev().enumerate() {
        for (dr, nr) in num2.chars().rev().enumerate() {
            let nld: u32 = nl.to_digit(10).unwrap();
            let nrd: u32 = nr.to_digit(10).unwrap();
            let out = format!("{}", nld * nrd);


            let digit = dl + dr;
            let max_digit = digit + out.len();
            if res.len() < max_digit {
                while res.len() < max_digit {
                    res.push(0u32);
                }
            }

            println!("{:?} {} {}", res, digit, out);

            let mut state = 0;
            for (i, c) in out.chars().rev().enumerate() {
                let currdigit = digit + i;
                let curr = res[currdigit];
                let cd = c.to_digit(10).unwrap();
                let out = curr + cd + state;

                if out <= 9 {
                    state = 0;
                    res[currdigit] = out;
                } else {
                    // incr number at most give state 1: 9 + 9 -> 18
                    res[currdigit] = out - 10;
                    state = 1;
                }
            }

            let mut digit = max_digit;
            while state > 0 {
                if res.len() <= digit {
                    res.push(1u32);
                    state = 0;
                } else {
                    let num = res[digit];
                    if num == 9 {
                        res[digit] = 0;
                        digit += 1;
                    } else {
                        res[digit] += 1;
                        state = 0;
                    }
                }
            }

            //res += nld * nrd * 10u32.pow(dl as u32) * 10u32.pow(dr as u32);
        }
    }

    let out = res.iter().rev().map(|v| format!("{}", v)).collect::<Vec<String>>().concat();
    let count = out.chars().take_while(|&v| v == '0').count();
    if out.len() > count {
        out[count..].to_string()
    } else {
        "0".to_string()
    }
}

// too slow to use recurrsion
#[warn(dead_code)]
pub fn p044_is_match_rec(string: String, pattern: String) -> bool {
    let string: Vec<char> = string.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();
    p044_is_match_(&string, &pattern)
}

pub fn p044_is_match_(string: &[char], pattern: &[char]) -> bool {
    //println!("{:?}, {:?}", string.iter().collect::<String>(), pattern.iter().collect::<String>());
    if pattern.len() > 0 {
        match pattern[0] {
            '?' => {
                if string.len() > 0 {
                    return p044_is_match_(&string[1..], &pattern[1..]);
                } else {
                    return false;
                }
            },
            '*' => {
                if string.len() > 0 {
                    for i in (0..=string.len()).rev() {
                        //println!("{:?} {:?} {}", string, pattern, i);
                        if p044_is_match_(&string[i..], &pattern[1..]) {
                            return true;
                        }
                    }
                    return false;
                } else {
                    return p044_is_match_(&string, &pattern[1..]);
                }
            },
            c   => {
                if string.len() > 0 {
                    return string[0] == c && p044_is_match_(&string[1..], &pattern[1..]);
                } else {
                    return false;
                }
            },
        }

    } else {
        return string.len() == 0;
    }

}

// dynamic programming wins
// maybe dynamic programming (use hash map) + recurrsion will be the best?
pub fn p044_is_match(string: String, pattern: String) -> bool {
    let string: Vec<char> = string.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();

    let string_len = string.len();
    let pattern_len = pattern.len();
    let mut rest: Vec<Vec<bool>> = vec![ vec![false; pattern_len]; string_len ];

    for idp in 0..pattern_len {
        for ids in 0..string_len {
            match pattern[idp] {
                '*' => {
                    if ids > 0 && idp > 0 {
                        rest[ids][idp] = rest[ids-1][idp] || rest[ids][idp-1];
                    } else if idp > 0 {
                        rest[ids][idp] = rest[ids][idp-1];
                    } else if ids > 0 {
                        rest[ids][idp] = rest[ids-1][idp];
                    } else {
                        rest[ids][idp] = true;
                    };
                },
                c   => {
                    rest[ids][idp] = if c == '?' || c == string[ids] {
                        if ids > 0 && idp > 0 {
                            rest[ids-1][idp-1]
                        } else if idp > 0 {
                            pattern[..idp].iter().all(|&v| v == '*')
                        } else if ids > 0 {
                            false
                        } else {
                            true
                        }
                    } else {
                        false
                    };
                }
            }

            // DEBUG: useful print message
            // println!("({:?},{:?}) {:?} {:?} {}", ids, idp, &string[..ids+1].iter().collect::<String>(), &pattern[..idp+1].iter().collect::<String>(), rest[ids][idp]);
        }
    }

    if string_len > 0 && pattern_len > 0 {
        rest[string_len-1][pattern_len-1]
    } else if string_len > 0 {
        false
    } else if pattern_len > 0 {
        pattern.iter().all(|&v| v == '*')
    } else {
        true
    }
}

pub fn p046_permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![ vec![] ];

    for num in nums.into_iter() {
        let mut passing = vec![];
        for res in result.into_iter() {
            for j in 0..res.len() {
                let mut tmp = res.clone();
                tmp.insert(j, num);
                passing.push(tmp);
            }
            let mut tmp = res;
            tmp.push(num);
            passing.push(tmp);
        }

        result = passing;
    }

    result
}

pub fn p047_permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![ vec![] ];

    let mut nums = nums;
    nums.sort();
    let maxlen = nums.len();

    let mut i = 0;
    while i < nums.len() {
        let mut count = 0;
        let num = nums[i];
        while (i+count < maxlen) && nums[i+count] == num {
            count += 1;
        }

        let hist = std::mem::replace(&mut result, vec![]);
        for res in hist.into_iter() {
            if res.len() == 0 {
                let mut tmp = Vec::with_capacity(maxlen);
                for _ in 0..count {
                    tmp.push(num);
                }
                result.push( tmp );
            } else {
                let mut splitpoints: Vec<(usize, Vec<(usize, usize)>)> = vec![ (count, vec![]) ];
                let mut satisfiedpoints = vec![];

                for curr in 0..res.len() + 1 {
                    // tricky
                    let old_splitpoints = std::mem::replace(&mut splitpoints, vec![]);
                    for (rest, pairs) in old_splitpoints.into_iter() {
                        if rest > 0 {
                            for i in 0..rest {
                                let mut npairs = pairs.clone();
                                if i > 0 {
                                    npairs.push( (curr, i) );
                                }
                                splitpoints.push( (rest-i, npairs) );
                            }

                            // reuse last pair
                            let mut pairs = pairs;
                            pairs.push( (curr, rest) );
                            satisfiedpoints.push( pairs );
                        } else {
                            //splitpoints.push( (rest, pairs) );
                            satisfiedpoints.push( pairs );
                        }
                    }
                }

                for pairs in satisfiedpoints.into_iter() {
                    let mut nres = res.clone();
                    let mut insert_count = 0;
                    for (i, count) in pairs.into_iter() {
                        let insert_pos = i + insert_count;
                        for _ in 0..count {
                            nres.insert( insert_pos, num );
                        }
                        insert_count += count;
                    }

                    result.push( nres );
                }
            }
        }
        i += count;
    }

    result
}

pub fn p048_rotate(matrix: &mut Vec<Vec<i32>>) {
    if matrix.len() == 0 { return; }
    let rows = matrix.len(); let cols = matrix[0].len();
    assert_eq!(rows, cols, "works only on square matrix");
    let mut tmpvar = 0;

    // can't use std::mem::replace?
    for row in 0..((rows + 1) / 2) {
        for col in row..cols-1-row {
            if (rows % 2 != 0) && row == rows / 2 && col == cols / 2 { continue; }
            for (srow, scol) in [(row+0+col-row, cols-1-row), (rows-1-row, cols-1-(col-row)-row), (rows-1-(col-row)-row, row)].iter() {
                std::mem::swap( &mut matrix[row][col], &mut tmpvar );
                std::mem::swap( &mut matrix[*srow][*scol], &mut tmpvar );
                std::mem::swap( &mut matrix[row][col], &mut tmpvar );
            }
        }
    }
}

pub fn p049_group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {

    let mut str_map: HashMap<BTreeMap<u8, u8>, Vec<String>> = HashMap::new();
    for item in strs.into_iter() {
        let mut set: BTreeMap<u8, u8> = BTreeMap::new();
        for &uitem in item.as_bytes().into_iter() {
            set.entry(uitem).and_modify(|v| { *v +=1 ; }).or_insert(0);
        }

        str_map.entry(set).and_modify(|v| { v.push(item.clone()); }).or_insert( vec![item.clone()] );
    }

    str_map.values().map(|v| v.clone()).collect()
}

pub fn p050_my_pow(x: f64, n: i32) -> f64 {
    static LIMIT: i32 = 2147483647;

    if n < 0 {
        if n == -2147483648 {
            p050_my_pow(1.0/x, -(n+1)) / x
        } else if x != 0.0 {
            p050_my_pow(1.0/x, -n)
        } else {
            x
        }
    } else if n > 0 {
        let mut n = n;
        let mut counter = vec![(1, x)];
        counter.reserve( (n as f64).log2() as usize );
        let mut x = 1.0;
        while n > 0 {
            while let Some((count, cum)) = counter.pop() {
                if count <= n {
                    counter.push( (count, cum) );
                    break;
                }
            }

            let (count, cum) = counter.last().unwrap().clone();
            x *= cum;
            n -= count;
            if count <= LIMIT / 2 {
                counter.push( (count + count, cum*cum) );
            }
        }
        x
    } else {
        1.0
    }
}

pub fn p045_jump(nums: Vec<i32>) -> i32 {
    let mut hist = HashMap::new();
    let nums: Vec<usize> = nums.into_iter().map(|v| v as usize).collect();

    match p045_jump_rec(&nums, 0, nums.len()-1, &mut hist ) {
        Some(n) => n as i32,
        None    => -1,
    }
}

fn p045_jump_rec(nums: &[usize], start: usize, stop: usize, hist: &mut HashMap<(usize, usize), Option<usize>>) -> Option<usize> {
    println!("{}, {}", start, stop);
    if !hist.contains_key(&(start, stop)) {
        let dist = stop - start;
        if nums[start] >= dist {
            if start == stop {
                hist.insert((start, stop), Some(0));
            } else {
                hist.insert((start, stop), Some(1));
            }
        } else {
            let mut cached = vec![];
            for movelen in (1..nums[start]+1).rev() {
                match p045_jump_rec(nums, start+movelen, stop, hist) {
                    Some(n) => {
                        cached.push(n);
                        if n == 1 {
                            break; // use this to faster recursion
                        }
                    },
                    None => {
                    },
                }
            }

            let out = match cached.iter().min() {
                Some(n) => { Some(n+1) },
                _    => { None }
            };
            hist.insert((start, stop), out);
        }
    }
    hist[&(start, stop)]
}

pub fn p053_max_sub_array(nums: Vec<i32>) -> i32 {
    assert!(nums.len() > 0, "not work on empty vector");
    let mut maxsum = 0;
    let mut last_sum_cum = 0;
    for &num in nums.iter() {
        last_sum_cum = std::cmp::max(0, last_sum_cum+num);
        if num > 0 {
            maxsum = std::cmp::max(maxsum, last_sum_cum);
        }
    }
    if maxsum == 0 {
        *nums.iter().max().unwrap()
    } else {
        maxsum
    }
}

enum P045Dirction {
    Left,
    Right,
    Down,
    Up,
}

pub fn p054_spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let rows = matrix.len();
    let cols = if rows > 0 {matrix[0].len()} else {0};
    if rows == 0 || cols == 0 {
        return vec![];
    }
    if rows == 1 {
        return matrix[0].clone();
    }
    if cols == 1 {
        return matrix.into_iter().map(|v| v[0]).collect();
    }

    let mut res = vec![];
    let mut direction = P045Dirction::Right;

    let mut bound_left = 0;
    let mut bound_up = 0;
    let mut bound_right = cols - 1;
    let mut bound_down  = rows - 1;

    let mut row = 0; let mut col = 0;

    while bound_down >= row && col <= bound_right && row >= bound_up && col >= bound_left {
        res.push( matrix[row][col] );
        match direction {
            P045Dirction::Right => {
                if col < bound_right {
                    col += 1;
                } else {
                    bound_up += 1;
                    direction = P045Dirction::Down;
                    row += 1;
                }
            },
            P045Dirction::Down  => {
                if row < bound_down {
                    row += 1;
                } else {
                    bound_right -= 1;
                    direction = P045Dirction::Left;
                    col -= 1;
                }

            },
            P045Dirction::Left  => {
                if col > bound_left {
                    col -= 1;
                } else {
                    bound_down -= 1;
                    direction = P045Dirction::Up;
                    row -= 1;
                }
            },
            P045Dirction::Up  => {
                if row > bound_up {
                    row -= 1;
                } else {
                    bound_left += 1;
                    direction = P045Dirction::Right;
                    col += 1;
                }

            },
        }
    }

    res
}


pub fn p051_solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    if n == 0 { return vec![]; }

    let mut permutations: Vec<Vec<usize>> = vec![vec![]];
    for i in 0..n {
        let old_permutations = std::mem::replace(&mut permutations, vec![]);
        for perm in old_permutations.into_iter() {
            for j in 0..perm.len()+1 {
                let mut nperm = perm.clone();
                nperm.insert(j, i);
                permutations.push(nperm);
            }
        }
    }

    let mut res = vec![];
    for perm in permutations.into_iter() {

        let mut is_conflict = false;
        for (col, &row) in perm.iter().enumerate() {
            // check diagonal conflicts.
            for col_shift in 0..n {
                if col == col_shift { continue; }

                let shift = if col_shift < col { col - col_shift } else { col_shift - col };
                if (row >= shift && perm[col_shift] == row - shift) || (row + shift < n && perm[col_shift] == row+shift) {
                    is_conflict = true;
                    break;
                }
            }
        }
        if is_conflict { continue; }

        let mut matrix = vec![];
        for &row in perm.iter() {
            let mut col = String::new();
            for i in 0..n {
                if i == row {
                    col.push('Q');
                } else {
                    col.push('.');
                }
            }

            matrix.push(col);
        }
        res.push(matrix);
    }

    res
}

pub fn p052_total_n_queens(n: i32) -> i32 {
    p051_solve_n_queens(n).len() as i32
}


pub fn p058_length_of_last_word(s: String) -> i32 {
    match s.trim().split(" ").last() {
        None => 0,
        Some(v) => v.len() as i32,
    }
}

enum P059Dirction {
    Left,
    Right,
    Down,
    Up,
}

pub fn p059_generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut res = vec![ vec![0; n ] ; n ];

    let mut direction = P059Dirction::Right;

    let mut bound_left = 0;
    let mut bound_up = 0;
    let mut bound_right = n - 1;
    let mut bound_down  = n - 1;

    let mut row: usize = 0; let mut col: usize = 0;
    let mut curr = 0;
    while bound_down >= row && col <= bound_right && row >= bound_up && col >= bound_left {
        curr += 1;
        res[row][col] = curr;
        match direction {
            P059Dirction::Right => {
                if col < bound_right {
                    col += 1;
                } else {
                    bound_up += 1;
                    direction = P059Dirction::Down;
                    row += 1;
                }
            },
            P059Dirction::Down  => {
                if row < bound_down {
                    row += 1;
                } else {
                    bound_right -= 1;
                    direction = P059Dirction::Left;
                    col -= 1;
                }

            },
            P059Dirction::Left  => {
                if col > bound_left {
                    col -= 1;
                } else {
                    bound_down -= 1;
                    direction = P059Dirction::Up;
                    row -= 1;
                }
            },
            P059Dirction::Up  => {
                if row > bound_up {
                    row -= 1;
                } else {
                    bound_left += 1;
                    direction = P059Dirction::Right;
                    col += 1;
                }
            },
        }
    }

    res
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

pub fn p060_get_permutation_(n: i32, k: i32) -> Vec<i32> {
    let mut permutations: Vec<i32> = (1..n+1).collect();
    for _ in 1..k {
        p031_next_permutation(&mut permutations);
    }

    permutations
}

pub fn p060_get_permutation(n: i32, k: i32) -> String {
    let mut out = String::new();
    for n in p060_get_permutation_(n, k) {
        out.push_str( &n.to_string() );
    }
    out
}

pub fn p062_unique_path(m: i32, n: i32) -> i32 {
    // C(n, k)
    let k = (n-1) as i64;
    let n = (m+n-2) as i64;
    let k = k.min(n-k);

    let mut num = 1i64;
    let all = 1..=n;
    for (i, j) in (all.clone().rev()).zip(all) {
        if i >= n-k+1 {
            num *= i;
        }

        if j <= k {
            num /= j;
        }
    }

    num as i32
}

pub fn p063_unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
/*
 *    let n = obstacle_grid.len();
 *    if n == 0 { return 0; }
 *    let m = obstacle_grid[0].len();
 *    if m == 0 { return 0; }
 *    if obstacle_grid[0][0] == 1 { return 0; }
 *
 *    fn move_curr((sx, sy): (usize, usize), (ex, ey): (usize, usize), grid: &Vec<Vec<i32>>, cum: i32) -> i32 {
 *        if sx == ex && sy == ey { return cum+1; }
 *        let (mut right, mut down) = (0, 0);
 *        if sx < ex && grid[sx+1][sy] == 0 {
 *            right = move_curr((sx+1, sy), (ex, ey), grid, cum);
 *        }
 *        if sy < ey && grid[sx][sy+1] == 0 {
 *            down = move_curr((sx, sy+1), (ex, ey), grid, cum);
 *        }
 *        right + down
 *    }
 *
 *    move_curr((0, 0), (n-1, m-1), &obstacle_grid, 0)
 */
    let n = obstacle_grid.len();
    if n == 0 { return 0; }
    let m = obstacle_grid[0].len();
    if m == 0 { return 0; }
    if obstacle_grid[0][0] == 1 { return 0; }

    let mut walked = std::collections::HashMap::new();
    walked.insert((0, 0), 1);

    for _ in 0..(m+n-2) {
        let mut next = std::collections::HashMap::new();
        for ((sx, sy), possible) in walked.into_iter() {
            if sx < n-1 && obstacle_grid[sx+1][sy] == 0 {
                let val = next.entry((sx+1, sy)).or_insert(0);
                *val += possible;
            }

            if sy < m-1 && obstacle_grid[sx][sy+1] == 0 {
                let val = next.entry((sx, sy+1)).or_insert(0);
                *val += possible;
            }
        }
        walked = next;
    }

    walked.remove(&(n-1, m-1)).unwrap_or(0)
}

pub fn p064_min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len(); if n == 0 { return 0; }
    let m = grid[0].len(); if m == 0 { return 0; }

    let mut walked = std::collections::HashMap::new();
    walked.insert((0, 0), grid[0][0]);

    for _ in 0..(m+n-2) {
        let mut next = std::collections::HashMap::new();
        for ((sx, sy), val) in walked.into_iter() {
            if sx < n-1 {
                let old = next.entry((sx+1, sy)).or_insert(i32::max_value());
                *old = std::cmp::min(*old, val+grid[sx+1][sy]);
            }

            if sy < m-1 {
                let old = next.entry((sx, sy+1)).or_insert(i32::max_value());
                *old = std::cmp::min(*old, val+grid[sx][sy+1]);
            }
        }
        //println!("{:?}", next);
        walked = next;
    }

    walked.remove(&(n-1, m-1)).unwrap()
}

pub fn p065_is_number(s: String) -> bool {
    let all_digits = vec!['0','1','2','3','4','5','6','7','8','9','.','e','+','-'];
    let mut digits: &str = s.trim();

    if digits.contains(' ') { return false; }
    if digits.len() == 0 { return false; }
    if !digits.chars().all(|v| all_digits.contains(&v)) { return false; }

    let contain_sign = (&digits[..=0] == "+") || (&digits[..=0] == "-");
    if contain_sign {
        digits = &digits[1..digits.len()];
        if digits.len() == 0 { return false; }
    }

    fn integral(s: &str) -> bool {
        //if s == "0" { return true; }
        //&s[..=0] != "0" && s[1..].chars().all(|v| '0' <= v && v <= '9')
        s.chars().all(|v| '0' <= v && v <= '9')
    }

    let mut before_e = ""; let mut after_e = "";
    if let Some(pos) = digits.find('e') {
        let (before_e_, after_e_) = digits.split_at(pos);
        before_e = before_e_; after_e = &after_e_[1..];
        if after_e == "+" { return false; }
        if after_e.len() == 0 { return false; }
        if before_e.len() == 0 { return false; }
        if &after_e[..=0] == "-" || &after_e[..=0] == "+" {
            after_e = &after_e[1..];
            if after_e.len() == 0 { return false; }
            if !integral(after_e) { return false; }
        } else {
            if !integral(after_e) { return false; }
        }
    } else {
        before_e = digits;
    }

    let mut int_part = ""; let mut decimal_part = "";
    if let Some(pos) = before_e.find('.') {
        let (int_part_, decimal_part_) = before_e.split_at(pos);
        int_part = int_part_; decimal_part = &decimal_part_[1..];
        if int_part.len() == 0 && decimal_part.len() == 0 { return false; }
        if int_part.len() == 0 { int_part = "0"; }
        decimal_part = decimal_part.trim_start_matches('0');
        if !integral(int_part) { return false; }
        if decimal_part.len() > 0 && !integral(decimal_part) { return false; }
    } else {
        int_part = before_e;
        if contain_sign && int_part.len() == 0 { return false; }
        if !integral(int_part) { return false; }
    }

    true
}

pub fn p066_plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut out = Vec::new();
    let mut cum = 1;
    for digit in digits.into_iter().rev() {
        out.push((cum + digit) % 10);
        cum = (cum+digit) / 10;
    }
    if cum != 0 {
        out.push(cum);
    }
    out.into_iter().rev().collect()
}

pub fn p067_add_binary(a: String, b: String) -> String {
    let mut out = Vec::new();
    let mut cum = 0;
    let a: Vec<_> = a.chars().rev().collect();
    let b: Vec<_> = b.chars().rev().collect();

    for idx in 0..a.len().max(b.len()) {
        let digit1 = a.get(idx).unwrap_or(&'0');
        let digit2 = b.get(idx).unwrap_or(&'0');
        let mut curr = match (digit1, digit2) {
            ('0', '1') => 1,
            ('1', '0') => 1,
            ('1', '1') => 2,
            ('0', '0') => 0,
            _ => panic!("not binary"),
        };
        curr += cum;
        println!("{} {}", digit1, digit2);

        if curr % 2 == 0 {
            out.push('0');
        } else {
            out.push('1');
        }
        cum = curr / 2;
    }

    println!("{:?} {}", out, cum);
    if cum != 0 {
        out.push('1');
    }

    out.into_iter().rev().collect()
}

pub fn p068_my_sqrt(x: i32) -> i32 {
    (x as f64).sqrt() as i32
}

pub fn p069_climb_stairs(n: i32) -> i32 {
    let mut walked = std::collections::HashMap::new(); walked.insert(0, 1);
    let mut curr = 0;
    for _ in 0..n {
        let mut next = std::collections::HashMap::new();
        for (pos, count) in std::mem::replace(&mut walked, std::collections::HashMap::new()).into_iter() {
            if pos <= n-1 {
                *next.entry(pos+1).or_insert(0) += count;
            }
            if pos <= n-2 {
                *next.entry(pos+2).or_insert(0) += count;
            }

            if pos == n {
                curr += count;
            }
        }
        walked = next;
    }
    curr + walked.remove(&n).unwrap_or(0)
}

pub fn p071_simplify_path(path: String) -> String {
    let mut path_stack = vec![];
    for p in path.split("/") {
        match p {
            "." | "" => {  },
            ".." => {
                if path_stack.len() > 0 {
                    path_stack.pop().unwrap();
                }
            },
            p => { path_stack.push(p); }
        }
    }

    let out = format!("/{}", path_stack.join("/"));
    out
}

pub fn p073_set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    use std::collections::HashSet;

    let m = matrix.len(); if m == 0 { return; }
    let n = matrix[0].len(); if n == 0 { return; }
    let mut rows = HashSet::new(); let mut columns = HashSet::new();

    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 0 {
                rows.insert(i);
                columns.insert(j);
            }
        }
    }

    for row in rows.into_iter() {
        for i in 0..n {
            *matrix.get_mut(row).unwrap().get_mut(i).unwrap() = 0;
        }
    }

    for col in columns.into_iter() {
        for j in 0..m {
            *matrix.get_mut(j).unwrap().get_mut(col).unwrap() = 0;
        }
    }
}

pub fn p074_search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.len() == 0 || matrix[0].len() == 0 { return false; }
    let first: Vec<i32> = matrix.iter().map(|v| v[0]).collect();
    if let Err(idx) = first.binary_search(&target) {
        if idx == 0 { return false; }
        matrix[idx-1].binary_search(&target).is_ok()
    } else {
        true
    }
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

pub fn p075_sort_colors(nums: &mut Vec<i32>) {
    /*
     *    let (mut count1, mut count2, _) = (0, 0, 0);
     *    for num in nums.iter() {
     *        match num {
     *            0 => count1 += 1,
     *            1 => count2 += 1,
     *            2 => {  },
     *            _ => panic!("Argument not right"),
     *        }
     *    }
     *
     *    for (idx, num) in nums.iter_mut().enumerate() {
     *        if idx < count1 {
     *            *num = 0;
     *        } else if idx < count1+count2 {
     *            *num = 1;
     *        } else {
     *            *num = 2;
     *        }
     *    }
     */

    let len = nums.len();
    let (mut count1, mut id2) = (0, 0);
    for idx in 0..len {
        match nums[idx] {
            0 => {
                nums.rotate_left(idx);
                nums[1..].rotate_right(idx); // nums[idx..] contais at least one
                count1 += 1; id2 += 1;
            },
            1 => {
                nums[count1..].rotate_left(idx-count1);
                nums[count1+1..].rotate_right(idx-count1);
                id2 += 1;
            },
            _ => { },
        }
    }
}

pub fn p077_combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut walked: Vec<Vec<i32>> = vec![]; walked.push(vec![]);
    let mut out = Vec::new(); let k = k as usize;
    if k == 0 { return out; }

    for i in 1..=n {
        let mut nwalked = vec![];
        for mut curr in walked.into_iter() {
            match curr.len() {
                len if len == k - 1 => {
                    nwalked.push(curr.clone());
                    curr.push(i); out.push(curr);
                },
                _ => {
                    nwalked.push(curr.clone());
                    curr.push(i);
                    nwalked.push(curr.clone());
                }
            }
        }
        walked = nwalked;
    }
    out
}

pub fn p078_subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut out = vec![vec![]];
    for num in nums.into_iter() {
        for mut curr in out.clone().into_iter() {
            curr.push(num);
            out.push(curr);
        }
    }
    out
}

pub fn p079_exist(board: Vec<Vec<char>>, word: String) -> bool {
    use std::collections::HashSet;

    let wlen = word.len(); let m = board.len();
    if m == 0 { if wlen == 0 { return true; } else { return true; } }

    let n = board[0].len();
    if n == 0 { if wlen == 0 { return true; } else { return true; } }

    let words: Vec<_> = word.chars().collect();
    if wlen == 0 { return true; }

    fn dfs(board: &Vec<Vec<char>>, x: i32, y: i32, target: &[char], walked: &mut HashSet<(i32, i32)>, m: i32, n: i32) -> bool {
        if target.len() == 0 { return false; }
        if x >= m || x < 0 || y < 0 || y >= n { return false }
        if target[0] != board[x as usize][y as usize] { return false; }
        if walked.contains(&(x, y)) { return false; }

        if target.len() == 1 { return true; }

        walked.insert((x, y));

        if dfs(board, x+1, y, &target[1..], walked, m, n) || dfs(board, x, y+1, &target[1..], walked, m, n) ||
           dfs(board, x-1, y, &target[1..], walked, m, n) || dfs(board, x, y-1, &target[1..], walked, m, n) {
            return true;
        }
        walked.remove(&(x, y));

        false
    }

    for i in 0..m {
        for j in 0..n {
            if board[i][j] == words[0] {
                let mut walked = HashSet::new();
                if dfs(&board, i as i32, j as i32, &words, &mut walked, m as i32, n as i32) {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::listnode;

    #[test]
    fn test_p001() {
        assert_eq!(p001_two_sum(&vec![2, 7, 11, 15], 9), (0, 1));
        assert_eq!(p001_two_sum(&vec![3, 2, 4], 6), (1, 2));
    }

    #[test]
    fn test_p002() {
        let mut num1 = LinkedList::new(); num1.push_back(2); num1.push_back(4); num1.push_back(3);
        let mut num2 = LinkedList::new(); num2.push_back(5); num2.push_back(6); num2.push_back(4);
        let mut num3 = LinkedList::new(); num3.push_back(7); num3.push_back(0); num3.push_back(8);
        assert_eq!(p002_add_two_numbers(&num1, &num2), num3);

        let mut num1 = LinkedList::new(); num1.push_back(9); num1.push_back(9); num1.push_back(9); num1.push_back(9);
        let mut num2 = LinkedList::new(); num2.push_back(9); num2.push_back(9); num2.push_back(9); num2.push_back(9); num2.push_back(9); num2.push_back(9);
        let mut num3 = LinkedList::new(); num3.push_back(8); num3.push_back(9); num3.push_back(9); num3.push_back(9); num3.push_back(0); num3.push_back(0); num3.push_back(1);
        assert_eq!(p002_add_two_numbers(&num1, &num2), num3);
    }

    #[test]
    fn test_p003() {
        assert_eq!(p003_longest_substring("abcabcabc"), "abc".to_string());
        assert_eq!(p003_longest_substring("abcbcabcabc"), "abc".to_string());
        assert_eq!(p003_longest_substring("pwwkew"), "wke".to_string());
        assert_eq!(p003_longest_substring("bbbbb"), "b".to_string());
    }

    #[test]
    fn test_p004() {
        assert_eq!(p004_median_of_two_sorted_arrays(&vec![1, 3], &vec![2, 4]), 2.5);
        assert_eq!(p004_median_of_two_sorted_arrays(&vec![1, 2], &vec![3, 4, 5]), 3.0);
        assert_eq!(p004_median_of_two_sorted_arrays(&vec![2], &vec![]), 2.0);
        assert_eq!(p004_median_of_two_sorted_arrays(&vec![1, 2], &vec![]), 1.5);
    }

    #[test]
    fn test_p010() {
        assert!(p010_regular_expression_matching("mississippi".to_string(), "mis*is*ip*.".to_string()));
        assert!(!p010_regular_expression_matching("mississippi".to_string(), "mis*is*p*.".to_string()));
        assert!(p010_regular_expression_matching("ab".to_string(), ".*".to_string()));
        assert!(p010_regular_expression_matching("aab".to_string(), "c*a*b".to_string()));
        assert!(!p010_regular_expression_matching("aa".to_string(), "a".to_string()));
        assert!(p010_regular_expression_matching("aa".to_string(), "a*".to_string()));
    }

    #[test]
    fn test_p012() {
        assert_eq!(p012_int_to_roman(3), "III".to_string());
        assert_eq!(p012_int_to_roman(39), "XXXIX".to_string());
        assert_eq!(p012_int_to_roman(99), "XCIX".to_string());
        assert_eq!(p012_int_to_roman(98), "XCVIII".to_string());
        assert_eq!(p012_int_to_roman(993), "CMXCIII".to_string());
    }

    #[test]
    fn test_p013() {
        assert_eq!(p013_roman_to_int("III".to_string()), 3);
        assert_eq!(p013_roman_to_int("CMXCIII".to_string()), 993);
        assert_eq!(p013_roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn test_p014() {
        assert_eq!(p014_longest_common_prefix(
                vec![ "flower".to_string(), "flow".to_string(), "flight".to_string()
        ]), "fl".to_string());
        assert_eq!(p014_longest_common_prefix(
                vec![ "dog".to_string(), "racecar".to_string(), "car".to_string()
        ]), "".to_string());

    }

    #[test]
    fn test_p015() {
        assert_eq!(p015_three_sum( vec![ -1, 0, 1, 2, -1, -4 ] ), vec![ vec![-1, -1, 2], vec![-1, 0, 1] ]);
        assert_eq!(p015_three_sum( vec![ -1, 0, 2 ] ), vec![  ] as Vec<Vec<i32>>);
        assert_eq!(p015_three_sum( vec![ 0, 0, 0, 0 ] ), vec![ vec![ 0, 0, 0 ] ] );
        assert_eq!(p015_three_sum( vec![2,0,-2,-5,-5,-3,2,-4] ), vec![ vec![-4, 2, 2], vec![-2, 0, 2] ]);
        assert_eq!(p015_three_sum( vec![2,-3,0,-2,-5,-5,-4,1,2,-2,2,0,2,-4] ), vec![ vec![-4, 2, 2], vec![-3, 1, 2], vec![-2, 0, 2] ]);
        assert_eq!(p015_three_sum( vec![-2,0,1,1,2] ), vec![ vec![-2,0,2], vec![-2, 1, 1] ]);
    }

    #[test]
    fn test_p016() {
        assert_eq!(p016_three_sum_closest( vec![-1, 2, 1, -4, -1, 1], 1), 1);
        assert_eq!(p016_three_sum_closest( vec![0, 1, 2], 0), 3);
        assert_eq!(p016_three_sum_closest( vec![0, 2, 1, -3], 1), 0);
    }

    #[test]
    fn test_p017() {
        assert_eq!(p017_letter_combinations("23".to_string()),
                   vec!["ad", "bd", "cd", "ae", "be", "ce", "af", "bf", "cf"].into_iter().map(
                       |s| s.to_string()).collect::<Vec<String>>()
                   );
    }

    #[test]
    fn test_p018() {
        assert_eq!(p018_four_sum( vec![1, 0, -1, 0, -2, 2], 0 ), vec![ vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1] ]);
        assert_eq!(p018_four_sum( vec![-3,-2,-1,0,0,1,2,3], 0 ), vec![
                   vec![-3, -2, 2, 3], vec![-3, -1, 1, 3], vec![-3, 0, 0, 3], vec![-3, 0, 1, 2],
                   vec![-2, -1, 0, 3], vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]);
    }

    #[test]
    fn test_p019() {
        assert_eq!(
            p019_remove_nth_from_end(Some(Box::new(listnode!(1, 2, 3, 4, 5))), 2), Some(Box::new( listnode!(1, 2, 3, 5) ))
        );
        assert_eq!(
            p019_remove_nth_from_end(Some(Box::new(listnode!(1, 2, 3, 4, 5))), 1), Some(Box::new( listnode!(1, 2, 3, 4) ))
        );
    }

    #[test]
    fn test_p020() {
        assert!(p020_is_valid("()".to_string()));
        assert!(p020_is_valid("()[]{}".to_string()));
        assert!(!p020_is_valid("([)]".to_string()));
        assert!(p020_is_valid("([])".to_string()));
        assert!(!p020_is_valid("(".to_string()));
    }

    #[test]
    fn test_p022() {
        assert_eq!(p022_generate_parenthesis(1), vec!["()".to_string()]);
        //assert_eq!(p022_generate_parenthesis(4), vec!["()".to_string()]);
    }

    #[test]
    fn test_p023() {
        //println!("{:?}", p023_merge_k_lists(
                //vec![
                //Some(Box::new(listnode!(1, 3))),
                //Some(Box::new(listnode!(1, 2))),
                //]
        //));
        //assert_eq!(
            //p023_merge_k_lists(
                //vec![
                //Some(Box::new(listnode!(1, 3))),
                //Some(Box::new(listnode!(1, 2))),
                //]
                //),
            //Some(Box::new( listnode!(1, 2, 3, 4, 5) ))
        //);
    }

    #[test]
    fn test_p024() {
        assert_eq!(
            p024_swap_pairs(Some(Box::new(listnode!(1, 2, 3, 4, 5)))),
            Some(Box::new(listnode!(2, 1, 4, 3, 5)))
            );
    }

    #[test]
    fn test_p026() {
        let mut v1 = vec![0,0,1,1,1,2,2,3,3,4];
        assert_eq!( p026_remove_duplicates(&mut v1), 5 );
    }

    #[test]
    fn test_p027() {
        let mut v = vec![0,1,2,2,3,0,4,2];
        assert_eq!( p027_remove_element(&mut v, 2), 5 );
    }

    #[test]
    fn test_p028() {
        assert_eq!( p028_str_str( "hello".into(), "ll".into() ), 2 );
        assert_eq!( p028_str_str( "aaaaa".into(), "bba".into() ), -1 );
        assert_eq!( p028_str_str( "aaaaa".into(), "".into() ), 0 );
        assert_eq!( p028_str_str( "".into(), "a".into() ), -1 );
        assert_eq!( p028_str_str( "".into(), "".into() ), 0 );
        assert_eq!( p028_str_str( "ab".into(), "ab".into() ), 0 );
    }

    #[test]
    fn test_p029() {
        assert_eq!( p029_divide(10, 3), 3 );
        assert_eq!( p029_divide(7, -3), -2 );
        assert_eq!( p029_divide(7, 1), 7 );
        assert_eq!( p029_divide(-7, 1), -7 );
        assert_eq!( p029_divide(-7, -1), 7 );
        assert_eq!( p029_divide(2147483647, 1), 2147483647);
        assert_eq!( p029_divide(-2147483648, -1), 2147483647);
    }

    #[test]
    fn test_p030() {
        assert_eq!( p030_find_substring( "".to_string(), vec![ "a".to_string() ] ), vec![] );
        assert_eq!( p030_find_substring( "a".to_string(), vec![ "a".to_string() ] ), vec![0] );
        assert_eq!( p030_find_substring( "a".to_string(), vec![ ] ), vec![] );
        assert_eq!( p030_find_substring( "barfoothefoobarman".to_string(), vec![ "foo".to_string(), "bar".to_string() ] ), vec![0, 9] );
        assert_eq!(
            p030_find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec!["word","good","best", "word"].into_iter().map(|s| s.into()).collect::<Vec<String>>() ),
            vec![] );

        assert_eq!(
            p030_find_substring(
                "xcsgedisbnkkiperkawetuiokxjmrapqcjyjpgbqulcecgxoitudpcrcccotcglhpjqeptwlhasjgpaqlutaznebptwszhbostvhmtvtunfcehtpboscbwdrpzlqgohahcivxfpruwuydpqgdijhgmaymloubxvizfdxkuqeqmetduajejqnxqlldbgezdoaitzuosagegakdcthnjwmzjyeleimjyotrqphipooxqyasrihagtbqthdzppipfbhvqodheufushomrvmyrqokxrkpiuepwnloeqyikfdfmrrepfcgvqsvjektbqixetnkmlsyqxddpwhgclozdgumnghoxpndlapxohvghbjyxsebfsbiaxwnedddvsggvxdjgapnbblbvpcbhdjibixhlbkgtsooptzvurlxswynmdoviafjidsvgcebwsslmrjiiufcsgqgjgcrghdomvmuaqwwkokwhvgsrmujskqbruszdxqqtckvuirewddgyjypxszpdrswlnvoklefprajzsqyxtewecncuorzfmvqztfjglrwcrfelxcjqghvkuzgjsgoedfdwmpdxgbcxiglgiuyqdtaxuginoxrsevqsmvpuwrrhxenhalxdhzbbilqwiiqofjgrewldpemplzwlmupvvsxddncoxsccdlvkjinypbnaamloiakdujhyylmwdqajbwtkgijvjyvlkhzsjlyeufctsorvergipzswhdrqcpbowdjfohjjonegdvdkoksejkcrovjsklgiorqeybnmprusoyedkwjthnmxkwpxjxfzpvdpxtcokyibwnggjrcseopqmgnvgtuvqamntqbfpmgnzowifydloscdbpyhkvebvqqqhuvwgclfshpyfsjwnnzodzxpudqrtjhcddajhmqztfzbajxnywddxatsdllyuvbzabkjnaihikiivhvtfyxcaswfdidafebfimovoyeyioidvfzadwffqbkvlovquzvcnjydkecstkyoqxrvvwdlznildebyfzasiavufznamnqcmhzhfcufscsvitvpswhdyfxdemfqbwundwwlaqsuvkqffnvalcfkjepotvgurdiwzehbxbwsnozvbuvnzcxigmyzjfuaicxjigkfkgzxuzuytplutkdaybbiixisbhdkqopawrztqurlleghojhmmkuxifrjovtellghcicsetfrxlylwhalsuiczblqwhuhsdpwlrqpnvimhhafoaqiuakwcwmyfiizlzvyqlpfiysrfsxvsneoqomsmasrjaqwznvsakzjiraplxlfnrwdfixujpluseqtrlluyldiedasdlxscvvjeudplrjdxbxqpkkpxpxctxuyktqornyxhdmuwxytaxmphwefoejhbfhmazarmaovecpczpwcokrwiydwcofftmttlwnzwbwceoffddhsnbqxzvubjzieocxbymduozzungztjjlykdxlarojtwpjyokwbntppujcakvlvilfniqnceyvdnebcqadgtuvpfzppxanhlsvvlkxrjuuyywarwdzrzwgevxwuzjemdzholfgwzcvayvtwbspaoxhlwdivmmhpnpgywovxqqzrnsnqmfrceaobdywrkeixvovrcsqtkqkyizovghxljnmmlkfvqoulesehkvcxlo".to_string(),
                vec!["dbpyhkvebvqqqhuvwgclfshpyfs","jwnnzodzxpudqrtjhcddajhmqzt","fzbajxnywddxatsdllyuvbzabkj","naihikiivhvtfyxcaswfdidafeb","fimovoyeyioidvfzadwffqbkvlo","vquzvcnjydkecstkyoqxrvvwdlz","nildebyfzasiavufznamnqcmhzh","fcufscsvitvpswhdyfxdemfqbwu","ndwwlaqsuvkqffnvalcfkjepotv","gurdiwzehbxbwsnozvbuvnzcxig","myzjfuaicxjigkfkgzxuzuytplu","tkdaybbiixisbhdkqopawrztqur","lleghojhmmkuxifrjovtellghci","csetfrxlylwhalsuiczblqwhuhs","dpwlrqpnvimhhafoaqiuakwcwmy","fiizlzvyqlpfiysrfsxvsneoqom","smasrjaqwznvsakzjiraplxlfnr","wdfixujpluseqtrlluyldiedasd","lxscvvjeudplrjdxbxqpkkpxpxc","txuyktqornyxhdmuwxytaxmphwe","foejhbfhmazarmaovecpczpwcok","rwiydwcofftmttlwnzwbwceoffd","dhsnbqxzvubjzieocxbymduozzu","ngztjjlykdxlarojtwpjyokwbnt","ppujcakvlvilfniqnceyvdnebcq"].into_iter().map(|s| s.into()).collect::<Vec<String>>()
            ),
            vec![873]);

        assert_eq!(
            p030_find_substring(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
                vec!["a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a"].into_iter().map(|s| s.into()).collect::<Vec<String>>()
            ),
            vec![]);
    }

    #[test]
    fn test_p031() {
        let mut arr = vec![1,2,3,4];
        p031_next_permutation(&mut arr);
        assert_eq!( arr, vec![1, 2, 4, 3] );

        let mut arr = vec![3,2,1];
        p031_next_permutation(&mut arr);
        assert_eq!( arr, vec![1,2,3] );

        let mut arr = vec![1,3,2];
        p031_next_permutation(&mut arr);
        assert_eq!( arr, vec![2,1,3] );
    }

    #[test]
    fn test_p032() {
        assert_eq!( p032_longest_valid_parentheses( "()(())".into() ), 6 );
        assert_eq!( p032_longest_valid_parentheses( "(()".into() ), 2 );
        assert_eq!( p032_longest_valid_parentheses( ")()())".into() ), 4 );
        assert_eq!( p032_longest_valid_parentheses( ")((()))())".into() ), 8 );
    }

    #[test]
    fn test_p033() {
        assert_eq!(p033_search(vec![4,5,6,7,0,1,2], 4), 0);
        assert_eq!(p033_search(vec![4,5,6,7,0,1,2], 7), 3);
        assert_eq!(p033_search(vec![4,5,6,7,0,1,2], 0), 4);
        assert_eq!(p033_search(vec![4,5,6,7,0,1,2], 1), 5);
        assert_eq!(p033_search(vec![0,1], 3), -1);
        assert_eq!(p033_search(vec![0,1], 1), 1);
        assert_eq!(p033_search(vec![1], 0), -1);
        assert_eq!(p033_search(vec![3, 1], 1), 1);
    }

    #[test]
    fn test_p034() {
        assert_eq!(p034_search_range(vec![5,7,7,8,8,10], 8), vec![3,4]);
        assert_eq!(p034_search_range(vec![5,7,7,8,8,10], 6), vec![-1,-1]);
        assert_eq!(p034_search_range(vec![5,7,7,8,8,10], 10), vec![5,5]);
        assert_eq!(p034_search_range(vec![5,7,7,8,8,10], 5), vec![0,0]);
    }

    #[test]
    fn test_p035() {
        assert_eq!(p035_search_insert(vec![1,3,5,6], 5), 2);
    }

    #[test]
    fn test_p036() {
        let sudoku1: Vec<Vec<char>> = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9'],
        ];

        assert!(p036_is_valid_sudoku(sudoku1));
    }

    #[test]
    #[warn(dead_code)] // not work now
    #[allow(unused_variables)]
    fn test_p037() {
        let mut sudoku1: Vec<Vec<char>> = vec![
            vec!['.','.','9','7','4','8','.','.','.'],
            vec!['7','.','.','.','.','.','.','.','.'],
            vec!['.','2','.','1','.','9','.','.','.'],
            vec!['.','.','7','.','.','.','2','4','.'],
            vec!['.','6','4','.','1','.','5','9','.'],
            vec!['.','9','8','.','.','.','3','.','.'],
            vec!['.','.','.','8','.','3','.','2','.'],
            vec!['.','.','.','.','.','.','.','.','6'],
            vec!['.','.','.','2','7','5','9','.','.'],
        ];
        //let mut sudoku1: Vec<Vec<char>> = vec![
            //vec!['.','.','9','7','4','8','.','.','.'],
            //vec!['7','.','.','.','.','.','.','.','.'],
            //vec!['.','2','.','1','.','9','.','.','.'],
            //vec!['.','.','7','.','.','.','2','4','.'],
            //vec!['.','6','4','.','1','.','5','9','.'],
            //vec!['.','9','8','.','.','.','3','.','.'],
            //vec!['.','.','.','8','.','3','.','2','.'],
            //vec!['.','.','.','.','.','.','.','.','6'],
            //vec!['.','.','.','2','7','5','9','.','.'],
        //];
        let sol: Vec<Vec<char>> = vec![
            vec!['5','1','9','7','4','8','6','3','2'],
            vec!['7','8','3','6','5','2','4','1','9'],
            vec!['4','2','6','1','3','9','8','7','5'],
            vec!['3','5','7','9','8','6','2','4','1'],
            vec!['2','6','4','3','1','7','5','9','8'],
            vec!['1','9','8','5','2','4','3','6','7'],
            vec!['9','7','5','8','6','3','1','2','4'],
            vec!['8','3','2','4','9','1','7','5','6'],
            vec!['6','4','1','2','7','5','9','8','3'],
        ];
        p037_solve_sudoku(&mut sudoku1);
        //assert_eq!( sudoku1, sol );


        /*
         *let mut sudoku1: Vec<Vec<char>> = vec![
         *    vec!['5','3','.','.','7','.','.','.','.'],
         *    vec!['6','.','.','1','9','5','.','.','.'],
         *    vec!['.','9','8','.','.','.','.','6','.'],
         *    vec!['8','.','.','.','6','.','.','.','3'],
         *    vec!['4','.','.','8','.','3','.','.','1'],
         *    vec!['7','.','.','.','2','.','.','.','6'],
         *    vec!['.','6','.','.','.','.','2','8','.'],
         *    vec!['.','.','.','4','1','9','.','.','5'],
         *    vec!['.','.','.','.','8','.','.','7','9'],
         *];
         *let sol: Vec<Vec<char>> = vec![
         *    vec!['5','3','4','6','7','8','9','1','2'],
         *    vec!['6','7','2','1','9','5','3','4','8'],
         *    vec!['1','9','8','3','4','2','5','6','7'],
         *    vec!['8','5','9','7','6','1','4','2','3'],
         *    vec!['4','2','6','8','5','3','7','9','1'],
         *    vec!['7','1','3','9','2','4','8','5','6'],
         *    vec!['9','6','1','5','3','7','2','8','4'],
         *    vec!['2','8','7','4','1','9','6','3','5'],
         *    vec!['3','4','5','2','8','6','1','7','9']
         *];
         *p037_solve_sudoku(&mut sudoku1);
         *assert_eq!( sudoku1, sol )
         */
    }

    #[test]
    fn test_p038() {
        assert_eq!(p038_count_and_say(1), "1".to_string());
        assert_eq!(p038_count_and_say(4), "1211".to_string());
        assert_eq!(p038_count_and_say(6), "312211".to_string());
    }

    #[test]
    fn test_p039() {
        assert_eq!(p039_combination_sum(vec![2,3,6,7], 7), vec![vec![7], vec![2,2,3]]);
        assert_eq!(p039_combination_sum(vec![7], 7), vec![vec![7]]);
    }

    #[test]
    fn test_p040() {
        assert_eq!(p040_combination_sum(vec![10,1,2,7,6,1,5], 8), vec![vec![2, 6], vec![1, 7], vec![1, 2, 5], vec![1, 1, 6]]);
    }

    #[test]
    fn test_p041() {
        assert_eq!(p041_first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(p041_first_missing_positive(vec![7,8,9,11,12]), 1);
        assert_eq!(p041_first_missing_positive(vec![3,4,-1,1]), 2);
    }

    #[test]
    fn test_p042() {
        assert_eq!(p042_trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
        assert_eq!(p042_trap(vec![0,1,0,2,1,0]), 1);
        assert_eq!(p042_trap(vec![0,1,0,2,1,0,1]), 2);
    }

    #[test]
    fn test_p043() {
        assert_eq!(p043_multiply("999".into(), "999".into()), "998001".to_string());
        assert_eq!(p043_multiply("123".into(), "456".into()), "56088".to_string());
        assert_eq!(p043_multiply("123456789".into(), "987654321".into()), "121932631112635269".to_string());
        assert_eq!(p043_multiply("999".into(), "0".into()), "0".to_string());
    }

    #[test]
    fn test_p044() {
        assert_eq!(p044_is_match("aab".into(), "c*a*b".into()), false);
        assert_eq!(p044_is_match("aa".into(), "a".into()), false);
        assert_eq!(p044_is_match("aa".into(), "*".into()), true);
        assert_eq!(p044_is_match("cb".into(), "?a".into()), false);
        assert_eq!(p044_is_match("adceb".into(), "*a*b".into()), true);
        assert_eq!(p044_is_match("ssippi".into(), "ss*?i*pi".into()), false);
        assert_eq!(p044_is_match("mississippi".into(), "m??*ss*?i*pi".into()), false);
        assert_eq!(p044_is_match("ho".into(), "ho**".into()), true);
        assert_eq!(p044_is_match("bbbbbbbabbaabbabbbbaaabbabbabaaabbababbbabbbabaaabaab".into(), "b*b*ab**ba*b**b***bba".into()), false);
    }

    #[test]
    fn test_p045() {
        assert_eq!(p045_jump(vec![2,3,1,1,4]), 2);
        assert_eq!(p045_jump(vec![1,1,1,1,1]), 4);
        assert_eq!(p045_jump(vec![1,2,1,1,1]), 3);
        assert_eq!(p045_jump(vec![2,3,0,1,4]), 2);
        assert_eq!(p045_jump(vec![0]), 0);
    }

    #[test]
    fn test_p046() {
        assert_eq!(p046_permute(vec![1]), vec![vec![1]]);
        assert_eq!(p046_permute(vec![1, 2]), vec![vec![2, 1], vec![1, 2]]);
    }

    #[test]
    fn test_p047() {
        assert_eq!(p047_permute_unique(vec![1]), vec![vec![1]]);

        let mut res = p047_permute_unique(vec![1, 1, 2]);
        let mut matched = vec![vec![2, 1, 1], vec![1, 2, 1], vec![1, 1, 2]];
        res.sort();matched.sort();
        assert_eq!(res, matched);

        let mut res = p047_permute_unique(vec![1, 1, 2, 2]);
        let mut matched = vec![vec![2, 1, 2, 1], vec![2, 1, 1, 2], vec![1, 2, 1, 2], vec![2, 2, 1, 1], vec![1, 2, 2, 1], vec![1, 1, 2, 2]];
        res.sort();matched.sort();
        assert_eq!(res, matched);

        let mut res = p047_permute_unique(vec![2, 3, 3, 3, 2]);
        let mut matched = vec![vec![2,2,3,3,3], vec![2,3,2,3,3], vec![2,3,3,2,3], vec![2,3,3,3,2], vec![3,2,2,3,3], vec![3,2,3,2,3], vec![3,2,3,3,2], vec![3,3,2,2,3], vec![3,3,2,3,2], vec![3,3,3,2,2], ];
        res.sort();matched.sort();
        assert_eq!(res, matched);
    }

    #[test]
    fn test_p048() {
        let mut input = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9], ];
        p048_rotate(&mut input);
        let matched = vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ];
        assert_eq!(input, matched);

        let mut input = vec![
            vec![ 5, 1, 9,11],
            vec![ 2, 4, 8,10],
            vec![13, 3, 6, 7],
            vec![15,14,12,16],
        ];
        p048_rotate(&mut input);
        let matched = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        assert_eq!(input, matched);
    }

    #[test]
    fn test_p049() {
        assert_eq!(p049_group_anagrams(vec!["eat".to_string(), "tea".to_string()]), vec![ vec!["eat".to_string(), "tea".to_string()] ]);
    }

    #[test]
    fn test_p050() {
        assert_eq!(p050_my_pow(2.0, 1), 2.0);
        assert_eq!(p050_my_pow(2.0, 10), 1024.0);
        assert_eq!(p050_my_pow(1.0, 214748364), 1.0);
        assert_eq!(p050_my_pow(0.00001, 2147483647), 0.0);
        assert_eq!(p050_my_pow(1.0, 2147483647), 1.0);
    }

    #[test]
    fn test_p053() {
        assert_eq!(p053_max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
        assert_eq!(p053_max_sub_array(vec![-2,1,-3,4,-1,2]), 5);
        assert_eq!(p053_max_sub_array(vec![-2]), -2);
    }

    #[test]
    fn test_p054() {
        assert_eq!(
            p054_spiral_order(
                vec![
                    vec![1, 2, 3],
                    vec![4, 5, 6],
                    vec![7, 8, 9],
                ]
            ),
            vec![1,2,3,6,9,8,7,4,5],
            );
        assert_eq!(
            p054_spiral_order(
                vec![
                    vec![1, 2, 3, 4],
                    vec![5, 6, 7, 8],
                    vec![9,10,11,12],
                ]
            ),
            vec![1,2,3,4,8,12,11,10,9,5,6,7],
            );
        assert_eq!(
            p054_spiral_order(
                vec![
                    vec![3],
                    vec![2],
                ]
            ),
            vec![3, 2],
            );
        assert_eq!(
            p054_spiral_order(
                vec![
                    vec![1, 2, 3],
                    vec![4, 5, 6],
                ]
            ),
            vec![1, 2, 3, 6, 5, 4],
            );
        assert_eq!(
            p054_spiral_order(
                vec![
                    vec![1, 2],
                    vec![3, 4],
                ]
            ),
            vec![1, 2, 4, 3],
            );
    }

    #[test]
    fn test_p051() {
        assert_eq!(p052_total_n_queens(1), 1);
        assert_eq!(p052_total_n_queens(2), 0);
        assert_eq!(p052_total_n_queens(3), 0);
        assert_eq!(p052_total_n_queens(4), 2);
        assert_eq!(p052_total_n_queens(5), 10);
        assert_eq!(p052_total_n_queens(8), 92);
        //assert_eq!(p052_total_n_queens(12), 14200); // emmm..., it can't reach here
    }

    #[test]
    fn test_p055() {
        assert_eq!(p055_can_jump(vec![1, 2, 3]), true);
        assert_eq!(p055_can_jump(vec![8,2,4,4,4,9,5,2,5,8,8,0,8,6,9,1,1,6,3,5,1,2,6,6,0,4,8,6,0,3,2,8,7,6,5,1,7,0,3,4,8,3,5,9,0,4,0,1,0,5,9,2,0,7,0,2,1,0,8,2,5,1,2,3,9,7,4,7,0,0,1,8,5,6,7,5,1,9,9,3,5,0,7,5]), true);
        assert_eq!(p055_can_jump(vec![2, 0, 2]), true);
        assert_eq!(p055_can_jump(vec![2, 0]), true);
        assert_eq!(p055_can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(p055_can_jump(vec![3, 2, 1, 0]), true);
        assert_eq!(p055_can_jump(vec![3, 2, 2, 0, 0]), true);
        assert_eq!(p055_can_jump(vec![3, 2, 2, 0, 0, 1]), false);
        assert_eq!(p055_can_jump(vec![3, 0, 0, 0]), true);
        assert_eq!(p055_can_jump(vec![0]), true);
        assert_eq!(p055_can_jump(vec![2, 1, 0, 0]), false);

        //let mut vec: Vec<i32> = (0..25000+1).rev().collect();
        //vec.push(0);
        //assert_eq!(p055_can_jump(vec), false);
    }

    #[test]
    fn test_p056() {
        assert_eq!(p056_merge(vec![vec![1,4], vec![4,5]]), vec![vec![1, 5]]);
        assert_eq!(p056_merge(vec![ vec![1,3], vec![2,6], vec![8,10], vec![15,18] ]), vec![ vec![1,6], vec![8,10], vec![15,18] ]);
        assert_eq!(p056_merge(vec![ vec![-1, 5], vec![2,3], vec![4,5], vec![6,7], vec![8,9], vec![1,10] ]), vec![ vec![-1,10] ]);
    }

    #[test]
    fn test_p057() {
        assert_eq!(p057_insert(vec![vec![1,3], vec![6,9]], vec![2,5]), vec![vec![1, 5], vec![6, 9]]);
    }

    #[test]
    fn test_p058() {
        assert_eq!(p058_length_of_last_word("Hello World".into()), 5);
        assert_eq!(p058_length_of_last_word("a ".into()), 1);
    }

    #[test]
    fn test_p060() {
        assert_eq!(p060_get_permutation(3, 3), "213");
        assert_eq!(p060_get_permutation(4, 9), "2314");
        assert_eq!(p060_get_permutation(9, 24479), "168975423");
    }

    #[test]
    fn test_p061() {
        //assert_eq!( p061_rotate_right( Some(Box::new(listnode!(1, 2, 3, 4, 5))), 1 ), Some( Box::new(listnode!(2, 1, 4, 3, 5)) ));
    }

    #[test]
    fn test_p1223() {
        assert_eq!( p1223_die_simulator( 2, vec![1, 1, 2, 2, 2, 3] ), 34 );
    }

    #[test]
    fn test_p062() {
        assert_eq!(p062_unique_path(2, 100), 100);
        assert_eq!(p062_unique_path(3, 2), 3);
        assert_eq!(p062_unique_path(4, 2), 4);
        assert_eq!(p062_unique_path(7, 3), 28);
        assert_eq!(p062_unique_path(10, 10), 48620);
    }

    #[test]
    fn test_p063() {
        assert_eq!(p063_unique_paths_with_obstacles(vec![vec![0,0,0,0], vec![0,0,0,0], vec![0,0,0,0], vec![0,0,1,0]]), 10);

        assert_eq!(p063_unique_paths_with_obstacles(vec![vec![1]]), 0);
        assert_eq!(p063_unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]), 2);
        assert_eq!(p063_unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 1, 0]]), 3);
        assert_eq!(p063_unique_paths_with_obstacles(vec![vec![0, 0, 0, 0], vec![1, 0, 0, 0]]), 3);
        assert_eq!(p063_unique_paths_with_obstacles(vec![vec![0, 0, 0, 0], vec![1, 1, 1, 0]]), 1);
        assert_eq!(p063_unique_paths_with_obstacles(vec![vec![0, 0, 0, 0], vec![1, 1, 1, 0], vec![1, 1, 1, 0]]), 1);
        assert_eq!(p063_unique_paths_with_obstacles(vec![vec![0, 0, 0, 0], vec![1, 1, 1, 0], vec![1, 1, 1, 1]]), 0);
    }

    #[test]
    fn test_p064() {
        assert_eq!(p064_min_path_sum(vec![vec![1,1,1]]), 3);
        assert_eq!(p064_min_path_sum(vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]]), 7);
    }


    #[test]
    fn test_p065() {
        assert!( p065_is_number("0".into()));
        assert!( p065_is_number(" 0.1 ".into()));
        assert!(!p065_is_number("abc".into()));
        assert!( p065_is_number("2e10".into()));
        assert!(!p065_is_number("1 a".into()));
        assert!(!p065_is_number(" 1e".into()));
        assert!(!p065_is_number(" e3".into()));
        assert!( p065_is_number(" 6e-1".into()));
        assert!(!p065_is_number("99e2.5".into()));
        assert!( p065_is_number("54.5e93".into()));
        assert!(!p065_is_number(" --6".into()));
        assert!(!p065_is_number("-+3".into()));
        assert!( p065_is_number("00.3".into()));
        assert!(!p065_is_number("95a54e53".into()));
        assert!( p065_is_number(".1".into()));
        assert!(!p065_is_number(".".into()));
        assert!( p065_is_number("-1.".into()));
        assert!( p065_is_number("01".into()));
        assert!( p065_is_number("03.004e005".into()));
        assert!( p065_is_number("03.004e5".into()));
        assert!(!p065_is_number("4e+".into()));
        assert!( p065_is_number("4e+10".into()));
    }

    #[test]
    fn test_p066() {
        assert_eq!(p066_plus_one(vec![4,3,2,1]), vec![4,3,2,2]);
        assert_eq!(p066_plus_one(vec![9]), vec![1,0]);
    }

    #[test]
    fn test_p067() {
        assert_eq!(&p067_add_binary("11".into(), "1".into()), "100");
        assert_eq!(&p067_add_binary("1010".into(), "1011".into()), "10101");
    }

    #[test]
    fn test_p068() {
        assert_eq!(p068_my_sqrt(4), 2);
        assert_eq!(p068_my_sqrt(8), 2);
        assert_eq!(p068_my_sqrt(2147395599), 46339);
    }

    #[test]
    fn test_p069() {
        assert_eq!(p069_climb_stairs(3), 3);
        assert_eq!(p069_climb_stairs(2), 2);
    }

    #[test]
    fn test_p071() {
        assert_eq!(&p071_simplify_path("/../".into()), "/");
        assert_eq!(&p071_simplify_path("/home/".into()), "/home");
        assert_eq!(&p071_simplify_path("/a/./b/../../c/".into()), "/c");
        assert_eq!(&p071_simplify_path("/home//foo/".into()), "/home/foo");
        assert_eq!(&p071_simplify_path("/a//b////c/d//././/..".into()), "/a/b/c");
    }

    #[test]
    fn test_p074() {
        assert!(!p074_search_matrix(vec![ vec![1,   3,  5,  7], vec![10, 11, 16, 20], vec![23, 30, 34, 50] ], 13) );
        assert!(!p074_search_matrix(vec![vec![]], 1) );
    }

    #[test]
    fn test_p075() {
        fn test(mut v: Vec<i32>) -> Vec<i32> {
            p075_sort_colors(&mut v);
            v
        }

        assert_eq!(test(vec![2,0,2,1,1,0]), vec![0,0,1,1,2,2]);
    }

    #[test]
    fn test_p077() {
        assert_eq!(p077_combine(4, 2).len(), 6);
        assert_eq!(p077_combine(5, 1).len(), 5);
    }

    #[test]
    fn test_p078() {
        assert_eq!(p078_subsets(vec![1,2,3]).len(), 8);
        assert_eq!(p078_subsets(vec![1,2,3,4]).len(), 16);
    }

    #[test]
    fn test_p079() {
        fn test(s: &str, target: &str, res: bool) {
            let mut out: Vec<Vec<char>> = vec![];
            for row in s.split(",") {
                out.push(row.chars().collect());
            }
            println!("compare {:?} {}", out, target);
            assert_eq!(res, p079_exist(out, target.into()));
        }

        test("ABCE,SFCS,ADEE", "ABCCED", true);
        test("ABCE,SFCS,ADEE", "SEE", true);
        test("ABCE,SFCS,ADEE", "ABCB", false);
        test("ABCE,SFCS,ADEE", "ABCB", false);
        test("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,aaaaaaaaaaaaaaaaaaaaaaaaaaaaab",
            "baaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            true);
    }
}
