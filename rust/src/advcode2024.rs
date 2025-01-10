#![allow(unused_imports)]

use std::{
    collections::{HashMap, HashSet},
    hash::{DefaultHasher, Hash},
    iter::FromIterator,
    ops::{BitOr, BitXor},
};

use nom::{
    character::streaming::{newline, space1},
    multi::many1,
};

use crate::advcode2023::p13_parse;

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
    let contents = std::fs::read_to_string("./assets/adv2024/adv02.txt").unwrap();

    let reports: Vec<Vec<usize>> = contents
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
    let count1 = reports
        .iter()
        .filter(|levels| {
            let is_increase = levels.iter().zip(levels.iter().skip(1)).all(|(x, y)| x < y);
            let is_decrease = levels.iter().zip(levels.iter().skip(1)).all(|(x, y)| x > y);
            let is_adj = levels.iter().zip(levels.iter().skip(1)).all(|(&x, &y)| {
                let adj = (x as isize - y as isize).abs();
                adj >= 1 && adj <= 3
            });
            (is_increase || is_decrease) && is_adj
        })
        .count();
    dbg!(&count1);

    let count2 = reports
        .iter()
        .filter(|ori| {
            (0..ori.len()).any(|idx| {
                let mut levels: Vec<_> = (*ori).clone();
                levels.remove(idx);

                let is_increase = levels.iter().zip(levels.iter().skip(1)).all(|(x, y)| x < y);
                let is_decrease = levels.iter().zip(levels.iter().skip(1)).all(|(x, y)| x > y);
                let is_adj = levels.iter().zip(levels.iter().skip(1)).all(|(&x, &y)| {
                    let adj = (x as isize - y as isize).abs();
                    adj >= 1 && adj <= 3
                });
                (is_increase || is_decrease) && is_adj
            })
        })
        .count();
    dbg!(&count2);
}

#[derive(Debug, Clone)]
pub enum P03Instruction {
    Mul(usize, usize),
    Do,
    Donot,
}

impl P03Instruction {
    pub fn parse_one(input: &str) -> nom::IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{newline, space0, space1, u64};
        use nom::combinator::{map, value};
        use nom::multi::separated_list0;
        use nom::sequence::tuple;

        nom::branch::alt((
            value(Self::Do, tag("do()")),
            value(Self::Donot, tag("don't()")),
            map(
                tuple((tag("mul("), u64, tag(","), u64, tag(")"))),
                |(_, x, _, y, _)| Self::Mul(x as usize, y as usize),
            ),
        ))(input)
    }

    pub fn parse_multi(input: &str) -> nom::IResult<&str, Vec<Self>> {
        use nom::combinator::{map, value};

        let opt = nom::branch::alt((
            map(Self::parse_one, |v| Some(v)),
            value(None, nom::character::complete::anychar),
        ));
        let (input, guess) = nom::multi::many0(opt)(input)?;
        Ok((input, guess.into_iter().flat_map(|x| x).collect()))
    }
}

pub fn p03() {
    let contents = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    let contents = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    let contents = std::fs::read_to_string("./assets/adv2024/adv03.txt").unwrap();

    let instructions: Vec<P03Instruction> =
        P03Instruction::parse_multi(contents.as_ref()).unwrap().1;
    let sum1: usize = instructions
        .iter()
        .map(|ins| match ins {
            P03Instruction::Mul(x, y) => x * y,
            _ => 0,
        })
        .sum();
    dbg!(&sum1);

    let mut sum2: usize = 0;
    let mut flag: bool = true;
    for ins in instructions.iter() {
        match ins {
            P03Instruction::Mul(x, y) => {
                if flag {
                    sum2 += x * y
                }
            }
            P03Instruction::Do => flag = true,
            P03Instruction::Donot => flag = false,
        }
    }
    dbg!(&sum2);
}

pub fn p04() {
    let contents = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    let contents = std::fs::read_to_string("./assets/adv2024/adv04.txt").unwrap();

    let xmas: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let height = xmas.len();
    assert!(height > 0);
    let width = xmas[0].len();
    assert!(width > 0);

    let mut count1 = 0;
    for idx in 0..height {
        for idy in 0..width {
            if xmas[idx][idy] != 'X' {
                continue;
            }

            for (sx, sy) in [
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ] {
                let mut res = [(idx, idy); 3];
                for count in 1..4 {
                    let ux = idx as isize + sx * count;
                    let uy = idy as isize + sy * count;
                    if ux >= 0 && ux < height as isize && uy >= 0 && uy < width as isize {
                        res[count as usize - 1] = (ux as usize, uy as usize);
                    }
                }
                let mas: String = res.iter().map(|&(x, y)| xmas[x][y]).collect();
                if mas == "MAS" {
                    count1 += 1;
                }
            }
        }
    }
    dbg!(count1);

    let mut count2 = 0;
    for idx in 0..height {
        for idy in 0..width {
            if xmas[idx][idy] != 'A' {
                continue;
            }

            let mut diag = vec![];

            for (ax, ay, bx, by) in [(1, 1, -1, -1), (1, -1, -1, 1)] {
                let ux = idx as isize + ax;
                let uy = idy as isize + ay;
                let vx = idx as isize + bx;
                let vy = idy as isize + by;
                if ux >= 0
                    && ux < height as isize
                    && uy >= 0
                    && uy < width as isize
                    && vx >= 0
                    && vx < height as isize
                    && vy >= 0
                    && vy < width as isize
                {
                    let mut res: Vec<char> = vec![
                        xmas[ux as usize][uy as usize],
                        xmas[vx as usize][vy as usize],
                    ];
                    res.sort();
                    let res: String = res.into_iter().collect();
                    // dbg!(&res);
                    diag.push(res == "MS");
                }
            }
            if diag.len() == 2 && diag.into_iter().all(|x| x) {
                count2 += 1;
            }
        }
    }
    dbg!(count2);
}

pub fn p05_parse(input: &str) -> nom::IResult<&str, (Vec<(usize, usize)>, Vec<Vec<usize>>)> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{newline, space0, space1, u64};
    use nom::combinator::map;
    use nom::multi::{many1, separated_list0};
    use nom::sequence::tuple;

    let ordering = separated_list0(
        newline,
        map(tuple((u64, tag("|"), u64)), |(x, _, y)| {
            (x as usize, y as usize)
        }),
    );
    let pages = separated_list0(newline, separated_list0(tag(","), map(u64, |x| x as usize)));

    nom::sequence::separated_pair(ordering, many1(newline), pages)(input)
}

pub fn p05() {
    let contents = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    let contents = std::fs::read_to_string("./assets/adv2024/adv05.txt").unwrap();

    let (ordering, pages) = p05_parse(contents.as_ref()).unwrap().1;

    let selected: Vec<_> = pages
        .iter()
        .filter(|page| {
            if page.len() == 0 {
                return false;
            }
            for &(x, y) in ordering.iter() {
                let mut first = None;
                let mut second = None;
                for (pdx, &p) in page.iter().enumerate() {
                    if p == x {
                        first = Some(pdx);
                    }
                    if p == y {
                        second = Some(pdx);
                    }
                }
                if let (Some(first), Some(second)) = (first, second) {
                    if first >= second {
                        return false;
                    }
                }
            }
            true
        })
        .cloned()
        .collect();
    // dbg!(&selected);
    let sum1: usize = selected.iter().map(|page| page[page.len() / 2]).sum();
    dbg!(&sum1);

    ////////////////////////////////////////////////////////////////////////////////
    let mut sum2 = 0;
    for page in pages.iter() {
        let mut relations: HashMap<usize, HashSet<usize>> =
            page.iter().map(|&k| (k, Default::default())).collect();
        for &(x, y) in ordering.iter() {
            if !relations.contains_key(&y) {
                continue;
            }
            if let Some(xs) = relations.get_mut(&x) {
                xs.insert(y);
            }
        }
        // 本来还需要跟多检测，但题目貌似没有很刁难，不需要合并 relations
        // loop {
        //     let mut is_changed: bool = false;
        //     for (x, xs) in relations.iter_mut() {
        //         for y in xs.iter() {
        //             let update: Vec<_> = relations.get(&y).unwrap().difference(xs).collect();
        //             is_changed |= update.len() > 0;
        //             for z in update.into_iter() {
        //                 xs.insert(*z);
        //             }
        //         }
        //     }
        // }

        // println!("{:?}", page);
        //
        // let mut wtf: Vec<_> = relations
        //     .iter()
        //     .map(|(k, w)| {
        //         let mut w: Vec<_> = w.iter().collect();
        //         w.sort();
        //         (k, w)
        //     })
        //     .collect();
        // wtf.sort();
        // for (k, w) in wtf.into_iter() {
        //     println!("\t{} {:?}", k, w);
        // }

        let kinds: HashMap<usize, usize> = relations.iter().map(|(&x, xs)| (x, xs.len())).collect();
        if !page.is_sorted_by_key(|x| std::cmp::Reverse(kinds.get(&x).unwrap())) {
            let mut sorted = page.clone();
            sorted.sort_by_key(|x| std::cmp::Reverse(kinds.get(&x).unwrap()));
            // eprintln!("\t\t{:?}", sorted);
            sum2 += sorted[sorted.len() / 2];
        }
        // eprintln!();
        // eprintln!();
    }
    dbg!(&sum2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum P06Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub enum P06Symbol {
    Dot,
    Obstacle,
    Guard(P06Dir),
}

impl P06Dir {
    pub fn next(self, x: isize, y: isize) -> (isize, isize) {
        match self {
            Self::Right => (x, y + 1),
            Self::Left => (x, y - 1),
            Self::Down => (x + 1, y),
            Self::Up => (x - 1, y),
        }
    }
}

impl P06Symbol {
    pub fn parse_one(input: &str) -> nom::IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::combinator::{map, value};
        use nom::sequence::tuple;

        let dot = value(Self::Dot, tag("."));
        let obstacle = value(Self::Obstacle, tag("#"));
        let guard = nom::branch::alt((
            value(Self::Guard(P06Dir::Left), tag("<")),
            value(Self::Guard(P06Dir::Right), tag(">")),
            value(Self::Guard(P06Dir::Up), tag("^")),
            value(Self::Guard(P06Dir::Down), tag("v")),
        ));

        nom::branch::alt((dot, obstacle, guard))(input)
    }

    pub fn parse_multi(input: &str) -> nom::IResult<&str, Vec<Vec<Self>>> {
        nom::multi::separated_list0(
            nom::character::complete::newline,
            nom::multi::many1(Self::parse_one),
        )(input)
    }
}

pub fn p06_walk(
    height: usize,
    width: usize,
    obstacle: &HashSet<(usize, usize)>,
    guard: (isize, isize, P06Dir),
) -> (bool, Vec<(isize, isize, P06Dir)>) {
    let (mut x0, mut y0, mut dir) = guard;
    let mut trace: Vec<(isize, isize, P06Dir)> = vec![];
    while x0 >= 0 && y0 >= 0 && x0 < height as isize && y0 < width as isize {
        if trace.iter().any(|p| p == &(x0, y0, dir)) {
            // looped
            return (true, trace);
        }

        trace.push((x0, y0, dir));

        let (x1, y1) = dir.next(x0, y0);
        if x1 >= 0
            && y1 >= 0
            && x1 < height as isize
            && y1 < width as isize
            && obstacle.contains(&(x1 as usize, y1 as usize))
        {
            dir = match dir {
                P06Dir::Up => P06Dir::Right,
                P06Dir::Right => P06Dir::Down,
                P06Dir::Down => P06Dir::Left,
                P06Dir::Left => P06Dir::Up,
            };
        } else {
            x0 = x1;
            y0 = y1;
        }
    }
    (false, trace)
}

pub fn p06() {
    let contents = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    let contents = std::fs::read_to_string("./assets/adv2024/adv06.txt").unwrap();

    let symbols = P06Symbol::parse_multi(contents.as_ref()).unwrap().1;

    let height = symbols.len();
    assert!(height > 0);
    let width = symbols[0].len();
    assert!(width > 0);

    let mut obstacle: HashSet<(usize, usize)> = Default::default();
    let mut guard: (isize, isize, P06Dir) = (height as isize, width as isize, P06Dir::Up);
    for idx in 0..height {
        for idy in 0..width {
            match symbols[idx][idy] {
                P06Symbol::Dot => {}
                P06Symbol::Obstacle => {
                    obstacle.insert((idx, idy));
                }
                P06Symbol::Guard(dir) => {
                    guard = (idx as isize, idy as isize, dir);
                }
            }
        }
    }

    eprintln!("{}x{} {:?} {:?}", height, width, obstacle, guard);
    let (_, trace) = p06_walk(height, width, &obstacle, guard);
    eprintln!("{:?}", trace);
    dbg!(trace
        .into_iter()
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<_>>()
        .len());

    let mut count = 0;
    for idx in 0..height {
        for idy in 0..width {
            if (idx == guard.0 as usize && idy == guard.1 as usize)
                || obstacle.contains(&(idx, idy))
            {
                continue;
            }

            obstacle.insert((idx, idy));
            let (flag, trace) = p06_walk(height, width, &obstacle, guard);
            // eprintln!("{:?}", (idx, idy, flag));
            if flag {
                eprintln!("Inserted@{:?}", (idx, idy));
                count += 1;
            }

            obstacle.remove(&(idx, idy));
        }
    }
    dbg!(count);
}

pub fn p07_parse(input: &str) -> nom::IResult<&str, Vec<(isize, Vec<isize>)>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{i64, newline, space1};
    use nom::combinator::{map, value};
    use nom::multi::separated_list0;
    use nom::sequence::tuple;

    let expr = map(
        tuple((i64, tag(":"), space1, separated_list0(space1, i64))),
        |(x, _, _, xs)| (x as isize, xs.into_iter().map(|y| y as isize).collect()),
    );

    separated_list0(newline, expr)(input)
}

pub fn p07_check(nums: &[isize], stack: isize, out: isize) -> bool {
    if nums.len() == 0 {
        return stack == out;
    }

    return p07_check(&nums[1..], stack * nums[0], out)
        || p07_check(&nums[1..], stack + nums[0], out);
}

pub fn p07_check3(nums: &[isize], stack: isize, out: isize) -> bool {
    if nums.len() == 0 {
        return stack == out;
    }
    // if stack > out { return false; }

    fn concat(mut left: isize, right: isize) -> isize {
        let mut exp = right;
        while exp >= 10 {
            left *= 10;
            exp /= 10;
        }
        left * 10 + right
    }

    return p07_check3(&nums[1..], stack * nums[0], out)
        || p07_check3(&nums[1..], stack + nums[0], out)
        || p07_check3(&nums[1..], concat(stack, nums[0]), out);
}

pub fn p07() {
    let contents = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    let contents = std::fs::read_to_string("./assets/adv2024/adv07.txt").unwrap();

    let exprs = p07_parse(contents.as_ref()).unwrap().1;

    // // eprintln!("{:?}", exprs);
    // for expr in exprs.iter() {
    //     eprintln!("{:?}", expr);
    // }
    // todo!();
    let sum1: isize = exprs
        .iter()
        .filter(|(out, nums)| p07_check(&nums[1..], nums[0], *out))
        .map(|(out, nums)| {
            // eprintln!("\t{:?}", nums);
            out
        })
        .sum();
    dbg!(&sum1);

    // let mut exprs = vec![(85102760464, vec![8, 5, 102, 760, 457, 7])];
    // let exprs = vec![(85102760464, vec![8, 5, 102])];
    let sum2: isize = exprs
        .iter()
        .filter(|(out, nums)| {
            let flag = p07_check3(&nums[1..], nums[0], *out);
            // if !flag {
            //     eprintln!("\t{:>16} != {:?}", out, nums);
            // }
            flag
        })
        .map(|(out, nums)| out)
        .sum();
    dbg!(&sum2);
}

pub fn p08() {
    let contents = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    let contents = std::fs::read_to_string("./assets/adv2024/adv08.txt").unwrap();

    let antennas: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = antennas.len();
    assert!(height > 0);
    let width = antennas[0].len();
    assert!(width > 0);
    let mut antennas_pos: HashMap<char, Vec<(usize, usize)>> = Default::default();
    for idx in 0..height {
        for idy in 0..width {
            let symbol = antennas[idx][idy];
            if symbol != '.' {
                antennas_pos.entry(symbol).or_default().push((idx, idy));
            }
        }
    }

    // eprintln!("{:?}", antennas_pos);
    let mut antinodes: HashSet<(usize, usize)> = Default::default();
    for pos in antennas_pos.values() {
        for idx in 0..pos.len() {
            for idj in (idx + 1)..pos.len() {
                let (x0, y0) = pos[idx];
                let (x1, y1) = pos[idj];
                let (x0, y0, x1, y1) = (x0 as isize, y0 as isize, x1 as isize, y1 as isize);
                // eprintln!("{:?} - {:?}", (x0, y0), (x1, y1));

                let xa = x0 - (x1 - x0);
                let ya = y0 - (y1 - y0);
                let xb = x1 + (x1 - x0);
                let yb = y1 + (y1 - y0);

                for (xx, yy) in [(xa, ya), (xb, yb)] {
                    if xx >= 0 && xx < height as isize && yy >= 0 && yy < height as isize {
                        antinodes.insert((xx as usize, yy as usize));
                    }
                }
            }
        }
    }
    dbg!(&antinodes.len());

    let mut antinodes: HashSet<(usize, usize)> = Default::default();
    for pos in antennas_pos.values() {
        for idx in 0..pos.len() {
            for idj in (idx + 1)..pos.len() {
                let (x0, y0) = pos[idx];
                let (x1, y1) = pos[idj];
                let (x0, y0, x1, y1) = (x0 as isize, y0 as isize, x1 as isize, y1 as isize);
                // eprintln!("{:?} - {:?}", (x0, y0), (x1, y1));

                for is_negative in [true, false] {
                    for count in 0.. {
                        let (xx, yy) = if is_negative {
                            (x0 - (x1 - x0) * count, y0 - (y1 - y0) * count)
                        } else {
                            (x1 + (x1 - x0) * count, y1 + (y1 - y0) * count)
                        };
                        if xx >= 0 && xx < height as isize && yy >= 0 && yy < height as isize {
                            antinodes.insert((xx as usize, yy as usize));
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    dbg!(&antinodes.len());
}

pub fn p09() {
    let contents = r#"2333133121414131402"#;
    let contents = std::fs::read_to_string("./assets/adv2024/adv09.txt").unwrap();
    // let contents = r#"90909"#;

    let disk: Vec<_> = contents
        .trim()
        .chars()
        .map(|n| (n as u8 - '0' as u8) as usize)
        .collect();
    dbg!(&disk.len());

    let mut count = -1;
    let mut is_zero: bool = false;
    let disk: Vec<(isize, usize)> = disk
        .into_iter()
        .map(|len| {
            let out = if !is_zero {
                count += 1;
                (count, len)
            } else {
                (-1, len)
            };

            is_zero = !is_zero;
            out
        })
        .collect();
    // eprintln!("{:?}", disk);

    ////////////////////////////////////////////////////////////////////////////////
    let mut checksum = 0;
    let mut left = 0;
    let mut left_idx = 0;
    let mut right = disk.len() - 1;
    let mut right_idx = disk[right].1;
    let mut pos = 0;

    while (left, left_idx) < (right, right_idx) {
        let (start_id, start_len) = disk[left];

        if left_idx >= start_len {
        } else if start_id == -1 {
            let (end_id, end_len) = disk[right];

            if end_id == -1 {
                right -= 1;
                right_idx = disk[right].1;
            } else {
                if right_idx == 0 {
                    right -= 1;
                    right_idx = disk[right].1;
                } else {
                    eprintln!("F: {:>5} x {:>5} = {}", pos, end_id, end_id * pos);
                    checksum += end_id * pos;

                    right_idx -= 1;
                    left_idx += 1;
                    pos += 1;
                }
            }
        } else {
            eprintln!("X: {:>5} x {:>5} = {}", pos, start_id, start_id * pos);
            checksum += start_id * pos;
            pos += 1;

            left_idx += 1;
        }

        // 左侧进位
        if left_idx >= start_len {
            left += 1;
            left_idx = 0;
        }
    }
    dbg!(checksum);

    ////////////////////////////////////////////////////////////////////////////////
    // eprintln!("{:?}", disk);
    let mut disk = disk.clone();
    let mut pos = disk.len() - 1;
    while pos > 0 {
        let (symbol, count) = disk[pos];
        if symbol > 0 {
            if let Some(inserted) = disk[..pos]
                .iter()
                .position(|&(sym, space)| sym < 0 && space >= count)
            {
                let (_, space) = disk[inserted];
                disk[pos] = (-1, count);

                if space > count {
                    disk[inserted] = (-1, space - count);
                    disk.insert(inserted, (symbol, count));
                    pos += 1;
                } else {
                    disk[inserted] = (symbol, count);
                }
            }
        }
        pos -= 1;
    }
    // eprintln!("{:?}", disk);
    let mut checksum = 0;
    let mut pos = 0;
    for &(symbol, count) in disk.iter() {
        for idx in 0..count {
            if symbol >= 0 {
                checksum += symbol * pos;
            }

            pos += 1;
        }
    }
    dbg!(checksum);
}

fn p10_trace(
    heights: &Vec<Vec<usize>>,
    height: usize,
    width: usize,
    nine: &(usize, usize),
    (x0, y0): (usize, usize),
) -> bool {
    if &(x0, y0) == nine {
        return true;
    }

    let curr = heights[x0][y0];
    for (offsetx, offsety) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let nx = x0 as isize + offsetx;
        let ny = y0 as isize + offsety;

        // eprintln!("{} {} {} {}", nx, ny, height, width);
        if nx >= 0 && nx < height as isize && ny >= 0 && ny < width as isize {
            if heights[nx as usize][ny as usize] == curr + 1 {
                if p10_trace(heights, height, width, nine, (nx as usize, ny as usize)) {
                    return true;
                }
            }
        }
    }
    return false;
}

fn p10_trace_rating(
    heights: &Vec<Vec<usize>>,
    height: usize,
    width: usize,
    nine: &(usize, usize),
    (x0, y0): (usize, usize),
) -> usize {
    if &(x0, y0) == nine {
        return 1;
    }

    let curr = heights[x0][y0];
    let mut sum = 0;
    for (offsetx, offsety) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let nx = x0 as isize + offsetx;
        let ny = y0 as isize + offsety;

        // eprintln!("{} {} {} {}", nx, ny, height, width);
        if nx >= 0 && nx < height as isize && ny >= 0 && ny < width as isize {
            if heights[nx as usize][ny as usize] == curr + 1 {
                sum += p10_trace_rating(heights, height, width, nine, (nx as usize, ny as usize));
            }
        }
    }
    return sum;
}

pub fn p10() {
    let contents = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
    let contents = std::fs::read_to_string("./assets/adv2024/adv10.txt").unwrap();

    let heights: Vec<Vec<usize>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c as u8 - '0' as u8) as usize)
                .collect()
        })
        .collect();
    // eprintln!("{:?}", heights);

    let height = heights.len();
    assert!(height > 0);
    let width = heights[0].len();
    assert!(width > 0);
    let nines: Vec<(usize, usize)> = heights
        .iter()
        .enumerate()
        .flat_map(|(idx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &h)| h == 9)
                .map(move |(idy, _)| (idx, idy))
        })
        .collect();

    let zeros: Vec<(usize, usize)> = heights
        .iter()
        .enumerate()
        .flat_map(|(idx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &h)| h == 0)
                .map(move |(idy, _)| (idx, idy))
        })
        .collect();

    eprintln!("{:?}", nines);
    eprintln!("{:?}", zeros);
    ////////////////////////////////////////////////////////////////////////////////
    let count1: usize = zeros
        .iter()
        .map(|&xy| {
            nines
                .iter()
                .filter(|nine| p10_trace(&heights, height, width, nine, xy))
                .count()
        })
        .inspect(|x| eprintln!("\t{}", x))
        .sum();
    dbg!(count1);

    ////////////////////////////////////////////////////////////////////////////////
    let count2: usize = zeros
        .iter()
        .map(|&xy| {
            nines
                .iter()
                .map(|nine| p10_trace_rating(&heights, height, width, nine, xy))
                .sum::<usize>()
        })
        .inspect(|x| eprintln!("\t{}", x))
        .sum();
    dbg!(count2);
}

fn p11_exp(mut num: usize) -> usize {
    for idx in 1.. {
        if num < 10 {
            return idx;
        }
        num /= 10;
    }
    unreachable!();
}

pub fn p11() {
    let contents = r#"0 1 10 99 999"#;
    let contents = r#"125 17"#;

    let contents = std::fs::read_to_string("./assets/adv2024/adv11.txt").unwrap();

    let stones: Vec<usize> = contents
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    eprintln!("{:?}", stones);

    let mut blinked = stones.clone();
    for idx in 0..25 {
        for mut num in std::mem::replace(&mut blinked, Default::default()).into_iter() {
            let exp = p11_exp(num);
            if num == 0 {
                blinked.push(1);
            } else if exp % 2 == 0 {
                let half = exp / 2;
                let mut right = 0;
                for idx in 0..half {
                    right += (num % 10) * (10_i32.pow(idx as u32) as usize);
                    num /= 10;
                }
                blinked.push(num);
                blinked.push(right);
            } else {
                blinked.push(num * 2024);
            };
        }
        eprintln!(
            "\t{:>2} {:>12}- {:?}",
            idx + 1,
            blinked.len(),
            &blinked[..10.min(blinked.len())]
        );
    }

    ////////////////////////////////////////////////////////////////////////////////
    eprintln!();
    let mut blinked: HashMap<usize, usize> = stones.iter().cloned().map(|n| (n, 1)).collect();
    for idx in 0..75 {
        for (mut num, count) in std::mem::replace(&mut blinked, Default::default()).into_iter() {
            let exp = p11_exp(num);
            if num == 0 {
                *blinked.entry(1).or_default() += count;
            } else if exp % 2 == 0 {
                let half = exp / 2;
                let mut right = 0;
                for idx in 0..half {
                    right += (num % 10) * (10_i32.pow(idx as u32) as usize);
                    num /= 10;
                }

                *blinked.entry(num).or_default() += count;
                *blinked.entry(right).or_default() += count;
            } else {
                *blinked.entry(num * 2024).or_default() += count;
            };
        }
        eprintln!(
            "\t{:>2} {:>18} - {}",
            idx + 1,
            blinked.values().sum::<usize>(),
            blinked.len(), // &blinked
        );
    }
}

#[derive(Debug, Clone)]
struct P12Plant {
    symbol: char,
    // outside border
    bounds: Vec<(usize, usize, usize)>,
    // other plant inside
    disjoint: Vec<P12Plant>,
}

impl P12Plant {
    fn fense(&self) -> usize {
        self.area() * self.perimeter() + self.disjoint.iter().map(|p| p.fense()).sum::<usize>()
    }

    fn area(&self) -> usize {
        let count: usize = self.bounds.iter().map(|&(xx, sy, ey)| ey - sy + 1).sum();
        let disjoint: usize = self.disjoint.iter().map(|p| p.area()).sum();

        // eprintln!("{:?}", self);
        // eprintln!("{} - {}", count, disjoint);
        count - disjoint
    }
    fn perimeter(&self) -> usize {
        let mut outer: usize = 0;

        let mut is_first = true;
        let mut oey = 0;
        let mut osy = 0;

        for &(xx, sy, ey) in self.bounds.iter() {
            let hori = ey - sy + 1;
            let vert = 1;
            let combined = if is_first {
                0
            } else {
                ey.min(oey) - sy.max(osy) + 1
            };

            outer += hori + vert * 2 + hori - combined * 2;

            osy = sy;
            oey = ey;
            is_first = false;
        }

        let inside: usize = self.disjoint.iter().map(|p| p.perimeter()).sum();
        outer + inside
    }
}

fn p12_search(
    garden: &Vec<Vec<char>>,
    height: usize,
    width: usize,
    (sx, sy): (usize, usize),
    searched: &mut Vec<Vec<bool>>,
) -> P12Plant {
    let symbol = garden[sx][sy];
    let mut bounds: Vec<(usize, usize, usize)> = vec![];

    let mut xx = sx;
    let mut sy = sy;
    let mut ey = sy;

    while xx < height {
        if garden[xx][sy] != symbol && garden[xx][ey] != symbol {
            break;
        }
        if garden[xx][sy] != symbol {
            sy = ey;
        }
        if garden[xx][ey] != symbol {
            ey = sy;
        }

        while sy > 0 && garden[xx][sy] == symbol {
            sy -= 1;
        }
        if garden[xx][sy] != symbol {
            sy += 1;
        }
        while ey < width - 1 && garden[xx][ey] == symbol {
            ey += 1;
        }
        if garden[xx][ey] != symbol {
            ey -= 1;
        }
        bounds.push((xx, sy, ey));

        xx += 1;
    }
    let mut disjoint = vec![];

    for (idx, &(xx, sy, ey)) in bounds.iter().enumerate() {
        for yy in sy..(ey + 1) {
            if searched[xx][yy] {
            } else if garden[xx][yy] == symbol {
                searched[xx][yy] = true;
            } else {
                if bounds.len() > 0 && idx >= 1 && idx + 1 < bounds.len() {
                    let (_, prev_sx, prev_ey) = bounds[idx - 1];
                    let (_, next_sx, next_ey) = bounds[idx + 1];
                    if ey >= prev_sx && ey <= prev_ey && ey >= next_sx && ey <= next_ey {
                        disjoint.push(p12_search(garden, height, width, (xx, yy), searched));
                    }
                }
            }
        }
    }

    let minx = bounds[0].0;
    let maxx = bounds.last().unwrap().0;
    assert_eq!(bounds.len(), maxx - minx + 1);

    // let mut others = vec![];
    for dis in std::mem::replace(&mut disjoint, Default::default()).into_iter() {
        if dis.bounds.iter().all(|&(dis_xx, dis_sy, dis_ey)| {
            if dis_xx > minx && dis_xx < maxx {
                let (out_xx, out_sy, out_ey) = bounds[dis_xx - minx];
                if out_sy < dis_sy && dis_ey < out_ey {
                    return true;
                }
            }
            return false;
        }) {
            disjoint.push(dis);
        } else {
            // patch
            for &(xx, sy, ey) in dis.bounds.iter() {
                for yy in sy..(ey + 1) {
                    searched[xx][yy] = false;
                }
            }
            // others.push(dis);
        }
    }

    // eprintln!("{} - {:?}", symbol, bounds);

    P12Plant {
        symbol,
        bounds,
        disjoint,
    }
}

fn p12_line(points: &Vec<usize>) -> usize {
    if points.len() == 0 {
        return 0;
    }
    let mut first = 0;
    let mut is_first = true;
    let mut count = 0;
    for idx in 0..points.len() {
        if is_first || points[idx] != first + 1 {
            count += 1;
        }
        is_first = false;
        first = points[idx];
    }

    count
}

#[derive(Debug, Clone)]
struct P12PlantRegion {
    symbol: char,
    minx: usize,
    points: Vec<Vec<(usize, usize)>>,
    height: usize,
    width: usize,
}

fn p12_expand(
    garden: &Vec<Vec<char>>,
    height: usize,
    width: usize,
    (sx, sy): (usize, usize),
) -> P12PlantRegion {
    let mut searched: HashSet<(usize, usize)> = Default::default();
    let symbol = garden[sx][sy];
    let mut curr: HashSet<(usize, usize)> = vec![(sx, sy)].into_iter().collect();
    while curr.len() > 0 {
        for (sx, sy) in std::mem::replace(&mut curr, Default::default()).into_iter() {
            searched.insert((sx, sy));
            for (offsetx, offsety) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = sx as isize + offsetx;
                let ny = sy as isize + offsety;
                if nx >= 0 && nx < height as isize && ny >= 0 && ny < height as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if garden[nx][ny] == symbol && !searched.contains(&(nx, ny)) {
                        curr.insert((nx, ny));
                    }
                }
            }
        }
    }

    let minx = searched.iter().map(|&(xx, _)| xx).min().unwrap();
    let mut points = vec![];
    for offset in 0.. {
        let mut row = vec![];
        for &(xx, yy) in searched.iter() {
            if xx == minx + offset {
                row.push((xx, yy));
            }
        }
        if row.len() == 0 {
            break;
        }
        row.sort_by_key(|&(_, yy)| yy);
        points.push(row);
    }

    // eprintln!("{:?}", points);

    P12PlantRegion {
        symbol,
        minx,
        points,
        height,
        width,
    }
}

impl P12PlantRegion {
    fn maxx(&self) -> usize {
        return self.minx + self.points.len();
    }

    fn contain_point(&self, (sx, sy): (usize, usize)) -> bool {
        for (offsetx, offsety) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            for count in 1.. {
                let nx = (sx as isize) + offsetx * count;
                let ny = (sy as isize) + offsety * count;
                if nx >= 0 && nx < self.height as isize && ny >= 0 && ny < self.width as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if self.points[nx - self.minx].iter().any(|&(_, yy)| yy == ny) {
                        break;
                    } else {
                        eprintln!("Checking {:?}, find {:?}", (sx, sy), (nx, ny));
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        return true;
    }

    fn contains(&self, other: &Self) -> bool {
        if self.minx >= other.minx || self.maxx() <= other.maxx() {
            return false;
        }

        for row in other.points.iter() {
            for point in row.iter() {
                if !self.contain_point(*point) {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn area(&self) -> usize {
        self.points.iter().map(|row| row.len()).sum()
    }

    pub fn perimeter(&self) -> usize {
        let mut count = 0;
        for idx in 0..self.points.len() {
            for idj in 0..self.points[idx].len() {
                let (sx, sy) = self.points[idx][idj];
                count += 4;
                if idj > 0 && self.points[idx][idj - 1].1 + 1 == sy {
                    count -= 2;
                }
                if idx > 0 && self.points[idx - 1].iter().any(|&(_, yy)| yy == sy) {
                    count -= 2;
                }
            }
        }

        count
    }

    pub fn side(&self) -> usize {
        let mut count = 0;
        let points_vert: Vec<Vec<usize>> = self
            .points
            .iter()
            .map(|line| line.iter().map(|&(_, yy)| yy).collect())
            .collect();
        let mut points_hori: HashMap<usize, Vec<usize>> = Default::default();
        for line in self.points.iter() {
            for &(xx, yy) in line.iter() {
                points_hori.entry(yy).or_default().push(xx);
            }
        }
        let mut points_hori: Vec<_> = points_hori
            .into_iter()
            .map(|(k, mut line)| {
                line.sort();
                (k, line)
            })
            .collect();
        points_hori.sort();
        let points_hori: Vec<Vec<usize>> = points_hori.into_iter().map(|(_, line)| line).collect();

        for points in [points_vert, points_hori] {
            for idx in 1..points.len() {
                let prevline: HashSet<_> = points[idx - 1].iter().cloned().collect();
                let currline: HashSet<_> = points[idx].iter().cloned().collect();
                let mut points: Vec<_> = currline
                    .difference(&prevline)
                    .into_iter()
                    .cloned()
                    .collect();
                points.sort();
                count += p12_line(&points);

                let mut points: Vec<_> = prevline
                    .difference(&currline)
                    .into_iter()
                    .cloned()
                    .collect();
                points.sort();
                count += p12_line(&points);

                // eprintln!(">>>> {:?}", count);
            }
            count += p12_line(&Vec::from_iter(points[0].iter().cloned()));
            count += p12_line(&Vec::from_iter(points.last().unwrap().iter().cloned()));
        }

        // eprintln!("Fin {}", count);

        count
    }
}

pub fn p12() {
    let contents = r#"OOOOO
OXOXO
OOOOO
OXOXO
OXOXO"#;

    let contents = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
    let contents = r#"AAAA
BBCD
BBCC
EEEC"#;
    let contents = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;
    let contents = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
    let contents = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;
    let contents = std::fs::read_to_string("./assets/adv2024/adv12.txt").unwrap();

    let garden: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    // eprintln!("{:?}", garden);

    let height = garden.len();
    assert!(height > 0);
    let width = garden[0].len();
    assert!(width > 0);

    ////////////////////////////////////////////////////////////////////////////////
    // let mut plants: Vec<P12Plant> = vec![];
    // let mut searched: Vec<Vec<bool>> = vec![vec![false; width]; height];
    // let (mut sx, mut sy) = (0, 0);
    // loop {
    //     let plant = p12_search(&garden, height, width, (sx, sy), &mut searched);
    //     plants.push(plant);
    //
    //     let mut is_found = false;
    //     for idx in 0..height {
    //         for idy in 0..width {
    //             if is_found {
    //                 break;
    //             }
    //             if !searched[idx][idy] {
    //                 sx = idx;
    //                 sy = idy;
    //                 is_found = true;
    //             }
    //         }
    //     }
    //     if !is_found {
    //         break;
    //     }
    // }
    //
    // for plant in plants.iter() {
    //     eprintln!("{:?}", plant);
    //     eprintln!(
    //         "\t{}: {} x {}",
    //         plant.symbol,
    //         plant.area(),
    //         plant.perimeter()
    //     );
    // }
    //
    // dbg!(plants.iter().map(|p| p.fense()).sum::<usize>());
    ////////////////////////////////////////////////////////////////////////////////

    let mut searched: Vec<Vec<bool>> = vec![vec![false; width]; height];
    let (mut sx, mut sy) = (0, 0);
    let mut plants: Vec<P12PlantRegion> = vec![];
    loop {
        let plant = p12_expand(&garden, height, width, (sx, sy));
        for row in plant.points.iter() {
            for &(xx, yy) in row.iter() {
                searched[xx][yy] = true;
            }
        }
        eprintln!("{:?}", plant);
        plants.push(plant);

        let mut is_found = false;
        for idx in 0..height {
            for idy in 0..width {
                if is_found {
                    break;
                }
                if !searched[idx][idy] {
                    sx = idx;
                    sy = idy;
                    is_found = true;
                }
            }
        }
        if !is_found {
            break;
        }
    }

    // the solution is ugly and fucked, no time to refactor
    ////////////////////////////////////////////////////////////////////////////////
    let mut sum = 0;
    for idx in 0..plants.len() {
        let plant = &plants[idx];
        let area = plant.area();
        let perimeter = plant.perimeter();

        eprintln!("{:?}", plant);
        eprintln!("\t{}: {} x {}", plant.symbol, area, perimeter,);
        sum += area * perimeter;
    }
    eprintln!("Part A: {}", sum);
    eprintln!();

    ////////////////////////////////////////////////////////////////////////////////
    let mut sum = 0;
    for idx in 0..plants.len() {
        let plant = &plants[idx];

        let area = plant.area();
        let side = plant.side();

        eprintln!("{:?}", plant);
        eprintln!("\t{}: {} x {}", plant.symbol, area, side,);

        sum += area * side;
    }
    eprintln!("Part B: {}", sum);
}

#[derive(Debug, Clone)]
struct P13Puzzle {
    ax: isize,
    ay: isize,
    bx: isize,
    by: isize,
    x: isize,
    y: isize,
}

impl P13Puzzle {
    pub fn parse_one(input: &str) -> nom::IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{i64, newline, space0, space1, u64};
        use nom::combinator::{map, value};
        use nom::multi::separated_list0;
        use nom::sequence::tuple;

        let (input, _) = tuple((tag("Button A:"), space0))(input)?;
        let (input, ax) = map(tuple((tag("X+"), i64)), |(_, n)| n as isize)(input)?;
        let (input, _) = tuple((tag(","), space0))(input)?;
        let (input, ay) = map(tuple((tag("Y+"), i64)), |(_, n)| n as isize)(input)?;

        let (input, _) = newline(input)?;

        let (input, _) = tuple((tag("Button B:"), space0))(input)?;
        let (input, bx) = map(tuple((tag("X+"), i64)), |(_, n)| n as isize)(input)?;
        let (input, _) = tuple((tag(","), space0))(input)?;
        let (input, by) = map(tuple((tag("Y+"), i64)), |(_, n)| n as isize)(input)?;

        let (input, _) = newline(input)?;

        let (input, _) = tuple((tag("Prize:"), space0))(input)?;
        let (input, x) = map(tuple((tag("X="), i64)), |(_, n)| n as isize)(input)?;
        let (input, _) = tuple((tag(","), space0))(input)?;
        let (input, y) = map(tuple((tag("Y="), i64)), |(_, n)| n as isize)(input)?;

        Ok((
            input,
            Self {
                ax,
                ay,
                bx,
                by,
                x,
                y,
            },
        ))
    }

    pub fn parse_multi(input: &str) -> nom::IResult<&str, Vec<Self>> {
        nom::multi::separated_list0(
            nom::multi::many1(nom::character::complete::newline),
            Self::parse_one,
        )(input)
    }

    pub fn solve(&self) -> Option<(isize, isize)> {
        // let A = (self.x * self.by - self.y * self.bx) / (self.ax * self.by - self.bx * self.ay);
        let a0 = self.x * self.by - self.y * self.bx;
        let a1 = self.ax * self.by - self.bx * self.ay;
        if a0 % a1 == 0 {
            let a = a0 / a1;
            // let B = (self.x - self.ax * A) / self.bx;
            let b0 = self.x - self.ax * a;
            let b1 = self.bx;
            if b0 % b1 == 0 {
                let b = b0 / b1;
                return Some((a, b));
            }
        }
        None
    }
}

pub fn p13() {
    let contents = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    let contents = std::fs::read_to_string("./assets/adv2024/adv13.txt").unwrap();

    let puzzles: Vec<P13Puzzle> = P13Puzzle::parse_multi(contents.as_ref()).unwrap().1;
    eprintln!(
        "{:?}",
        puzzles.iter().map(|p| p.solve()).collect::<Vec<_>>()
    );
    dbg!(&puzzles
        .iter()
        .filter_map(|p| {
            if let Some((x, y)) = p.solve() {
                if x >= 0 && y >= 0 && x <= 100 && y <= 100 {
                    return Some(x * 3 + y);
                }
            }
            None
        })
        .sum::<isize>());
    eprintln!();

    ////////////////////////////////////////////////////////////////////////////////
    let puzzles: Vec<_> = puzzles
        .into_iter()
        .map(|mut p| {
            p.x += 10000000000000;
            p.y += 10000000000000;
            p
        })
        .collect();
    eprintln!(
        "{:?}",
        puzzles.iter().map(|p| p.solve()).collect::<Vec<_>>()
    );
    dbg!(&puzzles
        .iter()
        .filter_map(|p| {
            if let Some((x, y)) = p.solve() {
                if x >= 0 && y >= 0 {
                    return Some(x * 3 + y);
                }
            }
            None
        })
        .sum::<isize>());
}

#[derive(Debug, Clone, Copy)]
struct P14Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl P14Robot {
    pub fn parse_one(input: &str) -> nom::IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{i64, newline, space0, space1};
        use nom::combinator::{map, value};
        use nom::multi::separated_list0;
        use nom::sequence::tuple;

        let (input, _) = tag("p=")(input)?;
        let (input, x) = i64(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = i64(input)?;

        let (input, _) = space1(input)?;

        let (input, _) = tag("v=")(input)?;
        let (input, vx) = i64(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, vy) = i64(input)?;

        Ok((
            input,
            Self {
                x: x as isize,
                y: y as isize,
                vx: vx as isize,
                vy: vy as isize,
            },
        ))
    }
    pub fn parse_multi(input: &str) -> nom::IResult<&str, Vec<Self>> {
        nom::multi::separated_list0(nom::character::complete::newline, Self::parse_one)(input)
    }

    pub fn iter(&self, maxx: isize, maxy: isize) -> Self {
        let x = (self.x + self.vx).rem_euclid(maxx);
        let y = (self.y + self.vy).rem_euclid(maxy);

        Self {
            x,
            y,
            vx: self.vx,
            vy: self.vy,
        }
    }

    pub fn iter_with(&self, maxx: isize, maxy: isize, count: usize) -> Self {
        let x = (self.x + self.vx * count as isize).rem_euclid(maxx);
        let y = (self.y + self.vy * count as isize).rem_euclid(maxy);

        Self {
            x,
            y,
            vx: self.vx,
            vy: self.vy,
        }
    }
}

pub fn p14() {
    let contents = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    let contents = std::fs::read_to_string("./assets/adv2024/adv14.txt").unwrap();

    let robots: Vec<P14Robot> = P14Robot::parse_multi(contents.as_ref()).unwrap().1;
    assert!(robots.len() > 0);
    let maxx = robots.iter().map(|p| p.x).max().unwrap() + 1;
    let maxy = robots.iter().map(|p| p.y).max().unwrap() + 1;

    eprintln!("{}x{}", maxx, maxy);
    if maxx > 50 {
        assert_eq!(maxx, 101);
        assert_eq!(maxy, 103);
    }

    let historian: Vec<P14Robot> = robots
        .iter()
        .map(|r| r.iter_with(maxx, maxy, 100))
        .collect();
    // eprintln!("{:?}", historian);

    let mut quadrant: HashMap<(isize, isize), Vec<P14Robot>> = Default::default();
    for robot in historian.into_iter() {
        let xx = (robot.x - maxx / 2).signum();
        let yy = (robot.y - maxy / 2).signum();
        quadrant.entry((xx, yy)).or_default().push(robot);
    }

    let mut factor = 1;
    for (&(xx, yy), robot) in quadrant.iter() {
        if xx == 0 || yy == 0 {
            continue;
        }
        eprintln!("{:>2} x {:>2}: {}", xx, yy, robot.len());
        factor *= robot.len();
    }
    // dbg!(factor);
    // todo!();

    ////////////////////////////////////////////////////////////////////////////////
    for idx in 0.. {
        let historian: Vec<_> = robots
            .iter()
            .map(|r| {
                let rx = r.iter_with(maxx, maxy, idx);
                (rx.x, rx.y)
            })
            .collect();

        // try check symetric: not working, since "most robots"
        // let mut quadrant: HashMap<(isize, isize), usize> = Default::default();
        // for &(x, y) in historian.iter() {
        //     let xx = (x - maxx / 2).signum();
        //     let yy = (y - maxy / 2).signum();
        //     if xx == 0 || yy == 0 {
        //         continue;
        //     }
        //     *quadrant.entry((xx, yy)).or_default() += 1;
        // }
        // let factor: HashSet<_> = quadrant.values().collect();

        // the tree must contain a continus line(maybe?)
        // - we don't know if its horizontal or vertial
        // - we don't know if the tree hollow inside
        // - but just check vertical line >= 10 can lead us to the right answer.
        let mut lines: HashMap<isize, Vec<isize>> = Default::default();
        for &(x, y) in historian.iter() {
            lines.entry(x).or_default().push(y);
        }
        let mut maxline = 0;
        let mut start = -2;
        let mut count = 1;
        for (x, mut ys) in lines.into_iter() {
            ys.sort();
            for y in ys {
                if start + 1 == y || y == start {
                    if start + 1 == y {
                        count += 1;
                    }
                } else {
                    maxline = maxline.max(count);
                    count = 1;
                }

                start = y;
            }
        }

        if maxline >= 10 {
            eprintln!("\n\n\n\nID:{} factor={}", idx, maxline);
            let rxs: HashSet<_> = historian.iter().collect();
            for idy in 0..maxy {
                for idx in 0..maxx {
                    if rxs.contains(&(idx, idy)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum P15Dir {
    Left,
    Right,
    Up,
    Down,
}

fn p15_parse(input: &str) -> nom::IResult<&str, (Vec<Vec<char>>, Vec<P15Dir>)> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::one_of;
    use nom::character::complete::{newline, space0, space1, u64};
    use nom::combinator::{map, value};
    use nom::multi::separated_list0;
    use nom::sequence::tuple;

    let (input, warehouse) = separated_list0(newline, many1(one_of("#@.O")))(input)?;

    let (input, _) = many1(newline)(input)?;

    let (input, actions) = separated_list0(
        newline,
        many1(alt((
            value(P15Dir::Left, tag("<")),
            value(P15Dir::Right, tag(">")),
            value(P15Dir::Up, tag("^")),
            value(P15Dir::Down, tag("v")),
        ))),
    )(input)?;
    Ok((
        input,
        (
            warehouse,
            actions
                .into_iter()
                .map(|vs| vs.into_iter())
                .flatten()
                .collect(),
        ),
    ))
}

pub fn p15() {
    let contents = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    let contents = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;

    let contents = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    let contents = std::fs::read_to_string("./assets/adv2024/adv15.txt").unwrap();

    let (warehouse, actions) = p15_parse(contents.as_ref()).unwrap().1;
    // eprintln!("{:?}", warehouse);

    let height = warehouse.len();
    assert!(height > 0);
    let width = warehouse[0].len();
    assert!(width > 0);

    let mut walls = vec![];
    let mut boxes = vec![];
    let mut robot = None;
    for idx in 0..height {
        for idy in 0..width {
            match warehouse[idx][idy] {
                '#' => walls.push((idx, idy)),
                'O' => boxes.push((idx, idy)),
                '@' => robot = Some((idx, idy)),
                _ => {}
            }
        }
    }
    assert!(robot.is_some());
    let initial = (walls, boxes, robot.unwrap());

    ////////////////////////////////////////////////////////////////////////////////
    let (walls, mut boxes, (mut sx, mut sy)) = initial;
    for (ida, act) in actions.iter().enumerate() {
        let (offsetx, offsety) = match act {
            P15Dir::Down => (1, 0),
            P15Dir::Up => (-1, 0),
            P15Dir::Right => (0, 1),
            P15Dir::Left => (0, -1),
        };

        let mut pushed = vec![];
        let mut ox = sx;
        let mut oy = sy;
        loop {
            let (nx, ny) = (ox as isize + offsetx, oy as isize + offsety);
            if nx >= 0 && nx < height as isize && ny >= 0 && ny < width as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if walls.iter().find(|&&p| p == (nx, ny)).is_some() {
                    // stop, this is wall
                    // nothing changed.
                    break;
                } else if let Some(idx) = boxes.iter().position(|p| p == &(nx, ny)) {
                    pushed.push(idx);
                } else {
                    // stop, now do your actions.
                    sx = (sx as isize + offsetx) as usize;
                    sy = (sy as isize + offsety) as usize;

                    for idx in pushed.into_iter() {
                        let (bx, by) = &mut boxes[idx];
                        *bx = (*bx as isize + offsetx) as usize;
                        *by = (*by as isize + offsety) as usize;
                    }
                    break;
                }
                ox = nx;
                oy = ny;
            } else {
                // since we have outside boundary
                unreachable!();
            }
        }

        if ida + 1 == actions.len() {
            println!("{:?}", act);
            for idx in 0..height {
                for idy in 0..width {
                    if walls.iter().find(|&&p| p == (idx, idy)).is_some() {
                        print!("#");
                    } else if boxes.iter().find(|&&p| p == (idx, idy)).is_some() {
                        print!("O");
                    } else if idx == sx && idy == sy {
                        print!("@");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }
    }

    dbg!(boxes.iter().map(|&(xx, yy)| 100 * xx + yy).sum::<usize>());
    ////////////////////////////////////////////////////////////////////////////////
    let mut walls = vec![];
    let mut boxes = vec![];
    let mut robot = None;
    for idx in 0..height {
        for idy in 0..width {
            match warehouse[idx][idy] {
                '#' => {
                    walls.push((idx, idy * 2));
                    walls.push((idx, idy * 2 + 1));
                }
                'O' => {
                    boxes.push((idx, idy * 2, idy * 2 + 1));
                }
                '@' => robot = Some((idx, idy * 2)),
                _ => {}
            }
        }
    }
    assert!(robot.is_some());
    let width = width * 2;
    let initial = (walls, boxes, robot.unwrap());

    let (walls, mut boxes, (mut sx, mut sy)) = initial;
    for (ida, act) in actions.iter().enumerate() {
        let (offsetx, offsety) = match act {
            P15Dir::Down => (1, 0),
            P15Dir::Up => (-1, 0),
            P15Dir::Right => (0, 1),
            P15Dir::Left => (0, -1),
        };

        let (nx, ny) = (sx as isize + offsetx, sy as isize + offsety);
        if nx >= 0 && nx < height as isize && ny >= 0 && ny < width as isize {
            let nx = nx as usize;
            let ny = ny as usize;
            if walls.iter().find(|&&p| p == (nx, ny)).is_some() {
                // stop, @ can't be moved
            } else if let Some(idx) = boxes
                .iter()
                .position(|&(boxx, boxy0, boxy1)| nx == boxx && (ny == boxy0 || ny == boxy1))
            {
                let mut pushed = vec![idx];
                let mut waiting = vec![];
                let mut is_movable = true;
                while let Some(idx) = pushed.pop() {
                    let (boxx, boxy0, boxy1) = boxes[idx];
                    let nboxx = boxx as isize + offsetx;
                    let nboxy0 = boxy0 as isize + offsety;
                    let nboxy1 = boxy1 as isize + offsety;
                    assert!(nboxx >= 0 && nboxx < height as isize);
                    assert!(nboxy0 >= 0 && nboxy0 < width as isize);
                    assert!(nboxy1 >= 0 && nboxy1 < width as isize);
                    let nboxx = nboxx as usize;
                    let nboxy0 = nboxy0 as usize;
                    let nboxy1 = nboxy1 as usize;

                    let mut is_next_movable = true;
                    if walls.iter().find(|&&p| p == (nboxx, nboxy0)).is_some() {
                        is_next_movable = false;
                    } else if let Some(ndx) = boxes.iter().position(|&(boxx, boxy0, boxy1)| {
                        nboxx == boxx && (nboxy0 == boxy0 || nboxy0 == boxy1)
                    }) {
                        // overlap with self
                        if idx != ndx {
                            pushed.push(ndx);
                        }
                    }

                    if walls.iter().find(|&&p| p == (nboxx, nboxy1)).is_some() {
                        is_next_movable = false;
                    } else if let Some(ndx) = boxes.iter().position(|&(boxx, boxy0, boxy1)| {
                        nboxx == boxx && (nboxy1 == boxy0 || nboxy1 == boxy1)
                    }) {
                        if idx != ndx {
                            pushed.push(ndx);
                        }
                    }

                    if is_next_movable {
                        waiting.push((idx, nboxx, nboxy0, nboxy1))
                    } else {
                        is_movable = false;
                        break;
                    }
                }

                // stop, not only move @, but move boxes inside pushed
                if is_movable {
                    sx = nx;
                    sy = ny;
                    for (idx, boxx, boxy0, boxy1) in waiting.into_iter() {
                        boxes[idx] = (boxx, boxy0, boxy1);
                    }
                }
            } else {
                // stop, only move @, no boxes need to be moved;
                sx = nx;
                sy = ny;
            }
        }

        if ida + 1 == actions.len() {
            // if true {
            println!("{:?}", act);
            for idx in 0..height {
                for idy in 0..width {
                    if walls.iter().find(|&&p| p == (idx, idy)).is_some() {
                        print!("#");
                    } else if boxes
                        .iter()
                        .find(|&&(boxx, boxy0, boxy1)| (idx, idy) == (boxx, boxy0))
                        .is_some()
                    {
                        print!("[");
                    } else if boxes
                        .iter()
                        .find(|&&(boxx, boxy0, boxy1)| (idx, idy) == (boxx, boxy1))
                        .is_some()
                    {
                        print!("]");
                    } else if idx == sx && idy == sy {
                        print!("@");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }
    }

    dbg!(boxes
        .iter()
        .map(|&(xx, y0, y1)| 100 * xx + y0)
        .sum::<usize>());
}

type P16Dir = (isize, isize);

pub fn p16() {
    let contents = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
    let contents = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    let contents = std::fs::read_to_string("./assets/adv2024/adv16.txt").unwrap();

    let maze: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = maze.len();
    assert!(height > 0);
    let width = maze[0].len();
    assert!(width > 0);

    ////////////////////////////////////////////////////////////////////////////////
    let mut start = None;
    let mut end = None;
    for idx in 0..height {
        for idy in 0..width {
            match maze[idx][idy] {
                'S' => start = Some((idx, idy)),
                'E' => end = Some((idx, idy)),
                _ => {}
            }
        }
    }
    assert!(start.is_some());
    assert!(end.is_some());
    let initial = (start.unwrap(), end.unwrap());

    ////////////////////////////////////////////////////////////////////////////////
    // let ((sx, sy), (ex, ey)) = initial;
    // let mut pathes = vec![vec![(sx, sy)]];
    // let mut reached: Vec<Vec<(usize, usize)>> = vec![];
    // eprintln!("{:?} -> {:?}", (sx, sy), (ex, ey));
    // while let Some(path) = pathes.pop() {
    //     // eprintln!("{:?}", path);
    //     let (sx, sy) = path.last().unwrap().clone();
    //     if sx == ex && sy == ey {
    //         eprintln!("Found one: {}", path.len());
    //         reached.push(path);
    //     } else {
    //         for (offsetx, offsety) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
    //             let nx = sx as isize + offsetx;
    //             let ny = sy as isize + offsety;
    //             if nx >= 0 && nx < height as isize && ny >= 0 && ny < width as isize {
    //                 let nx = nx as usize;
    //                 let ny = ny as usize;
    //
    //                 if maze[nx][ny] != '#' && (!path.iter().any(|o| o == &(nx, ny))) {
    //                     let mut np = path.clone();
    //                     np.push((nx, ny));
    //                     pathes.push(np);
    //                 }
    //             }
    //         }
    //     }
    // }
    // // eprintln!("{}: {:?}", reached.len(), reached);
    // let mut count = usize::MAX;
    // for path in reached.iter() {
    //     let mut dir = (0, 1);
    //     let mut countdir = 0;
    //     let (mut sx, mut sy) = path[0];
    //     for &(nx, ny) in path.iter().skip(1) {
    //         let ndir = (nx as isize - sx as isize, ny as isize - sy as isize);
    //         if dir != ndir {
    //             countdir += 1;
    //             dir = ndir;
    //         }
    //
    //         sx = nx;
    //         sy = ny;
    //     }
    //     let score = 1000 * countdir + path.len() - 1;
    //     count = score.min(count);
    //     eprintln!(
    //         "\t{}+{}={} {:?}\n",
    //         countdir,
    //         path.len() - 1,
    //         score,
    //         &path[..5.min(path.len())]
    //     );
    // }
    // dbg!(&count);

    ////////////////////////////////////////////////////////////////////////////////
    // let ((sx, sy), (ex, ey)) = initial;
    // let mut maze_graph: HashMap<(usize, usize, P16Dir, usize, usize, P16Dir), usize> =
    //     Default::default();
    // let score_continue = 1;
    // let score_rotate = 1000;
    // let mut points = vec![(sx, sy, (0, 1), 0)];
    // while let Some(((mut sx, mut sy), (mut offsetx, mut offsety), score)) = points.pop() {
    //     let mut path = vec![];
    //     loop {
    //         let nx = sx as isize + offsetx;
    //         let ny = sy as isize + offsety;
    //         if nx >= 0 && nx < height as isize && ny >= 0 && ny < width as isize {
    //             let nx = nx as usize;
    //             let ny = ny as usize;
    //
    //             if maze[nx][ny] != '#' && (!path.iter().any(|o| o == &(nx, ny))) {
    //                 let nextpoint = vec![];
    //                 for (offsetx0, offsety0) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
    //
    //                 }
    //                 if nextpoint.len() == 1 {
    //                     path.push((nx, ny));
    //                 } else {
    //                     maze_graph.insert((sx, sy)) = ...;
    //                     points.push((nx, ny, ))
    //                 }
    //
    //             } else {
    //                 break;
    //             }
    //         }
    //     }
    //
    //     loop {
    //         let mut nextpoints = vec![];
    //         for (offsetx, offsety) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {}
    //         if nextpoints.len() == 1 {
    //         } else {
    //             points.push();
    //         }
    //     }
    // }

    ////////////////////////////////////////////////////////////////////////////////
    let ((sx, sy), (ex, ey)) = initial;
    let mut pathes = vec![(0, vec![(sx, sy)], (0, 1))];
    let mut reached = vec![];
    let mut minscore = usize::MAX;
    eprintln!("{:?} -> {:?}", (sx, sy), (ex, ey));
    while let Some((mut score, mut path, mut dir)) = pathes.pop() {
        // 优化： 部分点的通路是确定的，我们一直往下走，知道遇到岔路，以控制 pathes 的大小
        while let Some((sx, sy)) = path.last().cloned() {
            if sx == ex && sy == ey {
                let oldpathlen = pathes.len();
                pathes = pathes.into_iter().filter(|&(s, _, _)| s < score).collect();
                minscore = minscore.min(score);
                let newpathlen = pathes.len();
                eprintln!(
                    "Found one Limit={} ({})x({}): {} -> {}",
                    minscore,
                    score,
                    path.len(),
                    oldpathlen,
                    newpathlen,
                    // path
                );
                reached.push((score, path.clone(), dir));
                break;
            } else {
                let mut nextpoints = vec![];
                for (offsetx, offsety) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nx = sx as isize + offsetx;
                    let ny = sy as isize + offsety;
                    if nx >= 0 && nx < height as isize && ny >= 0 && ny < width as isize {
                        let nx = nx as usize;
                        let ny = ny as usize;

                        if maze[nx][ny] != '#' && (!path.iter().any(|o| o == &(nx, ny))) {
                            let mut inc = 1;
                            if dir != (offsetx, offsety) {
                                if dir == (-offsetx, offsety) || dir == (offsetx, -offsety) {
                                    inc += 1000 * 2;
                                } else {
                                    inc += 1000;
                                }
                            }
                            if score + inc < minscore {
                                nextpoints.push((score + inc, (nx, ny), (offsetx, offsety)));
                            }
                        }
                    }
                }

                if nextpoints.len() >= 2 {
                    // eprintln!(
                    //     "Found tow dir {:?} -> {:?} <- {}",
                    //     (sx, sy),
                    //     nextpoints,
                    //     pathes.len()
                    // );
                    for (nextscore, nextpoint, nextdir) in nextpoints.into_iter() {
                        let mut np = path.clone();
                        np.push(nextpoint);
                        pathes.push((nextscore, np, nextdir));
                    }
                    break;
                } else if nextpoints.len() == 1 {
                    let (nextscore, nextpoint, nextdir) = nextpoints[0];
                    // eprintln!(
                    //     "Find a Continuous path: {:?} vs {}",
                    //     nextpoint,
                    //     pathes.len()
                    // );
                    score = nextscore;
                    path.push(nextpoint);
                    dir = nextdir;
                } else {
                    // eprintln!("Removing one path");
                    break;
                }
            }
        }

        // pathes.sort_by_key(|&(s, _, _)| std::cmp::Reverse(s));
    }
    dbg!(&reached.iter().map(|&(s, _, _)| s).min().unwrap_or_default());
}

#[derive(Clone)]
struct P17Program {
    ra: usize,
    rb: usize,
    rc: usize,

    instructions: Vec<usize>,

    pointer: usize,
    output: Vec<usize>,
}

impl std::fmt::Debug for P17Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("P17Program")
            .field("a", &self.ra)
            .field("b", &self.rb)
            .field("c", &self.rc)
            .field("instructions", &self.instructions)
            .finish()
    }
}

impl P17Program {
    fn calculate(&mut self, opcode: usize, operand: usize) {}

    fn read_opcode(&mut self) -> Option<usize> {
        if self.pointer < self.instructions.len() {
            let opcode = self.instructions[self.pointer];
            self.pointer += 1;
            return Some(opcode);
        }
        None
    }

    fn iterate(&mut self) {
        if let Some(opcode) = self.read_opcode() {
            // eprintln!("\t - {}@{}", opcode, self.pointer);
            match opcode {
                0 => {
                    self.ra /= 2_usize.pow(self.read_combo() as u32);
                }
                1 => {
                    self.rb = self.rb.bitxor(self.read_lit());
                }
                2 => {
                    self.rb = self.read_combo() % 8;
                }
                3 => {
                    if self.ra != 0 {
                        self.pointer = self.read_lit();
                    } else {
                        let _ = self.read_lit();
                    }
                }
                4 => {
                    self.rb = self.rb.bitxor(self.rc);
                    let _ = self.read_lit();
                }
                5 => {
                    let val = self.read_combo() % 8;
                    self.output.push(val);
                }
                6 => {
                    self.rb = self.ra / 2_usize.pow(self.read_combo() as u32);
                }
                7 => {
                    self.rc = self.ra / 2_usize.pow(self.read_combo() as u32);
                }
                _ => unreachable!(),
            }
        }
    }

    fn read_lit(&mut self) -> usize {
        let operand = self.instructions[self.pointer];
        self.pointer += 1;
        operand
    }

    fn read_combo(&mut self) -> usize {
        let operand = self.instructions[self.pointer];
        self.pointer += 1;
        match operand {
            0..=3 => operand,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            7 => todo!(),
            _ => unreachable!(),
        }
    }

    fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{newline, space0, space1, u64};
        use nom::combinator::{map, value};
        use nom::multi::separated_list0;
        use nom::sequence::tuple;

        let (input, _) = tag("Register A: ")(input)?;
        let (input, ra) = map(u64, |n| n as usize)(input)?;
        let (input, _) = newline(input)?;

        let (input, _) = tag("Register B: ")(input)?;
        let (input, rb) = map(u64, |n| n as usize)(input)?;
        let (input, _) = newline(input)?;

        let (input, _) = tag("Register C: ")(input)?;
        let (input, rc) = map(u64, |n| n as usize)(input)?;
        let (input, _) = newline(input)?;

        let (input, _) = newline(input)?;

        let (input, _) = tag("Program: ")(input)?;
        let (input, instructions) = separated_list0(tag(","), map(u64, |n| n as usize))(input)?;

        Ok((
            input,
            Self {
                ra,
                rb,
                rc,
                instructions,
                pointer: 0,
                output: vec![],
            },
        ))
    }
}

pub fn p17() {
    for contents in [
        &std::fs::read_to_string("./assets/adv2024/adv17.txt").unwrap(),
        r#"Register A: 2024
Register B: 0
Register C: 9

Program: 2,6"#,
        r#"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4"#,
        r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#,
        r#"Register A: 2024
Register B: 29
Register C: 0

Program: 1,7"#,
        r#"Register A: 2024
Register B: 2024
Register C: 43690

Program: 4,0"#,
        r#"Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#,
    ] {
        // FIXME: too annoying
        // break;
        eprintln!("################################################################");
        eprintln!("{}", contents);
        let mut px = P17Program::parse(contents.as_ref()).unwrap().1;
        // px.ra = 38886110969332;

        while px.pointer < px.instructions.len() {
            px.iterate();
        }
        eprintln!("{:?}", px);
        eprintln!(
            "\t{:?}",
            px.output
                .iter()
                .map(|&n| n.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        eprintln!();
        // todo!();
    }

    let contents = r#"Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
    let contents = r#"Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;

    let contents = std::fs::read_to_string("./assets/adv2024/adv17.txt").unwrap();
    let puzzle = P17Program::parse(contents.as_ref()).unwrap().1;
    let mut guesses = vec![(0, 0, vec![])];

    while let Some((stack, printed_count, path)) = guesses.pop() {
        if printed_count >= puzzle.instructions.len() {
            eprintln!("Find one possible answer: {}@{:?}", stack, path);
            continue;
        }

        let num = puzzle.instructions[puzzle.instructions.len() - 1 - printed_count];

        let mut found = vec![];
        for ra in 0..8 {
            let mut px = puzzle.clone();
            px.ra = stack * 8 + ra;
            // eprintln!(">>>> {:?}", px);
            while px.pointer + 2 < px.instructions.len() {
                px.iterate();
            }

            assert_eq!(px.output.len(), 1);
            if px.output[0] == num {
                found.push(ra);
            }
        }

        for guess in found.into_iter() {
            let mut np = path.clone();
            np.push(guess);
            guesses.push((stack * 8 + guess, printed_count + 1, np))
        }
    }

    // // 这题只能使用逆向的法子，找到 instructions 里面特殊的部分，以此做迭代
    // // 由于逆向并不是一对一的关系，我们还需要尝试所有可能的组合，找到最小值
    // //
    // // 使用逆向的办法，可以看到实例中，运算部分简单就是 x / 8,
    // // 所以逆向就是 x * 8，打印的时候是 x % 8 反推，最小的x起点是0
    // let mut stack = 0;
    // for num in puzzle.instructions.iter().rev() {
    //     stack = (stack + num) * 8
    // }
    // dbg!(stack);
}
