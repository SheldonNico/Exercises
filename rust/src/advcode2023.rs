#![allow(unused_imports)]

pub fn p01() {
    let contents = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    let contents = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
eightwothree
oneeightwo
7pqrstsixteen"#;
    let contents = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    let contents = std::fs::read_to_string("./assets/adv2023/adv01.txt").unwrap();

    let calibration: Vec<u8> = contents
        .lines()
        .map(|line| {
            let mut num1 = None;
            let mut num2 = None;
            for ch in line.chars() {
                if ch.is_digit(10) {
                    let num = ch as u8 - '0' as u8;
                    if num1.is_none() {
                        num1 = Some(num);
                    } else {
                        num2 = Some(num);
                    }
                }
            }

            let num1 = num1.unwrap_or(0);
            let num2 = num2.unwrap_or(num1);

            num1 * 10 + num2
        })
        .collect();
    dbg!(&calibration);

    let sum1: usize = calibration.iter().map(|&n| n as usize).sum::<usize>();
    eprintln!("sum1 = {sum1}");

    const PATTERNS: [(&'static str, u8); 18] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        // ("0"     , 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    ////////////////////////////////////////////////////////////////////////////
    let calibration: Vec<u8> = contents
        .lines()
        .map(|line| {
            let mut num = None;
            for offset in 0..line.len() {
                for (pattern, pattern_to) in PATTERNS {
                    if line[offset..].starts_with(pattern) {
                        num = Some(pattern_to);
                        break;
                    }
                }
                if num.is_some() {
                    break;
                }
            }
            let num1 = num.unwrap_or(0);

            let mut num = None;
            for offset in (1..line.len() + 1).rev() {
                for (pattern, pattern_to) in PATTERNS {
                    if line[..offset].ends_with(pattern) {
                        num = Some(pattern_to);
                        break;
                    }
                }
                if num.is_some() {
                    break;
                }
            }
            let num2 = num.unwrap_or(0);

            num1 * 10 + num2
        })
        .collect();
    dbg!(&calibration);

    let sum2: usize = calibration.iter().map(|&n| n as usize).sum::<usize>();
    eprintln!("sum1 = {sum2}");
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct P02GameSet {
    red: usize,
    blue: usize,
    green: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P02Game {
    idx: usize,
    subsets: Vec<P02GameSet>,
}

impl P02Game {
    pub fn parse_games(input: &str) -> nom::IResult<&str, Vec<Self>> {
        nom::multi::separated_list0(nom::character::complete::newline, Self::parse)(input)
    }

    pub fn parse_game_set(input: &str) -> nom::IResult<&str, P02GameSet> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{newline, space0, space1, u64};
        use nom::combinator::map;
        use nom::sequence::tuple;

        let (input, cubes) = nom::multi::separated_list0(
            tuple((tag(","), space0)),
            tuple((
                u64,
                space0,
                nom::branch::alt((tag("red"), tag("blue"), tag("green"))),
            )),
        )(input)?;

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for (num, _, cube) in cubes {
            match cube {
                "red" => {
                    red = num as usize;
                }
                "blue" => {
                    blue = num as usize;
                }
                "green" => {
                    green = num as usize;
                }
                _ => {}
            }
        }
        Ok((input, P02GameSet { red, green, blue }))
    }

    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{newline, space0, space1, u64};
        use nom::combinator::map;
        use nom::sequence::tuple;

        let (input, _) = tag("Game")(input)?;
        let (input, _) = space1(input)?;
        let (input, idx) = u64(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = space1(input)?;
        let (input, subsets) =
            nom::multi::separated_list0(tuple((tag(";"), space0)), Self::parse_game_set)(input)?;

        Ok((
            input,
            Self {
                idx: idx as usize,
                subsets,
            },
        ))
    }
}

pub fn p02() {
    let contents = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv02.txt").unwrap();

    let (_, games) = P02Game::parse_games(&contents).unwrap();

    let mut sum1 = 0;
    for game in games.iter() {
        if game
            .subsets
            .iter()
            .all(|gs| gs.red <= 12 && gs.green <= 13 && gs.blue <= 14)
        {
            sum1 += game.idx;
        }
    }
    eprintln!("sum1: {sum1}");

    let mut sum2 = 0;
    for game in games.iter() {
        let mut opt = P02GameSet::default();
        for gs in game.subsets.iter() {
            opt.red = opt.red.max(gs.red);
            opt.blue = opt.blue.max(gs.blue);
            opt.green = opt.green.max(gs.green);
        }

        let score = opt.red * opt.green * opt.blue;
        sum2 += score;

        if score == 0 {
            eprintln!("{:?}: {}", game, score);
        }
    }
    eprintln!("sum2: {sum2}");
}

#[derive(Debug)]
pub enum P03Digit {
    Digit(u8),
    Dot,
    Symbol(char),
}

impl P03Digit {
    pub fn is_symbol(&self) -> bool {
        matches!(self, P03Digit::Symbol(_))
    }
    pub fn is_digit(&self) -> bool {
        matches!(self, P03Digit::Digit(_))
    }
}

pub fn p03() {
    let contents = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv03.txt").unwrap();

    let matrix: Vec<Vec<P03Digit>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    if ch.is_digit(10) {
                        P03Digit::Digit(ch as u8 - '0' as u8)
                    } else if ch == '.' {
                        P03Digit::Dot
                    } else {
                        P03Digit::Symbol(ch)
                    }
                })
                .collect()
        })
        .collect();
    let height = matrix.len();
    let width = matrix[0].len();
    // eprintln!("{:?}", matrix);

    let mut sum1 = 0;
    for idy in 0..height {
        let mut num = 0;
        let mut is_part = false;
        for idx in 0..width {
            match matrix[idy][idx] {
                P03Digit::Digit(digit) => {
                    if !is_part {
                        if idy + 1 < height {
                            is_part |= matrix[idy + 1][idx].is_symbol();
                        }
                        if idy > 0 {
                            is_part |= matrix[idy - 1][idx].is_symbol();
                        }
                        if num == 0 && idx > 0 {}
                    }

                    num = num * 10 + digit as usize;
                }
                _ => {
                    if num != 0 {
                        if idy + 1 < height {
                            is_part |= matrix[idy + 1][idx].is_symbol();
                        }
                        if idy > 0 {
                            is_part |= matrix[idy - 1][idx].is_symbol();
                        }
                        is_part |= matrix[idy][idx].is_symbol();

                        eprintln!("{}: {}", is_part, num);
                        if is_part {
                            sum1 += num;
                        }
                        num = 0;
                    }

                    // calculate the left diag
                    is_part = matrix[idy][idx].is_symbol();
                    if idy + 1 < height {
                        is_part |= matrix[idy + 1][idx].is_symbol();
                    }
                    if idy > 0 {
                        is_part |= matrix[idy - 1][idx].is_symbol();
                    }
                }
            }
        }

        if num != 0 {
            eprintln!("{}: {}", is_part, num);
            if is_part {
                sum1 += num;
            }
        }
    }
    eprintln!("sum1={}", sum1);

    let mut sum2 = 0;
    for idy in 0..height {
        for idx in 0..width {
            if matches!(matrix[idy][idx], P03Digit::Symbol('*')) {
                let mut numbers: Vec<usize> = vec![];

                if idx > 0 && matrix[idy][idx - 1].is_digit() {}
                if idx + 1 < width && matrix[idy][idx + 1].is_digit() {}

                let mut overlayed: Vec<(isize, isize)> = vec![];

                for (xoffset, yoffset) in [
                    (0, 1),
                    (0, -1),
                    (1, 0),
                    (-1, 0),
                    (-1, 1),
                    (-1, -1),
                    (1, 1),
                    (1, -1),
                ] {
                    let mut nx = idx as isize + xoffset;
                    let ny = idy as isize + yoffset;
                    if nx < 0 || nx >= (width as isize) || ny < 0 || ny >= (height as isize) {
                        continue;
                    }
                    if let P03Digit::Digit(digit) = matrix[ny as usize][nx as usize] {
                        // eprintln!("Search {} {}", ny, nx);
                        let point = (nx, ny);

                        while nx >= 0 {
                            if let P03Digit::Digit(digit) = matrix[ny as usize][nx as usize] {
                            } else {
                                nx += 1;
                                break;
                            }
                            nx -= 1;
                        }
                        nx = nx.max(0);

                        let mut num = 0;
                        let mut is_overlay = false;
                        while nx < width as isize {
                            if overlayed.iter().any(|&(ox, oy)| ny == oy && nx == ox) {
                                is_overlay = true;
                                break;
                            }
                            if let P03Digit::Digit(digit) = matrix[ny as usize][nx as usize] {
                                num = digit as isize + num * 10;
                            } else {
                                break;
                            }

                            nx += 1;
                        }

                        overlayed.push(point);
                        if !is_overlay {
                            numbers.push(num as usize);
                        }
                    }
                }

                if numbers.len() == 2 {
                    sum2 += numbers[0] * numbers[1];
                }

                eprintln!("{} x {}: {:?}", idy, idx, numbers);
            }
        }
    }
    eprintln!("sum2={}", sum2);
}

#[derive(Debug, Clone)]
pub struct P04Card {
    idx: usize,
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

impl P04Card {
    fn parse_one(input: &str) -> nom::IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{newline, space0, space1, u64};
        use nom::combinator::map;
        use nom::sequence::tuple;

        let (input, _) = tag("Card")(input)?;
        let (input, _) = space0(input)?;
        let (input, idx) = u64(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = space0(input)?;

        let (input, winning) =
            nom::multi::separated_list0(space1, map(u64, |ix| ix as usize))(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("|")(input)?;
        let (input, _) = space1(input)?;
        let (input, numbers) =
            nom::multi::separated_list0(space1, map(u64, |ix| ix as usize))(input)?;
        Ok((
            input,
            Self {
                idx: idx as usize,
                winning,
                numbers,
            },
        ))
    }

    fn parse(input: &str) -> nom::IResult<&str, Vec<Self>> {
        nom::multi::separated_list0(nom::character::complete::newline, Self::parse_one)(input)
    }
}

pub fn p04() {
    let contents = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv04.txt").unwrap();

    let (_, cards) = P04Card::parse(&contents).unwrap();
    // eprintln!("{:?}", cards);

    let mut sum1 = 0;
    for card in cards.iter() {
        let winning = card
            .winning
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<usize>>();
        let numbers = card
            .numbers
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<usize>>();
        // eprintln!("{:?}", winning.intersection(&numbers).collect::<Vec<_>>());
        let score = (winning.intersection(&numbers)).collect::<Vec<_>>().len();
        if score > 0 {
            sum1 += 2i32.pow((score as u32) - 1);
        }
    }
    eprintln!("sum1={}", sum1);

    let mut scores: Vec<usize> = vec![1; cards.len()];
    for card in cards.iter() {
        let winning = card
            .winning
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<usize>>();
        let numbers = card
            .numbers
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<usize>>();
        let score = (winning.intersection(&numbers)).collect::<Vec<_>>().len();
        if score > 0 {
            for offset in 0..score {
                scores[card.idx - 1 + offset + 1] += scores[card.idx - 1];
            }
            eprintln!("{} {:?}", score, scores);
        }
    }
    eprintln!("sum2={}", scores.iter().sum::<usize>());
}

#[derive(Debug, Clone)]
pub struct P05SeedDestion {
    name: String,
    convert: Vec<(usize, usize, usize)>,
}

#[derive(Debug, Clone)]
pub struct P05SeedMap {
    seeds: Vec<usize>,
    maps: std::collections::HashMap<String, P05SeedDestion>,
}

impl P05SeedMap {
    pub fn parse_one(input: &str) -> nom::IResult<&str, (String, P05SeedDestion)> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{alphanumeric1, newline, space0, space1, u64};
        use nom::combinator::map;
        use nom::sequence::tuple;

        let (input, from) = alphanumeric1(input)?;
        let (input, _) = tag("-to-")(input)?;
        let (input, name) = alphanumeric1(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("map:")(input)?;
        let (input, _) = newline(input)?;
        let (input, convert) = nom::multi::separated_list0(
            newline,
            map(tuple((u64, space1, u64, space1, u64)), |(x, _, y, _, z)| {
                (x as usize, y as usize, z as usize)
            }),
        )(input)?;

        Ok((
            input,
            (
                from.to_string(),
                P05SeedDestion {
                    name: name.to_string(),
                    convert,
                },
            ),
        ))
    }

    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{newline, space0, space1, u64};
        use nom::combinator::map;
        use nom::sequence::tuple;

        let (input, _) = tag("seeds:")(input)?;
        let (input, _) = space1(input)?;
        let (input, seeds) =
            nom::multi::separated_list0(space1, map(u64, |ix| ix as usize))(input)?;
        let (input, _) = nom::multi::many1(newline)(input)?;

        let (input, maps) =
            nom::multi::separated_list0(nom::multi::many1(newline), Self::parse_one)(input)?;

        Ok((
            input,
            Self {
                seeds,
                maps: maps.into_iter().collect(),
            },
        ))
    }
}

pub fn p05() {
    let contents = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv05.txt").unwrap();

    let (_, seedmap) = P05SeedMap::parse(&contents).unwrap();
    eprintln!("{:?}", seedmap);

    let mut location = usize::MAX;
    for point in seedmap.seeds.iter().cloned() {
        let mut name = "seed".to_string();
        let mut point = point;
        // let mut point = 82;
        while name != "location" {
            let next_map = seedmap.maps.get(&*name).unwrap();
            for (to, from, len) in next_map.convert.iter().cloned() {
                if point >= from && point < from + len {
                    point = to + (point - from);
                    break;
                }
            }
            name = next_map.name.clone();
            // eprintln!("{name} {point}");
        }
        location = location.min(point);

        // eprintln!();
    }
    eprintln!("location1: {location}");

    let mut location = usize::MAX;
    for idx in (0..seedmap.seeds.len()).step_by(2) {
        let start = seedmap.seeds[idx];
        let length = seedmap.seeds[idx + 1];
        eprintln!("{start} - {length}");

        let mut points = vec![(start, length)];
        let mut name = "seed".to_string();
        while name != "location" && points.len() > 0 {
            let next_map = seedmap.maps.get(&*name).unwrap();

            let mut stopped = vec![];
            for (to, from, len) in next_map.convert.iter().cloned() {
                for (start, length) in std::mem::replace(&mut points, Default::default()) {
                    if start >= from + len || start + length <= from {
                        points.push((start, length));
                        continue;
                    }

                    let inner_from = to + start.max(from) - from;
                    let inner_to = to + (start + length).min(from + len) - from;
                    if inner_from < inner_to {
                        stopped.push((inner_from, inner_to - inner_from));
                    }

                    // [start, from) [from+len, start+length)
                    if start < from {
                        points.push((start, from - start));
                    }
                    if from + len < start + length {
                        points.push((from + len, start + length - from - len));
                    }
                    // eprintln!("    start={start} from={from} len={len} length={length} {:?}", points);
                }
            }

            points.append(&mut stopped);

            name = next_map.name.clone();
            // eprintln!("{name} {:?}", points); eprintln!();
        }
        location = location.min(points.iter().map(|&(s, _)| s).min().unwrap());
    }
    eprintln!("location2: {location}");
}

pub fn p06_parse(input: &str) -> nom::IResult<&str, Vec<(u64, u64)>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{newline, space0, space1, u64};
    use nom::combinator::map;
    use nom::sequence::tuple;

    let p_time = nom::sequence::preceded(
        tuple((tag("Time:"), space0)),
        nom::multi::separated_list0(space1, u64),
    );
    let p_distance = nom::sequence::preceded(
        tuple((tag("Distance:"), space0)),
        nom::multi::separated_list0(space1, u64),
    );

    let (input, (time, _, distance)) = tuple((p_time, newline, p_distance))(input)?;
    assert_eq!(time.len(), distance.len());

    Ok((input, time.into_iter().zip(distance.into_iter()).collect()))
}

pub fn p06_parse2(input: &str) -> nom::IResult<&str, (u64, u64)> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{newline, space0, space1, u64};
    use nom::combinator::map;
    use nom::multi::many1;
    use nom::sequence::tuple;

    let p_time = nom::sequence::preceded(
        tuple((tag("Time:"), space0)),
        many1(nom::character::complete::satisfy(|c| {
            c == ' ' || c.is_digit(10)
        })),
    );
    let p_distance = nom::sequence::preceded(
        tuple((tag("Distance:"), space0)),
        many1(nom::character::complete::satisfy(|c| {
            c == ' ' || c.is_digit(10)
        })),
    );

    let (input, (time, _, distance)) = tuple((p_time, newline, p_distance))(input)?;

    Ok((
        input,
        (
            time.into_iter()
                .filter(|&c| c != ' ')
                .collect::<String>()
                .parse()
                .unwrap(),
            distance
                .into_iter()
                .filter(|&c| c != ' ')
                .collect::<String>()
                .parse()
                .unwrap(),
        ),
    ))
}

pub fn p06() {
    let contents = r#"Time:      7  15   30
Distance:  9  40  200"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv06.txt").unwrap();
    let (_, races) = p06_parse(&contents).unwrap();
    eprintln!("{:?}", races);

    let mut margin = 1;
    for &(time, distance) in races.iter() {
        for offset in 1..time {
            if offset * (time - offset) > distance {
                println!("{offset} {}", time - offset);
                margin *= (time - offset) - offset + 1;
                break;
            }
        }
    }
    eprintln!("margin: {margin}");

    let (_, (time, distance)) = p06_parse2(&contents).unwrap();
    for offset in 1..time {
        if offset * (time - offset) > distance {
            println!("{offset} {}", time - offset);
            let margin = (time - offset) - offset + 1;
            eprintln!("margin2: {margin}");
            break;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum P07Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl P07Type {
    pub fn new_joke(cards: &[P07Card; 5]) -> Self {
        use P07Type::*;

        let mut counts: std::collections::HashMap<P07Card, usize> = Default::default();
        let mut jcount: usize = 0;
        for &cx in cards.iter() {
            if cx == P07Card::J {
                jcount += 1;
            } else {
                *counts.entry(cx).or_default() += 1;
            }
        }

        // JJJAA -> FiveOfAKind
        if counts.len() == 1 || counts.len() == 0 {
            return FiveOfAKind;
        }

        // 1122J
        if counts.len() == 2 {
            let mut paris: Vec<usize> = counts.values().cloned().collect();
            paris.sort();
            // (1, 4) (2, 3) (1, 1) (1, 2) (1, 3) (2, 2)
            if paris[..1] != vec![1] {
                return FullHouse;
            } else {
                return FourOfAKind;
            }
        }
        if counts.len() == 3 {
            let mut paris: Vec<usize> = counts.values().cloned().collect();
            paris.sort();
            // (1, 1, 1)
            if paris[..2] != vec![1, 1] {
                return TwoPair;
            } else {
                return ThreeOfAKind;
            }
        }
        if counts.len() == 4 {
            // 1 1 1 1
            // 1 1 1 2
            return OnePair;
        }

        assert_eq!(jcount, 0);
        assert_eq!(counts.len(), 5);
        HighCard
    }

    pub fn new(cards: &[P07Card; 5]) -> Self {
        use P07Type::*;

        let mut counts: std::collections::HashMap<P07Card, usize> = Default::default();
        for &cx in cards.iter() {
            *counts.entry(cx).or_default() += 1;
        }

        if counts.len() == 1 {
            return FiveOfAKind;
        }
        if counts.len() == 2 {
            let mut paris: Vec<usize> = counts.values().cloned().collect();
            paris.sort();
            if paris == vec![1, 4] {
                return FourOfAKind;
            }
            if paris == vec![2, 3] {
                return FullHouse;
            }
        }
        if counts.len() == 3 {
            let mut paris: Vec<usize> = counts.values().cloned().collect();
            paris.sort();
            if paris == vec![1, 1, 3] {
                return ThreeOfAKind;
            }
            if paris == vec![1, 2, 2] {
                return TwoPair;
            }
        }
        if counts.len() == 4 {
            return OnePair;
        }

        HighCard
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum P07Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

pub fn p07_joke_card(c: &P07Card) -> usize {
    use P07Card::*;
    match *c {
        J => 1,
        N2 => 2,
        N3 => 3,
        N4 => 4,
        N5 => 5,
        N6 => 6,
        N7 => 7,
        N8 => 8,
        N9 => 9,
        T => 10,
        Q => 11,
        K => 12,
        A => 13,
    }
}

pub fn p07_parse(input: &str) -> nom::IResult<&str, Vec<([P07Card; 5], u64)>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, newline, space0, space1, u64};
    use nom::combinator::{map, value};
    use nom::sequence::tuple;

    let p_card = nom::branch::alt((
        value(P07Card::N2, char('2')),
        value(P07Card::N3, char('3')),
        value(P07Card::N4, char('4')),
        value(P07Card::N5, char('5')),
        value(P07Card::N6, char('6')),
        value(P07Card::N7, char('7')),
        value(P07Card::N8, char('8')),
        value(P07Card::N9, char('9')),
        value(P07Card::T, char('T')),
        value(P07Card::J, char('J')),
        value(P07Card::Q, char('Q')),
        value(P07Card::K, char('K')),
        value(P07Card::A, char('A')),
    ));
    let p_card5 = map(nom::multi::count(p_card, 5), |cs| {
        assert_eq!(cs.len(), 5);
        [cs[0], cs[1], cs[2], cs[3], cs[4]]
    });

    nom::multi::separated_list0(newline, nom::sequence::separated_pair(p_card5, space1, u64))(input)
}

pub fn p07_compare(cx: &[P07Card; 5], cy: &[P07Card; 5]) -> std::cmp::Ordering {
    match P07Type::new(cx).cmp(&P07Type::new(cy)) {
        std::cmp::Ordering::Equal => cx.cmp(&cy),
        found => found,
    }
}

pub fn p07_compare_joke(cx: &[P07Card; 5], cy: &[P07Card; 5]) -> std::cmp::Ordering {
    match P07Type::new_joke(cx).cmp(&P07Type::new_joke(cy)) {
        std::cmp::Ordering::Equal => {
            let cx: Vec<_> = cx.iter().map(p07_joke_card).collect();
            let cy: Vec<_> = cy.iter().map(p07_joke_card).collect();
            cx.cmp(&cy)
        }
        found => found,
    }
}

pub fn p07() {
    let contents = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv07.txt").unwrap();

    let (_, mut cards) = p07_parse(&contents).unwrap();
    cards.sort_by(|(cx, _), (cy, _)| p07_compare(cx, cy));
    eprintln!("{:?}", cards);

    let mut winnings = 0;
    for (rank, &(_, bid)) in cards.iter().enumerate() {
        winnings += (rank as u64 + 1) * bid;
    }
    eprintln!("winnings: {winnings}");

    cards.sort_by(|(cx, _), (cy, _)| p07_compare_joke(cx, cy));
    let mut winnings = 0;
    for (rank, &(_, bid)) in cards.iter().enumerate() {
        winnings += (rank as u64 + 1) * bid;
    }
    eprintln!("winnings joke: {winnings}");
}

#[derive(Debug, Clone, Default)]
pub struct P08Nodes {
    node_to_name: Vec<String>,
    name_to_node: std::collections::HashMap<String, usize>,
    node: Vec<(usize, usize)>,
}

impl P08Nodes {
    fn insert_or_get(&mut self, name: &str) -> usize {
        if let Some(&idx) = self.name_to_node.get(name) {
            return idx;
        } else {
            let idx = self.node_to_name.len();
            self.node_to_name.push(name.to_owned());
            self.node.push((idx, idx));
            self.name_to_node.insert(name.to_owned(), idx);
            return idx;
        }
    }
}

pub fn p08_parse(input: &str) -> nom::IResult<&str, (Vec<char>, P08Nodes)> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, alphanumeric1, newline, satisfy, space0, space1, u64};
    use nom::combinator::map;
    use nom::multi::many1;
    use nom::sequence::{delimited, separated_pair, tuple};

    let (input, instruction) = many1(satisfy(|c| c == 'L' || c == 'R'))(input)?;
    let (input, _) = many1(newline)(input)?;
    let p_node = tuple((
        alphanumeric1,
        tuple((space0, tag("="), space0)),
        delimited(
            tag("("),
            separated_pair(
                alphanumeric1,
                tuple((space0, tag(","), space0)),
                alphanumeric1,
            ),
            tag(")"),
        ),
    ));
    let mut nodes: P08Nodes = Default::default();
    let (input, nodes_str) = nom::multi::separated_list0(newline, p_node)(input)?;

    for (from, _, (left, right)) in nodes_str {
        let from_idx = nodes.insert_or_get(from);
        let left_idx = nodes.insert_or_get(left);
        let right_idx = nodes.insert_or_get(right);
        nodes.node[from_idx] = (left_idx, right_idx);
    }
    Ok((input, (instruction, nodes)))
}

pub fn p08_search_z(nodes: &P08Nodes, instructions: &[char], mut curr: usize) -> Vec<usize> {
    let mut step_debug: std::collections::HashSet<(usize, usize)> = Default::default();
    let mut zzz: std::collections::HashSet<usize> = nodes
        .node_to_name
        .iter()
        .enumerate()
        .filter_map(|(idx, name)| if name.ends_with("Z") { Some(idx) } else { None })
        .collect();

    let mut steps = vec![];
    for (step, &ins) in instructions.iter().cycle().enumerate() {
        // eprintln!("curr = {curr} {}", nodes.node_to_name[curr]);
        if nodes.node_to_name[curr].ends_with("Z") {
            if zzz.contains(&curr) {
                steps.push(step);
                zzz.remove(&curr);
                if zzz.len() == 0 {
                    return steps;
                }
            }
        }

        let ins_idx = step % instructions.len();
        if step_debug.contains(&(curr, ins_idx)) {
            return steps; // 发现回环
        }
        step_debug.insert((curr, ins_idx));

        match ins {
            'L' => {
                curr = nodes.node[curr].0;
            }
            'R' => {
                curr = nodes.node[curr].1;
            }
            _ => unreachable!(),
        }
    }

    steps
}

pub fn p08_search(nodes: &P08Nodes, instructions: &[char], mut curr: usize, zzz: usize) -> usize {
    let mut step_debug: std::collections::HashSet<(usize, usize)> = Default::default();
    let mut step = 0;
    for (ins_idx, &ins) in instructions.iter().cycle().enumerate() {
        let ins_idx = ins_idx % instructions.len();
        eprintln!("-> {}@{}", nodes.node_to_name[curr], ins_idx);
        if curr == zzz {
            break;
        }
        assert!(!step_debug.contains(&(curr, ins_idx)), "Cycle founded.");
        step_debug.insert((curr, ins_idx));
        step += 1;
        match ins {
            'L' => {
                curr = nodes.node[curr].0;
            }
            'R' => {
                curr = nodes.node[curr].1;
            }
            _ => unreachable!(),
        }
    }
    step
}

pub fn p08() {
    let contents = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    // let contents = std::fs::read_to_string("./assets/adv2023/adv08.txt").unwrap();

    let (_, (instructions, nodes)) = p08_parse(&contents).unwrap();
    eprintln!("{:?}", nodes);

    let curr = nodes.name_to_node.get("AAA").cloned().unwrap();
    let zzz = nodes.name_to_node.get("ZZZ").cloned().unwrap();
    let step = p08_search(&nodes, &instructions, curr, zzz);
    eprintln!("Step: {}", step);

    ////////////////////////////////////////////////////////////////////////////
    let contents = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
    let contents = std::fs::read_to_string("./assets/adv2023/adv08.txt").unwrap();
    let (_, (instructions, nodes)) = p08_parse(&contents).unwrap();
    eprintln!("{:?}", nodes);
    let zzz: Vec<usize> = nodes
        .node_to_name
        .iter()
        .enumerate()
        .filter_map(|(idx, name)| if name.ends_with("Z") { Some(idx) } else { None })
        .collect();
    eprintln!("{:?}", zzz);
    let mut multiplier = vec![];
    for (curr, name) in nodes.node_to_name.iter().enumerate() {
        if name.ends_with("A") {
            eprintln!("{} -> {:?}", curr, zzz);
            let mut steps = p08_search_z(&nodes, &instructions, curr);
            eprintln!("Steps: {:?}", steps);
            multiplier.append(&mut steps);
        }
    }

    // Rust implementation: https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
    // or just go online: https://www.calculator.net/lcm-calculator.html?numberinputs=17141%2C18827%2C20513%2C12083%2C22199%2C19951&x=Calculate
    // eprintln!("{:?} => lcm = ???({})", multiplier, 20220305520997);
}

pub fn p09_parse(input: &str) -> nom::IResult<&str, Vec<Vec<i64>>> {
    nom::multi::separated_list0(
        nom::character::complete::newline,
        nom::multi::separated_list1(
            nom::character::complete::space1,
            nom::character::complete::i64,
        ),
    )(input)
}

pub fn p09() {
    let contents = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv09.txt").unwrap();

    let (_, history) = p09_parse(&contents).unwrap();
    eprintln!("{:?}", history);

    let mut sum1 = 0;
    for sequence in history.iter() {
        let mut sx = sequence.clone();
        let mut head = vec![];
        while sx.len() > 0 {
            let mut s0 = sx[0];
            head.push(s0);
            let mut is_nonzeroed = false;
            for s1 in std::mem::replace(&mut sx, Default::default())
                .into_iter()
                .skip(1)
            {
                let delta = s1 - s0;
                is_nonzeroed |= delta != 0;
                sx.push(delta);
                s0 = s1;
            }
            if !is_nonzeroed {
                break;
            }
        }
        // eprintln!("{:?} {:?}", sx, head);

        sx.push(0);
        for mut s0 in head.into_iter().rev() {
            for delta in
                std::iter::once(0).chain(std::mem::replace(&mut sx, Default::default()).into_iter())
            {
                sx.push(s0 + delta);
                s0 = s0 + delta;
            }
        }
        // eprintln!("{:?}", sx);
        // eprintln!();
        sum1 += sx.last().cloned().unwrap();
    }
    eprintln!("sum1 = {sum1}");

    let mut sum2 = 0;
    for sequence in history.iter() {
        let mut sx = sequence.clone();
        let mut head = vec![];
        while sx.len() > 0 {
            let mut s0 = sx[0];
            head.push(s0);
            let mut is_nonzeroed = false;
            for s1 in std::mem::replace(&mut sx, Default::default())
                .into_iter()
                .skip(1)
            {
                let delta = s1 - s0;
                is_nonzeroed |= delta != 0;
                sx.push(delta);
                s0 = s1;
            }
            if !is_nonzeroed {
                break;
            }
        }
        // eprintln!("{:?} {:?}", sx, head);

        let mut s0 = 0;
        for first in head.into_iter().rev() {
            s0 = first - s0;
        }
        // eprintln!("{:?}", s0);
        // eprintln!();
        sum2 += s0;
    }
    eprintln!("sum2 = {sum2}");
}

pub fn p10_parse(input: &str) -> nom::IResult<&str, Vec<Vec<char>>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{anychar, newline, satisfy, space0, space1, u64};
    use nom::combinator::map;
    use nom::multi::{many1, separated_list0, separated_list1};
    use nom::sequence::tuple;

    separated_list0(newline, many1(satisfy(|c| c != '\n')))(input)
}

pub fn p10() {
    let contents = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
    //     let contents = r#".....
    // .S-7.
    // .|.|.
    // .L-J.
    // ....."#;

    // let contents = std::fs::read_to_string("./assets/adv2023/adv10.txt").unwrap();

    let (_, nodes) = p10_parse(&contents).unwrap();
    // eprintln!("{:?}", nodes);

    let height = nodes.len();
    let width = nodes[0].len();
    let mut sx = 0;
    let mut sy = 0;
    for idy in 0..height {
        for idx in 0..width {
            if nodes[idy][idx] == 'S' {
                sx = idx;
                sy = idy;
            }
        }
    }
    assert_eq!(nodes[sy][sx], 'S');
    let mut distance = vec![vec![-1; width]; height];
    let mut points = vec![(sy as isize, sx as isize)];
    let mut curr = 0;

    while points.len() > 0 {
        for (idy, idx) in std::mem::replace(&mut points, Default::default()).into_iter() {
            if nodes[idy as usize][idx as usize] != '.' && distance[idy as usize][idx as usize] < 0
            {
                distance[idy as usize][idx as usize] = curr;
            }
            match nodes[idy as usize][idx as usize] {
                'S' => {
                    points.push((idy - 1, idx));
                    points.push((idy + 1, idx));

                    // points.push((idy, idx - 1));
                    // points.push((idy, idx + 1));
                }
                '|' => {
                    points.push((idy - 1, idx));
                    points.push((idy + 1, idx));
                }
                '-' => {
                    points.push((idy, idx - 1));
                    points.push((idy, idx + 1));
                }
                'F' => {
                    points.push((idy, idx + 1));
                    points.push((idy + 1, idx));
                }
                '7' => {
                    points.push((idy, idx - 1));
                    points.push((idy + 1, idx));
                }
                'J' => {
                    points.push((idy, idx - 1));
                    points.push((idy - 1, idx));
                }
                'L' => {
                    points.push((idy, idx + 1));
                    points.push((idy - 1, idx));
                }
                '.' => {}
                _ => unreachable!(),
            }
        }

        points = points
            .into_iter()
            .filter(|&(idy, idx)| {
                idy >= 0
                    && idy < height as isize
                    && idx >= 0
                    && idx < width as isize
                    && nodes[idy as usize][idx as usize] != '.'
                    && distance[idy as usize][idx as usize] == -1
            })
            .collect();

        curr += 1;
    }
    eprintln!("{:?}", distance);
    eprintln!("{}", curr - 1);

    // let mut max1 = isize::MIN;
    // for idy in 0..height {
    //     for idx in 0..width {
    //         max1 = max1.max(distance[idy][idx]);
    //     }
    // }
    // eprintln!("max1 = {max1}");

    //ref: https://www.geeksforgeeks.org/how-to-check-if-a-given-point-lies-inside-a-polygon/
    let contents = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
    let contents = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

    let contents = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;
    let contents = std::fs::read_to_string("./assets/adv2023/adv10.txt").unwrap();

    let (_, (height, width, nodes, graph)) = p10_parse_graph(&contents).unwrap();
    let mut count = 0;
    for idy in 0..height {
        for idx in 0..width {
            let node = nodes[idy][idx];
            if graph
                .iter()
                .any(|&edgenode| edgenode == (idy as isize, idx as isize))
            {
                continue;
            }
            // if !(node == '.' || node == '|' || node == '-') { continue; }

            // if (idy, idx) != (0, 2) { continue; }

            let is_inside = p10_check_point_in_polygon(idy as isize, idx as isize, &graph);

            // eprintln!("{idy}x{idx}: {}", is_inside);
            if is_inside {
                count += 1;
            }
        }
    }
    eprintln!("count = {count}");
}

// ref: https://observablehq.com/@tmcw/understanding-point-in-polygon
// 需要注意：这blog中关于边界上的点没有处理
pub fn p10_check_point_in_polygon(py: isize, px: isize, graph: &[(isize, isize)]) -> bool {
    let mut is_inside = false;
    let mut j = graph.len() - 1;
    for i in 0..graph.len() {
        let (yi, xi) = graph[i];
        let (yj, xj) = graph[j];

        // on the edge?
        if yi == py && yj == py && ((xi > px) != (xj > px) || (xi == px) || (xj == px)) {
            return false;
        }
        if xi == px && xj == px && ((yi > py) != (yj > py) || (yi == py) || (yj == py)) {
            return false;
        }

        // if ((yi > py) != (yj > py) || (yi == py) || (yj == py)) && (px - xi) * (yj - yi) == (py - yi) * (xj - xi) { return false; }

        let interset = ((yi > py) != (yj > py))
            && ((yi == yj && false)
                || (yi != yj
                    && (px as f64)
                        < xi as f64
                            + (xj as f64 - xi as f64) * (py as f64 - yi as f64)
                                / (yj as f64 - yi as f64)));
        if interset {
            is_inside = !is_inside;
        }
        // eprintln!("{i} {j} ({yi},{xi}) - ({yj}, {xj}): {py} {px} {interset}");

        j = i;
    }

    is_inside
}

pub fn p10_next(ny: isize, nx: isize, dir: char) -> ((isize, isize), (isize, isize)) {
    match dir {
        '|' => ((ny - 1, nx), (ny + 1, nx)),
        '-' => ((ny, nx - 1), (ny, nx + 1)),
        'F' => ((ny, nx + 1), (ny + 1, nx)),
        '7' => ((ny, nx - 1), (ny + 1, nx)),
        'J' => ((ny, nx - 1), (ny - 1, nx)),
        'L' => ((ny, nx + 1), (ny - 1, nx)),
        _ => unreachable!(),
    }
}

pub fn p10_parse_graph(
    input: &str,
) -> nom::IResult<&str, (usize, usize, Vec<Vec<char>>, Vec<(isize, isize)>)> {
    let (input, nodes) = p10_parse(input)?;
    let height = nodes.len();
    let width = nodes[0].len();
    let mut sx = 0;
    let mut sy = 0;
    for idy in 0..height {
        for idx in 0..width {
            if nodes[idy][idx] == 'S' {
                sx = idx;
                sy = idy;
            }
        }
    }
    assert_eq!(nodes[sy][sx], 'S');

    let mut walked: Vec<(isize, isize)> = vec![];
    let (mut cy, mut cx) = (sy as isize, sx as isize);
    let (mut py, mut px) = (sy as isize, sx as isize);
    while !((sy as isize, sx as isize) == (cy, cx) && walked.len() > 0) {
        if nodes[cy as usize][cx as usize] == 'S' {
            // adjust cy cx to right version
            for (oy, ox) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (ny, nx) = (cy + oy, cx + ox);
                if !(ny >= 0 && ny < height as isize && nx >= 0 && nx < width as isize) {
                    continue;
                }
                if matches!(nodes[ny as usize][nx as usize], '.' | 'S') {
                    continue;
                }
                let (p0, p1) = p10_next(ny, nx, nodes[ny as usize][nx as usize]);
                if p0 == (cy, cx) {
                    (cy, cx) = (ny, nx);
                    // walked.push((ny as usize, nx as usize));
                    break;
                } else if p1 == (cy, cx) {
                    (cy, cx) = (ny, nx);
                    // walked.push((ny as usize, nx as usize));
                    break;
                }
            }

            assert_ne!((cy, cx), (py, px));
            continue;
        }

        walked.push((cy, cx)); // need sanity 如果input正确，这一步就不出错，否则绝对出错
        let (p0, p1) = p10_next(cy, cx, nodes[cy as usize][cx as usize]);

        let cyx_origin = (cy, cx);
        (cy, cx) = if p0 == (py, px) { p1 } else { p0 };
        (py, px) = cyx_origin;
        // eprintln!(">>> {:?} {:?} {:?}", walked, (py, px), (cy, cx));
    }

    walked.push((sy as isize, sx as isize));
    walked = walked
        .into_iter()
        .filter(|&(yy, xx)| {
            nodes[yy as usize][xx as usize] != '-' && nodes[yy as usize][xx as usize] != '|'
        })
        .collect();
    eprintln!("{:?}", walked);

    Ok((input, (height, width, nodes, walked)))
}

pub fn p11_parse(input: &str) -> nom::IResult<&str, Vec<Vec<char>>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{anychar, newline, satisfy, space0, space1, u64};
    use nom::combinator::map;
    use nom::multi::{many1, separated_list0, separated_list1};
    use nom::sequence::tuple;

    separated_list0(newline, many1(satisfy(|c| c != '\n')))(input)
}

pub fn p11() {
    let contents = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv11.txt").unwrap();
    let (_, galaxy) = p11_parse(&contents).unwrap();
    // eprintln!("{:?}", galaxy);

    let height = galaxy.len();
    let width = galaxy[0].len();
    let mut points_origin = vec![];
    for idy in 0..height {
        for idx in 0..width {
            if galaxy[idy][idx] == '#' {
                points_origin.push((idx as isize, idy as isize))
            }
        }
    }
    points_origin.sort();

    let mut points = points_origin.clone();

    let x0 = points.iter().map(|&(x, y)| x).min().unwrap();
    let x1 = points.iter().map(|&(x, y)| x).max().unwrap() + 1;
    let y0 = points.iter().map(|&(x, y)| y).min().unwrap();
    let y1 = points.iter().map(|&(x, y)| y).max().unwrap() + 1;

    let mut xs_expand = vec![];
    for idx in x0..x1 {
        if (y0..y1).all(|idy| points.iter().find(|&&point| point == (idx, idy)).is_none()) {
            xs_expand.push(idx as isize);
        }
    }
    let mut ys_expand = vec![];
    for idy in y0..y1 {
        if (x0..x1).all(|idx| points.iter().find(|&&point| point == (idx, idy)).is_none()) {
            ys_expand.push(idy as isize);
        }
    }

    // eprintln!("Origin: {:?}{:?} {:?}", points, xs_expand, ys_expand);
    points = points
        .into_iter()
        .map(|(px, py)| {
            let ox = xs_expand.iter().filter(|&&x| px > x).count() as isize;
            let oy = ys_expand.iter().filter(|&&y| py > y).count() as isize;
            (px + ox, py + oy)
        })
        .collect();
    eprintln!("{:?}", points);
    eprintln!("sum1={}", p11_sum(&points));

    let mut points = points_origin.clone();
    let multiplier = 1000000;
    points = points
        .into_iter()
        .map(|(px, py)| {
            let ox = xs_expand.iter().filter(|&&x| px > x).count() as isize;
            let oy = ys_expand.iter().filter(|&&y| py > y).count() as isize;
            (px + ox * (multiplier - 1), py + oy * (multiplier - 1))
        })
        .collect();
    // eprintln!("{:?}", points);
    eprintln!("sum2={}", p11_sum(&points));
}

pub fn p11_sum(points: &[(isize, isize)]) -> isize {
    let mut sum1 = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = (points[i].0 - points[j].0).abs() + (points[i].1 - points[j].1).abs();
            sum1 += distance;
            // println!("{:5}: {:?} - {:?}", distance, points[i], points[j]);
        }
    }
    sum1
}

pub fn p12_parse(input: &str) -> nom::IResult<&str, Vec<(Vec<char>, Vec<u64>)>> {
    nom::multi::separated_list0(
        nom::character::complete::newline,
        nom::sequence::separated_pair(
            nom::multi::many1(nom::character::complete::one_of("?.#")),
            nom::character::complete::space1,
            nom::multi::separated_list1(
                nom::character::complete::char(','),
                nom::character::complete::u64,
            ),
        ),
    )(input)
}

pub fn p12() {
    let contents = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
    let contents = std::fs::read_to_string("./assets/adv2023/adv12.txt").unwrap();

    let (_, springs) = p12_parse(&contents).unwrap();
    eprintln!("{:?}", springs);

    let mut sum1 = 0;
    for (puzzle, constraint) in springs.iter() {
        let mut solved: std::collections::HashMap<(usize, usize, usize), usize> =
            Default::default();
        let count = p12_dfs(
            &puzzle,
            &constraint,
            0,
            1,
            constraint[0] as usize,
            &mut solved,
        );
        eprintln!("{:?}: {count} ", puzzle);
        sum1 += count;
    }
    eprintln!("sum1 = {sum1}");

    let mut sum2 = 0;
    for (puzzle, constraint) in springs.iter() {
        let mut solved: std::collections::HashMap<(usize, usize, usize), usize> =
            Default::default();
        let puzzle: Vec<_> = puzzle
            .iter()
            .cloned()
            .chain(std::iter::once('?'))
            .cycle()
            .take((puzzle.len() + 1) * 5 - 1)
            .collect();
        let constraint: Vec<_> = constraint
            .iter()
            .cloned()
            .cycle()
            .take(constraint.len() * 5)
            .collect();

        let count = p12_dfs(
            &puzzle,
            &constraint,
            0,
            1,
            constraint[0] as usize,
            &mut solved,
        );
        eprintln!(">>> {count} ");
        sum2 += count;
    }
    eprintln!("sum2 = {sum2}");
}

pub fn p12_dfs(
    puzzle: &[char],
    constraint: &[u64],
    px: usize,
    cx: usize,
    count: usize,
    solved: &mut std::collections::HashMap<(usize, usize, usize), usize>,
) -> usize {
    // if count == 0 && cx >= constraint.len() && px >= puzzle.len() {
    //     return 1;
    // }
    // eprintln!("{px} {cx} {count}");
    assert!(count >= 1);

    if let Some(&out) = solved.get(&(px, cx, count)) {
        return out;
    }

    let eat_sharp_from = |mut pxcursor: usize, mut count: usize| -> Option<Option<usize>> {
        while count > 0 {
            if pxcursor >= puzzle.len() {
                return None;
            }

            match puzzle[pxcursor] {
                '?' | '#' => {
                    count -= 1;
                    pxcursor += 1;
                }
                _ => {
                    return None;
                }
            }
        }

        if pxcursor < puzzle.len() {
            if matches!(puzzle[pxcursor], '.' | '?') {
                if cx < constraint.len() {
                    return Some(Some(pxcursor + 1));
                } else {
                    if puzzle[pxcursor + 1..].iter().all(|&ch| ch != '#') {
                        return Some(None);
                    } else {
                        return None;
                    }
                }
            } else {
                return None;
            }
        } else {
            if cx < constraint.len() {
                return None;
            } else {
                return Some(None);
            }
        }
    };

    let out = || -> usize {
        let mut pxcursor = px;
        // 先清除 .
        while pxcursor < puzzle.len() && puzzle[pxcursor] == '.' {
            pxcursor += 1;
        }
        if pxcursor >= puzzle.len() {
            return 0;
        }

        match puzzle[pxcursor] {
            '?' => {
                // cout not touched
                let not_eat = p12_dfs(puzzle, constraint, pxcursor + 1, cx, count, solved);
                let eat = match eat_sharp_from(pxcursor + 1, count - 1) {
                    Some(Some(pxcursor)) => p12_dfs(
                        puzzle,
                        constraint,
                        pxcursor,
                        cx + 1,
                        constraint[cx] as usize,
                        solved,
                    ),
                    Some(None) => 1,
                    _ => 0,
                };
                return not_eat + eat;
            }
            '#' => match eat_sharp_from(pxcursor + 1, count - 1) {
                Some(Some(pxcursor)) => p12_dfs(
                    puzzle,
                    constraint,
                    pxcursor,
                    cx + 1,
                    constraint[cx] as usize,
                    solved,
                ),
                Some(None) => 1,
                _ => 0,
            },
            _ => unreachable!(),
        }
    }();

    solved.insert((px, cx, count), out);
    out
}

pub type P13Matrix = Vec<Vec<u8>>;
pub fn p13_parse(input: &str) -> nom::IResult<&str, Vec<P13Matrix>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, newline, space0, space1, u64};
    use nom::combinator::{map, value};
    use nom::multi::many1;
    use nom::sequence::tuple;

    let rock = nom::branch::alt((value(0, char('.')), value(1, char('#'))));

    nom::multi::separated_list1(
        tuple((newline, newline)),
        nom::multi::separated_list1(newline, many1(rock)),
    )(input)
}

pub fn p13() {
    let contents = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv13.txt").unwrap();
    let (_, rocks) = p13_parse(&contents).unwrap();
    eprintln!("{:?}", rocks);

    let mut sum1 = 0;
    for rock in rocks.iter() {
        match p13_reflection(&rock) {
            Ok(col) => {
                eprintln!("Reflect vertical: {col}");
                sum1 += (col + 1) * 1;
            }
            Err(row) => {
                eprintln!("Reflect horizon: {row}");
                sum1 += (row + 1) * 100;
            }
        }
    }
    eprintln!("sum1 = {sum1}");

    let mut sum2 = 0;
    for rock in rocks.iter() {
        match p13_reflection_modified(&rock) {
            Ok(col) => {
                eprintln!("Reflect vertical: {col}");
                sum2 += (col + 1) * 1;
            }
            Err(row) => {
                eprintln!("Reflect horizon: {row}");
                sum2 += (row + 1) * 100;
            }
        }
    }
    eprintln!("sum1 = {sum2}");
}

pub fn p13_reflection(rock: &Vec<Vec<u8>>) -> Result<usize, usize> {
    let height = rock.len();
    let width = rock[0].len();

    for column in 0..(width - 1) {
        if (0..((column + 1).min(width - column - 1))).all(|offset| {
            rock.iter()
                .all(|row| row[column - offset] == row[column + offset + 1])
        }) {
            return Ok(column);
        }
    }

    for row in 0..(height - 1) {
        if (0..((row + 1).min(height - row - 1)))
            .all(|offset| rock[row - offset] == rock[row + offset + 1])
        {
            return Err(row);
        }
    }

    unreachable!()
}

pub fn p13_reflection_modified(rock: &Vec<Vec<u8>>) -> Result<usize, usize> {
    let height = rock.len();
    let width = rock[0].len();
    let ori = p13_reflection(rock);

    let mut mem: std::collections::HashMap<(bool, usize, usize), bool> = Default::default();
    // column: (true, 0, 1)

    let mut origin: Option<(bool, usize)> = None;
    for column in 0..(width - 1) {
        let mut is_reflect_all = true;
        for offset in 0..((column + 1).min(width - column - 1)) {
            let column_left = column - offset;
            let column_right = column + offset + 1;
            let is_reflect = rock.iter().all(|row| row[column_left] == row[column_right]);
            mem.insert((true, column_left, column_right), is_reflect);
            is_reflect_all &= is_reflect;
        }

        if is_reflect_all {
            assert!(origin.is_none(), "Duplicated: {:?} vs {}", origin, column);
            origin = Some((true, column));
        }
    }

    for row in 0..(height - 1) {
        let mut is_reflect_all = true;
        for offset in 0..((row + 1).min(height - row - 1)) {
            let row_top = row - offset;
            let row_bottom = row + offset + 1;
            let is_reflect = rock[row_top] == rock[row_bottom];
            mem.insert((false, row_top, row_bottom), is_reflect);
            is_reflect_all &= is_reflect;
        }
        if is_reflect_all {
            assert!(origin.is_none());
            origin = Some((false, row));
        }
    }
    // dbg!(&origin);

    let mut rock = rock.clone();
    for idy in 0..height {
        for idx in 0..width {
            rock[idy][idx] = 1 - rock[idy][idx];

            for column in 0..(width - 1) {
                if origin == Some((true, column)) {
                    continue;
                }
                let mut is_reflect_all = true;
                for offset in 0..((column + 1).min(width - column - 1)) {
                    let column_left = column - offset;
                    let column_right = column + offset + 1;

                    let is_reflect = if column_left == idx || column_right == idx {
                        rock.iter().all(|row| row[column_left] == row[column_right])
                    } else {
                        mem.get(&(true, column_left, column_right))
                            .cloned()
                            .unwrap()
                    };

                    is_reflect_all &= is_reflect;
                }

                if is_reflect_all {
                    return Ok(column);
                }
            }

            for row in 0..(height - 1) {
                if origin == Some((false, row)) {
                    continue;
                }

                let mut is_reflect_all = true;
                for offset in 0..((row + 1).min(height - row - 1)) {
                    let row_top = row - offset;
                    let row_bottom = row + offset + 1;

                    let is_reflect = if row_top == idy || row_bottom == idy {
                        rock[row_top] == rock[row_bottom]
                    } else {
                        mem.get(&(false, row_top, row_bottom)).cloned().unwrap()
                    };
                    is_reflect_all &= is_reflect;
                }
                if is_reflect_all {
                    return Err(row);
                }
            }

            rock[idy][idx] = 1 - rock[idy][idx];
        }
    }

    unreachable!();
}

pub fn p14_parse(input: &str) -> nom::IResult<&str, Vec<Vec<char>>> {
    nom::multi::separated_list0(
        nom::character::complete::newline,
        nom::multi::many1(nom::character::complete::one_of("O.#")),
    )(input)
}

pub fn p14() {
    let contents = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv14.txt").unwrap();

    let (_, rocks_origin) = p14_parse(&contents).unwrap();
    eprintln!("{:?}", rocks_origin);
    eprintln!();

    let mut rocks = rocks_origin.clone();
    let height = rocks.len();
    let width = rocks[0].len();
    for idx in 0..width {
        for idy in 1..height {
            let mut ndy = idy;
            while ndy > 0 && rocks[ndy][idx] == 'O' && rocks[ndy - 1][idx] == '.' {
                rocks[ndy][idx] = '.';
                rocks[ndy - 1][idx] = 'O';
                ndy -= 1;
            }
        }
    }
    eprintln!("{:?}", rocks);
    let mut sum1 = 0;
    for idy in 0..height {
        for idx in 0..width {
            if rocks[idy][idx] == 'O' {
                sum1 += height - idy;
            }
        }
    }
    eprintln!("sum1 = {sum1}");

    ////////////////////////////////////////////////////////////////////////////
    let mut rocks = rocks_origin.clone();
    let height = rocks.len();
    let width = rocks[0].len();
    let mut sum2_fin = 0;
    let mut hashed: Vec<(usize, Vec<Vec<char>>)> = Vec::with_capacity(1024);
    for cycle in 0..1000000000 {
        // north
        for idx in 0..width {
            for idy in 1..height {
                let mut ndy = idy;
                while ndy > 0 && rocks[ndy][idx] == 'O' && rocks[ndy - 1][idx] == '.' {
                    rocks[ndy][idx] = '.';
                    rocks[ndy - 1][idx] = 'O';
                    ndy -= 1;
                }
            }
        }
        // west
        for idy in 0..height {
            for idx in 1..width {
                let mut ndx = idx;
                while ndx > 0 && rocks[idy][ndx] == 'O' && rocks[idy][ndx - 1] == '.' {
                    rocks[idy][ndx] = '.';
                    rocks[idy][ndx - 1] = 'O';
                    ndx -= 1;
                }
            }
        }
        // south
        for idx in 0..width {
            for idy in (0..(height - 1)).rev() {
                let mut ndy = idy;
                while ndy + 1 < height && rocks[ndy][idx] == 'O' && rocks[ndy + 1][idx] == '.' {
                    rocks[ndy][idx] = '.';
                    rocks[ndy + 1][idx] = 'O';
                    ndy += 1;
                }
            }
        }
        // east
        for idy in 0..height {
            for idx in (0..(width - 1)).rev() {
                let mut ndx = idx;
                while ndx + 1 < width && rocks[idy][ndx] == 'O' && rocks[idy][ndx + 1] == '.' {
                    rocks[idy][ndx] = '.';
                    rocks[idy][ndx + 1] = 'O';
                    ndx += 1;
                }
            }
        }

        // eprintln!("{:?}", rocks);
        let mut sum2 = 0;
        for idy in 0..height {
            for idx in 0..width {
                if rocks[idy][idx] == 'O' {
                    sum2 += height - idy;
                }
            }
        }
        eprintln!("{sum2}@{cycle}");
        if hashed.iter().any(|(_, last)| last == &rocks) {
            eprintln!("Breaking Loop {sum2}@{cycle}\n: {:?}\n", rocks);
            hashed.push((sum2, rocks.clone()));
            break;
        }
        hashed.push((sum2, rocks.clone()));

        sum2_fin = sum2;
    }

    eprintln!("sum2_fin = {sum2_fin}");
    let cycled_len = hashed
        .iter()
        .rev()
        .skip(1)
        .take_while(|(_, last)| last != &hashed.last().unwrap().1)
        .count()
        + 1;
    let init_len = hashed.len() - 1 - cycled_len;
    eprintln!("{:?} = {} + {} + 1", hashed.len(), cycled_len, init_len);

    let searched = 1000000000;

    let ans = &hashed[init_len - 1 + (searched - init_len) % cycled_len];
    eprintln!("ans = {}", ans.0);
}

pub fn p15() {
    let contents = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv15.txt").unwrap();
    let words: nom::IResult<&str, Vec<Vec<char>>> = nom::multi::separated_list0(
        nom::character::complete::char(','),
        nom::multi::many1(nom::character::complete::satisfy(|ch| {
            ch != ',' && ch != '\n'
        })),
    )(&contents);
    let (_, words) = words.unwrap();

    // eprintln!("{:?}", words);
    let mut sum1 = 0;
    for word in words.iter() {
        let mut current = 0;
        for &w in word.iter() {
            current += w as isize;
            current *= 17;
            current %= 256;
        }
        eprintln!("{:?} {current}", word.iter().cloned().collect::<String>());
        sum1 += current;
    }
    eprintln!("sum1 = {sum1}");

    ////////////////////////////////////////////////////////////////////////////
    let words: nom::IResult<&str, Vec<(&str, Option<u64>)>> = nom::multi::separated_list0(
        nom::character::complete::char(','),
        nom::sequence::tuple((
            nom::character::complete::alpha1,
            nom::branch::alt((
                nom::combinator::value(None, nom::character::complete::char('-')),
                nom::sequence::preceded(
                    nom::character::complete::char('='),
                    nom::combinator::map(nom::character::complete::u64, |n| Some(n)),
                ),
            )),
        )),
    )(&contents);
    let (_, words) = words.unwrap();
    let mut map: Vec<Vec<(&str, u64)>> = vec![vec![]; 256];
    for (word, operation) in words.iter().cloned() {
        let mut current = 0;
        for w in word.chars() {
            current += w as isize;
            current *= 17;
            current %= 256;
        }
        // eprintln!("{:?} {}", word, current);

        let bucket = &mut map[current as usize];
        match operation {
            Some(nvalue) => {
                let mut is_found = false;
                for (idx, &mut (entry_name, ref mut entry_value)) in bucket.iter_mut().enumerate() {
                    if entry_name == word {
                        is_found = true;
                        *entry_value = nvalue;
                        break;
                    }
                }
                if !is_found {
                    bucket.push((word, nvalue));
                }
            }
            None => {
                for (idx, &(entry_name, _)) in bucket.iter().enumerate() {
                    if entry_name == word {
                        bucket.remove(idx);
                        break;
                    }
                }
            }
        }
    }

    let mut sum2 = 0;
    for (k, vs) in map.iter().enumerate() {
        if vs.len() > 0 {
            println!("{}: {:?}", k, vs);
        }

        for (local_id, &(_, focal_length)) in vs.iter().enumerate() {
            sum2 += (1 + k) * (local_id + 1) * (focal_length as usize);
        }
    }
    eprintln!("sum2 = {sum2}");
}

const RIGHT: u8 = 0b0001;
const LEFT: u8 = 0b0010;
const UP: u8 = 0b0100;
const DOWN: u8 = 0b1000;
pub fn p16() {
    let contents = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv16.txt").unwrap();

    let instructions: nom::IResult<&str, Vec<Vec<char>>> = nom::multi::separated_list1(
        nom::character::complete::newline,
        nom::multi::many1(nom::character::complete::one_of(r"|-/\.")),
    )(&contents);
    let (_, instructions) = instructions.unwrap();

    let sum1 = p16_energy(&instructions, 0, 0, RIGHT);
    eprintln!("sum1 = {sum1}");

    ////////////////////////////////////////////////////////////////////////////////
    let height = instructions.len();
    let width = instructions[0].len();
    let mut optima = usize::MIN;
    for idy in 0..height {
        optima = optima.max(p16_energy(&instructions, 0, idy, RIGHT));
        optima = optima.max(p16_energy(&instructions, width - 1, idy, LEFT));
    }
    for idx in 0..width {
        optima = optima.max(p16_energy(&instructions, idx, 0, DOWN));
        optima = optima.max(p16_energy(&instructions, idx, height - 1, UP));
    }
    eprintln!("optima = {optima}");
}

pub fn p16_energy(
    instructions: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    start_dir: u8,
) -> usize {
    let height = instructions.len();
    let width = instructions[0].len();
    let mut walked = vec![vec![0; width]; height];
    let mut cursor: Vec<(isize, isize, u8)> = vec![(start_x as isize, start_y as isize, start_dir)];

    while cursor.len() > 0 {
        for (x, y, dir) in std::mem::replace(&mut cursor, Default::default()).into_iter() {
            if !(x >= 0 && x < (width as isize) && y >= 0 && (y < height as isize)) {
                continue;
            }
            if (walked[y as usize][x as usize] & dir) > 0 {
                continue;
            }
            walked[y as usize][x as usize] |= dir;
            match instructions[y as usize][x as usize] {
                '\\' => match dir {
                    RIGHT => {
                        cursor.push((x, y + 1, DOWN));
                    }
                    LEFT => {
                        cursor.push((x, y - 1, UP));
                    }
                    UP => {
                        cursor.push((x - 1, y, LEFT));
                    }
                    DOWN => {
                        cursor.push((x + 1, y, RIGHT));
                    }
                    _ => {}
                },
                '/' => match dir {
                    RIGHT => {
                        cursor.push((x, y - 1, UP));
                    }
                    LEFT => {
                        cursor.push((x, y + 1, DOWN));
                    }
                    UP => {
                        cursor.push((x + 1, y, RIGHT));
                    }
                    DOWN => {
                        cursor.push((x - 1, y, LEFT));
                    }
                    _ => {}
                },
                '|' => match dir {
                    RIGHT | LEFT => {
                        cursor.push((x, y - 1, UP));
                        cursor.push((x, y + 1, DOWN));
                    }
                    UP => {
                        cursor.push((x, y - 1, dir));
                    }
                    DOWN => {
                        cursor.push((x, y + 1, dir));
                    }
                    _ => {}
                },
                '-' => match dir {
                    UP | DOWN => {
                        cursor.push((x - 1, y, LEFT));
                        cursor.push((x + 1, y, RIGHT));
                    }
                    RIGHT => {
                        cursor.push((x + 1, y, dir));
                    }
                    LEFT => {
                        cursor.push((x - 1, y, dir));
                    }
                    _ => {}
                },
                _ => match dir {
                    RIGHT => {
                        cursor.push((x + 1, y, dir));
                    }
                    LEFT => {
                        cursor.push((x - 1, y, dir));
                    }
                    UP => {
                        cursor.push((x, y - 1, dir));
                    }
                    DOWN => {
                        cursor.push((x, y + 1, dir));
                    }
                    _ => {}
                },
            }
        }
        // eprintln!("{:?}", cursor);
    }

    let mut sum1 = 0;
    for idy in 0..height {
        for idx in 0..width {
            if walked[idy][idx] > 0 {
                sum1 += 1;
            }
        }
    }

    sum1
}

#[allow(unused_mut)]
pub fn p17() {
    let contents = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
    let contents = r#"111111111111
999999999991
999999999991
999999999991
999999999991"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv17.txt").unwrap();

    let costs: nom::IResult<&str, Vec<Vec<i64>>> = nom::multi::separated_list1(
        nom::character::complete::newline,
        nom::multi::many1(nom::combinator::map(
            nom::character::complete::satisfy(|ch| ch.is_digit(10)),
            |ch| (ch as i64) - ('0' as i64),
        )),
    )(&contents);
    let (_, costs_origin) = costs.unwrap();

    ////////////////////////////////////////////////////////////////////////////
    let costs = costs_origin.clone();
    let height = costs.len();
    let width = costs[0].len();
    let mut cursor = vec![(0, 0, 0, LEFT | RIGHT | UP | DOWN)];
    let mut walked: std::collections::HashMap<(isize, isize, u8), i64> = Default::default();
    loop {
        cursor.sort();
        // eprintln!("{:?}", cursor);
        let (energy, x0, y0, dir) = cursor.remove(0);

        if let Some(&old_energy) = walked.get(&(x0, y0, dir)) {
            if old_energy <= energy {
                continue;
            }
        }

        walked.insert((x0, y0, dir), energy);

        if (x0, y0) == (width as isize - 1, height as isize - 1) {
            eprintln!("Smarllest energy found: {}", energy);
            break;
        }

        if (dir & LEFT) > 0 {
            let mut energy = energy;
            for offset in 0..3 {
                if x0 - offset - 1 >= 0 {
                    energy += costs[y0 as usize][(x0 - offset - 1) as usize];
                    cursor.push((energy, x0 - offset - 1, y0, UP | DOWN));
                }
            }
        }

        if (dir & RIGHT) > 0 {
            let mut energy = energy;
            for offset in 0..3 {
                if x0 + offset + 1 < width as isize {
                    energy += costs[y0 as usize][(x0 + offset + 1) as usize];
                    cursor.push((energy, x0 + offset + 1, y0, UP | DOWN));
                }
            }
        }

        if (dir & DOWN) > 0 {
            let mut energy = energy;
            for offset in 0..3 {
                if y0 + offset + 1 < height as isize {
                    energy += costs[(y0 + offset + 1) as usize][x0 as usize];
                    cursor.push((energy, x0, y0 + offset + 1, LEFT | RIGHT));
                }
            }
        }

        if (dir & UP) > 0 {
            let mut energy = energy;
            for offset in 0..3 {
                if y0 - offset - 1 >= 0 {
                    energy += costs[(y0 - offset - 1) as usize][x0 as usize];
                    cursor.push((energy, x0, y0 - offset - 1, LEFT | RIGHT));
                }
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    let costs = costs_origin.clone();
    let height = costs.len();
    let width = costs[0].len();

    let mut cursor = vec![(0, 0, 0, LEFT | RIGHT | UP | DOWN)];
    let mut walked: std::collections::HashMap<(isize, isize, u8), i64> = Default::default();
    loop {
        cursor.sort();
        // eprintln!("{:?}", cursor);
        let (energy, x0, y0, dir) = cursor.remove(0);

        if let Some(&old_energy) = walked.get(&(x0, y0, dir)) {
            if old_energy <= energy {
                continue;
            }
        }

        walked.insert((x0, y0, dir), energy);

        if (x0, y0) == (width as isize - 1, height as isize - 1) {
            eprintln!("Smarllest energy found: {}", energy);
            break;
        }

        if (dir & LEFT) > 0 {
            let mut energy = energy;
            for offset in 0..10 {
                if x0 - offset - 1 >= 0 {
                    energy += costs[y0 as usize][(x0 - offset - 1) as usize];
                    if offset >= 3 {
                        cursor.push((energy, x0 - offset - 1, y0, UP | DOWN));
                    }
                }
            }
        }

        if (dir & RIGHT) > 0 {
            let mut energy = energy;
            for offset in 0..10 {
                if x0 + offset + 1 < width as isize {
                    energy += costs[y0 as usize][(x0 + offset + 1) as usize];
                    if offset >= 3 {
                        cursor.push((energy, x0 + offset + 1, y0, UP | DOWN));
                    }
                }
            }
        }

        if (dir & DOWN) > 0 {
            let mut energy = energy;
            for offset in 0..10 {
                if y0 + offset + 1 < height as isize {
                    energy += costs[(y0 + offset + 1) as usize][x0 as usize];
                    if offset >= 3 {
                        cursor.push((energy, x0, y0 + offset + 1, LEFT | RIGHT));
                    }
                }
            }
        }

        if (dir & UP) > 0 {
            let mut energy = energy;
            for offset in 0..10 {
                if y0 - offset - 1 >= 0 {
                    energy += costs[(y0 - offset - 1) as usize][x0 as usize];
                    if offset >= 3 {
                        cursor.push((energy, x0, y0 - offset - 1, LEFT | RIGHT));
                    }
                }
            }
        }
    }
}

pub fn p18_parse(input: &str) -> nom::IResult<&str, Vec<(u8, i64, u64)>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, i64, newline, space0, space1, u64};
    use nom::combinator::{map, map_res, value};
    use nom::multi::many1;
    use nom::sequence::{delimited, terminated, tuple};

    let instruction = tuple((
        terminated(
            nom::branch::alt((
                value(RIGHT, char('R')),
                value(LEFT, char('L')),
                value(DOWN, char('D')),
                value(UP, char('U')),
            )),
            space1,
        ),
        terminated(i64, space1),
        delimited(
            tag("(#"),
            map_res(nom::character::complete::hex_digit1, |text| {
                u64::from_str_radix(text, 16)
            }),
            tag(")"),
        ),
    ));

    nom::multi::separated_list1(newline, instruction)(input)
}

fn p18_find_point_inside_polygon(instructions: impl Iterator<Item = (u8, i64)>) {
    let (mut sx, mut sy) = (0i64, 0i64);
    let mut pathes = vec![(sx, sy)];
    for (dir, width) in instructions {
        let (nx, ny) = match dir {
            RIGHT => (sx + width, sy),
            LEFT => (sx - width, sy),
            UP => (sx, sy - width),
            DOWN => (sx, sy + width),
            _ => unreachable!(),
        };

        pathes.push((nx, ny));
        (sx, sy) = (nx, ny);
    }

    assert_eq!(pathes.first(), pathes.last());
    pathes.pop().unwrap();

    let (mut sum1, mut sum2) = (0, 0);
    let xmin = pathes.iter().map(|&(sx, _)| sx).min().unwrap();
    let xmax = pathes.iter().map(|&(sx, _)| sx).max().unwrap();
    let ymin = pathes.iter().map(|&(_, sy)| sy).min().unwrap();
    let ymax = pathes.iter().map(|&(_, sy)| sy).max().unwrap();

    eprintln!("Search over ({xmin}, {ymin}) - ({xmax}, {ymax})");
    for idx in xmin..(xmax + 1) {
        for idy in ymin..(ymax + 1) {
            let mut is_on_edge = false;
            let mut pointx = pathes.len() - 1;
            let mut is_inside = false;
            for pointy in 0..pathes.len() {
                let (x0, y0) = pathes[pointx];
                let (x1, y1) = pathes[pointy];

                let is_on_xx =
                    x0 == x1 && x0 == idx && ((idy > y1) != (idy > y0) || idy == y1 || idy == y0);
                let is_on_yy =
                    y0 == y1 && y0 == idy && ((idx > x1) != (idx > x0) || idx == x1 || idx == x0);

                if is_on_xx || is_on_yy {
                    is_on_edge = true;
                    is_inside = false;
                    break;
                }

                // ref: https://stackoverflow.com/questions/47004208/should-point-on-the-edge-of-polygon-be-inside-polygon
                // 切点,对于浮点数，这里是完全忽略的，参看blog的算法 由于 interset 没有比较 <= 所以可以忽略
                // 同时这也说明 p10 的算法不对，但为啥成功了，我也不太清楚
                //
                // 一般情况，只考虑整数是会简单的，但这题如果考虑整数就必须考虑切点
                if y0 == y1 && (idy == y0 && idx < x0.min(x1)) {
                    let pointx_prev = if pointx > 0 {
                        pointx - 1
                    } else {
                        pathes.len() - 1
                    };
                    let pointy_next = if pointy + 1 < pathes.len() {
                        pointy + 1
                    } else {
                        0
                    };

                    let (_, y_prev) = pathes[pointx_prev];
                    let (_, y_next) = pathes[pointy_next];
                    // 共线的线已经被消除了
                    assert_ne!(y_prev, y0);
                    assert_ne!(y_next, y0);

                    if (y0 < y_prev) != (y0 < y_next) {
                        is_inside = !is_inside;
                    }
                }

                // 非切点
                if x0 == x1 && ((idy > y1) != (idy > y0) || idy == y1 || idy == y0) && idx < x0 {
                    is_inside = !is_inside;
                }

                pointx = pointy;
            }

            if is_on_edge {
                sum1 += 1;
                // eprintln!("{idy} x {idx}: Edge")
            } else if is_inside {
                sum2 += 1;
                // eprintln!("{idy} x {idx}: Inside")
            } else {
                // eprintln!("{idy} x {idx}: Outside")
            }
            assert!(!(is_inside && is_on_edge));
        }
    }
    eprintln!("{} = {sum1} + {sum2}", sum1 + sum2);
}

pub fn p18() {
    let contents = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv18.txt").unwrap();

    let (_, instructions) = p18_parse(&contents).unwrap();
    eprintln!("{:?}", instructions);

    p18_find_point_inside_polygon_fast(instructions.iter().map(|&(dir, width, _)| (dir, width)));

    p18_find_point_inside_polygon_fast(instructions.iter().map(|(_, _, rgb)| {
        let ss: Vec<_> = format!("{:0>6x}", rgb).chars().collect();
        let width: String = ss.iter().take(5).cloned().collect();
        // 0 means R, 1 means D, 2 means L, and 3 means U
        let dir: u8 = match ss[5] {
            '0' => RIGHT,
            '1' => DOWN,
            '2' => LEFT,
            '3' => UP,
            _ => unreachable!(),
        };
        (dir, i64::from_str_radix(&width, 16).unwrap())
    }));
}

fn p18_find_point_inside_polygon_fast(instructions: impl Iterator<Item = (u8, i64)>) {
    let (mut sx, mut sy) = (0i64, 0i64);
    let mut pathes = vec![(sx, sy)];
    for (dir, width) in instructions {
        let (nx, ny) = match dir {
            RIGHT => (sx + width, sy),
            LEFT => (sx - width, sy),
            UP => (sx, sy - width),
            DOWN => (sx, sy + width),
            _ => unreachable!(),
        };

        pathes.push((nx, ny));
        (sx, sy) = (nx, ny);
    }

    assert_eq!(pathes.first(), pathes.last());
    pathes.pop().unwrap();

    eprintln!("{:?}", pathes);
    let mut edges: Vec<(i64, Vec<(i64, i64, bool, bool, bool)>)> = Default::default();
    let mut pointx = pathes.len() - 1;
    let mut on_the_edge = 0;
    for pointy in 0..pathes.len() {
        let (x0, y0) = pathes[pointx];
        let (x1, y1) = pathes[pointy];
        pointx = pointy;
        if x0 != x1 {
            assert!((x1 - x0).abs() > 0, "{} vs {}", x1, x0);
            on_the_edge += (x1 - x0).abs();
            continue;
        }
        assert!((y1 - y0).abs() > 0, "{} vs {}", y1, y0);
        on_the_edge += (y0 - y1).abs();

        if let Some(idx) = edges.iter().position(|(xx, _)| *xx == x0) {
            edges[idx]
                .1
                .push((y0.min(y1), y0.max(y1), true, true, true));
        } else {
            edges.push((x0, vec![(y0.min(y1), y0.max(y1), true, true, true)]));
        }
    }

    edges.sort();
    println!("Edges: {:?}", edges);

    let mut area = 0;
    for idx in (0..edges.len()).rev() {
        let x0 = edges[idx].0;
        // eprintln!(">> {:?}", edges[idx]);
        for (y0, y1, is_rightmost_edge, is_y0_left_connect, is_y1_left_connect) in
            edges[idx].1.clone()
        {
            let mut segments = vec![(y0, y1)];
            let y0_connect = y0;
            let y1_connect = y1;

            for pidx in (0..idx).rev() {
                let px0 = edges[pidx].0;
                for &mut (
                    py0,
                    py1,
                    ref mut pis_rightmost_edge,
                    ref mut pis_y0_left_connect,
                    ref mut pis_y1_left_connect,
                ) in edges[pidx].1.iter_mut()
                {
                    for (y0, y1) in std::mem::replace(&mut segments, Default::default()).into_iter()
                    {
                        if py0 > y1 || py1 < y0 {
                            segments.push((y0, y1));
                            continue;
                        }

                        let intersect0 = y0.max(py0);
                        let intersect1 = y1.min(py1);
                        let mut intersect = intersect1 - intersect0 + 1;
                        if intersect0 == y0_connect && is_y0_left_connect {
                            intersect -= 1;
                        }
                        if intersect1 == y1_connect && is_y1_left_connect {
                            intersect -= 1;
                        }
                        if intersect > 0 {
                            let insides = (x0 - px0 - 1) * intersect;
                            if is_rightmost_edge {
                                area += insides;
                                // eprintln!("Segment Interaction ({y0}, {y1}) vs ({py0}, {py1}) {intersect1} - {intersect0} + 1 = {}: {intersect} *({x0} - {px0} - 1): {insides}", intersect1-intersect0 + 1);
                            }
                        }

                        if intersect > 0 {
                            *pis_rightmost_edge = !is_rightmost_edge;
                        }

                        if py1 == y0 {
                            *pis_y1_left_connect = !(is_y0_left_connect);
                        }
                        if py1 == y1 {
                            *pis_y1_left_connect = !(is_y1_left_connect);
                        }
                        if py0 == y0 {
                            *pis_y0_left_connect = !(is_y0_left_connect);
                        }
                        if py0 == y1 {
                            *pis_y0_left_connect = !(is_y1_left_connect);
                        }

                        if y0 < py0 {
                            segments.push((y0, py0 - 1));
                        }

                        if y1 > py1 {
                            segments.push((py1 + 1, y1));
                        }
                    }
                }
            }
        }
    }

    eprintln!("Found {area} + {on_the_edge} = {}", area + on_the_edge);
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum P19Destination {
    Accept,
    Reject,
    Named(String),
}

#[derive(Debug, Clone)]
pub enum P19Rule {
    Dest(P19Destination),
    Less(String, u64, P19Destination),
    Greater(String, u64, P19Destination),
}

pub type P19Part = std::collections::HashMap<String, u64>;

#[derive(Debug, Clone)]
pub struct P19Workflow {
    rules: std::collections::HashMap<String, Vec<P19Rule>>,
    parts: Vec<P19Part>,
}

impl P19Workflow {
    pub fn parse_des(input: &str) -> nom::IResult<&str, P19Destination> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{alphanumeric1, char, newline, one_of, space0, space1, u64};
        use nom::combinator::{map, value};
        use nom::multi::many1;
        use nom::sequence::tuple;

        nom::branch::alt((
            value(P19Destination::Accept, char('A')),
            value(P19Destination::Reject, char('R')),
            map(alphanumeric1, |name: &str| {
                P19Destination::Named(name.to_owned())
            }),
        ))(input)
    }

    pub fn parse_rule(input: &str) -> nom::IResult<&str, (String, Vec<P19Rule>)> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{alphanumeric1, char, newline, one_of, space0, space1, u64};
        use nom::combinator::{map, value};
        use nom::multi::many1;
        use nom::sequence::tuple;

        let (input, name) = alphanumeric1(input)?;
        let (input, _) = char('{')(input)?;

        let rulefn = map(
            tuple((alphanumeric1, one_of("><"), u64, char(':'), Self::parse_des)),
            |(name, op, other, _, des)| match op {
                '<' => P19Rule::Less(name.to_owned(), other, des),
                '>' => P19Rule::Greater(name.to_owned(), other, des),
                _ => unreachable!(),
            },
        );

        let (input, rules) = nom::multi::separated_list1(
            char(','),
            nom::branch::alt((rulefn, map(Self::parse_des, |des| P19Rule::Dest(des)))),
        )(input)?;
        let (input, _) = char('}')(input)?;
        Ok((input, (name.to_owned(), rules)))
    }

    pub fn parse_part(input: &str) -> nom::IResult<&str, P19Part> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{
            alphanumeric1, anychar, char, newline, one_of, space0, space1, u64,
        };
        use nom::combinator::{map, value};
        use nom::multi::many1;
        use nom::sequence::tuple;

        nom::sequence::delimited(
            char('{'),
            map(
                nom::multi::separated_list1(
                    char(','),
                    nom::sequence::separated_pair(anychar, char('='), u64),
                ),
                |vs| {
                    vs.into_iter()
                        .map(|(nm, value)| (nm.to_string(), value))
                        .collect()
                },
            ),
            char('}'),
        )(input)
    }

    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::character::complete::newline;
        use nom::multi::many1;

        let (input, (rules, parts)) = nom::sequence::separated_pair(
            nom::multi::separated_list1(newline, Self::parse_rule),
            many1(newline),
            nom::multi::separated_list1(newline, Self::parse_part),
        )(input)?;
        Ok((
            input,
            Self {
                rules: rules.into_iter().collect(),
                parts,
            },
        ))
    }
}

pub fn p19() {
    let contents = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv19.txt").unwrap();

    let (_, workflows) = P19Workflow::parse(&contents).unwrap();
    eprintln!("{:?}", workflows);

    let mut sum1 = 0;
    for part in workflows.parts.iter() {
        let mut name = "in".to_string();
        loop {
            let rules = workflows.rules.get(&name).unwrap();
            let mut des = P19Destination::Named(name.clone());
            for rule in rules.iter() {
                match rule {
                    P19Rule::Dest(inner) => {
                        des = inner.clone();
                        break;
                    }
                    P19Rule::Less(name, value, inner) => {
                        if part.get(name).unwrap() < value {
                            des = inner.clone();
                            break;
                        }
                    }
                    P19Rule::Greater(name, value, inner) => {
                        if part.get(name).unwrap() > value {
                            des = inner.clone();
                            break;
                        }
                    }
                }
            }
            assert_ne!(des, P19Destination::Named(name.clone()));

            match des {
                P19Destination::Accept => {
                    eprintln!("A: {:?}", part);
                    sum1 += part.values().sum::<u64>();
                    break;
                }
                P19Destination::Reject => {
                    eprintln!("R: {:?}", part);
                    break;
                }
                P19Destination::Named(inner) => {
                    name = inner;
                }
            }
        }
    }
    eprintln!("sum1 = {sum1}");

    const PMIN: u64 = 1;
    const PMAX: u64 = 4000;

    let mut sum2 = 0;
    let mut parts: Vec<(
        P19Destination,
        Vec<P19Destination>,
        std::collections::HashMap<String, (u64, u64)>,
    )> = vec![(P19Destination::Accept, vec![], Default::default())];
    while let Some((des_next, walked, candidates)) = parts.pop() {
        let mut guess = vec![];
        for (rule_name, rules) in workflows.rules.iter() {
            if walked
                .iter()
                .any(|old| old == &P19Destination::Named(rule_name.clone()))
            {
                continue;
            }
            let mut candidates = candidates.clone();

            for rule in rules.iter() {
                match rule {
                    P19Rule::Dest(inner) => {
                        if inner == &des_next {
                            let mut walked = walked.clone();
                            walked.push(des_next.clone());

                            guess.push((
                                P19Destination::Named(rule_name.clone()),
                                walked,
                                candidates.clone(),
                            ))
                        }
                    }
                    P19Rule::Less(name, value, inner) | P19Rule::Greater(name, value, inner) => {
                        let is_less = matches!(rule, P19Rule::Less(..));

                        let (x0, x1) = candidates.get(name).cloned().unwrap_or((PMIN, PMAX + 1));

                        // 进入该条件
                        if inner == &des_next {
                            let (x0, x1) =
                                candidates.get(name).cloned().unwrap_or((PMIN, PMAX + 1));
                            let mut orig = None;
                            if is_less {
                                if x1 <= *value {
                                    orig = Some((x0, x1));
                                } else if x0 < *value {
                                    orig = Some((x0, *value));
                                }
                            } else {
                                if x0 > *value {
                                    orig = Some((x0, x1));
                                } else if x1 > *value {
                                    orig = Some((*value + 1, x1));
                                }
                            }
                            if let Some((x0, x1)) = orig {
                                let mut walked = walked.clone();
                                walked.push(des_next.clone());
                                let mut candidates = candidates.clone();
                                candidates.insert(name.clone(), (x0, x1));
                                guess.push((
                                    P19Destination::Named(rule_name.clone()),
                                    walked,
                                    candidates,
                                ));
                            }
                        }

                        // 不进入该条件
                        let (x0, x1) = candidates.get(name).cloned().unwrap_or((PMIN, PMAX + 1));
                        let mut orig = None;

                        if is_less {
                            if x0 >= *value {
                                orig = Some((x0, x1));
                            } else if x1 > *value {
                                orig = Some((*value, x1));
                            }
                        } else {
                            if x1 <= *value {
                                orig = Some((x0, x1));
                            } else if x0 < *value + 1 {
                                orig = Some((x0, *value + 1));
                            }
                        }

                        if let Some((x0, x1)) = orig {
                            candidates.insert(name.clone(), (x0, x1));
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        for (des_curr, walked, candidates) in guess.into_iter() {
            if des_curr == P19Destination::Named("in".to_owned()) {
                let mut count = 1;
                for (x0, x1) in candidates.values() {
                    count *= x1 - x0;
                }
                count *= (PMAX + 1 - PMIN).pow(4 - candidates.len() as u32);
                sum2 += count;

                eprintln!("{} {:?} {:?} {:?}", count, des_curr, walked, candidates);
            } else {
                parts.push((des_curr, walked, candidates));
            }
        }

        // if parts.len() >= 10 { break; }
    }
    eprintln!("sum2 = {sum2}");
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum P20ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct P20Module {
    mtype: P20ModuleType,
    name: String,
    next: Vec<String>,

    flip_flop: bool,                                      // false means off
    conjunction: std::collections::HashMap<String, bool>, // false means low
}

impl P20Module {
    pub fn is_off(&self) -> bool {
        match self.mtype {
            P20ModuleType::Broadcaster => true,
            P20ModuleType::FlipFlop => !self.flip_flop,
            P20ModuleType::Conjunction => self.conjunction.values().all(|&v| !v),
        }
    }

    pub fn parse(input: &str) -> nom::IResult<&str, P20Module> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{alphanumeric1, newline, space0, space1, u64};
        use nom::combinator::map;
        use nom::multi::many1;
        use nom::sequence::tuple;

        let (input, prefix) = nom::branch::alt((tag("broadcaster"), tag("%"), tag("&")))(input)?;
        let (input, name, mtype) = match prefix {
            "broadcaster" => (input, prefix.to_string(), P20ModuleType::Broadcaster),
            "%" => {
                let (input, name) = alphanumeric1(input)?;
                (input, name.to_string(), P20ModuleType::FlipFlop)
            }
            "&" => {
                let (input, name) = alphanumeric1(input)?;
                (input, name.to_string(), P20ModuleType::Conjunction)
            }
            _ => unreachable!(),
        };
        let (input, _) = tuple((space1, tag("->"), space1))(input)?;
        let (input, next) =
            nom::multi::separated_list1(tuple((tag(","), space0)), alphanumeric1)(input)?;

        let next = next.into_iter().map(|s| s.to_string()).collect();
        Ok((
            input,
            Self {
                mtype,
                name,
                next,
                flip_flop: false,
                conjunction: Default::default(),
            },
        ))
    }
}

pub fn p20_display(modules: &std::collections::HashMap<String, P20Module>) {
    let mut item: Vec<String> = vec![];
    for module in modules.values() {
        item.push(format!(
            "{}({: >5}) -> {}",
            module.name,
            module.is_off(),
            module.next.join(","),
        ));
    }

    item.sort();
    for line in item {
        eprintln!("{}", line);
    }
}

pub fn p20_parse(input: &str) -> nom::IResult<&str, std::collections::HashMap<String, P20Module>> {
    let (input, modules) =
        nom::multi::separated_list1(nom::character::complete::newline, P20Module::parse)(input)?;
    let mut modules: std::collections::HashMap<String, P20Module> =
        modules.into_iter().map(|m| (m.name.clone(), m)).collect();
    let mut inserted = vec![];
    for module in modules.values() {
        if let P20ModuleType::Conjunction = module.mtype {
            for module_other in modules.values() {
                if module_other.next.iter().any(|nm| nm == &module.name) {
                    inserted.push((module.name.clone(), module_other.name.clone()));
                }
            }
        }
    }
    for (mname, oname) in inserted.into_iter() {
        modules
            .get_mut(&mname)
            .unwrap()
            .conjunction
            .insert(oname, false);
    }
    Ok((input, modules))
}

pub fn p20() {
    let contents = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

    let contents = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv20.txt").unwrap();

    let (_, modules_origin) = p20_parse(&contents).unwrap();
    let mut modules = modules_origin.clone();
    eprintln!("{:?}", modules);

    // false means low, true means high
    let mut patterns: Vec<(usize, usize)> = vec![];
    for idx in 0..1000 {
        patterns.push(p20_iter(&mut modules));

        let is_stable = modules.values().all(|m| match m.mtype {
            P20ModuleType::Broadcaster => true,
            P20ModuleType::FlipFlop => !m.flip_flop,
            P20ModuleType::Conjunction => m.conjunction.values().all(|&v| !v),
        });
        // eprintln!("{idx} Is stable: {}\n", is_stable);
        if is_stable {
            break;
        }
    }

    eprintln!("{:?}", patterns);

    let mut sum_low = 0;
    let mut sum_high = 0;
    for idx in 0..1000 {
        let (low, high) = patterns[idx % patterns.len()];
        sum_low += low;
        sum_high += high;
    }
    eprintln!("{} * {} = {}", sum_low, sum_high, sum_low * sum_high);

    eprintln!("################################################################################");
    let modules = modules_origin.clone();
    // see it online: http://graphonline.ru/en/?graph=NuUIMlanjwKMFYyB
    // 可以明显看到 rx 被 四个点决定，同时这四个点与 broadcster 共同将顶点分割为4块
    // 这题又不是那种宽泛应用的题目，必须要观察输入，将大图换小图
    // 而且这题目的输入恰好保证了，lcm的时候 rx 都有有效的信号，其实观察下来这个假设非常强
    // 因为完全有可能一个 rx 拿到了信号，但在第二个信号出现时，上一个信号过期了。这样rx永远
    // 无法得到有效信号
    for module in modules.values() {
        if module.next.iter().any(|nm| nm == "rx") {
            eprintln!("rx: {}", module.name);
        }
    }
    let mut ends = vec![];
    for module in modules.values() {
        if module.next.iter().any(|nm| nm == "jm") {
            ends.push(module.name.clone());
            eprintln!("jm: {}", module.name);
        }
    }
    let mut third_back: Vec<(String, std::collections::HashSet<String>)> = vec![];
    for end in ends.into_iter() {
        eprintln!("Locate for {}", end);
        let mut points = vec![end.clone()];
        let mut walked: std::collections::HashSet<String> = Default::default();
        while points.len() > 0 {
            for point in std::mem::replace(&mut points, Default::default()).into_iter() {
                if walked.contains(&point) {
                    continue;
                }
                walked.insert(point.clone());
                if point == "broadcaster" {
                    continue; // skip broadcaster
                }

                for module in modules.values() {
                    // if walked.contains(&module.name) { continue; }
                    if module.next.iter().any(|nm| nm == &point) {
                        // dbg!(&module.name);
                        points.push(module.name.clone());
                    }
                }
            }
        }
        eprintln!("{}: {:?} {}/{}", end, walked, walked.len(), modules.len());
        third_back.push((end, walked));
    }
    third_back.sort_by_key(|(key, _)| key.clone());

    eprintln!("Search over each block");
    for idx in 0..third_back.len() {
        // for idy in 0..third_back.len() {
        //     if idx == idy { continue; }
        //     let (left_name, left) = &third_back[idx];
        //     let (right_name, right) = &third_back[idy];
        //
        //     println!("{} vs {}: {:?}", left_name, right_name, left.intersection(right));
        // }

        let (subname, submodules) = &third_back[idx];
        let mut modules: std::collections::HashMap<String, P20Module> = modules
            .iter()
            .filter_map(|(k, v)| {
                if k == subname || submodules.contains(k) {
                    Some((k.clone(), (*v).clone()))
                } else {
                    None
                }
            })
            .collect();

        eprintln!(
            "Block broadcaster ->{:?}: {:?} x {}",
            subname,
            submodules,
            modules.len()
        );
        // let mut patterns: Vec<std::collections::HashMap<String, P20Module>> = vec![];
        let mut init = 0;
        let mut print_count = 0;
        for idx in 0..1_000_000_00 {
            let out = p20_iter_check(&mut modules, subname.clone());
            if out.iter().any(|&(_, v)| v) {
                if print_count <= 10 {
                    eprintln!(
                        "Got you {}({}): {:?} {:?}",
                        idx + 1,
                        idx - init,
                        subname,
                        out
                    );
                } else {
                    break;
                }
                init = idx;
                print_count += 1;
            }
        }
    }

    // ?? = 4079n = 3889n = 3851n == 4027n
    // ref: https://www.calculatorsoup.com/calculators/math/lcm.php
    let out = 246_006_621_493_687i64;
    eprintln!(
        "{}: {} {} {} {}",
        out,
        (out + 1000) % 4079,
        (out + 1000) % 3889,
        (out + 1000) % 3851,
        (out + 1000) % 4027
    );
}

pub fn p20_iter_check(
    modules: &mut std::collections::HashMap<String, P20Module>,
    end: String,
) -> Vec<(usize, bool)> {
    let mut pulses = vec![("broadcaster".to_string(), false, "button".to_string())];
    let mut out = vec![];
    let mut count = 0;
    while pulses.len() > 0 {
        count += 1;
        let (name, low_or_high, name_prev) = pulses.remove(0);
        // eprintln!("{} {} > {}", name_prev, low_or_high, name);
        assert!(!(name == "rx" && low_or_high == false), "stopped??");
        if let Some(module) = modules.get_mut(&name) {
            match module.mtype {
                P20ModuleType::Broadcaster => {
                    assert_eq!(low_or_high, false);
                    for nname in module.next.iter() {
                        pulses.push((nname.clone(), low_or_high, name.clone()));
                    }
                }
                P20ModuleType::FlipFlop => {
                    if !low_or_high {
                        let next_pulse = !module.flip_flop;
                        module.flip_flop = !module.flip_flop;

                        for nname in module.next.iter() {
                            pulses.push((nname.clone(), next_pulse, name.clone()));
                        }
                    }
                }
                P20ModuleType::Conjunction => {
                    *module.conjunction.get_mut(&name_prev).unwrap() = low_or_high;
                    let next_pulse = !module.conjunction.values().all(|&v| v);

                    if name == end {
                        // if out.is_some() {
                        //     assert_eq!(out.unwrap(), low_or_high);
                        // }
                        // eprintln!("Signal {}", next_pulse);
                        out.push((count, next_pulse));
                    }
                    for nname in module.next.iter() {
                        pulses.push((nname.clone(), next_pulse, name.clone()));
                    }
                }
            }
        }
    }

    out
}

pub fn p20_iter(modules: &mut std::collections::HashMap<String, P20Module>) -> (usize, usize) {
    let mut pulses = vec![("broadcaster".to_string(), false, "button".to_string())];
    let mut low = 0;
    let mut high = 0;
    while pulses.len() > 0 {
        let (name, low_or_high, name_prev) = pulses.remove(0);
        // eprintln!("{} {} > {}", name_prev, low_or_high, name);
        if low_or_high {
            high += 1;
        } else {
            low += 1;
        }
        assert!(!(name == "rx" && low_or_high == false), "stopped??");
        if let Some(module) = modules.get_mut(&name) {
            match module.mtype {
                P20ModuleType::Broadcaster => {
                    assert_eq!(low_or_high, false);
                    for nname in module.next.iter() {
                        pulses.push((nname.clone(), low_or_high, name.clone()));
                    }
                }
                P20ModuleType::FlipFlop => {
                    if !low_or_high {
                        let next_pulse = !module.flip_flop;
                        module.flip_flop = !module.flip_flop;

                        for nname in module.next.iter() {
                            pulses.push((nname.clone(), next_pulse, name.clone()));
                        }
                    }
                }
                P20ModuleType::Conjunction => {
                    *module.conjunction.get_mut(&name_prev).unwrap() = low_or_high;
                    let next_pulse = !module.conjunction.values().all(|&v| v);
                    for nname in module.next.iter() {
                        pulses.push((nname.clone(), next_pulse, name.clone()));
                    }
                }
            }
        }
    }

    (low, high)
}

pub fn p21() {
    let contents = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    // let contents = std::fs::read_to_string("./assets/adv2023/adv21.txt").unwrap();

    let garden: nom::IResult<&str, Vec<Vec<char>>> = nom::multi::separated_list1(
        nom::character::complete::newline,
        nom::multi::many1(nom::character::complete::one_of(".#S")),
    )(&contents);
    let (_, garden) = garden.unwrap();

    // eprintln!("{:?}", garden);

    let height = garden.len();
    let width = garden[0].len();

    let mut points = std::collections::HashSet::new();
    for idy in 0..height {
        for idx in 0..width {
            if garden[idy][idx] == 'S' {
                points.insert((idx as isize, idy as isize));
            }
        }
    }
    assert!(points.len() > 0);

    let step = if height > 20 { 64 } else { 6 };
    for _ in 0..step {
        for (px, py) in std::mem::replace(&mut points, Default::default()) {
            for (ox, oy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = px + ox;
                let ny = py + oy;
                if nx >= 0 && nx < (width as isize) && ny >= 0 && ny < (height as isize) {
                    if garden[ny as usize][nx as usize] != '#' {
                        points.insert((nx, ny));
                    }
                }
            }
        }
    }

    if step <= 12 {
        eprintln!("{}: {:?}", points.len(), points);
    } else {
        eprintln!("{}", points.len());
    }

    // println!("################################################################################");
    // // 最原始的方法，把所有的点都放进 HashSet 中，非常慢
    // let mut points: std::collections::HashMap<
    //     (isize, isize), std::collections::HashSet<(isize, isize)>
    // > = std::collections::HashMap::new();
    // for idy in 0..height {
    //     for idx in 0..width {
    //         if garden[idy][idx] == 'S' {
    //             points.insert((idx as isize, idy as isize), vec![(0, 0)].into_iter().collect());
    //         }
    //     }
    // }
    // assert!(points.len() == 1);
    // let step = 26501365;
    // let step = 50*2;
    // for idx in 1..(step + 1) {
    //     for ((px, py), scales) in std::mem::replace(&mut points, Default::default()).into_iter() {
    //         for (ox, oy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
    //             let mut sx = (px + ox) / (width as isize);
    //             let mut sy = (py + oy) / (width as isize);
    //             let mut nx = (px + ox) % (width as isize);
    //             let mut ny = (py + oy) % (width as isize);
    //
    //             if nx < 0 { nx += width as isize; sx -= 1; }
    //             if ny < 0 { ny += height as isize; sy -= 1; }
    //
    //             if garden[ny as usize][nx as usize] != '#' {
    //                 let target = points.entry((nx, ny)).or_default();
    //
    //                 for (osx, osy) in scales.iter().cloned() {
    //                     target.insert((osx+sx, osy+sy));
    //                 }
    //             }
    //         }
    //     }
    //
    //     if idx == 6 || idx == 10 || idx == 50 || idx == 100 || idx == 500 || idx == 1000 || idx == 5000 || idx == step {
    //         if idx != step { continue;}
    //         eprintln!("{}: {}", idx, points.iter().map(|(_, vs)| vs.len()).sum::<usize>());
    //         for ((px, py), scales) in points.iter() {
    //             if scales.len() > 1 {
    //                 let mut ss = scales.iter().collect::<Vec<_>>();
    //                 ss.sort();
    //                 eprintln!("{}x{}: {:?}", px, py, ss);
    //                 let xmin: isize = ss.iter().map(|&&(x, _)| x).min().unwrap();
    //                 let xmax = ss.iter().map(|&&(x, _)| x).max().unwrap();
    //                 let ymin = ss.iter().map(|&&(_, y)| y).min().unwrap();
    //                 let ymax = ss.iter().map(|&&(_, y)| y).max().unwrap();
    //
    //                 // 通过这个观察，矩阵逐步变成菱形
    //                 eprintln!("{xmin} - {xmax} vs {ymin} - {ymax}");
    //                 for idy in ymin..(ymax+1) {
    //                     for idx in xmin..(xmax+1) {
    //                         if ss.iter().find(|&(x, y)| (x, y) == (&idx, &idy)).is_some() {
    //                             eprint!("#");
    //                         } else {
    //                             eprint!(".");
    //                         }
    //                     }
    //                     eprintln!();
    //                 }
    //
    //                 eprintln!();
    //                 eprintln!();
    //             }
    //         }
    //         eprintln!("{:?}", points);
    //         eprintln!();
    //     }
    // }

    // println!("################################################################################");
    // // 我们只需要关注一个点上下左右是否还有 Rect，由此确定是否重复即可
    // // let dir = LEFT | RIGHT | UP | DOWN = 3
    // let empty_points = vec![vec![(0, 0); height]; width];
    // let mut points: Vec<Vec<(isize, isize)>> = empty_points.clone();
    // for idy in 0..height {
    //     for idx in 0..width {
    //         if garden[idy][idx] == 'S' {
    //             points[idx][idy] = (0, 1);
    //         }
    //     }
    // }
    //
    //
    // let step = 26501365;
    // for idx in 1..(step + 1) {
    //     let points_old = std::mem::replace(&mut points, empty_points.clone());
    //     for idy in 0..height {
    //         for idx in 0..width {
    //             for (ox, oy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
    //                 let mut nx = (idx as isize) + ox;
    //                 let mut ny = (idy as isize) + oy;
    //                 let is_inside = nx >= 0 && nx < (width as isize) && ny >= 0 && ny < (height as isize);
    //                 nx = nx % (width as isize);
    //                 ny = ny % (width as isize);
    //                 if nx < 0 { nx += width as isize; }
    //                 if ny < 0 { ny += height as isize; }
    //
    //                 if garden[ny as usize][nx as usize] != '#' {
    //                     let (old_scale, old_segment) = points[nx as usize][ny as usize];
    //                     let (new_scale, new_segment) = points_old[nx as usize][ny as usize];
    //                     points[nx as usize][ny as usize] = ();
    //                 }
    //             }
    //         }
    //     }
    //
    //
    //     if idx == 6 || idx == 10 || idx == 50 || idx == 100 || idx == 500 || idx == 1000 || idx == 5000 || idx == step
    //     {
    //         eprintln!("{}: size of points {}", idx, points.len());
    //         // if idx != step { continue;}
    //         // eprintln!( "{}: {}", idx, points.values() .sum::<isize>() );
    //     }
    // }
    // todo!();

    println!("################################################################################");
    // 观察发现，这些点都是由菱形区域构成，考虑用菱形区域代替点，减小 points 的大小
    // 但实践发现，step步想要迭代完成也是不行的，还是不够快
    //
    // 注意即使 outliers 不去更新也不行，还是无法迭代到 step 步数上
    let mut points: std::collections::HashMap<
        (isize, isize),
        (isize, std::collections::HashSet<(isize, isize)>),
    > = std::collections::HashMap::new();
    for idy in 0..height {
        for idx in 0..width {
            if garden[idy][idx] == 'S' {
                points.insert((idx as isize, idy as isize), (0, Default::default()));
            }
        }
    }
    assert!(points.len() == 1);
    let step = 26501365;
    // let step = 50 * 2;
    for idx in 1..(step + 1) {
        for ((px, py), (diamondw, outliers)) in
            std::mem::replace(&mut points, Default::default()).into_iter()
        {
            for (ox, oy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let mut nx = px + ox;
                let mut ny = py + oy;
                let is_inside =
                    nx >= 0 && nx < (width as isize) && ny >= 0 && ny < (height as isize);
                nx = nx % (width as isize);
                ny = ny % (width as isize);
                if nx < 0 {
                    nx += width as isize;
                }
                if ny < 0 {
                    ny += height as isize;
                }

                if garden[ny as usize][nx as usize] != '#' {
                    let mut new_width = diamondw;
                    let mut new_outliers = outliers.clone();

                    // 生成新的菱形区域
                    if !is_inside {
                        new_width += 1;
                        new_outliers = new_outliers
                            .into_iter()
                            .map(|(x, y)| (x + ox, y + oy))
                            .collect();

                        assert!(new_width >= 0);
                        for diamondx in 0..(new_width + 1) {
                            for diamondy in 0..(new_width - diamondx + 1) {
                                for (shiftx, shifty) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                                    let dx = diamondx * shiftx;
                                    let dy = diamondy * shifty;
                                    if (dy - oy).abs() + (dx - ox).abs() <= diamondw as isize {
                                        continue;
                                    }
                                    new_outliers.insert((dx, dy));
                                }
                            }
                        }
                    }

                    if let Some((old_width, old_outliers)) = points.get(&(nx, ny)).cloned() {
                        // 合并两个菱形区域
                        // points.insert((nx, ny), (diamondw, outliers));

                        let (min_width, max_width, min_outliers, mut max_outliers) =
                            if old_width <= new_width {
                                (old_width, new_width, old_outliers, new_outliers)
                            } else {
                                (new_width, old_width, new_outliers, old_outliers)
                            };
                        let merged_width = max_width;
                        let oldlen = max_outliers.len();
                        let old_outliers_cloned = max_outliers.clone();

                        max_outliers = max_outliers
                            .into_iter()
                            .filter(|&(xx, yy)| {
                                xx.abs() + yy.abs() > min_width || min_outliers.contains(&(xx, yy))
                            })
                            .collect();

                        // eprintln!("({}) {} -> {}", max_width, oldlen, max_outliers.len());
                        if max_width > 20 {
                            eprintln!(
                                "Two diamond merged {} vs {}: {} -> {}",
                                min_width,
                                max_width,
                                oldlen,
                                max_outliers.len()
                            );

                            for (diamondw, outliers) in [
                                (min_width, min_outliers.clone()),
                                (max_width, old_outliers_cloned),
                                (max_width, max_outliers.clone()),
                            ] {
                                for idy in (-diamondw)..(diamondw + 1) {
                                    for idx in (-diamondw)..(diamondw + 1) {
                                        if outliers
                                            .iter()
                                            .find(|&&(x, y)| (x, y) == (idx, idy))
                                            .is_some()
                                        {
                                            eprint!("#");
                                        } else {
                                            eprint!(".");
                                        }
                                    }
                                    eprintln!();
                                }
                            }

                            eprintln!();

                            // eprintln!("{:?}", max_outliers);
                            if max_width >= 12 {
                                todo!();
                            }
                        }

                        points.insert((nx, ny), (max_width, max_outliers));
                    } else {
                        points.insert((nx, ny), (new_width, new_outliers));
                    }
                }
            }
        }

        if idx == 6
            || idx == 10
            || idx == 50
            || idx == 100
            || idx == 500
            || idx == 1000
            || idx == 5000
            || idx == step
        {
            eprintln!("size of points {}", points.len());
            // if idx != step { continue;}
            eprintln!(
                "{}: {}",
                idx,
                points
                    .iter()
                    .map(|(_, (width, outliers))| {
                        1 + 2 * width * (width + 1) - (outliers.len() as isize)
                    })
                    .sum::<isize>()
            );
        }
    }
}

pub fn p22() {
    let contents = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv22.txt").unwrap();
    let bricks: nom::IResult<&str, Vec<((u64, u64, u64), (u64, u64, u64))>> =
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::sequence::separated_pair(
                nom::sequence::tuple((
                    nom::character::complete::u64,
                    nom::sequence::preceded(
                        nom::character::complete::char(','),
                        nom::character::complete::u64,
                    ),
                    nom::sequence::preceded(
                        nom::character::complete::char(','),
                        nom::character::complete::u64,
                    ),
                )),
                nom::character::complete::char('~'),
                nom::sequence::tuple((
                    nom::character::complete::u64,
                    nom::sequence::preceded(
                        nom::character::complete::char(','),
                        nom::character::complete::u64,
                    ),
                    nom::sequence::preceded(
                        nom::character::complete::char(','),
                        nom::character::complete::u64,
                    ),
                )),
            ),
        )(&contents);
    let (_, mut bricks) = bricks.unwrap();

    // bricks.sort_by_key(|&((_, _, z1), (_, _, z2))| z1.min(z2));

    // let ((bx1, by1, bz1), (bx2, by2, bz2)) = bricks[idx];
    // 的体积计算 -> (bx2-bx1+1) * (by2-by1+1) * (bz2-bz1+1)
    let mut is_movable = true;
    while is_movable {
        is_movable = false;
        for idx in 0..bricks.len() {
            let ((bx1, by1, bz1), (bx2, by2, bz2)) = bricks[idx];
            let bxmin = bx1.min(bx2);
            let bxmax = bx1.max(bx2);
            let bymin = by1.min(by2);
            let bymax = by1.max(by2);
            let bzmin = bz1.min(bz2);
            let bzmax = bz1.max(bz2);
            if bzmin == 1 {
                continue;
            }

            let mut zfinal = 1;
            // 所有在其下方的 可以挡住它的 rect 的 z的最大值
            // 这里 b 和 t 的命名反了，需要注意
            for jdx in 0..bricks.len() {
                if idx == jdx {
                    continue;
                }
                let ((tx1, ty1, tz1), (tx2, ty2, tz2)) = bricks[jdx];
                let txmin = tx1.min(tx2);
                let txmax = tx1.max(tx2);
                let tymin = ty1.min(ty2);
                let tymax = ty1.max(ty2);
                let tzmin = tz1.min(tz2);
                let tzmax = tz1.max(tz2);

                if tzmax >= bzmin {
                    continue;
                }

                let is_x_disjoin = bxmin > txmax || bxmax < txmin;
                let is_y_disjoin = bymin > tymax || bymax < tymin;
                // eprintln!("??? {} {}: {} vs {}", idx, jdx, is_x_disjoin, is_y_disjoin);

                if is_x_disjoin || is_y_disjoin {
                    // eprintln!("{} {}：不相交", idx, jdx);
                    // 不相交
                } else {
                    // eprintln!("{} {}：相交 {} {}", idx, jdx, is_x_disjoin, is_y_disjoin);
                    zfinal = zfinal.max(tzmax + 1);
                }
            }

            // eprintln!("WTF: {} {}", bzmin, zfinal);

            let shift = bzmin - zfinal;
            is_movable |= shift > 0;
            // eprintln!("Shift {} -> {} - {} = {}", idx, bzmin, zfinal, shift);
            bricks[idx] = ((bx1, by1, bz1 - shift), (bx2, by2, bz2 - shift));
        }
    }
    // eprintln!("{:?}", bricks);

    let mut supports: Vec<Vec<usize>> = vec![vec![]; bricks.len()];
    for idx in 0..bricks.len() {
        let ((bx1, by1, bz1), (bx2, by2, bz2)) = bricks[idx];
        let bxmin = bx1.min(bx2);
        let bxmax = bx1.max(bx2);
        let bymin = by1.min(by2);
        let bymax = by1.max(by2);
        let bzmin = bz1.min(bz2);
        let bzmax = bz1.max(bz2);
        // if bzmin == 1 { continue; }

        for jdx in 0..bricks.len() {
            if idx == jdx {
                continue;
            }
            let ((tx1, ty1, tz1), (tx2, ty2, tz2)) = bricks[jdx];
            let txmin = tx1.min(tx2);
            let txmax = tx1.max(tx2);
            let tymin = ty1.min(ty2);
            let tymax = ty1.max(ty2);
            let tzmin = tz1.min(tz2);
            let tzmax = tz1.max(tz2);

            if tzmin != bzmax + 1 {
                continue;
            }

            let is_x_disjoin = bxmin > txmax || bxmax < txmin;
            let is_y_disjoin = bymin > tymax || bymax < tymin;
            if is_x_disjoin || is_y_disjoin {
            } else {
                supports[idx].push(jdx);
            }
        }
    }

    eprintln!("{:?}", supports);
    let mut sum1 = 0;
    for (idx, supporting) in supports.iter().enumerate() {
        if supporting.iter().all(|&who_is_supported| {
            supports.iter().enumerate().any(|(jdx, next_supporting)| {
                jdx != idx && next_supporting.contains(&who_is_supported)
            })
        }) {
            // eprintln!("{} can be disintegrated", idx);
            sum1 += 1;
        }
    }
    eprintln!("sum1 = {sum1}");
    let grand: Vec<usize> = bricks
        .iter()
        .enumerate()
        .filter_map(
            |(idx, ((bx1, by1, bz1), (bx2, by2, bz2)))| {
                if bz1.min(bz2) <= &1 {
                    Some(idx)
                } else {
                    None
                }
            },
        )
        .collect();
    eprintln!("Floor: {:?}", grand);

    let mut sum2 = 0;
    for idx in 0..bricks.len() {
        // 检查连通性
        // 可以直接检查由初始点 （z=1的点都是初始点）出发能够到达的idx，不能到达则意味着收到了影响
        let mut is_walked = vec![false; bricks.len()];
        let mut points = grand.clone();
        while points.len() > 0 {
            let curr = points.pop().unwrap();
            if curr == idx {
                continue;
            }
            is_walked[curr] = true;

            for &next in supports[curr].iter() {
                if !is_walked[next] {
                    points.push(next);
                }
            }
        }
        let count = is_walked.into_iter().filter(|&p| p).count();
        let left = bricks.len() - count - 1;
        sum2 += left;
        eprintln!("{}: {} + {}", idx, count, left);
    }
    eprintln!("sum2 = {sum2}");
}
// [[], [0], [0], [1, 2], [1, 2], [3, 4], [5]] -> 被支撑
// 0        1       2       3    4    5    6
// [[1, 2], [3, 4], [3, 4], [5], [5], [6], []] -> 支撑
//
// 6 -> {}
// 5 -> {6}
// 4 -> {}
// 3 -> {}
// 2 -> {}
// 1 -> {}
// 0 -> {1, 2} -> {3, 4} -> { 5 } -> { 6 }

pub fn p23() {
    let contents = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

    let contents = std::fs::read_to_string("./assets/adv2023/adv23.txt").unwrap();

    let trails: nom::IResult<&str, Vec<Vec<char>>> = nom::multi::separated_list1(
        nom::character::complete::newline,
        nom::multi::many1(nom::character::complete::one_of(".#v>^<")),
    )(&contents);
    let (_, trails) = trails.unwrap();

    let height = trails.len();
    let width = trails[0].len();
    let start = (1, 0);
    let stop = ((width as isize) - 2, (height as isize) - 1);
    eprintln!("{} x {}: {:?} -> {:?}", height, width, start, stop);
    let mut walked = vec![];
    let out1 = p23_search_bfs(&trails, height, width, &mut walked, start, stop);
    eprintln!("out1 = {:?}", out1.unwrap() - 1);

    let reach_map = p23_search_map(&trails, height, width, start, stop);
    let mut walked = vec![];
    let out2 = p23_search_bfs_fast(&reach_map, (1, 0), stop, 0, &mut walked);
    eprintln!("out2 = {:?}", out2);
}

pub fn p23_search_bfs(
    trails: &Vec<Vec<char>>,
    height: usize,
    width: usize,
    walked: &mut Vec<(isize, isize)>,
    (sx, sy): (isize, isize),
    stop: (isize, isize),
) -> Option<usize> {
    walked.push((sx, sy));
    // eprintln!("{:?}", walked);
    if (sx, sy) == stop {
        return Some(walked.len());
    }
    let mut out = None;

    for (ox, oy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        match trails[sy as usize][sx as usize] {
            '^' if oy != -1 => {
                continue;
            }
            'v' if oy != 1 => {
                continue;
            }
            '<' if ox != -1 => {
                continue;
            }
            '>' if ox != 1 => {
                continue;
            }
            _ => {}
        }

        let nx = sx + ox;
        let ny = sy + oy;
        if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
            if trails[ny as usize][nx as usize] == '#' {
                continue;
            }
            if walked.contains(&(nx, ny)) {
                continue;
            }

            let orilen = walked.len();
            if let Some(len1) = p23_search_bfs(trails, height, width, walked, (nx, ny), stop) {
                if let Some(len2) = &mut out {
                    // eprintln!("Found: {}", len2);
                    *len2 = len1.max(*len2);
                } else {
                    out = Some(len1);
                }
            }
            walked.resize(orilen, (-1, -1));
        }
    }

    return out;
}

pub fn p23_search_map(
    trails: &Vec<Vec<char>>,
    height: usize,
    width: usize,
    (sx, sy): (isize, isize),
    stop: (isize, isize),
) -> std::collections::HashMap<(isize, isize), Vec<(isize, isize, usize)>> {
    let mut out: std::collections::HashMap<(isize, isize), Vec<(isize, isize, usize)>> =
        Default::default();
    let mut points = vec![(sx, sy)];
    while let Some((sx, sy)) = points.pop() {
        if out.contains_key(&(sx, sy)) {
            continue;
        }
        let mut reached = vec![];

        for (ox, oy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let mut nx = sx + ox;
            let mut ny = sy + oy;
            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                if trails[ny as usize][nx as usize] == '#' {
                    continue;
                }
                let mut walked = vec![(sx, sy)];

                loop {
                    walked.push((nx, ny));

                    let mut next = vec![];
                    for (ox, oy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let nnx = nx + ox;
                        let nny = ny + oy;
                        if nnx >= 0 && nnx < width as isize && nny >= 0 && nny < height as isize {
                            if trails[nny as usize][nnx as usize] == '#' {
                                continue;
                            }
                            if walked.contains(&(nnx, nny)) {
                                continue;
                            }
                            next.push((nnx, nny));
                        }
                    }
                    if next.len() == 1 {
                        let (nx_, ny_) = next.pop().unwrap();
                        nx = nx_;
                        ny = ny_;
                    } else {
                        reached.push((nx, ny, walked.len() - 1));
                        points.push((nx, ny));
                        break;
                    }
                }
            }
        }

        out.insert((sx, sy), reached);
    }

    // eprintln!("{:?}", out);
    // eprintln!("{:?}", out.get(&(1, 0)));
    // eprintln!("{:?}", out.get(&(3, 5)));
    // eprintln!("{:?}", out.get(&(5, 13)));
    // eprintln!("{:?}", out.get(&(13, 19)));
    // eprintln!("{:?}", out.get(&(19, 19)));

    out
}

pub fn p23_search_bfs_fast(
    trails: &std::collections::HashMap<(isize, isize), Vec<(isize, isize, usize)>>,
    (sx, sy): (isize, isize),
    stop: (isize, isize),
    weight: usize,
    walked: &mut Vec<(isize, isize)>,
) -> Option<usize> {
    walked.push((sx, sy));
    if (sx, sy) == stop {
        return Some(weight);
    }

    let mut out = None;
    if let Some(nextpoint) = trails.get(&(sx, sy)) {
        for &(nx, ny, step) in nextpoint.into_iter() {
            if walked.contains(&(nx, ny)) {
                continue;
            }
            let orilen = walked.len();
            if let Some(len1) = p23_search_bfs_fast(trails, (nx, ny), stop, weight + step, walked) {
                if let Some(len2) = &mut out {
                    // eprintln!("Found: {}", len2);
                    *len2 = len1.max(*len2);
                } else {
                    out = Some(len1);
                }
            }

            assert!(walked.len() > orilen);
            walked.resize(orilen, (-1, -1));
        }
    }

    out
}

pub fn p24_parse(input: &str) -> nom::IResult<&str, Vec<((i64, i64, i64), (i64, i64, i64))>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, i64, newline, space0, space1};
    use nom::combinator::map;
    use nom::multi::many1;
    use nom::sequence::{terminated, tuple};

    nom::multi::separated_list1(
        newline,
        nom::sequence::separated_pair(
            tuple((
                terminated(i64, tuple((char(','), space0))),
                terminated(i64, tuple((char(','), space0))),
                i64,
            )),
            nom::sequence::delimited(space0, tag("@"), space0),
            tuple((
                terminated(i64, tuple((char(','), space0))),
                terminated(i64, tuple((char(','), space0))),
                i64,
            )),
        ),
    )(input)
}

// (x - 19) / -2 = (y - 13) / 1 = (z -30) / -2 = t0
// (x - 19) / -2 = (y - 13) / 1 = (z -30) / -2 = t1

// (-2t0 + 19, t0 + 13, -2 t0 + 30)
// (-2t1 + 19, t1 + 13, -2 t1 + 30)
//
pub fn p24() {
    let contents = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

    // let contents = std::fs::read_to_string("./assets/adv2023/adv24.txt").unwrap();

    let (_, hailstones) = p24_parse(&contents).unwrap();

    let mut count = 0;
    let (axis_min, axis_max) = if hailstones.len() <= 20 {
        (7, 27)
    } else {
        (200000000000000i64, 400000000000000i64)
    };
    for idx in 0..hailstones.len() {
        for jdx in (idx + 1)..hailstones.len() {
            let ((x0, y0, z0), (vx0, vy0, vz0)) = hailstones[idx];
            let ((x1, y1, z1), (vx1, vy1, vz1)) = hailstones[jdx];

            // eprintln!("{:?} {:?}", (vx0, vy0, vz0), (vx1, vy1, vz1));
            eprint!("{:?} - {:?}: ", (x0, y0, z0), (x1, y1, z1));

            // WTF: number not engouh
            let (x0, y0, z0) = (x0 as i128, y0 as i128, z0 as i128);
            let (vx0, vy0, vz0) = (vx0 as i128, vy0 as i128, vz0 as i128);
            let (x1, y1, z1) = (x1 as i128, y1 as i128, z1 as i128);
            let (vx1, vy1, vz1) = (vx1 as i128, vy1 as i128, vz1 as i128);

            // (y - y0) * vx0 = (x - x0) * vy0;
            // (y - y1) * vx1 = (x - x1) * vy1;
            // y = ???;
            // x = ???;
            // manual calculate it not righ: ORZ
            // let x = ((y0 - y1) * vx1 * vx0 - vy0 * vx1 * x0 - vy1 * vx0 * x1) as f64 / ( vy1*vx0 - vy0 * vx1 ) as f64;
            // eprintln!("{}", x);
            // todo!();
            //

            // https://www.geeksforgeeks.org/program-for-point-of-intersection-of-two-lines/
            // (0, 0) - (-2, 1) => y = -0.5 x
            // (0, 0) - (vx0, vy0) => y = vy0 / vx0 * x
            // y * vx0 + x * (-vy0) = 0
            // x * (-vy0) + y * vx0 = x0 * (-vy0) + y0 * vx0
            // -- check one step still on this line:
            //
            // a0 = vx0 b0 = vy0 c0 = y0 * vy0 + x0 * vx0

            let a0 = -vy0;
            let b0 = vx0;
            let c0 = x0 * a0 + y0 * b0;
            let a1 = -vy1;
            let b1 = vx1;
            let c1 = x1 * a1 + y1 * b1;
            let determinant = a0 * b1 - a1 * b0;
            if determinant == 0 {
                eprintln!("NO interaction");
            } else {
                let x = (c0 * b1 - c1 * b0) as f64 / determinant as f64;
                let y = (a0 * c1 - a1 * c0) as f64 / determinant as f64;

                // (x0, y0) -> (x, y) & (x0+vx0, y0+vy0)
                // 检测 初始点是否在下一个点和交点的中心，如果在中心意味着交点无法抵达
                let is_xy0_inside = (vx0 == 0 || ((x0 > x0 + vx0) != (x0 as f64 > x)))
                    && (vy0 == 0 || ((y0 > y0 + vy0) != (y0 as f64 > y)));
                let is_xy1_inside = (vx1 == 0 || ((x1 > x1 + vx1) != (x1 as f64 > x)))
                    && (vy1 == 0 || ((y1 > y1 + vy1) != (y1 as f64 > y)));

                eprintln!(
                    "{:.5} x {:.5} => {} {}",
                    x, y, !is_xy0_inside, !is_xy1_inside
                );
                if !is_xy0_inside
                    && !is_xy1_inside
                    && x >= (axis_min as f64)
                    && x <= (axis_max as f64)
                    && y >= (axis_min as f64)
                    && y <= (axis_max as f64)
                {
                    count += 1;
                }
            }

            // eprintln!("{}x + {}y = {}", a0, b0, c0);
            // dbg!(a0 * 19 + b0 * 13);
            // dbg!(a0 as f64 * 14.33 + b0 as f64 * 15.333);
            //
            // eprintln!("{}x + {}y = {}", a1, b1, c1);
            // dbg!(a1 * 18 + b1 * 19);
            // dbg!(a1 as f64 * 14.33 + b1 as f64 * 15.333);
            // todo!();
        }
    }
    eprintln!("count = {count}");

    // 二维是为了欺骗我们，三维下，两条线如果相交有且只有一个交点，要么没有交点，这与二维有本质上的不同
    // 由此可以想想，三条三维空间不相交的曲线必定能确定一条共同相交的直线
    //
    // 或者简单情形，两对各两条线，分别在相交到 p0,p1，那么p0 与 p1 的连线则是唯一有可能同时与四条线相交
    // 的曲线，如果不是，那么则没有解。
    //
    // 所以现在问题来了，如何求三维空间内三条线的连线
    //
    // ref: https://math.stackexchange.com/questions/607348/line-intersecting-three-or-four-given-lines
    // 简而言之：如果我们碰巧（很容易）找到不共面的两条线，一条线上的点与另一条线会形成平面，那么剩下的线
    // 与该平面相交的点必须与第一个点共线
    // 更简而言之，三条线，任意一条线上的任意一点和另外两条线构成面，两条面相交得到共线，共线必须要其他
    // 所有线相交
    //
    //
    // 纯粹的数学题，跟 编程没有半毛钱关系。尝试计算的话，全是 代数运算，我尝试手算失败了
    //
    // (x - x0) / vx0 = (y - y0) / vy0 = (z - z0) / vz0
    let ((x0, y0, z0), (vx0, vy0, vz0)) = hailstones[0];
    let ((x1, y1, z1), (vx1, vy1, vz1)) = hailstones[1];

    let t0 = 1;
    let (px0, py0, pz0) = (x0 + vx0 * t0, y0 + vy0 * t0, z0 + vz0 * t0);
    let t0 = 2;
    let (px1, py1, pz1) = (x0 + vx0 * t0, y0 + vy0 * t0, z0 + vz0 * t0);
    eprintln!("{:?} - {:?}", (px0, py0, pz0), (px1, py1, pz1));

    let t0 = 1;
    let (qx0, qy0, qz0) = (x1 + vx1 * t0, y1 + vy1 * t0, z1 + vz1 * t0);
    let t0 = 2;
    let (qx1, qy1, qz1) = (x1 + vx1 * t0, y1 + vy1 * t0, z1 + vz1 * t0);
    eprintln!("{:?} - {:?}", (qx0, qy0, qz0), (qx1, qy1, qz1));

    // (a0, b0, c0, d0) = (??t0, ...)
}

// 298793064594510, 263093335773079, 376515029011499 @ -14, 59, -89
// 337669218265618, 218879644691028, 94758507317114 @ -60, -80, 532
// 226895012232653, 123459105544245, 213502199200302 @ 160, 303, 104
// 300501970298399, 314329447278049, 467279541164790 @ -32, 34, -91
// 388143561918162, 279175111435661, 308798092369721 @ -134, 61, 61

// Hailstone A: 19, 13, 30 @ -2, 1, -2
// Hailstone B: 18, 19, 22 @ -1, -1, -2
// Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).
//
// Hailstone A: 19, 13, 30 @ -2, 1, -2
// Hailstone B: 20, 25, 34 @ -2, -2, -4
// Hailstones' paths will cross inside the test area (at x=11.667, y=16.667).
//
// Hailstone A: 19, 13, 30 @ -2, 1, -2
// Hailstone B: 12, 31, 28 @ -1, -2, -1
// Hailstones' paths will cross outside the test area (at x=6.2, y=19.4).
//
// Hailstone A: 19, 13, 30 @ -2, 1, -2
// Hailstone B: 20, 19, 15 @ 1, -5, -3
// Hailstones' paths crossed in the past for hailstone A.
//
// Hailstone A: 18, 19, 22 @ -1, -1, -2
// Hailstone B: 20, 25, 34 @ -2, -2, -4
// Hailstones' paths are parallel; they never intersect.
//
// Hailstone A: 18, 19, 22 @ -1, -1, -2
// Hailstone B: 12, 31, 28 @ -1, -2, -1
// Hailstones' paths will cross outside the test area (at x=-6, y=-5).
//
// Hailstone A: 18, 19, 22 @ -1, -1, -2
// Hailstone B: 20, 19, 15 @ 1, -5, -3
// Hailstones' paths crossed in the past for both hailstones.
//
// Hailstone A: 20, 25, 34 @ -2, -2, -4
// Hailstone B: 12, 31, 28 @ -1, -2, -1
// Hailstones' paths will cross outside the test area (at x=-2, y=3).
//
// Hailstone A: 20, 25, 34 @ -2, -2, -4
// Hailstone B: 20, 19, 15 @ 1, -5, -3
// Hailstones' paths crossed in the past for hailstone B.
//
// Hailstone A: 12, 31, 28 @ -1, -2, -1
// Hailstone B: 20, 19, 15 @ 1, -5, -3
// Hailstones' paths crossed in the past for both hailstones.

pub fn p25() {
    let contents = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;

    // let contents = std::fs::read_to_string("./assets/adv2023/adv25.txt").unwrap();
    // 算法太难，不如用 gephi 直接看，选择 Yifan Hu 算法可以快速切割开
    // 直接导出成无向图即可
    eprintln!("766 + 741");
}
