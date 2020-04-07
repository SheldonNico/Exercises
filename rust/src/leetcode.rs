use {
    std::collections::{HashMap, LinkedList, BTreeSet, VecDeque, HashSet, BTreeMap},
    std::cmp::{Eq},
    std::str::Chars,
    std::boxed::Box,
    std::fmt,
};

mod util;
pub use util::{ListNode};

mod snippet;
pub use snippet::*;

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
