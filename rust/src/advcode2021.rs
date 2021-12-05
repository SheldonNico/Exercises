use std::collections::{HashSet, HashMap};
use nom::IResult;

pub fn p01() {
    let meters: Vec<usize> = std::fs::read_to_string("./assets/adv2021/adv01.txt").unwrap()
        .split_whitespace()
        .map(|line| line.parse().unwrap()).collect();

    let mut counter = 0;
    for (prev, next) in meters.iter().zip(meters.iter().skip(1)) {
        if next > prev {
            counter += 1;
        }
    }

    println!("Increase counter: {}", counter);

    let mut counter = 0;
    for idx in 3..meters.len() {
        if meters[idx-3] < meters[idx] {
            counter += 1;
        }
    }
    println!("Increase sum counter: {}", counter);
}

#[derive(Debug, Clone, Copy)]
enum SubmarineDir {
    Up,
    Down,
    Forward
}

#[derive(Debug, Clone)]
struct SubmarineInstruction {
    dir: SubmarineDir,
    step: usize,
}

impl SubmarineInstruction {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{space1, digit1};
        use nom::combinator::{map_res, value, eof};
        use nom::branch::alt;

        let (input, dir) = alt((
                value(SubmarineDir::Forward, tag("forward")),
                value(SubmarineDir::Down, tag("down")),
                value(SubmarineDir::Up, tag("up")),
        ))(input)?;
        let (input, _) = space1(input)?;
        let (input, step) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
        let (input, _) = eof(input)?;

        Ok((input, Self { dir, step }))
    }
}

pub fn p02() {
    let instructions: Vec<SubmarineInstruction> = std::fs::read_to_string("./assets/adv2021/adv02.txt").unwrap()
        .lines()
        .map(|line| SubmarineInstruction::parse(line).unwrap().1).collect();

    let mut x: usize = 0;
    let mut y: usize = 0;

    for SubmarineInstruction { dir, step } in instructions.iter() {
        match dir {
            SubmarineDir::Forward => { x += step; },
            SubmarineDir::Down => { y += step; },
            SubmarineDir::Up => { y -= step; },
        }
    }

    println!("the result is ({}, {}): {}", x, y, x*y);

    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut aim: isize = 0;

    for SubmarineInstruction { dir, step } in instructions.iter() {
        match dir {
            SubmarineDir::Forward => {
                x += *step as isize;
                y += aim * (*step as isize);
            },
            SubmarineDir::Down => { aim += *step as isize; },
            SubmarineDir::Up => { aim -= *step as isize; },
        }
    }
    println!("the result is ({}, {}) -> {}: {}", x, y, aim, x*y);
}

pub fn p03() {
    let contents = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
    let contents = std::fs::read_to_string("./assets/adv2021/adv03.txt").unwrap();

    let numbers: Vec<Vec<i32>> = contents.lines()
        .map(|line| line.chars().map(|c| {
            match c {
                '0' => -1,
                '1' =>  1,
                _   => unreachable!()
            }
        }).collect())
        .collect();

    let mut res = vec![];
    for idx in 0..numbers[0].len() {
        let mut start = 0;
        for nums in numbers.iter() {
            start += nums[idx];
        }
        res.push(start);
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for r in res.iter() {
        if *r > 0 {
            gamma = 2*gamma + 1;
            epsilon = 2*epsilon;
        } else {
            gamma = 2*gamma;
            epsilon = 2*epsilon + 1;
        }
    }

    println!("{:?} = {} {} => {}", res, gamma, epsilon, gamma*epsilon);

    let oxygen_gen = p03_find_magic(&numbers, true, true);
    let co2_scrub = p03_find_magic(&numbers, false, false);
    println!(
        "{:?}({}) {:?}({}) => {}",
        oxygen_gen, p03_bin(&oxygen_gen),
        co2_scrub, p03_bin(&co2_scrub),
        p03_bin(&oxygen_gen) * p03_bin(&co2_scrub),
    );
}

fn p03_bin(bins: &Vec<i32>) -> usize {
    let mut out = 0;
    for b in bins.iter() {
        if *b > 0 {
            out = out * 2 + 1;
        } else {
            out = out * 2
        }
    }
    out
}

fn p03_find_magic(numbers: &Vec<Vec<i32>>, is_most_select: bool, is_one_left: bool) -> Vec<i32> {
    let mut selects: HashSet<usize> = (0..numbers.len()).into_iter().collect();
    let mut curr = 0;

    while selects.len() > 1 {
        let mut ones: HashSet<usize> = HashSet::new();
        let mut zeros: HashSet<usize> = HashSet::new();
        for select in selects.iter() {
            match numbers[*select][curr] {
                1  => { ones.insert(*select); }
                -1 => { zeros.insert(*select); }
                _  => unreachable!(),
            }
        }
        if ones.len() > zeros.len() {
            if is_most_select {
                selects = ones;
            } else {
                selects = zeros;
            }
        } else if ones.len() < zeros.len() {
            if is_most_select {
                selects = zeros;
            } else {
                selects = ones;
            }
        } else {
            if is_one_left {
                selects = ones;
            } else {
                selects = zeros;
            }
        }

        curr += 1;
    }

    let idx = selects.into_iter().next().unwrap();
    return numbers[idx].clone()
}

type Board = Vec<Vec<i32>>;

fn p04_parse(input: &str) -> IResult<&str, (Vec<i32>, Vec<Board>)>{
    use nom::bytes::complete::tag;
    use nom::character::complete::{space1, digit1, newline, space0};
    use nom::combinator::{map_res, eof};
    use nom::multi::separated_list1;
    use nom::sequence::preceded;

    let (input, orders) = separated_list1(
        tag(","),
        map_res(digit1, |s: &str| s.parse::<i32>())
    )(input)?;

    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;

    let board_parser = preceded(
        space0, separated_list1(
            space1,
            map_res(digit1, |s: &str| s.parse::<i32>())
        )
    );
    let (input, boards) = separated_list1(
        preceded(newline, newline),
        separated_list1(
            newline,
            board_parser,
        )
    )(input)?;
    Ok((input, (orders,boards)))
}

pub fn p04() {
    let contents = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    let _contents = std::fs::read_to_string("./assets/adv2021/adv04.txt").unwrap(); let contents = &_contents;

    let (_, (orders, boards)) = p04_parse(contents).unwrap();
    println!("{:?}", orders);
    println!("{:?}", boards);

    let n_cols = 5;
    let n_rows = 5;
    let mut records: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; n_cols]; n_rows]; boards.len()];

    let mut is_break = false;
    // let mut wins = boards.len() - 1; // The first question
    let mut wins = HashSet::new(); // The second question
    for number in orders.iter() {
        if is_break { break; }

        for (bid, (board, record)) in boards.iter().zip(records.iter_mut()).enumerate() {
            if wins.contains(&bid) { continue; }
            if is_break { break; }

            for idx in 0..n_cols {
                if is_break { break; }
                for idy in 0..n_rows {
                    if is_break { break; }
                    if board[idx][idy] == *number {
                        record[idx][idy] = true;

                        let check_row = (0..n_rows).into_iter().map(
                            |id| if record[idx][id] { 1 } else { 0 }
                        ).sum::<usize>() == n_rows;
                        let check_col = (0..n_cols).into_iter().map(
                            |id| if record[id][idy] { 1 } else { 0 }
                        ).sum::<usize>() == n_cols;

                        if check_row || check_col {
                            wins.insert(bid);
                        }

                        if wins.len() == boards.len() {
                            let mut total = 0;
                            for idx in 0..n_cols {
                                for idy in 0..n_cols {
                                    if !record[idx][idy] {
                                        total += board[idx][idy];
                                    }
                                }
                            }

                            println!("{:?} {:?}", board, record);
                            println!("{:?}({}): {} x {} = {}", wins, boards.len(), total, number, total*number);
                            is_break = true;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vent {
    start: (usize, usize),
    end: (usize, usize),
}

impl Vent {
    fn parse_number(input: &str) -> IResult<&str, usize> {
        use nom::character::complete::digit1;
        use nom::combinator::map_res;
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn parse_point(input: &str) -> IResult<&str, (usize, usize)> {
        use nom::bytes::complete::tag;
        use nom::combinator::map;
        use nom::sequence::tuple;

        map(tuple((Self::parse_number, tag(","), Self::parse_number)), |(start, _, end)| (start, end))(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::space0;

        let (input, start) = Self::parse_point(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("->")(input)?;
        let (input, _) = space0(input)?;
        let (input, end) = Self::parse_point(input)?;

        Ok((input, Self {start, end}))
    }
}

pub fn p05() {
    let contents = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    let _contents = std::fs::read_to_string("./assets/adv2021/adv05.txt").unwrap(); let contents = &_contents;

    let vents: Vec<Vent> = contents.lines()
        .map(|line| Vent::parse(line).unwrap().1)
        .collect();

    p05_part1(&vents);

    p05_part2(&vents);
}

pub fn p05_part1(vents: &Vec<Vent>) {
    let mut acc: HashMap<(usize, usize), usize> = Default::default();

    for Vent { start: (sx, sy), end: (ex, ey) } in vents.iter() {
        if sx == ex {
            let mut sy = *sy; let mut ey = *ey;
            if sy > ey {
                std::mem::swap(&mut sy, &mut ey);
            }

            for inner in sy..ey+1 {
                *acc.entry((*sx, inner)).or_default() += 1;
            }
        } else if sy == ey {
            let mut sx = *sx; let mut ex = *ex;
            if sx > ex {
                std::mem::swap(&mut sx, &mut ex);
            }
            for inner in sx..ex+1 {
                *acc.entry((inner, *sy)).or_default() += 1;
            }
        }
    }

    let mut sum = 0;
    for (_, counter) in acc.iter() {
        if *counter >= 2 {
            sum += 1;
        }
    }

    println!("{:?} => {}", acc, sum);
}

pub fn p05_part2(vents: &Vec<Vent>) {
    let mut acc: HashMap<(usize, usize), usize> = Default::default();

    for Vent { start: (sx, sy), end: (ex, ey) } in vents.iter() {
        if sx == ex {
            let mut sy = *sy; let mut ey = *ey;
            if sy > ey {
                std::mem::swap(&mut sy, &mut ey);
            }

            for inner in sy..ey+1 {
                *acc.entry((*sx, inner)).or_default() += 1;
            }
        } else if sy == ey {
            let mut sx = *sx; let mut ex = *ex;
            if sx > ex {
                std::mem::swap(&mut sx, &mut ex);
            }
            for inner in sx..ex+1 {
                *acc.entry((inner, *sy)).or_default() += 1;
            }
        } else {
            let mut x0: isize = *sx as _;
            let mut y0: isize = *sy as _;
            let x1: isize = *ex as _;
            let y1: isize = *ey as _;

            // println!("{:?} -> {:?}", (x0, y0), (x1, y1));
            assert!((x0 - x1).abs() == (y0 - y1).abs());

            while x0 != x1 {
                *acc.entry((x0 as _, y0 as _)).or_default() += 1;
                if x0 < x1 { x0 += 1; } else { x0 -= 1; }
                if y0 < y1 { y0 += 1; } else { y0 -= 1; }
            }

            *acc.entry((x1 as _, y1 as _)).or_default() += 1;
        }
    }

    let mut sum = 0;
    for (_, counter) in acc.iter() {
        if *counter >= 2 {
            sum += 1;
        }
    }

    println!("{:?} => {}", acc, sum);
}
