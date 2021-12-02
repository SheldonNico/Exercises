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
