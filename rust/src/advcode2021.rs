use std::collections::{HashSet, HashMap, BTreeSet};
use nom::IResult;

type P08Signal = HashSet<char>;

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

pub fn p06() {
    let contents = r"3,4,3,1,2";
    let _contents = std::fs::read_to_string("./assets/adv2021/adv06.txt").unwrap(); let contents = &_contents;

    const MAX_LIFE: usize = 9;
    let mut state_ori: Vec<usize> = vec![0; MAX_LIFE];

    for life in contents.trim().split(",") {
        let life = life.parse::<usize>().unwrap();
        state_ori[life] += 1;
    }
    println!("inited state: {:?}", state_ori);

    let mut state = state_ori.clone();

    for _ in 0..80 {
        p06_next(&mut state);
    }
    println!("Sum: {}", state.iter().sum::<usize>());

    let mut state = state_ori.clone();
    for _ in 0..256 {
        p06_next(&mut state);
    }
    println!("Sum: {}", state.iter().sum::<usize>());
}

fn p06_next(state: &mut Vec<usize>) {
    let died = state[0];
    state[0] = 0;
    state.rotate_left(1);
    state[6] += died;
    state[8] += died;
}

pub fn p07() {
    let contents = r"16,1,2,0,4,2,7,1,2,14";
    let _contents = std::fs::read_to_string("./assets/adv2021/adv07.txt").unwrap(); let contents = &_contents;

    let mut positions = vec![];
    for pos in contents.trim().split(",") {
        let pos = pos.parse::<usize>().unwrap();
        if positions.len() < pos+1 {
            positions.resize(pos+1, 0);
        }
        positions[pos] += 1;
    }

    println!("Crub positions: {:?}", positions);

    let aligned: Vec<usize> = (0..positions.len()).into_iter().map(|to| p07_align(&positions, to)).collect();
    println!("Aligned {:?}: {:?}", aligned, aligned.iter().min());

    let aligned: Vec<usize> = (0..positions.len()).into_iter().map(|to| p07_align_inc(&positions, to)).collect();
    println!("Aligned {:?}: {:?}", aligned, aligned.iter().min());
}

fn p07_align(positions: &Vec<usize>, to: usize) -> usize {
    let mut sum = 0;
    for (idx, count) in positions.iter().enumerate() {
        let shift = if idx > to { idx - to } else { to - idx };
        sum += count * shift;
    }
    sum
}

fn p07_align_inc(positions: &Vec<usize>, to: usize) -> usize {
    let mut sum = 0;
    for (idx, count) in positions.iter().enumerate() {
        let shift = if idx > to { idx - to } else { to - idx };
        sum += count * ((shift+1)*shift / 2);
    }
    sum
}

pub fn p08() {
    let contents = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    // let _contents = std::fs::read_to_string("./assets/adv2021/adv08.txt").unwrap(); let contents = &_contents;

    let mut signals: Vec<Vec<P08Signal>> = vec![];
    let mut predicts: Vec<Vec<P08Signal>> = vec![];
    let mut sum = 0;
    for line in contents.trim().lines() {
        let line: Vec<_> = line.splitn(2, "|").collect();
        assert_eq!(line.len(), 2);

        let mut patterns = vec![];
        for word in line[0].trim().split(" ") {
            patterns.push(p08_encode(word.trim()));
        }
        signals.push(patterns.clone());

        patterns.clear();
        for word in line[1].trim().split(" ") {
            patterns.push(p08_encode(word.trim()));
        }
        sum += patterns.iter().filter(|s| match s.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        }).count();
        predicts.push(patterns);

    }
    println!("Sum of easy digits: {}", sum);

    let targets: Vec<P08Signal> = vec![
        "abcefg",
        "cf",
        "acdeg",
        "acdfg",
        "bcdf",
        "abdfg",
        "abdefg",
        "acf",
        "abcdefg",
        "abcdfg"
    ].into_iter().map(p08_encode).collect();

    let mut sum = 0;
    for (patterns, predicts) in signals.iter().zip(predicts.iter()) {
        let map = p08_solve(patterns, &targets);

        let mut digits = 0;
        for predict in predicts.iter() {
            let mut ori: Vec<&char> = predict.iter().collect();
            ori.sort();
            let ori: String = ori.into_iter().collect();

            digits = digits * 10 + map[&ori];
        }
        sum += digits;
    }
    println!("Sum of large digits: {}", sum);
}

fn p08_solve(patterns: &Vec<P08Signal>, targets: &Vec<P08Signal>) -> HashMap<String, usize> {
    let mut relations: Vec<((P08Signal, String), Vec<(P08Signal, usize)>)> = vec![];

    for pattern in patterns.iter() {
        let len_pat = pattern.len();
        let mut possible = vec![];
        for (idx, target) in targets.iter().enumerate() {
            if len_pat == target.len() {
                possible.push((target.clone(), idx));
            }
        }

        let mut ori: Vec<&char> = pattern.iter().collect();
        ori.sort();
        let ori = ori.into_iter().collect();

        relations.push(((pattern.clone(), ori), possible));
    }

    relations.sort_by_key(|(k, v)| -(v.len() as isize));

    let mut map: HashMap<String, usize> = Default::default();
    while let Some(((from, ori), mut matched)) = relations.pop() {
        assert_eq!(matched.len(), 1);
        let (to, target) = matched.remove(0);
        map.insert(ori, target);

        for ((from_next, ori_next), matched_next) in relations.iter_mut() {
            let valid_to_replace = std::mem::replace(matched_next, Default::default());
            let from_next_check: HashSet<&char> = from_next.difference(&from).collect();
            let valid_to_replace = valid_to_replace.into_iter().filter(|(to_next, target_next)| {
                let to_next_check: HashSet<&char> = to_next.difference(&to).collect();
                from_next_check.len() == to_next_check.len()
            }).collect();
            *matched_next = valid_to_replace;
        }
        relations.sort_by_key(|(k, v)| -(v.len() as isize));
    }

    map
}

fn p08_encode(s: &str) -> P08Signal {
    let mut out = HashSet::new();
    for c in s.chars() {
        out.insert(c);
    }

    out
}

pub fn p09() {
    let contents = r"2199943210
3987894921
9856789892
8767896789
9899965678";
    let _contents = std::fs::read_to_string("./assets/adv2021/adv09.txt").unwrap(); let contents = &_contents;

    let heatmap: Vec<Vec<usize>> = contents.trim().lines()
        .map(|line| line.chars().map(|c| { (c as u8 - '0' as u8) as usize }).collect())
        .collect();

    let mut sum = 0;
    let mut basin_all: Vec<usize> = vec![];

    let n_rows = heatmap.len();
    let n_cols = heatmap[0].len();

    for idr in 0..n_rows {
        for idc in 0..n_cols {
            let mut is_ok = true;
            for (idr_next, idc_next) in p09_adjacents(idr, idc, n_rows, n_cols) {
                if heatmap[idr][idc] >= heatmap[idr_next][idc_next] {
                    is_ok = false;
                    break;
                }
            }
            if !is_ok { continue; }

            // part 1
            sum += heatmap[idr][idc] + 1;

            // part 2
            let mut basin_map: HashSet<(usize, usize)> = vec![
                (idr, idc)
            ].into_iter().collect();

            let mut adjacents = p09_adjacents(idr, idc, n_rows, n_cols);

            while adjacents.len() > 0 {
                p09_find_basin(&heatmap, adjacents.clone(), &mut basin_map, n_rows, n_cols);
                let adjacents_next: Vec<(usize, usize)> = std::mem::replace(&mut adjacents, Default::default());
                for (idr_next, idc_next) in adjacents_next.into_iter() {
                    if basin_map.contains(&(idr_next, idc_next)) {
                        let mut next = p09_adjacents(idr_next, idc_next, n_rows, n_cols).into_iter().filter(
                            |id| !basin_map.contains(id)).collect();
                        adjacents.append(&mut next);
                    }
                }
            }
            println!("{} {} => {:?}", idr, idc, basin_map.iter().map(|(kr, kc)| heatmap[*kr][*kc]).collect::<Vec<_>>());
            basin_all.push(basin_map.iter().count());
        }
    }
    println!("Sum of risk levels: {}", sum);

    basin_all.sort_by_key(|v| -(*v as isize));
    println!("{:?} => {:?}", basin_all, &basin_all[..3].iter().product::<usize>());
}

fn p09_adjacents(idr: usize, idc: usize, n_rows: usize, n_cols: usize) -> Vec<(usize, usize)> {
    let mut out = vec![];
    if idc > 0 { out.push((idr, idc-1)) }
    if idc+1 < n_cols { out.push((idr, idc+1)); }
    if idr > 0 { out.push((idr-1, idc)); }
    if idr+1 < n_rows { out.push((idr+1, idc)); }
    out
}

fn p09_find_basin(
    heatmap: &Vec<Vec<usize>>, levels: Vec<(usize, usize)>, basin_map: &mut HashSet<(usize, usize)>,
    n_rows: usize, n_cols: usize
) {
    for (idr, idc) in levels.into_iter() {
        if heatmap[idr][idc] == 9 { continue; }

        let mut is_ok = true;
        for (idr_next, idc_next) in p09_adjacents(idr, idc, n_rows, n_cols) {
            if heatmap[idr][idc] > heatmap[idr_next][idc_next] {
                if !basin_map.contains(&(idr_next, idc_next)) {
                    is_ok = false;
                    break;
                }
            }
        }

        if is_ok {
            basin_map.insert((idr, idc));
        }
    }
}

enum NavigationError {
    Illegal(char),
    Incomplete,
}

pub fn p10() {
    let contents = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    let _contents = std::fs::read_to_string("./assets/adv2021/adv10.txt").unwrap(); let contents = &_contents;

    let navigations: Vec<Vec<char>> = contents.trim().lines().map(|s| s.chars().collect()).collect();

    let mut sum = 0;
    let mut incompletes: Vec<usize> = vec![];
    for navigation in navigations.iter() {
        let mut left = None;
        let mut stacked: Vec<char> = Vec::new();
        let mut broken = None;
        for dir in navigation.iter() {
            match dir {
                '(' | '{' | '[' | '<' => {
                    if let Some(left_inner) = left {
                        stacked.push(left_inner); left = Some(*dir);
                    } else {
                        left = Some(*dir);
                    }
                },
                ')' | '}' | ']' | '>' => {
                    if let Some(left_inner) = left {
                        if dir != &p10_right(left_inner) {
                            broken = Some(*dir);
                            break;
                        } else {
                            if stacked.len() == 0 {
                                left = None;
                            } else {
                                left = stacked.pop();
                            }
                        }
                    } else {
                        broken = Some(*dir);
                        break;
                    }
                }
                _ => unreachable!()
            }
        }

        if let Some(b) = broken {
            // println!("Broken: {:?} {:?}", navigation, b);
            sum += p10_score(b);
        } else {
            if let Some(l) = left {
                let mut incomplete: Vec<char> = vec![l];
                while let Some(s) = stacked.pop() {
                    incomplete.push(s);
                }
                // println!("{:?}", incomplete);
                incompletes.push(p10_score_incomplete(&incomplete));
            }
        }
    }

    println!("Sum of errors: {}", sum);
    incompletes.sort();
    assert!(incompletes.len() > 0);
    println!("Middle of incompletes: {:?} => {}", incompletes, incompletes[incompletes.len() / 2]);
}

fn p10_score_right(right: char) -> usize {
    match right {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!()
    }
}

fn p10_score_incomplete(left: &Vec<char>) -> usize {
    let mut sum = 0;
    for c in left.iter() {
        sum = sum * 5 + p10_score_right(p10_right(*c));
    }
    sum
}

fn p10_score(right: char) -> usize {
    match right {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!()
    }
}

fn p10_right(left: char) -> char {
    match left {
        '(' => ')',
        '<' => '>',
        '{' => '}',
        '[' => ']',
        _ => unreachable!()
    }
}

pub fn p11() {
    let contents = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    let _contents = std::fs::read_to_string("./assets/adv2021/adv11.txt").unwrap(); let contents = &_contents;

    let mut energy: Vec<Vec<u8>> = contents.trim().lines().map( |line| {
        line.trim().chars().map(|c| c as u8 - '0' as u8).collect()
    }).collect();

    let nrow = energy.len();
    assert!(nrow > 0);
    let ncol = energy[0].len();
    assert!(ncol > 0);
    println!("{}", p11_display(&energy));

    let mut count = 0;
    for idx in 0..3000 {
        count += p11_step(&mut energy, nrow, ncol);
        if idx + 1 == 100 {
            println!("Sum of flashes after 100: {}", count);
        }
        if energy.iter().all(|line| line.iter().all(|c| *c == 0)) {
            println!("Sync at {}:\n{}", idx+1, p11_display(&energy));
            break;
        }
    }
}

fn p11_display(energy: &Vec<Vec<u8>>) -> String {
    let mut out: String = "".into();
    for line in energy.iter() {
        let mut out_: String = "".into();
        for c in line.iter() {
            out_.push((*c + '0' as u8) as char);
        }
        out.push_str(&out_);
        out.push('\n');
    }
    out
}


fn p11_step_iter(energy: &mut Vec<Vec<u8>>, row: usize, col: usize, nrow: usize, ncol: usize) {
    if energy[row][col] <= 9 {
        energy[row][col] += 1;
        if energy[row][col] > 9 {
            let row  = row as isize; let col = col as isize;
            for (x, y) in [
                (0, 1), (0, -1), (1, 0), (-1, 0),
                (1, 1), (1, -1), (-1, 1), (-1, -1),
            ].iter() {
                if row + x >= 0 && col + y >= 0 && row+x < nrow as isize && col+y < ncol as isize {
                    p11_step_iter(energy, (row+x) as usize, (col+y) as usize, nrow, ncol);
                }
            }
        }
    }
}

fn p11_step(energy: &mut Vec<Vec<u8>>, nrow: usize, ncol: usize) -> usize {
    for idr in 0..nrow {
        for idc in 0..ncol {
            p11_step_iter(energy, idr, idc, nrow, ncol);
        }
    }

    let mut count = 0;
    for idr in 0..nrow {
        for idc in 0..ncol {
            if energy[idr][idc] > 9 {
                count += 1;
                energy[idr][idc] = 0;
            }
        }
    }
    count
}

pub fn p12() {
    let contents = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";
//     let contents = r"dc-end
// HN-start
// start-kj
// dc-start
// dc-HN
// LN-dc
// HN-end
// kj-sa
// kj-HN
// kj-dc";
//     let contents = r"fs-end
// he-DX
// fs-he
// start-DX
// pj-DX
// end-zg
// zg-sl
// zg-pj
// pj-he
// RW-he
// fs-DX
// pj-RW
// zg-RW
// start-pj
// he-WI
// zg-he
// pj-fs
// start-RW";
    let _contents = std::fs::read_to_string("./assets/adv2021/adv12.txt").unwrap(); let contents = &_contents;

    let mut map: HashMap<usize, HashSet<usize>> = Default::default();
    let mut encode: Vec<String> = vec![];
    for line in contents.trim().lines() {
        let mut points = line.split("-");
        let ax = p12_encode(&mut encode, points.next().unwrap());
        let bx = p12_encode(&mut encode, points.next().unwrap());
        assert_eq!(points.next(), None);
        map.entry(ax).or_default().insert(bx);
        map.entry(bx).or_default().insert(ax);
    }
    let start: usize = p12_decode(&encode, "start").unwrap();
    let end: usize = p12_decode(&encode, "end").unwrap();
    println!("{} -> {}: {:?} {:?}", start, end, encode, map);

    let size: Vec<_> = encode.iter().map(|s| s == &s.to_uppercase()).collect();
    let sols = p12_solve(&map, &size, start, end);

    println!("Solutions:");
    let mut sol_str: Vec<String> = vec![];
    for sol in sols.iter() {
        let mut out = "".to_owned();
        for idx in sol.iter() {
            out.push_str(&encode[*idx]);
            out.push_str(",");
        }
        sol_str.push(out);
    }
    sol_str.sort();
    println!("{}", sol_str.join("\n"));
    println!("Total: {}", sols.len());

    println!("====================================");
    let sols = p12_solve_more(&map, &size, start, end);

    println!("Solutions:");
    let mut sol_str: Vec<String> = vec![];
    for sol in sols.iter() {
        let mut out = "".to_owned();
        for idx in sol.iter() {
            out.push_str(&encode[*idx]);
            out.push_str(",");
        }
        sol_str.push(out);
    }
    sol_str.sort();
    println!("{}", sol_str.join("\n"));
    println!("Total: {}", sols.into_iter().collect::<HashSet<_>>().len());
}

fn p12_solve_more(map: &HashMap<usize, HashSet<usize>>, size: &Vec<bool>, start: usize, end: usize) -> Vec<Vec<usize>> {
    let mut pathes: Vec<(Vec<usize>, Vec<usize>)> = vec![(vec![start], vec![2; map.len()])];
    pathes[0].1[start] = 0;
    pathes[0].1[end] = 1;
    let mut finished = vec![];

    while pathes.len() > 0 {
        for (edge, walked) in std::mem::replace(&mut pathes, Default::default()).into_iter() {
            let last: usize = *edge.last().unwrap();
            let next: Vec<_> = map[&last].clone().into_iter().filter(|n| walked[*n] > 0).collect();

            for point in next.clone().into_iter() {
                if size[last] {
                    assert!(!size[point], "infinite solutions: {}-{}.", last, point);
                }
                let mut edge = edge.clone();
                let mut walked = walked.clone();

                edge.push(point);
                if point == end {
                    finished.push(edge);
                } else {
                    if !size[point] {
                        // println!("{:?} {} {:?}", walked, point, size);
                        walked[point] -= 1;

                        if walked[point] == 1 {
                            // just ignore
                            let mut reset = walked.clone();
                            reset[point] = 0;
                            pathes.push((edge.clone(), reset));

                            // change all other node
                            for (counter, is_bigger) in walked.iter_mut().zip(size.iter()) {
                                if !is_bigger && *counter == 2 {
                                    *counter = 1;
                                }
                            }
                        }
                    }

                    pathes.push((edge, walked));
                }
            }
        }
    }

    finished
}

fn p12_solve(map: &HashMap<usize, HashSet<usize>>, size: &Vec<bool>, start: usize, end: usize) -> Vec<Vec<usize>> {
    let mut pathes: Vec<(Vec<usize>, Vec<bool>)> = vec![(vec![start], vec![false; map.len()])];
    pathes[0].1[start] = true;
    let mut finished = vec![];

    while pathes.len() > 0 {
        for (edge, walked) in std::mem::replace(&mut pathes, Default::default()).into_iter() {
            let last: usize = *edge.last().unwrap();
            let next: Vec<_> = map[&last].clone().into_iter().filter(|n| size[*n] || !walked[*n]).collect();

            for point in next.into_iter() {
                let mut edge = edge.clone();
                let mut walked = walked.clone();
                if size[last] {
                    assert!(!size[point], "infinite solutions: {}-{}.", last, point);
                }

                edge.push(point);
                walked[point] = true;

                if point == end {
                    finished.push(edge);
                } else {
                    pathes.push((edge, walked));
                }
            }
        }
    }

    finished
}


fn p12_decode(encode: &Vec<String>, node: &str) -> Option<usize> {
    for (idx, to) in encode.iter().enumerate() {
        if &node == to {
            return Some(idx);
        }
    }
    None
}

fn p12_encode(encode: &mut Vec<String>, node: &str) -> usize {
    if let Some(idx) = p12_decode(encode, node) {
        return idx;
    }

    encode.push(node.to_owned());
    encode.len() - 1
}

pub fn p13() {
    let contents = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let _contents = std::fs::read_to_string("./assets/adv2021/adv13.txt").unwrap(); let contents = &_contents;

    let (_, (points, dirs)) = p13_parse(contents).unwrap();

    let mut ncol = points.iter().map(|(x, y)| x).max().unwrap() + 1;
    let mut nrow = points.iter().map(|(x, y)| y).max().unwrap() + 1;

    for (dir, position) in dirs.iter() {
        match dir {
            P13Dir::Y => { nrow = nrow.max(2*position+1); }
            P13Dir::X => { ncol = ncol.max(2*position+1); }
        }
    }

    println!("Grid size: {}x{}", ncol, nrow);
    let mut grid = vec![vec![0; ncol]; nrow];
    for (x, y) in points.iter() {
        grid[*y][*x] += 1;
    }

    for (idx, (dir, position)) in dirs.into_iter().enumerate() {
        let mut count = 0;
        for idr in 0..nrow {
            for idc in 0..ncol {
                match dir {
                    P13Dir::Y => {
                        if idr < position {
                            grid[idr][idc] += grid[position-idr+position][idc];
                        } else {
                            grid[idr][idc] = 0;
                        }
                    },
                    P13Dir::X => {
                        if idc < position {
                            grid[idr][idc] += grid[idr][position-idc+position]
                        } else {
                            grid[idr][idc] = 0;
                        }
                    }
                }

                if grid[idr][idc] > 0 { count += 1; }
            }
        }
        match dir {
            P13Dir::Y => {
                nrow /= 2;
            },
            P13Dir::X => {
                ncol /= 2;
            }
        }
        if nrow < 80 && ncol < 80 {
            println!("#{} {:?}:\n{}", idx, count, p13_display(&grid, nrow, ncol));
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum P13Dir {
    X,
    Y,
}

fn p13_display(grid: &Vec<Vec<usize>>, nrow: usize, ncol: usize) -> String {
    let mut out = "".to_owned();
    for row in grid.iter().take(nrow) {
        for c in row.iter().take(ncol) {
            if *c > 0 {
                out.push('#');
            } else {
                out.push('.');
            }
        }
        out.push('\n');
    }
    out
}

fn p13_parse(input: &str) -> IResult<&str, (Vec<(usize, usize)>, Vec<(P13Dir, usize)>)> {
    use nom::multi::{many1, separated_list1};
    use nom::character::complete::{newline, digit1};
    use nom::sequence::separated_pair;
    use nom::bytes::complete::tag;
    use nom::combinator::{map_res, value};
    use nom::branch::alt;

    let number = || map_res(digit1, |s: &str| s.parse::<usize>());
    let (input, points) = separated_list1(newline, separated_pair(number(), tag(","), number()))(input)?;
    let (input, _) = many1(newline)(input)?;
    let (input, dirs) = separated_list1(newline, |input| {
        let (input, _) = tag("fold along ")(input)?;
        let dir = alt((
                value(P13Dir::X, tag("x")),
                value(P13Dir::Y, tag("y")),
        ));
        separated_pair(dir, tag("="), number())(input)
    })(input)?;

    Ok((input, (points, dirs)))
}

pub fn p14() {
    let contents: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let _contents = std::fs::read_to_string("./assets/adv2021/adv14.txt").unwrap(); let contents: &str = &_contents;

    let (_, (polymer, insertions)) = p14_parse(contents).unwrap();
    println!("{:?} {:?}", polymer, insertions);

    let mut pairs: HashMap<(char, char), usize> = Default::default();
    let start = polymer[0];
    let end = polymer[polymer.len()-1];
    for idx in 0..polymer.len()-1 {
        let start = polymer[idx];
        let end = polymer[idx+1];
        *pairs.entry((start, end)).or_default() += 1;
    }

    for _ in 0..40 {
        let mut pending = vec![];
        for (start, end, to) in insertions.iter() {
            if let Some(count) = pairs.remove(&(*start, *end)) {
                pending.push((*start, *to, count));
                pending.push((*to, *end, count));
            }
        }
        for (start, end, count) in pending.into_iter() {
            *pairs.entry((start, end)).or_default() += count;
        }

        let counts = p14_count(&pairs, start, end);
        let mut counts_s: Vec<_> = counts.values().map(|v| *v).collect();
        counts_s.sort();
        println!("{:?} => {}", counts, counts_s[counts_s.len()-1] - counts_s[0]);
        // println!(">>>>> {:?}", pairs);
    }
}

pub fn p14_count(pairs: &HashMap<(char, char), usize>, start: char, end: char) -> HashMap<char, usize> {
    let mut out: HashMap<char, usize> = vec![(start, 1), (end, 1)].into_iter().collect();
    for ((start, end), count) in pairs.iter() {
        *out.entry(*start).or_default() += count;
        *out.entry(*end).or_default() += count;
    }
    for (_, count) in out.iter_mut() {
        assert!(*count % 2 == 0);
        *count = (*count) / 2;
    }
    out
}

pub fn p14_parse(input: &str) -> IResult<&str, (Vec<char>, Vec<(char, char, char)>)> {
    use nom::multi::many1;
    use nom::character::complete::{satisfy, newline};
    use nom::bytes::complete::tag;
    use nom::multi::separated_list1;

    let anychar = || satisfy(|c| c.is_alphabetic());

    let (input, polymer) = many1(anychar())(input)?;
    let (input, _) = many1(newline)(input)?;
    let (input, insts) = separated_list1(newline, move |input| {
        let (input, start) = anychar()(input)?;
        let (input, end) = anychar()(input)?;
        let (input, _) = tag(" -> ")(input)?;
        let (input, to) = anychar()(input)?;
        Ok((input, (start, end, to)))
    })(input)?;

    Ok((input, (polymer, insts)))
}

pub fn p15() {
    let contents: &str = r"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let _contents = std::fs::read_to_string("./assets/adv2021/adv15.txt").unwrap(); let contents: &str = &_contents;

    let levels: Vec<Vec<isize>> = contents.trim().lines().map(
        |line| line.chars().map(|c| c.to_digit(10).unwrap() as isize).collect()
    ).collect();

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let nrow = levels.len();
    let ncol = levels[0].len();
    let mut mem: Vec<Vec<isize>> = vec![vec![isize::MAX; ncol]; nrow];
    let mut curr: BTreeSet<(isize, usize, usize)> = Default::default();
    curr.insert((0, 0, 0));
    mem[0][0] = 0;

    while let Some((dis_last, px, py)) = curr.pop_first() {
        for (nx, ny) in p15_neighbor(px, py, nrow, ncol).into_iter() {
            let dis = dis_last + levels[nx][ny];
            if mem[nx][ny] > dis {
                mem[nx][ny] = dis;
                curr.insert((dis, nx, ny));
            }
        }
    }
    println!("{}x{}: {:?}", nrow, ncol, mem[nrow-1][ncol-1]);

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let mut nlevels: Vec<Vec<isize>> = vec![vec![0; ncol*5]; nrow*5];
    for idr in 0..(nrow*5) {
        for idc in 0..(ncol*5) {
            let xr = idr / nrow;
            let xc = idc / ncol;
            let val = levels[idr % nrow][idc % ncol] + ((xr+xc) as isize);
            nlevels[idr][idc] = (val-1) % 9 + 1;
        }
    }
    let levels = nlevels;

    let nrow = levels.len();
    let ncol = levels[0].len();
    let mut mem: Vec<Vec<isize>> = vec![vec![isize::MAX; ncol]; nrow];
    let mut curr: BTreeSet<(isize, usize, usize)> = Default::default();
    curr.insert((0, 0, 0));
    mem[0][0] = 0;

    while let Some((dis_last, px, py)) = curr.pop_first() {
        for (nx, ny) in p15_neighbor(px, py, nrow, ncol).into_iter() {
            let dis = dis_last + levels[nx][ny];
            if mem[nx][ny] > dis {
                mem[nx][ny] = dis;
                curr.insert((dis, nx, ny));
            }
        }
    }

    println!("{}x{}: {:?}", nrow, ncol, mem[nrow-1][ncol-1]);

    /*
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Shity solutions:
    let mut mem: HashMap<(usize, usize), isize> = Default::default();
    mem.insert((0, 0), 0);
    let mut points: Vec<((usize, usize), isize)> = vec![((0, 0), 0)];

    while points.len() > 0 {
        let mut neighbor: HashMap<(usize, usize), isize> = Default::default();
        for ((px, py), limit) in std::mem::replace(&mut points, Default::default()).into_iter() {
            for (nx, ny) in p15_neighbor(px, py, nrow, ncol) {
                if mem.contains_key(&(nx, ny)) { continue; }
                let distance = levels[nx][ny]+mem.get(&(px, py)).unwrap();
                let last = neighbor.entry((nx, ny)).or_insert(isize::MAX);
                *last = (*last).min(distance);
            }
        }
        // println!(">>>> {:?}", neighbor);

        for ((px, py), limit) in neighbor.into_iter() {
            let mut curr: HashMap<(usize, usize), isize> = vec![((px, py), limit)].into_iter().collect();
            let mut stacked = curr.clone();
            let mut minimum = limit;

            while curr.len() > 0 {
                // println!("----{:?}", curr);
                for ((cx, cy), left) in std::mem::replace(&mut curr, Default::default()).into_iter() {
                    let left = left - levels[cx][cy];
                    for (nx, ny) in p15_neighbor(cx, cy, nrow, ncol) {
                        if let Some(last_res) = stacked.get_mut(&(nx, ny)) {
                            if *last_res >= left { continue; }
                            *last_res = left;
                        }

                        if let Some(last) = mem.get(&(nx, ny)) {
                            let distance = limit - (left - last);
                            if distance < minimum { minimum = distance; }
                            continue;
                        }

                        if let Some(last_res) = curr.get(&(nx, ny)) {
                            if *last_res >= left {
                                continue;
                            }
                        }

                        if left > levels[nx][ny] {
                            curr.insert((nx, ny), left);
                        }
                        stacked.insert((nx, ny), left);
                    }
                }
            }

            mem.insert((px, py), minimum);
            points.push(((px, py), minimum));
        }
        println!("{:?}", points);
    }


    println!("{:?}", levels);
    println!("{}x{}: {:?}", nrow, ncol, mem.get(&(nrow-1, ncol-1)));
    */
}

fn p15_neighbor(x: usize, y: usize, nrow: usize, ncol: usize) -> Vec<(usize, usize)> {
    let mut out = vec![];
    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && nx < nrow as isize && ny >= 0 && ny < ncol as isize {
            out.push((nx as usize, ny as usize));
        }
    }
    out
}

pub fn p16() {
    let _contents = std::fs::read_to_string("./assets/adv2021/adv16.txt").unwrap(); let contents: &str = &_contents;

    for line in [
        "D2FE28",
        "38006F45291200",
        "EE00D40C823060",
        "8A004A801A8002F478",
        "620080001611562C8802118E34",
        "C0015000016115A2E0802F182340",
        "A0016C880162017C3686B18A3D4780",

        "C200B40A82",
        "04005AC33890",
        "880086C3E88112",
        "CE00C43D881120",
        "D8005AC2A8F0",
        "F600BC2D8F",
        "9C005AC2F8F0",
        "9C0141080250320F1802104A08",
        contents.trim(),
    ] {
        let r = P16Packet::parse_str(line).unwrap();
        println!("{} x {}: {:?}", r.calculate(), r.version_sum(), r);
    }

}

#[derive(Debug, Clone)]
pub struct P16Header {
    pub version: u8,
    pub id: u8,
}

#[derive(Debug, Clone)]
pub enum P16Packet {
    Lit(P16Header, usize),
    Op(P16Header, Vec<P16Packet>),
}

impl P16Packet {
    fn version_sum(&self) -> usize {
        let mut sum = 0;
        match self {
            Self::Lit(h, _) => { sum += h.version as usize; }
            Self::Op(h, vs) => {
                sum += h.version as usize;
                for v in vs.iter() {
                    sum += v.version_sum();
                }
            }
        }

        sum
    }

    fn calculate(&self) -> usize {
        match self {
            Self::Lit(_, v) => *v,
            Self::Op(h, vs) => {
                match h.id {
                    0 => vs.iter().map(|v| v.calculate()).sum::<usize>(),
                    1 => vs.iter().map(|v| v.calculate()).product::<usize>(),
                    2 => vs.iter().map(|v| v.calculate()).min().unwrap(),
                    3 => vs.iter().map(|v| v.calculate()).max().unwrap(),
                    5 => {
                        assert_eq!(vs.len(), 2);
                        if vs[0].calculate() > vs[1].calculate() { 1 } else { 0 }
                    },
                    6 => {
                        assert_eq!(vs.len(), 2);
                        if vs[0].calculate() < vs[1].calculate() { 1 } else { 0 }
                    },
                    7 => {
                        assert_eq!(vs.len(), 2);
                        if vs[0].calculate() == vs[1].calculate() { 1 } else { 0 }
                    },
                    _ => unreachable!(),
                }
            }
        }
    }

    fn parse_str(input: &str) -> Result<Self, String> {
        let mut chars: Vec<char> = vec![];

        for c in input.chars() {
            for i in format!("{:04b}", c.to_digit(16).unwrap()).chars() {
                chars.push(i);
            }
        }

        let (_, s) = Self::parse(&chars)?;
        Ok(s)
    }

    fn parse(input: &[char]) -> Result<(&[char], Self), String> {
        // println!(">>> {}", input.iter().collect::<String>());
        let version = Self::parse_usize(&input[..3])? as u8;
        let id = Self::parse_usize(&input[3..6])? as u8;
        let header = P16Header { version, id };

        match id {
            4 => Self::parse_lit(&input[6..], header),
            _ => Self::parse_op(&input[6..], header),
        }
    }

    fn parse_op(input: &[char], header: P16Header) -> Result<(&[char], Self), String> {
        if input.len() == 0 {
            return Err("parse_op failed: length is zero".into());
        }
        match input[0] {
            '0' => Self::parse_op1(&input[1..], header),
            '1' => Self::parse_op2(&input[1..], header),
            _ => unreachable!(),
        }
    }

    fn parse_op1(mut input: &[char], header: P16Header) -> Result<(&[char], Self), String> {
        if input.len() < 15 {
            return Err("parse_op1 failed: length is less than 15".into());
        }
        let len = Self::parse_usize(&input[..15])?;
        input = &input[15..];
        if input.len() < len {
            return Err(format!("parse_op1 failed: length should be greater than {}, but actually {}", len, input.len()));

        }

        let mut vs = vec![];
        let mut slice = &input[..len];
        while slice.len() > 0 {
            match Self::parse(slice) {
                Ok((slice_new, v)) => {
                    vs.push(v);
                    slice = slice_new;
                },
                Err(_) => {
                    if !slice.iter().all(|c| *c == '0') {
                        return Err("parse_op1 pase number failed.".into());
                    }
                    break;
                }
            }
        }


        Ok((&input[len..], Self::Op(header, vs)))
    }

    fn parse_op2(mut input: &[char], header: P16Header) -> Result<(&[char], Self), String> {
        if input.len() < 11 {
            return Err("parse_op2 failed: length is less than 15".into());
        }
        let len = Self::parse_usize(&input[..11])?;
        input = &input[11..];

        let mut vs = vec![];
        for _ in 0..len {
            let (input_new, v) = Self::parse(input)?;
            vs.push(v);
            input = input_new;
        }

        Ok((input, Self::Op(header, vs)))
    }

    fn parse_lit(input: &[char], header: P16Header) -> Result<(&[char], Self), String> {
        let mut start = 0;
        let mut vs = vec![];
        let mut is_exit = false;
        while start < input.len() {
            if input[start] == '0' {
                is_exit = true;
            }
            for idx in start+1..start+5 {
                vs.push(input[idx]);
            }
            start += 5;
            if is_exit { break; }
        }
        let num = Self::parse_usize(&vs)?;

        Ok((&input[start..], Self::Lit(header, num)))
    }

    fn parse_usize(vs: &[char]) -> Result<usize, String> {
        if vs.len() == 0 { return Err("Empty string is not valid number".to_owned()); }
        if vs.len() > 64 { return Err("length of usize must be less than 64".to_owned()); }
        let mut sum = 0;
        for c in vs.iter() {
            sum *= 2;
            if *c == '1' {
                sum += 1;
            }
        }

        Ok(sum)
    }
}

pub fn p17() {
    let contents: &str = r"target area: x=20..30, y=-10..-5";
    let _contents = std::fs::read_to_string("./assets/adv2021/adv17.txt").unwrap(); let contents: &str = &_contents;

    let (_, ((sx, ex), (ey, sy))) = p17_parse(contents).unwrap();
    println!("{} x {} => {} x {}", sx, ex, sy, ey);
    assert_eq!(true, p17_right((0, 4), (-1, 1), (1, -1)));
    assert_eq!(true, p17_left((0, -4), (-1, 1), (1, -1)));
    // println!("{:?}", p17_solve((30, -10), (sx, ex), (sy, ey)));

    let mut height = 0;
    let mut inits = vec![];
    for vx in 1..ex+1 {
        if (vx+1)*vx / 2 < sx { continue; }

        let mut vy = ey;

        loop {
            assert!(vy < 500);
            if vy > -ey { break; }

            match p17_solve((vx, vy), (sx, ex), (sy, ey)) {
                P17Dir::Left => { vy += 1; }
                _ => break,
            }
        }

        loop {
            assert!(vy < 500);
            if vy > -ey { break; }

            match p17_solve((vx, vy), (sx, ex), (sy, ey)) {
                P17Dir::Inner(h) => {
                    if h >= 0 {
                        inits.push((vx, vy));
                    }
                    height = height.max(h); vy += 1;
                }
                _ => break,
            }
        }

        // println!("--- {} {}: {}", vx, vy, height);
    }
    println!("Result: {} x {}: {:?}", height, inits.len(), &inits[..5]);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum P17Dir {
    Left,
    Right,
    Inner(i64)
}

fn p17_right((px, py): (i64, i64), (sx, sy): (i64, i64), (ex, ey): (i64, i64)) -> bool {
    (ey-sy)*(px-sx) < (py-sy)*(ex-sx)
}

fn p17_left((px, py): (i64, i64), (sx, sy): (i64, i64), (ex, ey): (i64, i64)) -> bool {
    (ey-sy)*(px-sx) > (py-sy)*(ex-sx)
}

fn p17_solve((mut vx, mut vy): (i64, i64), (sx, ex): (i64, i64), (sy, ey): (i64, i64)) -> P17Dir {
    // println!("-----------------------------: {} {}", vx, vy);
    let mut xx = 0; let mut yy = 0;
    let mut last_xx = 0; let mut last_yy = 0;
    let mut height: i64 = yy;
    while xx < ex && yy > ey {
        last_xx = xx;
        last_yy = yy;

        xx += vx;
        yy += vy;
        height = yy.max(height);

        vy -= 1;
        if vx > 0 {
            vx -= 1;
        } else if vx < 0 {
            vx += 1;
        }
        // println!(">>>> {} x {}: {} {}", xx, yy, vx, vy);

        if xx >= sx && xx <= ex && yy <= sy && yy >= ey {
            return P17Dir::Inner(height);
        }
    }

    if p17_right((sx, ey), (last_xx, last_yy), (xx, yy)) {
        P17Dir::Left
    } else if p17_left((ex, sy), (last_xx, last_yy), (xx, yy)) {
        P17Dir::Right
    } else {{
        P17Dir::Inner(i64::MIN)
    }}
}

fn p17_parse(input: &str) -> IResult<&str, ((i64, i64), (i64, i64))> {
    use nom::bytes::complete::tag;
    use nom::character::complete::i64;

    let (input, _) = tag("target area: x=")(input)?;
    let (input, sx) = i64(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, ex) = i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sy) = i64(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, ey) = i64(input)?;

    Ok((input, ((sx, ex), (sy, ey))))
}

pub fn p18() {
    let _contents = std::fs::read_to_string("./assets/adv2021/adv18.txt").unwrap(); let contents: &str = &_contents;

    for contents in vec![
        "[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]",
        r#"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]"#,
        r#"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]"#,
        r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"#,
        r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#,
        contents,
    ].into_iter() {
        let res: Vec<_> = contents.trim().lines().map(
            |line| {
                P18Pair::parse(line).unwrap().1
            }
        ).collect();
        // let res: Vec<_> = res[..2].iter().map(Clone::clone).collect();
        let mut st = res[0].clone();
        for idx in 1..res.len() {
            st = st + res[idx].clone();
        }
        let res_s = res.iter().take(5).map(|p| format!("{}", p)).collect::<Vec<_>>().join(" + ");

        let mut max = 0;
        for idx in 0..res.len() {
            for idy in 0..res.len() {
                if idx == idy { continue; }
                max = max.max((res[idx].clone()+res[idy].clone()).magnitude());
            }
        }
        println!("{} => {} {} = {}", max, st.magnitude(), st, res_s);
    }
}

#[derive(Clone)]
pub struct P18Pair {
    left: P18Elem,
    right: P18Elem,
}

#[derive(Clone)]
pub enum P18Elem {
    Elem(usize),
    Value(Box<P18Pair>),
}

impl P18Elem{
    fn _add_left(&mut self, val: Option<usize>) -> Option<usize> {
        if let Some(v) = val {
            match self {
                P18Elem::Elem(e) => { *e = *e + v; None }
                P18Elem::Value(b) => { b.left._add_left(Some(v)) }
            }
        } else {
            None
        }
    }

    fn _add_right(&mut self, val: Option<usize>) -> Option<usize> {
        if let Some(v) = val {
            match self {
                P18Elem::Elem(e) => { *e = *e + v; None }
                P18Elem::Value(b) => { b.right._add_right(Some(v)) }
            }
        } else {
            None
        }
    }

    pub fn magnitude(&self) -> usize {
        match self {
            P18Elem::Elem(e) => { *e }
            P18Elem::Value(b) => { b.magnitude() }
        }
    }
}

impl P18Pair {
    fn _explode(&mut self, level: i32)-> (bool, Option<usize>, Option<usize>) {
        if level > 0 {
            if let P18Elem::Value(v) = &mut self.left {
                if let (true, left, right) = v._explode(level-1) {
                    let right = self.right._add_left(right);
                    return (true, left, right);
                }
            }

            if let P18Elem::Value(v) = &mut self.right {
                if let (true, left, right) = v._explode(level-1) {
                    let left = self.left._add_right(left);
                    return (true, left, right);
                }
            }
        } else {
            if let P18Elem::Value(v) = &mut self.left {
                if let P18Pair { left: P18Elem::Elem(l), right: P18Elem::Elem(r) } = &mut **v {
                    let l = *l; let r = *r;
                    self.left = P18Elem::Elem(0);
                    let right = self.right._add_left(Some(r));
                    return (true, Some(l), right);
                } else {
                    if let (true, left, right) = v._explode(level-1) {
                        let right = self.right._add_left(right);
                        return (true, left, right)
                    }
                }
            }

            if let P18Elem::Value(v) = &mut self.right {
                if let P18Pair { left: P18Elem::Elem(l), right: P18Elem::Elem(r) } = &mut **v {
                    let l = *l; let r = *r;
                    self.right = P18Elem::Elem(0);
                    let left = self.left._add_right(Some(l));
                    return (true, left, Some(r));
                } else {
                    if let (true, left, right) = v._explode(level-1) {
                        let left = self.left._add_right(left);
                        return (true, left, right);
                    }
                }
            }
        }

        (false, None, None)
    }

    fn explode(&mut self) -> bool {
        self._explode(3).0
    }

    pub fn magnitude(&self) -> usize {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
    }


    fn split(&mut self) -> bool {
        for leaf in vec![&mut self.left, &mut self.right] {
            match leaf {
                P18Elem::Elem(v) => {
                    let v = *v;
                    if v >= 10 {
                        *leaf = P18Elem::Value(Box::new(P18Pair { left: P18Elem::Elem(v/2), right: P18Elem::Elem(v/2 +v%2) }));
                        return true;
                    }
                },
                P18Elem::Value(v) => {
                    if v.split() {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn parse_elem(input: &str) -> IResult<&str, P18Elem> {
        use nom::character::complete::digit1;
        use nom::combinator::{map_res, map};
        use nom::branch::alt;

        let elem = map_res(digit1, |s: &str| s.parse::<usize>());

        alt((
            map(elem, P18Elem::Elem),
            map(Self::parse, |v| P18Elem::Value(Box::new(v))),
        ))(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        use nom::bytes::complete::tag;

        let (input, _) = tag("[")(input)?;
        let (input, left) = Self::parse_elem(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, right) = Self::parse_elem(input)?;
        let (input, _) = tag("]")(input)?;
        Ok((input, P18Pair { left, right }))
    }

    pub fn reduce(&mut self) {
        loop {
            if !(self.explode() || self.split()) {
                break
            }
        }
    }
}

impl std::ops::Add for P18Pair {
    type Output =  Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut o = Self {
            left: P18Elem::Value(Box::new(self)),
            right: P18Elem::Value(Box::new(rhs)),
        };
        o.reduce();
        o
    }
}

impl std::fmt::Debug for P18Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl std::fmt::Display for P18Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            P18Elem::Elem(v) => write!(f, "{}", v),
            P18Elem::Value(v) => write!(f, "{}", v)
        }
    }
}

impl std::fmt::Display for P18Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

pub fn p19() {
    let contents = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"#;
    let _contents = std::fs::read_to_string("./assets/adv2021/adv19.txt").unwrap(); let contents: &str = &_contents;

    let (_, scanners) = p19_parse(contents.trim()).unwrap();

    assert!(scanners.len() > 0);
    let mut map = scanners[0].clone();
    let mut walked: HashSet<_> = (1..scanners.len()).into_iter().collect();
    let mut positions = vec![(0, 0, 0)];

    while walked.len() > 0 {
        let mut found = false;
        for &idx in walked.iter() {
            if let Some(shift) = p19_try(&mut map, &scanners[idx]) {
                found = true;
                walked.remove(&idx);
                println!("Add: {}, Left: {}", idx, walked.len());
                positions.push(shift);
                break;
            }
        }

        assert!(found, "no solution found");
    }

    let mut distance = 0;
    for i in 0..positions.len() {
        for j in 0..positions.len() {
            let (sx, sy, sz) = positions[i];
            let (ex, ey, ez) = positions[j];
            distance = distance.max((sx-ex).abs() + (sy-ey).abs() + (sz-ez).abs());
        }
    }

    println!("{}: {:?}", map.len(), map);
    println!("Scanners {}: {:?}", distance, positions);
}

fn p19_rotate((sx, sy, sz): (i64, i64, i64), dir: &[[i64; 3]; 3]) -> (i64, i64, i64) {
    (
        sx*dir[0][0]+sy*dir[0][1]+sz*dir[0][2],
        sx*dir[1][0]+sy*dir[1][1]+sz*dir[1][2],
        sx*dir[2][0]+sy*dir[2][1]+sz*dir[2][2],
    )
}

fn p19_move_to((fx, fy, fz): (i64, i64, i64), (sx, sy, sz): (i64, i64, i64)) -> (i64, i64, i64) {
    (fx+sx, fy+sy, fz+sz)
}

fn p19_try(map: &mut Vec<(i64, i64, i64)>, to: &Vec<(i64, i64, i64)>) -> Option<(i64, i64, i64)> {
    let map_s: HashSet<_> = map.iter().map(Clone::clone).collect();
    for idx in 0..3 {
        for sigx in [-1, 1] {
            for idy in 0..3 {
                for sigy in [-1, 1] {
                    for idz in 0..3 {
                        for sigz in [-1, 1] {
                            if idx == idy || idy == idz || idx == idz { continue; }
                            let mut rotate = [[0; 3]; 3];
                            rotate[0][idx] = sigx;
                            rotate[1][idy] = sigy;
                            rotate[2][idz] = sigz;

                            let curr: Vec<_> = to.iter().map(|p| p19_rotate(p.clone(), &rotate)).collect();

                            for idi in 0..curr.len() {
                                for idj in 0..map.len() {
                                    let (tx, ty, tz) = curr[idi];
                                    let (sx, sy, sz) = map[idj];
                                    let shift = (sx-tx, sy-ty, sz-tz);

                                    let curr: HashSet<_> = curr.iter().map(|p| p19_move_to(p.clone(), shift)).collect();

                                    if map_s.intersection(&curr).into_iter().count() >= 12 {
                                        for item in curr.difference(&map_s) {
                                            map.push(item.clone());
                                        }
                                        return Some(shift);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }


    None
}

fn p19_parse(input: &str) -> IResult<&str, Vec<Vec<(i64, i64, i64)>>> {
    use nom::multi::{separated_list0, separated_list1, many1};
    use nom::character::complete::{newline, i64 as i64_p, digit1};
    use nom::bytes::complete::tag;
    use nom::sequence::tuple;
    use nom::combinator::map;

    // let scaner = |input: &str| -> IResult<&str, Vec<(i64, i64, i64)>> {
    let scaner = |input| {
        let (input, _) = tag("--- scanner ")(input)?;
        let (input, _) = digit1(input)?;
        let (input, _) = tag(" ---")(input)?;
        let (input, _) = newline(input)?;
        let tup = map(tuple((i64_p, tag(","), i64_p, tag(","), i64_p)), |(i1, _, i2, _, i3)| (i1, i2, i3));

        separated_list1(newline, tup)(input)
    };

    separated_list0(many1(newline), scaner)(input)
}

pub fn p20() {
    let contents = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\
\n\
#..#.\n\
#....\n\
##..#\n\
..#..\n\
..###";
    let _contents = std::fs::read_to_string("./assets/adv2021/adv20.txt").unwrap(); let contents: &str = &_contents;

    let (_, (algorithm, mut image)) = p20_parse(contents.trim()).unwrap();
    println!("{} = {} x {}: \n{}", algorithm.len(), image.len(), image[0].len(), p20_display(&image));

    let mut outlier = false;
    for idx in 0..50 {
        let out = p20_iter(image, outlier, &algorithm);
        image = out.0;
        outlier = out.1;
        println!(
            "{} - {} - {} - {}:\n{}",
            idx,
            image.len(),
            image.iter().map(|line| line.iter().filter(|c| **c).count()).sum::<usize>(),
            outlier,
            if image.len() < 50 { p20_display(&image) } else { "".to_owned() },
        );
    }

}

fn p20_iter(image: Vec<Vec<bool>>, outlier: bool, algorithm: &[bool]) -> (Vec<Vec<bool>>, bool) {
    let nrow = image.len();
    let ncol = image[0].len();

    let out_outlier = if outlier { *algorithm.last().unwrap() } else { algorithm[0] };

    let mut out = vec![vec![out_outlier; ncol+2]; nrow+2];

    for idx in 0..nrow+2 {
        for idy in 0..ncol+2 {
            let mut encode = 0;

            for sx in [-1, 0, 1] {
                for sy in [-1, 0, 1] {
                    let nx = idx as isize + sx - 1;
                    let ny = idy as isize + sy - 1;

                    encode *= 2;
                    let is_light = if nx >= 0 && nx < nrow as isize && ny >= 0 && ny < ncol as isize {
                        image[nx as usize][ny as usize]
                    } else {
                        outlier
                    };

                    if is_light {
                        encode += 1;
                    }
                }
            }

            out[idx][idy] = algorithm[encode];
        }
    }

    (out, out_outlier)
}

fn p20_display(image: &Vec<Vec<bool>>) -> String {
    let mut out = "".to_owned();
    for line in image.iter() {
        for &c in line.iter() {
            if c {
                out.push('#');
            } else {
                out.push('.');
            }
        }
        out.push('\n');
    }
    out
}

fn p20_parse(input: &str) -> IResult<&str, (Vec<bool>, Vec<Vec<bool>>)> {
    use nom::character::complete::{char as char_p, newline};
    use nom::combinator::value;
    use nom::multi::{many1, many0, separated_list1};
    use nom::branch::alt;

    let bool_p = || alt((value(true, char_p('#')), value(false, char_p('.'))));
    let (input, algorithm) = many0(bool_p())(input)?;

    let (input, _) = many1(newline)(input)?;
    let (input, image) = separated_list1(newline, many1(bool_p()))(input)?;
    Ok((input, (algorithm, image)))
}

pub fn p21() {
    let contents = r#"Player 1 starting position: 4
Player 2 starting position: 8"#;
    let _contents = std::fs::read_to_string("./assets/adv2021/adv21.txt").unwrap(); let contents: &str = &_contents;

    let (_, (mut p1, mut p2)) = p21_parse(contents).unwrap();

    let mut dice = 0;
    let mut s1 = 0; let mut s2 = 0;
    let mut count = 0;
    loop {
        p21_roll(&mut dice, &mut p1);
        count += 3;
        s1 += p1 as usize;
        if s1 >= 1000 { break; }

        p21_roll(&mut dice, &mut p2);
        count += 3;
        s2 += p2 as usize;
        if s2 >= 1000 { break; }
    }

    println!("Res for part one: {} * ({}/{}) = {}", count, s1, s2, count*(s2.min(s1) as usize));

    let (_, (p1, p2)) = p21_parse(contents).unwrap();

    let mut ps1: Vec<HashMap<usize, usize>> = vec![Default::default(); 10]; ps1[p1 as usize - 1].insert(0, 1);
    let mut ps2: Vec<HashMap<usize, usize>> = vec![Default::default(); 10]; ps2[p2 as usize - 1].insert(0, 1);

    let mut win1 = 0;
    let mut win2 = 0;
    loop {
        println!("Looping ...");
        let len2 = ps2.iter().map(|h| h.values().sum::<usize>()).sum::<usize>();
        if len2 == 0 { break; }
        win1 += p21_roll_quantum(&mut ps1) * len2;

        let len1 = ps1.iter().map(|h| h.values().sum::<usize>()).sum::<usize>();
        if len1 == 0 { break; }
        win2 += p21_roll_quantum(&mut ps2) * len1;
    }
    println!("Res for part two: {} x {}", win1, win2);
}

fn p21_roll_quantum(ps: &mut Vec<HashMap<usize, usize>>) -> usize {
    let mut out = 0;
    for (idx, scores) in std::mem::replace(ps, vec![Default::default(); 10]).into_iter().enumerate() {
        let mut ndx = vec![idx];
        for _ in 0..3 {
            ndx = ndx.into_iter().map(
                |n| (1..4).into_iter().map(|s| (n+s) % 10).collect::<Vec<_>>()
            ).flatten().collect();
        }

        for n in ndx.into_iter() {
            for (&s, &c) in scores.iter() {
                let score = s+n+1; // NOTE: the score is index+1
                if score >= 21 {
                    out += c;
                } else {
                    *ps[n].entry(score).or_default() += c;
                }
            }
        }
    }

    out
}

fn p21_roll(dice: &mut usize, position: &mut u8) {
    for _ in 0..3 {
        *dice += 1;
        *dice = (*dice - 1) % 100 + 1;
        *position = (*position + (*dice as u8) - 1) % 10 + 1;
    }
}

fn p21_parse(input: &str) -> IResult<&str, (u8, u8)> {
    use nom::sequence::{preceded, tuple};
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, newline};
    use nom::character::complete::u8 as u8_p;

    let player = || preceded(tuple((tag("Player "), digit1, tag(" starting position: "))), u8_p);

    let (input, p1) = player()(input)?;
    let (input, _) = newline(input)?;
    let (input, p2) = player()(input)?;
    Ok((input, (p1, p2)))
}

pub fn p22() {
    let _contents = std::fs::read_to_string("./assets/adv2021/adv22.txt").unwrap(); let contents: &str = &_contents;

    for input in [
        r#"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"#,
        r#"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682"#,
    r#"on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507"#,
    contents,
    ] {
        let (_, cmds) = p22_parse(input).unwrap();

        let mut init = P22Range::default();
        for (on, xs, ys, zs) in cmds.clone().into_iter() {
            println!(">>>> {:?} {:?} {:?} {:?}", on, xs, ys, zs);
            if on {
                init.on((xs, ys, zs));
            } else {
                init.off((xs, ys, zs));
            }
            println!("{}: {:?}", init.count(), "");
        }
    }
}

type P22Area = (std::ops::Range<i32>, std::ops::Range<i32>, std::ops::Range<i32>);

#[derive(Debug, Default, Clone)]
pub struct P22Range {
    pos: Vec<P22Area>,
    neg: Vec<P22Area>,
}

impl P22Range {
    fn _intersect(
        std::ops::Range { start: nx, end: ny }: &std::ops::Range<i32>,
        std::ops::Range { start: sx, end: sy }: &std::ops::Range<i32>,
    ) -> std::ops::Range<i32>{
        use std::ops::Range;
        assert!( nx <= ny );
        assert!( sx <= sy );
        let start = (*sx).max(*nx);
        let mut end = (*sy).min(*ny);
        if start > end { end = start; }
        Range { start, end }
    }

    fn intersect((nx, ny, nz): &P22Area, (sx, sy, sz): &P22Area) -> P22Area {
        (Self::_intersect(nx, sx), Self::_intersect(ny, sy), Self::_intersect(nz, sz))
    }

    fn on(&mut self, area: P22Area) {
        let mut pos = vec![area.clone()];
        let mut neg = vec![];
        for old in self.pos.iter() {
            let inner = Self::intersect(old, &area);
            if Self::calculate(&inner) > 0 {
                neg.push(inner);
            }
        }
        for old in self.neg.iter() {
            let inner = Self::intersect(old, &area);
            if Self::calculate(&inner) > 0 {
                pos.push(inner);
            }
        }
        self.pos.append(&mut pos);
        self.neg.append(&mut neg);

        if self.count() == 0 { self.pos.clear(); self.neg.clear(); }
    }

    fn off(&mut self, area: P22Area) {
        let mut pos = vec![];
        let mut neg = vec![];

        for old in self.pos.iter() {
            let inner = Self::intersect(old, &area);
            if Self::calculate(&inner) > 0 {
                neg.push(inner);
            }
        }
        for old in self.neg.iter() {
            let inner = Self::intersect(old, &area);
            if Self::calculate(&inner) > 0 {
                pos.push(inner);
            }
        }
        self.pos.append(&mut pos);
        self.neg.append(&mut neg);

        if self.count() == 0 { self.pos.clear(); self.neg.clear(); }
    }

    fn calculate((xs, ys, zs): &P22Area) -> usize {
        xs.len() * ys.len() * zs.len()
    }

    fn count(&self) -> usize {
        let mut sum = 0;
        for area in self.pos.iter() {
            sum += Self::calculate(area);
        }
        for area in self.neg.iter() {
            sum -= Self::calculate(area);
        }
        sum
    }
}

fn p22_parse(input: &str) -> IResult<&str, Vec<(bool, std::ops::Range<i32>, std::ops::Range<i32>, std::ops::Range<i32>)>> {
    use nom::branch::alt;
    use nom::combinator::{value, map};
    use nom::bytes::complete::tag;
    use nom::character::complete::{space0, i32 as i32_p, newline};
    use nom::multi::separated_list1;
    use nom::sequence::tuple;

    let cmd = |input| {
        let (input, on) = alt((value(true, tag("on")), value(false, tag("off"))))(input)?;
        let (input, _) = space0(input)?;

        let range_p = || {
            map(
                tuple((i32_p, tag(".."), i32_p)),
                |(s, _, e)| {
                    std::ops::Range { start: s, end: (e+1) }
                    // Part one shoud chang in here. (I know it's dirty, but the main function is
                    // more dirty without mannualy change it in here.)
                    //
                    // if s >= -50 && s <= 50 && e >= -50 && e <= 50 {
                    //     std::ops::Range { start: s, end: (e+1) }
                    // } else {
                    //     std::ops::Range { start: s, end: s }
                    // }
                }
            )
        };

        let (input, _) = tag("x=")(input)?;
        let (input, r1) = range_p()(input)?;
        let (input, _) = tag(",")(input)?;

        let (input, _) = tag("y=")(input)?;
        let (input, r2) = range_p()(input)?;
        let (input, _) = tag(",")(input)?;

        let (input, _) = tag("z=")(input)?;
        let (input, r3) = range_p()(input)?;
        // let (input, _) = tag(",")(input)?;

        Ok((input, (on, r1, r2, r3)))
    };

    separated_list1(newline, cmd)(input)
}

pub fn p23() {
    let (_, init) = Amphipod::<2>::parse(r#"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
        "#.trim()).unwrap();
    p23_solve(init);

    let _contents = std::fs::read_to_string("./assets/adv2021/adv23.txt").unwrap(); let contents: &str = &_contents;
    let (_, init) = Amphipod::<2>::parse(contents.trim()).unwrap();
    p23_solve(init);

    let (_, init) = Amphipod::<4>::parse(r#"
#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########
        "#.trim()).unwrap();
    p23_solve(init);

    // manually change our input here.
    let (_, init) = Amphipod::<4>::parse(r#"
#############
#...........#
###D#D#B#A###
  #D#C#B#A#
  #D#B#A#C#
  #C#A#B#C#
  #########
        "#.trim()).unwrap();
    p23_solve(init);

    // harder
    let (_, init) = Amphipod::<6>::parse(r#"
#############
#...........#
###D#D#B#A###
  #D#C#B#A#
  #D#B#A#C#
  #D#C#B#A#
  #D#B#A#C#
  #C#A#B#C#
  #########
        "#.trim()).unwrap();
    p23_solve(init);

}

fn p23_solve<const N: usize>(init: Amphipod<N>) {
    let mut walked: BTreeSet<AmphipodMap<N>> = Default::default();
    walked.insert(AmphipodMap {
        cost: 0,
        map: init.clone()
    });
    let mut minimum = isize::MAX;

    let mut count = 0;
    while let Some(curr) = walked.pop_first() {
        assert!(!curr.map.finished());
        if curr.cost >= minimum { continue; }
        if count % 1000 == 0 || walked.len() == 0{
            if curr.map.finished() { minimum = curr.cost; }
            // println!("#{} {} {}", count, minimum.min(-1), curr);
        }

        for (mut delta, mut next) in curr.map.next() {
            delta += next.reduce();
            let cost = delta + curr.cost;
            if cost >= minimum { continue; }
            if next.finished() {
                minimum = cost;
            } else {
                walked.insert(AmphipodMap { cost, map: next });
            }
        }
        count += 1;
    }

    if minimum < isize::MAX {
        println!("Ans {}:\n{}", minimum, init);
    } else {
        println!("Fail to find solution for:\n{}", init);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bod {
    A,
    B,
    C,
    D,
    Dot,
}

impl Default for Bod {
    fn default() -> Self {
        Self::Dot
    }
}

// Size is small, should be copy efficiently...
// 我们需要一个结构能够储存位置信息，同时足够小。
// 理论上只记录每个点的位置就可以，但这样获取每行或者没列就比较麻烦
// 而且如何表示第一个限制条件?
//
// 可以使用这些方式:
// - [[u8; 11]; 5] 的矩阵
// - [(usize, usize)] 记录所有点位
// - 其实也可以不使用 enum, -1 代表空, 0-3 代表对应的位置
//
// ```rust
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// struct Amphipod<const N: usize> {
//     hall: [i8; 11],
//     buckets: [[i8; N]; 4]
// }
// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Amphipod<const N: usize> {
    hall: [Bod; 11],
    bucket1: [Bod; N],
    bucket2: [Bod; N],
    bucket3: [Bod; N],
    bucket4: [Bod; N],
}

impl<const N: usize> Default for Amphipod<N> {
    fn default() -> Self {
        Self {
            hall: [Bod::Dot; 11],
            bucket1: [Bod::Dot; N],
            bucket2: [Bod::Dot; N],
            bucket3: [Bod::Dot; N],
            bucket4: [Bod::Dot; N],
        }
    }
}

impl<const N: usize> Amphipod<N> {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{newline, char};
        use nom::multi::{count, separated_list0, many1};
        use nom::branch::alt;
        use nom::combinator::value;
        use nom::sequence::{delimited, };

        let bod = || alt((
                value(Bod::A, tag("A")),
                value(Bod::B, tag("B")),
                value(Bod::C, tag("C")),
                value(Bod::D, tag("D")),
                value(Bod::Dot, tag(".")),
        ));

        let mut out: Self = Default::default();
        let (input, _) = tag("#############")(input)?;
        let (input, _) = newline(input)?;

        let (input, _) = tag("#")(input)?;
        let (input, bods) = count(bod(), 11)(input)?;
        for idx in 0..11 { out.hall[idx] = bods[idx]; }
        let (input, _) = tag("#")(input)?;
        let (input, _) = newline(input)?;

        let space_or_sharp = || alt((char(' '), char('#')));

        let (input, arr) = separated_list0(
            newline,
            delimited(many1(space_or_sharp()), separated_list0(char('#'), bod()), many1(space_or_sharp()))
        )(input)?;

        for idx in 0..N {
            out.bucket1[idx] = arr[idx][0];
            out.bucket2[idx] = arr[idx][1];
            out.bucket3[idx] = arr[idx][2];
            out.bucket4[idx] = arr[idx][3];
        }
        let (input, _) = newline(input)?;

        let (input, _) = many1(space_or_sharp())(input)?;
        Ok((input, out))
    }
}

impl<const N: usize> Amphipod<N> {
    fn bot_cost(b: Bod) -> isize {
        match b {
            Bod::A => 1,
            Bod::B => 10,
            Bod::C => 100,
            Bod::D => 1000,
            Bod::Dot => 0,
        }
    }

    fn finished(&self) -> bool {
        self.bucket1.iter().all(|&b| b == Bod::A) &&
        self.bucket2.iter().all(|&b| b == Bod::B) &&
        self.bucket3.iter().all(|&b| b == Bod::C) &&
        self.bucket4.iter().all(|&b| b == Bod::D)
    }

    /////////////////////////
    fn _move_to_door(bucket: &[Bod]) -> Option<usize> {
        for (idx, &b) in bucket.iter().enumerate() {
            match b {
                Bod::Dot => {  },
                _ => { return Some(idx) }
            }
        }
        None
    }

    fn move_to_door(&self, bod: Bod) -> Option<usize> {
        let bucket = self.get_bucket(bod);
        let idx = Self::_move_to_door(bucket)?;
        if bucket[idx..N].iter().all(|&b| b == bod) {
            return None;
        } else {
            return Some(idx);
        }
    }

    fn move_to_target(&self, bod: Bod) -> Option<usize> {
        let bucket = self.get_bucket(bod);
        let mut last = 0;
        for &b in bucket.iter() {
            if b == Bod::Dot {
                last += 1;
            } else {
                break;
            }
        }
        for &b in bucket[last..].iter() {
            if b != bod {
                return None;
            }
        }

        if last > 0 {
            // assert!(last > 0, "the bucket is already, should not be possible if the graph is right");
            Some(last)
        } else {
            None
        }
    }

    fn get_bucket_mut(&mut self, bod: Bod) -> &mut [Bod; N] {
        match bod {
            Bod::A => &mut self.bucket1,
            Bod::B => &mut self.bucket2,
            Bod::C => &mut self.bucket3,
            Bod::D => &mut self.bucket4,
            _ => unreachable!(),
        }
    }

    fn get_bucket(&self, bod: Bod) -> &[Bod; N] {
        match bod {
            Bod::A => &self.bucket1,
            Bod::B => &self.bucket2,
            Bod::C => &self.bucket3,
            Bod::D => &self.bucket4,
            _ => unreachable!(),
        }
    }

    fn door_id(bod: Bod) -> usize {
        match bod {
            Bod::A => 2,
            Bod::B => 4,
            Bod::C => 6,
            Bod::D => 8,
            _ => unreachable!(),
        }
    }

    fn move_in_hall(&self, from: usize, to: usize) -> Option<usize> {
        if from < to {
            for &b in &self.hall[from+1..to+1] {
                if b != Bod::Dot { return None }
            }
            Some(to-from)
        } else {
            for &b in &self.hall[to..from] {
                if b != Bod::Dot { return None }
            }
            Some(from-to)
        }
    }

    fn go_from(&self, from: usize) -> Vec<usize> {
        let mut out = vec![];

        let mut idx = from;
        while idx > 0 {
            idx -= 1;
            if idx == 2 || idx == 4 || idx == 6 || idx == 8 { continue; }

            if self.hall[idx] == Bod::Dot {
                out.push(idx)
            } else {
                break;
            }
        }

        let mut idx = from;
        while idx < 10 {
            idx += 1;
            if idx == 2 || idx == 4 || idx == 6 || idx == 8 { continue; }

            if self.hall[idx] == Bod::Dot {
                out.push(idx)
            } else {
                break;
            }
        }

        out
    }

    /////////////////////////
    fn next(&self) -> Vec<(isize, Self)> {
        let mut out = vec![];

        for curr in vec![Bod::A, Bod::B, Bod::C, Bod::D].into_iter() {
            let door = Self::door_id(curr);
            if let Some(step0) = self.move_to_door(curr) {
                for idx in self.go_from(door) {
                    //           cost to walk out     cost for walk to right point
                    let cost = ((step0 as isize+1) + (idx as isize - door as isize).abs()) *
                        Self::bot_cost(self.get_bucket(curr)[step0]);

                    let mut next = self.clone();
                    next.hall[idx] = self.get_bucket(curr)[step0];
                    next.get_bucket_mut(curr)[step0] = Bod::Dot;
                    out.push((cost, next));
                }
            }
        }

        out
    }

    fn _reduce(&mut self) -> Option<isize> {
        for (idx, &curr) in self.hall.iter().enumerate() {
            if curr == Bod::Dot { continue; }
            if let Some(step1) = self.move_to_target(curr) {
                if let Some(step2) = self.move_in_hall(idx, Self::door_id(curr)) {
                    self.hall[idx] = Bod::Dot;
                    assert_eq!(self.get_bucket_mut(curr)[step1-1], Bod::Dot);
                    self.get_bucket_mut(curr)[step1-1] = curr;
                    return Some( ((step1+step2) as isize) * Self::bot_cost(curr) );
                }
            }
        }

        for curr in vec![Bod::A, Bod::B, Bod::C, Bod::D].into_iter() {
            if let Some(step0) = self.move_to_door(curr) {
                let to: Bod = self.get_bucket(curr)[step0];
                if to == curr { continue; }
                if let Some(step1) = self.move_to_target(to) {
                    if let Some(step2) = self.move_in_hall(Self::door_id(curr), Self::door_id(to)) {
                        self.get_bucket_mut(curr)[step0] = Bod::Dot;
                        assert_eq!(self.get_bucket_mut(to)[step1-1], Bod::Dot);
                        self.get_bucket_mut(to)[step1-1] = to;
                        return Some( ((step0+1)+step1+step2) as isize * Self::bot_cost(to) );
                    }
                }
            }
        }

        None
    }

    fn reduce(&mut self) -> isize {
        let mut sum = 0;
        while let Some(delta) = self._reduce() {
            sum += delta;
        }

        sum
    }

    // NOTE: this function is useless, since we just want one minimum solution
    // #[allow(dead_code)]
    fn rest(&self) -> isize {
        let mut sum = 0;
        for (from, &curr) in self.hall.iter().enumerate() {
            if curr == Bod::Dot { continue; }
            let to = Self::door_id(curr);
            sum += (from as isize - to as isize).abs() * Self::bot_cost(curr);
        }

        for curr in vec![Bod::A, Bod::B, Bod::C, Bod::D].into_iter() {
            let from = Self::door_id(curr);
            let bucket = self.get_bucket(curr);
            for &b in bucket.iter() {
                if b == Bod::Dot { continue; }
                if b != curr {
                    let to = Self::door_id(b);
                    sum += (from as isize - to as isize).abs() * Self::bot_cost(b);
                }
            }
        }

        sum
    }
}

impl<const N: usize> AmphipodMap<N> {
    fn estimate(&self) -> (isize, &Amphipod<N>) {
        (
            self.cost,
            &self.map,
        )
    }
}

impl<const N: usize> std::cmp::PartialOrd for AmphipodMap<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.estimate().partial_cmp(&other.estimate())
    }
}

impl<const N: usize> std::cmp::Ord for AmphipodMap<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.estimate().cmp(&other.estimate())
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct AmphipodMap<const N: usize> {
    cost: isize,
    map: Amphipod<N>,
}

impl<const N: usize> std::fmt::Display for AmphipodMap<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cost: {} \n{}", self.cost, self.map)
    }
}

impl std::fmt::Display for Bod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bod::Dot => write!(f, "{}", "."),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl<const N: usize> std::fmt::Display for Amphipod<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#############\n")?;
        write!(f, "#{}{}{}{}{}{}{}{}{}{}{}#\n",
            self.hall[0], self.hall[1], self.hall[2], self.hall[3], self.hall[4], self.hall[5], self.hall[6],
            self.hall[7], self.hall[8], self.hall[9], self.hall[10]
        )?;
        write!(f, "###{}#{}#{}#{}###\n", self.bucket1[0], self.bucket2[0], self.bucket3[0], self.bucket4[0])?;
        for idx in 1..N {
            write!(f, "  #{}#{}#{}#{}#  \n", self.bucket1[idx], self.bucket2[idx], self.bucket3[idx], self.bucket4[idx])?;
        }
        Ok(())
    }
}

pub fn p24() {
    let (_, cmds) = p24_parse(std::fs::read_to_string("./assets/adv2021/adv24.txt").unwrap().trim()).unwrap();

    let mut mem: HashMap<(usize, [i64; 4]), Option<(i64, i64)>> = Default::default();
    // change true to false to solve part 2
    let (_, ans) = p24_dfs(&cmds, [0; 4], 0, &mut mem, true).unwrap();
    println!("Res: {}", ans);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum P24Reg {
    Value(i64),
    Reg(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum P24Kind {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum P24Cmd {
    Inp(P24Reg),
    Op {kind: P24Kind, left: P24Reg, right: P24Reg }
}

pub fn p24_parse(input: &str) -> IResult<&str, Vec<P24Cmd>> {
    use nom::multi::separated_list0;
    use nom::branch::alt;
    use nom::combinator::{value, map};
    use nom::bytes::complete::tag;
    use nom::sequence::tuple;
    use nom::character::complete::{i64 as i64_p, space1, newline};

    let p_reg_reg = || alt((
        value(P24Reg::Reg(0), tag("z")),
        value(P24Reg::Reg(1), tag("y")),
        value(P24Reg::Reg(2), tag("x")),
        value(P24Reg::Reg(3), tag("w")),
    ));
    let p_reg = || alt((p_reg_reg(), map(i64_p, |v| P24Reg::Value(v))));
    let p_op = || alt((
        value(P24Kind::Add, tag("add")),
        value(P24Kind::Mul, tag("mul")),
        value(P24Kind::Div, tag("div")),
        value(P24Kind::Mod, tag("mod")),
        value(P24Kind::Eql, tag("eql")),
    ));

    let p_inp = map(tuple((tag("inp"), space1, p_reg_reg())), |(_, _, r)| P24Cmd::Inp(r));
    let p_op = map(
        tuple((p_op(), space1, p_reg(), space1, p_reg())),
        |(k, _, l, _, r)| P24Cmd::Op { kind: k, left: l, right: r }
    );

    let ins = alt((p_inp, p_op));

    separated_list0(newline, ins)(input)
}

fn p24_dfs(
    cmds: &[P24Cmd], reg: [i64; 4], pos_ori: usize, mem: &mut HashMap<(usize, [i64; 4]), Option<(i64, i64)>>, is_rev: bool
) -> Option<(i64, i64)> {
    if let Some(v) = mem.get(&(pos_ori, reg)) { return v.clone(); }
    let ptr = if let P24Cmd::Inp(P24Reg::Reg(ptr)) = cmds[pos_ori] {
        ptr
    } else {
        unreachable!()
    };

    let wtf: Vec<_> = if is_rev { (1..10).into_iter().rev().collect() } else { (1..10).into_iter().collect() };

    for guess in wtf.into_iter() {
        let mut reg = reg.clone();
        reg[ptr] = guess;

        let mut pos = pos_ori + 1;
        while pos < cmds.len() {
            match cmds[pos] {
                P24Cmd::Inp(_) => { break; }
                P24Cmd::Op { kind, left: P24Reg::Reg(ptr), right } => {
                    let right = match right {
                        P24Reg::Value(v) => v,
                        P24Reg::Reg(vp) => reg[vp],
                    };

                    match kind {
                        P24Kind::Add => { reg[ptr] += right },
                        P24Kind::Mul => { reg[ptr] *= right },
                        P24Kind::Div => { assert!(right != 0); reg[ptr] /= right },
                        P24Kind::Mod => { assert!(reg[ptr] >= 0 && right > 0); reg[ptr] %= right },
                        P24Kind::Eql => { reg[ptr] = if reg[ptr] == right { 1 } else { 0 } },
                    }
                }
                _ => unreachable!(),
            }
            pos += 1;
        }

        if pos < cmds.len() {
            if let Some((pow, old)) = p24_dfs(cmds, reg, pos, mem, is_rev) {
                let out = Some((pow+1, 10i64.pow(pow as u32) * guess + old));
                mem.insert((pos_ori, reg), out);
                return out
            }
        } else {
            if reg[0] == 0 {
                let out = Some((1, guess));
                mem.insert((pos_ori, reg), out);
                return out;
            }
        }
    }
    mem.insert((pos_ori, reg), None);

    None
}

pub fn p25() {
    let contents = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;
    let _contents = std::fs::read_to_string("./assets/adv2021/adv25.txt").unwrap(); let contents = &*_contents;

    let (_,mut map) = P25Map::parse(contents.trim()).unwrap();
    println!("{}", map);

    let mut idx = 0;
    loop {
        idx += 1;
        let r = map.move_forward();
        if !r {
            println!("#{} done={}:\n{}", idx, !r, map);
            break;
        }
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum P25Dir {
    Down,
    Right,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct P25Map(Vec<Vec<Option<P25Dir>>>);

impl std::fmt::Display for P25Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.0.iter() {
            for c in line.iter() {
                match c {
                    Some(P25Dir::Down) => write!(f, "v")?,
                    Some(P25Dir::Right) => write!(f, ">")?,
                    None => write!(f, ".")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl P25Map {
    fn move_right(&mut self) -> bool {
        let nrow = self.0.len();
        let ncol = self.0[0].len();
        let mut stack = vec![];
        for idr in 0..nrow {
            for idc in 0..ncol {
                if let Some(P25Dir::Right) = self.0[idr][idc] {
                    let mut ndc = idc+1;
                    if ndc >= ncol { ndc = 0; }
                    if self.0[idr][ndc] == None {
                        stack.push((idr, idc, ndc));
                    }
                }
            }
        }
        if stack.len() == 0 { return false; }
        for (idr, idc, ndc) in stack.into_iter() {
            self.0[idr][ndc] = Some(P25Dir::Right);
            self.0[idr][idc] = None;
        }

        true
    }

    fn move_down(&mut self) -> bool {
        let nrow = self.0.len();
        let ncol = self.0[0].len();
        let mut stack = vec![];
        for idr in 0..nrow {
            for idc in 0..ncol {
                if let Some(P25Dir::Down) = self.0[idr][idc] {
                    let mut ndr = idr+1;
                    if ndr >= nrow { ndr = 0; }
                    if self.0[ndr][idc] == None {
                        stack.push((idc, idr, ndr));
                    }
                }
            }
        }
        if stack.len() == 0 { return false; }
        for (idc, idr, ndr) in stack.into_iter() {
            self.0[ndr][idc] = Some(P25Dir::Down);
            self.0[idr][idc] = None;
        }
        true
    }

    fn move_forward(&mut self) -> bool {
        let r = self.move_right();
        let d = self.move_down();
        r || d
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::branch::alt;
        use nom::combinator::value;
        use nom::character::complete::{char as char_p, newline};
        use nom::multi::{separated_list1, many1};

        let dir_p = alt((
                value(None, char_p('.')),
                value(Some(P25Dir::Down), char_p('v')),
                value(Some(P25Dir::Right), char_p('>')),
        ));

        let (input, inner) = separated_list1(newline, many1(dir_p))(input)?;
        Ok((input, Self(inner)))
    }
}

