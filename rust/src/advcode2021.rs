use std::collections::{HashSet, HashMap, VecDeque};
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
