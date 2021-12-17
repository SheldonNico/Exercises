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
