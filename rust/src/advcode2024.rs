#![allow(unused_imports)]

use std::{
    collections::{HashMap, HashSet},
    hash::DefaultHasher,
    iter::FromIterator,
    usize,
};

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
