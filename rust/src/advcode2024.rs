#![allow(unused_imports)]

pub fn p01() {
    let contents = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    let contents = std::fs::read_to_string("./assets/adv2024/adv01.txt").unwrap();

    let pairs: Vec<(usize, usize)> = contents
        .lines()
        .map(|line| {
            let nums: Vec<_> = line.split_ascii_whitespace().collect();
            (nums[0].parse().unwrap(), nums[1].parse().unwrap())
        })
        .collect();

    let mut left: Vec<_> = pairs.iter().map(|(x, _)| x).cloned().collect();
    let mut right: Vec<_> = pairs.iter().map(|(_, x)| x).cloned().collect();
    left.sort();
    right.sort();

    let sum1: isize = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| (l as isize - r as isize).abs())
        .sum();
    dbg!(sum1);

    let sum2: usize = left
        .iter()
        .map(|&l| right.iter().filter(|&&r| r == l).count() * l)
        .sum();
    dbg!(sum2);
}

pub fn p02() {
    let contents = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
    // let contents = std::fs::read_to_string("./assets/adv2024/adv02.txt").unwrap();

    let reports: Vec<Vec<usize>> = contents
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
    let count1 = reports.iter().filter(|levels| true).count();
    dbg!(&count1);
}
