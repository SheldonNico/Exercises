#![allow(unused_imports)]

use std::{
    collections::{HashMap, HashSet},
    isize,
};

use nom::character::streaming::newline;

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
