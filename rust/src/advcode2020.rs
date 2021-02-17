use std::collections::{HashSet, HashMap};
use std::convert::TryInto;

pub fn p01() {
    let nums: Vec<i32> = std::fs::read_to_string("./assets/adv01.txt").unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let nums_c: HashSet<i32> = nums.clone().into_iter().collect();

    static SUM: i32 = 2020;
    for num in nums.iter() {
        let target = SUM - num;
        if nums.contains(&target) {
            println!("{} - {}: {}", num, target, num * target);
            break;
        }
    }

    assert!(nums.len() >= 3);

    for num1_idx in 0..nums.len()-2 {
        let num1 = nums[num1_idx];
        for num2_idx in num1_idx..nums.len()-1 {
            let num2 = nums[num2_idx];
            let rest = SUM - num1 - num2;
            if nums_c.contains(&rest) {
                println!("{} - {} - {}: {}", num1, num2, rest, num1 * num2 * rest);
                break;
            }
        }
    }
}

#[derive(Debug)]
struct Password {
    freq_min: usize,
    freq_max: usize,
    freq_l: char,
    content: String,
}

impl Password {
    fn from_raw(s: &str) -> Self {
        let idx = s.find('-').unwrap(); let (t, s) = s.split_at(idx);
        let freq_min = t.parse().unwrap();

        let idx = s[1..].find(' ').unwrap(); let (t, s) = s[1..].split_at(idx);
        let freq_max = t.parse().unwrap();

        let idx = s[1..].find(':').unwrap(); let (t, s) = s[1..].split_at(idx);
        let freq_l = t.parse().unwrap();

        let content = s[2..].to_string();

        Self {
            freq_min, freq_max, freq_l, content
        }
    }

    fn is_valid(&self) -> bool {
        let freq = self.content.chars().filter(|c| *c == self.freq_l).count();
        freq >= self.freq_min && freq <= self.freq_max
    }

    fn is_valid_newpolicy(&self) -> bool {
        let cc: Vec<char> = self.content.chars().collect();
        (cc[self.freq_min-1] == self.freq_l) ^ (cc[self.freq_max-1] == self.freq_l)
    }
}

pub fn p02() {
    let database: Vec<Password> = std::fs::read_to_string("./assets/adv02.txt").unwrap()
        .lines()
        .map(Password::from_raw)
        .filter(Password::is_valid)
        .collect();
    println!("{:?}", database.len());

    let database: Vec<Password> = std::fs::read_to_string("./assets/adv02.txt").unwrap()
        .lines()
        .map(Password::from_raw)
        .filter(Password::is_valid_newpolicy)
        .collect();
    println!("{:?}", database.len());
}

pub fn p03() {
    let map: Vec<Vec<i32>> = std::fs::read_to_string("./assets/adv03.txt").unwrap()
        .lines()
        .map(
            |line| line.chars().map(|c| match c {
                '.' => 0,
                '#' => 1,
                _  => unreachable!(),
            }).collect()
        )
        .collect();

    let mut curr = 0; let mut count = 0;
    static STEP: usize = 3;
    for line in map.iter() {
        let pos = curr % line.len();
        if line[pos] == 1 {
            count += 1;
        }
        curr += STEP;
    }

    println!("pre: {}", count);

    let mut stack: u64 = 1;
    for (action_x, action_y) in vec![
        (3, 1),
        (1, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ].into_iter() {
        let mut curr_x = 0; let mut curr_y = 0; let mut count = 0;
        while curr_y < map.len() {
            let line = &map[curr_y];
            let pos = curr_x % line.len();
            if line[pos] == 1 {
                count += 1;
            }

            curr_x += action_x;
            curr_y += action_y;
        }
        stack *= count;
        println!("after: ({}, {}) {}", action_x, action_y, count);
    }
    println!("after: final {}", stack);
}

fn kv(input: &str) -> nom::IResult<&str, (String, String)> {
    let (input, k) = nom::bytes::complete::take_while(|c: char| c.is_alphanumeric())(input)?;
    let (input, _) = nom::bytes::complete::tag(":")(input)?;
    let (input, v) = nom::bytes::complete::take_while(|c: char| !c.is_whitespace())(input)?;
    Ok((input, (k.to_string(), v.to_string())))
}

pub fn kvs_from_raw(input: &str) -> nom::IResult<&str, HashMap<String, String>> {
    let mut parser = nom::multi::separated_list0(nom::bytes::complete::take_while1(char::is_whitespace), kv);
    let (left, res): (_, Vec<_>) = parser(input)?;
    Ok((left, res.into_iter().collect()))
}

#[derive(Debug)]
pub enum EclItem {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth
}
impl EclItem {
    pub fn from_raw(input: &str) -> Option<Self> {
        match input {
            "amb" => Some(Self::Amb),
            "blu" => Some(Self::Blu),
            "brn" => Some(Self::Brn),
            "gry" => Some(Self::Gry),
            "grn" => Some(Self::Grn),
            "hzl" => Some(Self::Hzl),
            "oth" => Some(Self::Oth),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: i32,
    hcl: Vec<char>,
    ecl: EclItem,
    pid: u32,
    cid: String,
}

fn byr_p<'a>(input: &'a str) -> nom::IResult<&'a str, u32> {
    let (input, _) = nom::bytes::complete::tag("byr:")(input)?;
    let (input, res) = nom::character::complete::digit1(input)?;
    if res.len() != 4 {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo)));
    }
    let res = res.parse().map_err(
        |_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo))
    )?;
    if !(res >= 1920 && res <= 2002) {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo)));
    }
    let (input, _) = nom::multi::many0(nom::character::complete::satisfy(|c| c == ' ' || c == '\n' || c == '\t'))(input)?;
    Ok((input, res))
}

fn iyr_p<'a>(input: &'a str) -> nom::IResult<&'a str, u32> {
    let (input, _) = nom::bytes::complete::tag("iyr:")(input)?;
    let (input, res) = nom::character::complete::digit1(input)?;
    if res.len() != 4 {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo)));
    }
    let res = res.parse().map_err(
        |_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo))
    )?;
    if !(res >= 2010 && res <= 2020) {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo)));
    }
    let (input, _) = nom::multi::many0(nom::character::complete::satisfy(|c| c == ' ' || c == '\n' || c == '\t'))(input)?;
    Ok((input, res))
}

fn eyr_p<'a>(input: &'a str) -> nom::IResult<&'a str, u32> {
    let (input, _) = nom::bytes::complete::tag("eyr:")(input)?;
    let (input, res) = nom::character::complete::digit1(input)?;
    if res.len() != 4 {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo)));
    }
    let res = res.parse().map_err(
        |_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo))
    )?;
    if !(res >= 2020 && res <= 2030) {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo)));
    }
    let (input, _) = nom::multi::many0(nom::character::complete::satisfy(|c| c == ' ' || c == '\n' || c == '\t'))(input)?;
    Ok((input, res))
}

fn hgt_p<'a>(input: &'a str) -> nom::IResult<&'a str, i32> {
    let (input, _) = nom::bytes::complete::tag("hgt:")(input)?;
    let (input, res) = nom::character::complete::digit1(input)?;
    let res: i32 = res.parse().map_err(
        |_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo))
    )?;
    let input = match nom::bytes::complete::tag::<_, _, nom::error::Error<&str>>("cm")(input) {
        Ok((input, _)) => {
            if !(res >= 150 && res <= 193) {
                return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo)));
            }
            input
        },
        _ => {
            let (input, _) = nom::bytes::complete::tag("in")(input)?;
            if !(res >= 59 && res <= 76) {
                return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo)));
            }
            input
        }
    };

    let (input, _) = nom::multi::many0(nom::character::complete::satisfy(|c| c == ' ' || c == '\n' || c == '\t'))(input)?;
    Ok((input, res))
}

fn hcl_p<'a>(input: &'a str) -> nom::IResult<&'a str, Vec<char>> {
    let (input, _) = nom::bytes::complete::tag("hcl:")(input)?;
    let (input, _) = nom::bytes::complete::tag("#")(input)?;
    let (input, res) = nom::multi::count(
        nom::character::complete::one_of("0123456789abcedf"), 6
    )(input)?;

    let (input, _) = nom::multi::many0(nom::character::complete::satisfy(|c| c == ' ' || c == '\n' || c == '\t'))(input)?;
    Ok((input, res.into_iter().collect()))
}

fn ecl_p<'a>(input: &'a str) -> nom::IResult<&'a str, EclItem> {
    let (input, _) = nom::bytes::complete::tag("ecl:")(input)?;
    let (input, res) = nom::branch::alt( (
            nom::bytes::complete::tag("amb"),
            nom::bytes::complete::tag("blu"),
            nom::bytes::complete::tag("brn"),
            nom::bytes::complete::tag("gry"),
            nom::bytes::complete::tag("grn"),
            nom::bytes::complete::tag("hzl"),
            nom::bytes::complete::tag("oth"),
    ) )(input)?;

    let (input, _) = nom::multi::many0(nom::character::complete::satisfy(|c| c == ' ' || c == '\n' || c == '\t'))(input)?;
    Ok((input, EclItem::from_raw(res).unwrap()))
}

fn pid_p<'a>(input: &'a str) -> nom::IResult<&'a str, u32> {
    let (input, _) = nom::bytes::complete::tag("pid:")(input)?;
    let (input, res) = nom::character::complete::digit1(input)?;
    if res.len() != 9 {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo)));
    }
    let res = res.parse().map_err(
        |_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo))
    )?;
    let (input, _) = nom::multi::many0(nom::character::complete::satisfy(|c| c == ' ' || c == '\n' || c == '\t'))(input)?;
    Ok((input, res))
}

fn cid_p<'a>(input: &'a str) -> nom::IResult<&'a str, String> {
    let parser = |input| {
        let (input, _) = nom::bytes::complete::tag("cid:")(input)?;
        let (input, res) = nom::character::complete::digit0(input)?;
        Ok((input, res))
    };
    let (input, res) = nom::branch::alt(
        (
            parser,
            nom::character::complete::alpha0
        )
    )(input)?;
    let (input, _) = nom::multi::many0(nom::character::complete::satisfy(|c| c == ' ' || c == '\n' || c == '\t'))(input)?;
    Ok((input, res.to_string()))
}

impl Passport {
    pub fn from_raw<'a >(input: &'a str) -> Result<Self, Box<dyn std::error::Error + 'a>> {
        let mut parser = nom::branch::permutation(
            (byr_p, iyr_p, eyr_p, hgt_p, hcl_p, ecl_p, pid_p, cid_p)
        );
        let (_, (byr, iyr, eyr, hgt, hcl, ecl, pid, cid)) = parser(input)?;
        Ok(Self {
            byr, iyr, eyr, hgt, hcl, ecl, pid, cid
        })
    }
}

pub fn p04() {
    let content = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;

    let passports: Vec<HashMap<String, String>> = std::fs::read_to_string("./assets/adv04.txt").unwrap()
        .split("\n\n")
        .filter_map(|s| kvs_from_raw(s).ok())
        .map(|r| r.1)
        .filter(|r| {
            r.contains_key("byr") &&
                r.contains_key("iyr") &&
                r.contains_key("eyr") &&
                r.contains_key("hgt") &&
                r.contains_key("hcl") &&
                r.contains_key("ecl") &&
                r.contains_key("pid")
        })
        .collect();
    println!("after {:?}", passports.len());

    let content = r#"byr:1937 iyr:2017 cid:14

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;

    let passports: Vec<Passport> = std::fs::read_to_string("./assets/adv04.txt").unwrap()
        .split("\n\n")
        .filter_map(|s| Passport::from_raw(s).ok())
        .collect();
    println!("after {:?}", passports.len());

}

fn get_set_id(input: &str) -> (u32, u32) {
    let cs: Vec<char> = input.chars().collect();
    assert_eq!(cs.len(), 7+3);
    let row: String = cs[..7].iter().map(|c| match c {
        'B' => '1',
        'F' => '0',
        _   => unreachable!(),
    }).collect();
    let col: String = cs[7..10].iter().map(|c| match c {
        'R' => '1',
        'L' => '0',
        _   => unreachable!(),
    }).collect();

    (u32::from_str_radix(&row, 2).unwrap(), u32::from_str_radix(&col, 2).unwrap())
}

pub fn p05() {
    let contents = r#"BFFFBBFRRR
BBFFBBFRLL
FFFBBBFRRR"#;
    let ids: Vec<(u32, u32)> = std::fs::read_to_string("./assets/adv05.txt").unwrap()
        .lines()
        .map(get_set_id)
        .collect();
    // eprintln!("{:?}", ids.iter().map(|(r, c)| (r, c, r*8+c)).collect::<Vec<_>>());
    eprintln!("{:?}", ids.iter().map(|(r, c)| r*8+c).max());

    let mut ids: Vec<(u32, u32, u32, String)> = std::fs::read_to_string("./assets/adv05.txt").unwrap()
        .lines()
        .map(|n| {
            let (r, c) = get_set_id(n);
            (r*8+c, r, c, n.to_string())
        })
        .collect();
    ids.sort();
    for (id_p, id_n) in ids.iter().zip(ids.iter().skip(1)) {
        if id_p.0 + 1 != id_n.0 {
            eprintln!("{:?} {:?}", id_p, id_n)
        }
    }
}

pub fn p06() {
    let contents = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    let res: Vec<_> = std::fs::read_to_string("./assets/adv06.txt").unwrap()
        .split("\n\n")
        .map(|line| {
            let mut stack: HashMap<char, usize> = HashMap::new();
            for c in line.chars() {
                if c.is_alphanumeric() {
                    *stack.entry(c).or_default() += 1;
                }
            }
            stack.len()
        })
        .collect();
    eprintln!("before: {:?}", res.into_iter().sum::<usize>());

    let res: Vec<_> = std::fs::read_to_string("./assets/adv06.txt").unwrap()
        .split("\n\n")
        .map(|answers| {
            let mut stack: HashMap<char, usize> = HashMap::new();
            let count = answers.lines().count();
            for line in answers.lines() {
                for c in line.chars() {
                    if c.is_alphanumeric() {
                        *stack.entry(c).or_default() += 1;
                    }
                }
            }

            stack.iter().filter(|(_, v)| **v == count).count()
        })
        .collect();
    eprintln!("after: {:?}", res.into_iter().sum::<usize>());
}

fn packet_nonempty_p(input: &str) -> nom::IResult<&str, (String, Vec<(usize, String)>)> {
    let (input, (color, _)) = nom::multi::many_till(
        nom::branch::alt((nom::character::complete::alpha1, nom::character::complete::space1)),
        nom::bytes::complete::tag("bags contain"),
    )(input)?;

    let color = color.join("").trim().to_string();
    let (input, _) = nom::character::complete::space1(input)?;

    let (input, res) = nom::multi::separated_list0(
        nom::bytes::complete::tag(", "),
        nom::sequence::tuple((
            nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse::<usize>()) ,
            nom::character::complete::space1,
            nom::multi::many_till(
                nom::branch::alt((nom::character::complete::alpha1, nom::character::complete::space1)),
                nom::bytes::complete::tag(" bag"),
            ),
            nom::multi::many0(nom::bytes::complete::tag("s")),
        )),
    )(input)?;

    let (input, _) = nom::bytes::complete::tag(".")(input)?;

    Ok((
            input,
            (color, res.into_iter().map(|(num, _, (colors, _), _)| (num, colors.join(""))).collect())
    ))
}

fn packet_empty_p(input: &str) -> nom::IResult<&str, String> {
    let (input, (color, _)) = nom::multi::many_till(
        nom::branch::alt((nom::character::complete::alpha1, nom::character::complete::space1)),
        nom::bytes::complete::tag(" bags contain"),
    )(input)?;
    let (input, _) = nom::bytes::complete::tag(" no other bags.")(input)?;

    Ok((input, color.join("").trim().to_string()))
}

fn packet_p(input: &str) -> nom::IResult<&str, (String, Vec<(usize, String)>)> {
    nom::branch::alt((
        nom::combinator::map(packet_empty_p, |s| (s, vec![])),
        packet_nonempty_p
    ))(input)
}

pub fn p07() {
    let contents = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

    let contents = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;

    let graph_c: HashMap<String, Vec<(usize, String)>> = std::fs::read_to_string("./assets/adv07.txt").unwrap()
        .lines()
        .map(|line| {
            packet_p(line).unwrap().1
        }).collect();

    eprintln!("{:?}", packet_p("dotted black bags contain no other bags."));
    eprintln!("{:?}", packet_p("vibrant plum bags contain 5 faded blue bags."));
    eprintln!("{:?}", packet_p("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."));

    eprintln!("{:?}", graph_c.len());

    let mut graph = graph_c.clone();
    let mut out = HashSet::new();
    out.insert("shiny gold".into());
    loop {
        let mut satisfy = vec![];
        for (color, contains) in graph.iter() {
            for (_, c) in (*contains).iter() {
                if out.contains(c) {
                    satisfy.push(color.to_string()); break;
                }
            }
        }
        if satisfy.len() == 0 { break; }

        for color_depr in satisfy.into_iter() {
            graph.remove(&color_depr).unwrap();
            out.insert(color_depr);
        }
    }
    eprintln!("{:?}", out.len() - 1);

    let mut answers = HashMap::new();
    eprintln!("{}", packet_dfs(&mut answers, "shiny gold", &graph)-1);
}

pub fn packet_dfs<'a, 'b>(answer: &'b mut HashMap<&'a str, usize>, color: &'a str, graph: &'a HashMap<String, Vec<(usize, String)>>) -> usize {
    if answer.contains_key(color) {
        *answer.get(color).unwrap()
    } else {
        let mut res = 1;
        for (count, c) in graph.get(color).unwrap().iter() {
            res += count * packet_dfs(answer, c, graph);
        }
        answer.insert(color, res);
        res
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

pub fn instruction_p(input: &str) -> nom::IResult<&str, Instruction> {
    let nop_p =
        nom::combinator::map_res(
            nom::sequence::tuple((
                    nom::bytes::complete::tag("nop"),
                    nom::character::complete::space1,
                    nom::branch::alt((
                            nom::combinator::value(1, nom::bytes::complete::tag("+")),
                            nom::combinator::value(-1, nom::bytes::complete::tag("-"))
                    )),
                    nom::character::complete::digit1
            )),
            |(_, _, s, v): (_, _, _, &str)| -> Result<Instruction, std::num::ParseIntError> {
                Ok(Instruction::Nop(s * v.parse::<isize>().unwrap()))
            }
        );
    let jmp_p =
        nom::combinator::map_res(
            nom::sequence::tuple((
                    nom::bytes::complete::tag("jmp"),
                    nom::character::complete::space1,
                    nom::branch::alt((
                            nom::combinator::value(1, nom::bytes::complete::tag("+")),
                            nom::combinator::value(-1, nom::bytes::complete::tag("-"))
                    )),
                    nom::character::complete::digit1
            )),
            // |(_, _, _, _)| Ok(Instruction::Nop)
            |(_, _, s, v): (_, _, _, &str)| -> Result<Instruction, std::num::ParseIntError> {
                Ok(Instruction::Jmp(s * v.parse::<isize>().unwrap()))
            }
        );
    let acc_p =
        nom::combinator::map_res(
            nom::sequence::tuple((
                    nom::bytes::complete::tag("acc"),
                    nom::character::complete::space1,
                    nom::branch::alt((
                            nom::combinator::value(1, nom::bytes::complete::tag("+")),
                            nom::combinator::value(-1, nom::bytes::complete::tag("-"))
                    )),
                    nom::character::complete::digit1
            )),
            // |(_, _, _, _)| Ok(Instruction::Nop)
            |(_, _, s, v): (_, _, _, &str)| -> Result<Instruction, std::num::ParseIntError> {
                Ok(Instruction::Acc(s * v.parse::<isize>().unwrap()))
            }
        );

    nom::branch::alt((
            nop_p,
            jmp_p,
            acc_p
    ))(input)
}

pub fn p08() {
    let contents = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

    let ins: Vec<_> =
        std::fs::read_to_string("./assets/adv08.txt").unwrap()
        // contents
        .lines()
        .map(|line| instruction_p(line).unwrap().1)
        .collect();

    let mut cursor = 0; let mut state = 0;
    let mut executed = HashSet::new();
    loop {
        if cursor >= ins.len() {
            println!("exit successfully: {} -> {}", cursor, state);
            break;
        }
        if executed.contains(&cursor) {
            println!("infinite loop: {} -> {}", cursor, state);
            break;
        }
        executed.insert(cursor);
        // eprintln!("--- {:?} {}", ins[cursor], cursor);
        match ins[cursor] {
            Instruction::Nop(_)  => { cursor += 1; },
            Instruction::Acc(d)  => { state += d; cursor += 1; }
            Instruction::Jmp(d) => { assert!(cursor >= (-d).max(0) as usize); cursor = (cursor as isize + d) as usize; }
        }
    }

    let graph: Vec<(usize, usize)> = ins
        .iter()
        .enumerate()
        .map(|(pos, i)| match i {
            Instruction::Acc(d) => { (pos + 1, pos + 1)},
            Instruction::Jmp(d) => { ((pos as isize + d) as usize, pos + 1) }
            Instruction::Nop(d) => { (pos + 1, (pos as isize + d) as usize) }
        }).collect();

    let graph = ins.clone();
    let mut cursor = 0; let mut state = 0;
    let mut failed = HashSet::new();
    let mut walked_post = HashSet::new();
    let mut walked_pre = HashSet::new();

    while cursor < graph.len() {
        if walked_pre.contains(&cursor) {
            println!("solution not exist: {} -> {}", cursor, state);
            break;
        }
        walked_pre.insert(cursor);
        match graph[cursor] {
            Instruction::Acc(d) => {
                state += d;
                cursor += 1;
            },
            Instruction::Jmp(d) => {
                match walk_from(cursor+1, state, &graph, &failed, &walked_pre, &mut walked_post) {
                    Some(v) => { state = v; break; },
                    _         => {
                        for f in walked_post.iter() { failed.insert(*f); }
                        walked_post.clear();
                        if d < 0 && cursor < -d as usize {
                            println!("solution not exist: {} -> {}", cursor, state);
                            break;
                        }
                        cursor = (cursor as isize + d) as usize;
                    }
                }
            },
            Instruction::Nop(d) => {
                if !(d < 0 && cursor < -d as usize) {
                    match walk_from((cursor as isize + d) as usize, state, &graph, &failed, &walked_pre, &mut walked_post) {
                        Some(v) => { state = v; break; },
                        _         => {
                            for f in walked_post.iter() { failed.insert(*f); }
                            walked_post.clear();
                            if d < 0 && cursor < -d as usize {
                                println!("solution not exist: {} -> {}", cursor, state);
                                break;
                            }
                            cursor = cursor + 1;
                        }
                    }
                }
            }
        }
    }


    eprintln!("last state: {}", state);

}

pub fn walk_from(
    mut cursor: usize, mut state: isize, graph: &Vec<Instruction>, failed: &HashSet<usize>, walked_pre: &HashSet<usize>,
    walked_post: &mut HashSet<usize>) -> Option<isize>
{
    loop {
        if walked_post.contains(&cursor) || failed.contains(&cursor) || walked_pre.contains(&cursor) {
            return None;
        }

        if cursor >= graph.len() {
            return Some(state);
        }

        walked_post.insert(cursor);
        match graph[cursor] {
            Instruction::Jmp(d) => {
                if d < 0 && cursor < -d as usize { return None; }
                cursor = (cursor as isize + d) as usize;
            },
            Instruction::Nop(_) => { cursor += 1; },
            Instruction::Acc(d) => { cursor += 1; state += d; },
        }
    }
}

pub fn p09() {
    let contents = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;

    let (numbers, preamble_len): (Vec<i64>, usize) = (
        std::fs::read_to_string("./assets/adv09.txt").unwrap()
        // contents
        .lines()
        .map(|s| s.parse().unwrap())
        .collect(),

        25
        // 5
    );

    let mut first_solution = 0;
    for idx in preamble_len..numbers.len() {
        let target = numbers[idx];
        let slice = &numbers[idx-preamble_len..idx];

        let mut is_targetable = false;
        for right in slice {
            if is_targetable { break; }
            let left = target - right;
            for s in slice.iter() {
                if left == *s && left != *right {
                    println!("{} = {} + {}", target, right, left);
                    is_targetable = true;
                    break;
                }
            }
        }

        if !is_targetable {
            first_solution = target;
            println!("{}", target);
            break;
        }
    }

    let mut cumsum = numbers.clone(); let mut idx = 0; let mut is_founded = false; let mut idx_founded = 0;
    loop {
        idx += 1;
        if cumsum.len() <= 1 || is_founded { break; }
        cumsum.pop();
        cumsum = cumsum.iter().zip(numbers[idx..].iter()).map(|(a, b)| *a + *b).collect();
        for (i, n) in cumsum.iter().enumerate() {
            if *n == first_solution {
                idx_founded = i;
                is_founded = true;
                break;
            }
        }
    }
    if is_founded {
        let slice = &numbers[idx_founded..idx_founded+idx];
        let min = slice.iter().min().unwrap();
        let max = slice.iter().max().unwrap();
        eprintln!("{:?} {} + {} = {}",  slice, min, max, min+max);
    }
}


pub fn p10() {
    let contents = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;

    let contents2 = r#"16
10
15
5
1
11
7
19
6
12
4"#;

    let mut numbers: Vec<i32> =
        std::fs::read_to_string("./assets/adv10.txt").unwrap()
        // contents
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    numbers.push(0);
    numbers.sort();
    let mut differences = vec![];
    for (pre, post) in numbers.iter().zip(numbers.iter().skip(1)) {
        assert!(post - pre <= 3);
        differences.push(post - pre);
    }
    let mut differences_freq: HashMap<i32, usize> = HashMap::new();
    for d in differences.iter() {
        *differences_freq.entry(*d).or_default() += 1;
    }

    println!("{} => {:?} {:?}", numbers.len(), numbers, differences_freq);
    let left = differences_freq.get(&3).unwrap()+1;
    let right = differences_freq.get(&1).unwrap();
    println!("{} * {} = {}", left, right, left*right);

    let target = *numbers.iter().max().unwrap();
    eprintln!("{}", target);

    let mut mem = HashMap::new();
    eprintln!("{}", search_sort_sequence(target, &numbers, &mut mem));
    // eprintln!("{:?}", mem);
}

pub fn search_sort_sequence(target: i32, seq: &Vec<i32>, mem: &mut HashMap<i32, usize>) -> usize {
    assert!(seq.len() > 1);
    if mem.contains_key(&target) { return *mem.get(&target).unwrap(); }
    if target < seq[0] || target > seq[seq.len()-1] { return 0; }
    if target == seq[0] { return 1; }
    if seq.binary_search(&target).is_err() { return 0; }

    let out = search_sort_sequence(target-1, seq, mem) +
        search_sort_sequence(target-2, seq, mem) +
        search_sort_sequence(target-3, seq, mem);
    mem.insert(target, out);
    return out;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Seat {
    Occupy,
    Empty,
    Floor
}

fn displya_seats(seats: &[Seat]) -> String {
    seats.iter().map(|s| match s {
        Seat::Occupy => '#',
        Seat::Empty  => 'L',
        Seat::Floor  => '.',
    }).collect()
}

pub fn p11() {
    let contents = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;

    let seats: Vec<Vec<Seat>> =
        // contents
        std::fs::read_to_string("./assets/adv11.txt").unwrap()
        .lines()
        .map(|line| line.chars().map(|c| match c {
            '#' => Seat::Occupy,
            'L' => Seat::Empty,
            '.' => Seat::Floor,
            _   => unreachable!()
        }).collect()).collect();

/*
 *     assert!(seats.len() > 0);
 *     let rows = seats.len() as isize; let cols = seats[0].len() as isize;
 *     let mut seats_m = seats.clone();
 *     eprintln!("{} * {}", rows, cols);
 *     loop {
 *         println!("-------------");
 *         println!("{}", seats_m.iter().map(|s| displya_seats(s)).collect::<Vec<_>>().join("\n"));
 *         println!("");
 *
 *         let mut modified = 0;
 *         let mut seats_n = seats_m.clone();
 *         let mut count_occupied = 0;
 *         for i in 0..rows {
 *             for j in 0..cols {
 *                 let mut occupied = 0;
 *                 for (ic, jc) in [
 *                     (0, 1), (0, -1), (1, 0), (-1, 0),
 *                     (1, 1), (1, -1), (-1, 1), (-1, -1),
 *                 ].iter() {
 *                     if 0 > i + ic || 0 > j + jc || ic + i >= rows || jc + j >= cols {
 *                     } else {
 *                         if let Seat::Occupy = seats_m[(i + ic) as usize][(j + jc) as usize] {
 *                             occupied += 1;
 *                         }
 *                     }
 *                 }
 *
 *                 // eprintln!("{} {} - {} -- {:?}", i, j, occupied, seats_m[i as usize][j as usize]);
 *                 if seats_m[i as usize][j as usize] == Seat::Empty && occupied == 0 {
 *                     modified += 1;
 *                     seats_n[i as usize][j as usize] = Seat::Occupy;
 *                 } else if seats_m[i as usize][j as usize] == Seat::Occupy && occupied >= 4 {
 *                     modified += 1;
 *                     seats_n[i as usize][j as usize] = Seat::Empty;
 *                 }
 *
 *                 if seats_n[i as usize][j as usize] == Seat::Occupy {
 *                     count_occupied += 1;
 *                 }
 *             }
 *         }
 *         println!("End: {}", count_occupied);
 *         seats_m = seats_n;
 *         if modified == 0 { break; }
 *     }
 */

    let rows = seats.len() as isize; let cols = seats[0].len() as isize;
    let mut seats_m = seats.clone();
    eprintln!("{} * {}", rows, cols);
    loop {
        println!("-------------");
        println!("{}", seats_m.iter().map(|s| displya_seats(s)).collect::<Vec<_>>().join("\n"));
        println!("");

        let mut modified = 0;
        let mut seats_n = seats_m.clone();
        let mut count_occupied = 0;
        for i in 0..rows {
            for j in 0..cols {

                let mut occupied = 0;
                for (ic, jc) in [
                    (0, 1), (0, -1), (1, 0), (-1, 0),
                    (1, 1), (1, -1), (-1, 1), (-1, -1),
                ].iter() {
                    let mut ix = i; let mut jx = j;
                    while 0 <= ix + ic && 0 <= jx + jc && ic + ix < rows && jc + jx < cols {
                        ix = ix +ic; jx += jc;
                        match seats_m[ix as usize][jx as usize] {
                            Seat::Occupy => { occupied += 1; break; },
                            Seat::Empty  => { break; },
                            _            => {  },
                        }
                    }
                }

                // eprintln!("{} {} - {} -- {:?}", i, j, occupied, seats_m[i as usize][j as usize]);
                if seats_m[i as usize][j as usize] == Seat::Empty && occupied == 0 {
                    modified += 1;
                    seats_n[i as usize][j as usize] = Seat::Occupy;
                } else if seats_m[i as usize][j as usize] == Seat::Occupy && occupied >= 5 {
                    modified += 1;
                    seats_n[i as usize][j as usize] = Seat::Empty;
                }

                if seats_n[i as usize][j as usize] == Seat::Occupy {
                    count_occupied += 1;
                }
            }
        }
        println!("End: {}", count_occupied);
        seats_m = seats_n;
        if modified == 0 { break; }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MoveDir {
    N(usize),
    S(usize),
    E(usize),
    W(usize),
    L(usize),
    R(usize),
    F(usize),
}

type Location = (f64, f64, f64); // (x, y, direction)

pub fn movedir_p(input: &str) -> nom::IResult<&str, MoveDir> {
    let (input, c) = nom::character::complete::anychar(input)?;
    let (input, count) = nom::combinator::map_res(
        nom::character::complete::digit1,
        |s: &str| s.parse::<usize>()
    ) (input)?;
    match c {
        'N' => Ok((input, MoveDir::N(count))),
        'S' => Ok((input, MoveDir::S(count))),
        'E' => Ok((input, MoveDir::E(count))),
        'W' => Ok((input, MoveDir::W(count))),
        'L' => Ok((input, MoveDir::L(count))),
        'R' => Ok((input, MoveDir::R(count))),
        'F' => Ok((input, MoveDir::F(count))),
        _ => unreachable!()
    }
}

fn move_toward((mut x, mut y, mut face): Location, dir: MoveDir) -> Location {
    match dir {
        MoveDir::N(step) => { y += step as f64; },
        MoveDir::S(step) => { y -= step as f64; },
        MoveDir::E(step) => { x += step as f64; },
        MoveDir::W(step) => { x -= step as f64; },
        MoveDir::R(angle) => {
            face -= angle as f64;
            face %= 360.;
        },
        MoveDir::L(angle) => {
            face += angle as f64;
            face %= 360.;
        },
        MoveDir::F(step)  => {
            x += (std::f64::consts::PI * face / 180.).cos() * step as f64;
            y += (std::f64::consts::PI * face / 180.).sin() * step as f64;
        },
    }
    (x, y, face)
}

type LocationR = (isize, isize, (isize, isize)); // (x, y, direction)
fn move_toward_r((mut x, mut y, (mut xdir, mut ydir)): LocationR, dir: MoveDir) -> LocationR {
    match dir {
        MoveDir::N(step) => { ydir += step as isize; },
        MoveDir::S(step) => { ydir -= step as isize; },
        MoveDir::E(step) => { xdir += step as isize; },
        MoveDir::W(step) => { xdir -= step as isize; },
        MoveDir::L(90) | MoveDir::R(270) => {
            let tmp = xdir; xdir = -ydir; ydir = tmp;
        },
        MoveDir::L(180) | MoveDir::R(180) => {
            xdir = -xdir; ydir = -ydir;
        },
        MoveDir::L(270) | MoveDir::R(90) => {
            let tmp = xdir; xdir = ydir; ydir = -tmp;
        },

        MoveDir::F(step)  => {
            x += xdir * step as isize; y += ydir * step as isize;
        },
        _ => {  }
    }
    (x, y, (xdir, ydir))
}


pub fn p12() {
    let contents = r#"F10
N3
F7
R90
F11"#;

    let instructions: Vec<_> =
        // contents
        std::fs::read_to_string("./assets/adv12.txt").unwrap()
        .lines()
        .map(|l| movedir_p(l).unwrap().1)
        .collect();
    eprintln!("{:?}", instructions);
    let mut start = (0., 0., 0.);
    for move_dir in instructions.iter() {
        start = move_toward(start, *move_dir);
        // eprintln!(">>> {:?}", start);
    }
    eprintln!("{:?} => {}", start, start.0.abs() + start.1.abs());

    let mut start = (0, 0, (10, 1));
    for move_dir in instructions.iter() {
        start = move_toward_r(start, *move_dir);
        // eprintln!(">>> {:?}", start);
    }
    eprintln!("{:?} => {}", start, start.0.abs() + start.1.abs());
}

pub fn p13() {
    let contents = r#"939
7,13,x,x,59,x,31,19"#;

    // let contents = std::fs::read_to_string("./assets/adv13.txt").unwrap();

    let contents = r#"213
13,x,x,41,x,x,x,37,x,x,x,x,x,659,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,19,x,x,x,23,x,x,x,x,x,29,x,409,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17"#;

    let contents = r#"213
17,x,13,19"#;

    let contents = r#"213
67,7,x,59,61"#;

    let params: Vec<_> = contents.lines().collect();
    let arrived: usize = params[0].parse().unwrap();
    let freqs: Vec<usize> = params[1].split(",").filter_map(|x| {
        match x {
            "x" => None,
            _   => x.parse::<usize>().ok()
        }
    }).collect();

    println!("{} - {:?}", arrived, freqs);

    let (idx, left) = freqs.iter().map(|f| f - arrived % f).enumerate().min_by_key(|(_, v)| *v).unwrap();
    eprintln!("{}", freqs[idx] * left);

    let mut count = 0;

    let freqs: Vec<_> = params[1].split(",").enumerate().filter_map(|(i, x)| {
        match x {
            "x" => None,
            _   => Some((i, x.parse::<usize>().unwrap()))
        }
    }).collect();

    let (max_res, max_freq) = freqs.iter().max_by_key(|(_, m)| *m).unwrap();
    let mut number = max_freq - max_res;
    eprintln!("{:?} {}", freqs, max_freq);
    loop {
        let mut satisfied = true;
        for (m, num) in freqs.iter() {
            if (number + *m) % num != 0 {
                satisfied = false;
                break;
            }
        }
        if satisfied {
            eprintln!(">>> {:?}", number);
            break;
        }
        number = number + max_freq;
    }

    eprintln!("----------------------");
    let mut number = 0;
    loop {
        let mut target = number;
        for (m, num) in freqs.iter() {
            if (target + *m) % num != 0 {
                // eprintln!("{} {} {} {}", target, num, *m, (target / num + 1) * num + num - *m);
                target = target.max((target / num + 1) * num + num - *m);
            }
        }

        if number == target {
            eprintln!(">>> {:?}", number);
            break;
        }
        number = target;
    }

}


#[derive(Debug, Clone)]
pub enum MaskManip {
    SetMask(Vec<i32>),
    ManiMem(usize, u32),
}

pub fn mask_set_p(input: &str) -> nom::IResult<&str, MaskManip> {
    let (input, _) = nom::bytes::complete::tag("mask")(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, _) = nom::bytes::complete::tag("=")(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    let x = nom::combinator::value(-1, nom::character::complete::char('X'));
    let one = nom::combinator::value(1, nom::character::complete::char('1'));
    let zero = nom::combinator::value(0, nom::character::complete::char('0'));
    let (input, res) = nom::multi::many1(nom::branch::alt((one, x, zero)))(input)?;
    if res.len() != 36 {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo)));
    } else {
        return Ok((input, MaskManip::SetMask(res)));
    }
}

pub fn mask_man_p(input: &str) -> nom::IResult<&str, MaskManip> {
    let (input, _) = nom::bytes::complete::tag("mem")(input)?;
    let (input, _) = nom::bytes::complete::tag("[")(input)?;
    let (input, addr) = nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse())(input)?;
    let (input, _) = nom::bytes::complete::tag("]")(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, _) = nom::bytes::complete::tag("=")(input)?;
    let (input, _) = nom::character::complete::space0(input)?;

    let (input, res) = nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse())(input)?;
    Ok((input, MaskManip::ManiMem(addr, res)))
}

pub fn mask_p(input: &str) -> nom::IResult<&str, MaskManip> {
    nom::branch::alt((
            mask_set_p,
            mask_man_p
    ))(input)
}

impl MaskManip {
    pub fn handle(&self, stack: &mut HashMap<usize, u64>, last_mask: &mut Vec<i32>) {
        match self {
            MaskManip::SetMask(mask) => { assert_eq!(last_mask.len(), mask.len()); last_mask.copy_from_slice(mask); }
            MaskManip::ManiMem(addr, to) => {
                let oldv = stack.entry(*addr).or_default();
                let mut target: u64 = *to as u64;
                let mut result = 0;
                let mut base: u64 = 1;

                for m in last_mask.iter().rev() {
                    let d = target % 2;
                    target = target / 2;
                    match m {
                        0 => { result += 0 * base; },
                        1 => { result += 1 * base; },
                        _ => { result += d as u64 * base; },
                    }
                    // eprintln!(">>> {} {} {} {} {}", base, d, target, m, result);
                    base *= 2;
                }
                // eprintln!("{} {} {}", addr, to, result);
                *oldv = result;
            }
        }
    }

    pub fn handle_v2(&self, stack: &mut HashMap<usize, u64>, last_mask: &mut Vec<i32>) {
        match self {
            MaskManip::SetMask(mask) => { assert_eq!(last_mask.len(), mask.len()); last_mask.copy_from_slice(mask); }
            MaskManip::ManiMem(addr, to) => {
                // [addr] -> to;
                let mut addr = *addr;
                let mut base: u64 = 1;
                let mut addrs: Vec<usize> = vec![0];

                for m in last_mask.iter().rev() {
                    let d = addr % 2;
                    addr = addr / 2;
                    match m {
                        0 => { for a in addrs.iter_mut() { *a += d * base as usize; } }
                        1 => { for a in addrs.iter_mut() { *a += (1 * base) as usize; } }
                        _ => {
                            addrs = addrs.clone().into_iter().flat_map( |a| {
                                vec![ a + 0 * base as usize, a + 1 * base as usize ]
                            } ).collect();
                        }
                    }
                    base *= 2;
                }

                for addr in addrs.into_iter() {
                    stack.insert(addr, *to as u64);
                }
            }
        }
    }
}

pub fn p14() {
    let contents = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;

    let contents2 = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;

    let masks: Vec<MaskManip> =
        std::fs::read_to_string("./assets/adv14.txt").unwrap()
        // contents
        // contents2
        .lines()
        .map(|line| mask_p(line).unwrap().1)
        .collect();

    let mut stack = HashMap::new();
    let mut mask = vec![-1; 36];
    for cmd in masks.iter() {
        cmd.handle(&mut stack, &mut mask);
    }
    eprintln!("{:?}", stack);
    eprintln!("{:?}", stack.values().sum::<u64>());

    let mut stack = HashMap::new();
    let mut mask = vec![-1; 36];
    for cmd in masks.iter() {
        cmd.handle_v2(&mut stack, &mut mask);
    }
    eprintln!("{:?}", stack);
    eprintln!("{:?}", stack.values().sum::<u64>());
}

pub fn p15() {
    let contents = r#"3,1,2"#;
    let contents = r#"2,15,0,9,1,20"#;
    let numbers: Vec<usize> =
        contents
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    eprintln!("{:?}", numbers);

    let mut freqs: HashMap<usize, (usize, usize)> = numbers.iter().enumerate().map(|(idx, n)| (*n, (idx, idx))).collect();
    let mut curr = 0; let mut last_number = *numbers.last().unwrap(); let mut idx = numbers.len() - 1;
    // let take = 2020;
    let take = 30000000; // ORZ, 30000000 is not a proglem for rust, you should give us a bigger problem...
    loop {
        idx += 1;
        if let Some((first_idx, second_idx)) = freqs.get(&last_number) {
            if second_idx > first_idx {
                curr = second_idx - first_idx;
            } else {
                curr = 0;
            }
        }

        // eprintln!(">>> {} - {} - {:?}", idx + 1, curr, freqs);
        let curr_idx = freqs.entry(curr).or_insert((idx, idx));
        curr_idx.0 = curr_idx.1;
        curr_idx.1 = idx;
        last_number = curr;

        if idx + 1 == take {
            eprintln!("{} - {}", idx, curr);
            break;
        }
    }
}

#[derive(Debug)]
struct TicketPuzzle {
    class: HashMap<String, Vec<(usize, usize)>>,
    ticket: Vec<usize>,
    nearby: Vec<Vec<usize>>,
}

impl TicketPuzzle {
    pub fn any_invalid(&self, num: usize) -> bool {
        self.class.values().all(|cond| !cond.iter().any(|(s, e)| *s <= num && num <= *e) )
    }

    pub fn error_rate(&self) -> usize {
        let mut sum = 0;
        for ticket in self.nearby.iter() {
            for num in ticket.iter() {
                if self.any_invalid(*num) {
                    // println!("invalid {}", num);
                    sum += num;
                }
            }
        }
        sum
    }

    pub fn drop_invalid(&mut self) {
        for ticket in std::mem::replace(&mut self.nearby, vec![]).into_iter() {
            let mut any_invalid = false;
            for num in ticket.iter() {
                if self.any_invalid(*num) {
                    any_invalid = true;
                    break;
                }
            }
            if !any_invalid {
                self.nearby.push(ticket);
            }
        }
    }

    fn parse_c_one(input: &str) -> nom::IResult<&str, (String, Vec<(usize, usize)>)> {
        let (input, name) = nom::multi::many1(nom::character::complete::satisfy(|c| c != ':'))(input)?;
        let (input, _) = nom::character::complete::space0(input)?;
        let (input, _) = nom::bytes::complete::tag(":")(input)?;
        let (input, _) = nom::character::complete::space0(input)?;

        let sep = nom::sequence::tuple((
                nom::character::complete::space0,
                nom::bytes::complete::tag("or"),
                nom::character::complete::space0,
        ));
        let num_range = nom::combinator::map(nom::sequence::tuple((
                    nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse::<usize>()),
                    nom::character::complete::space0,
                    nom::bytes::complete::tag("-"),
                    nom::character::complete::space0,
                    nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse::<usize>()),
        )), |(d1, _, _, _, d2)| (d1, d2));
        let (input, res) = nom::multi::separated_list1(sep, num_range)(input)?;

        Ok((input, (name.into_iter().collect(), res)))
    }

    fn parse_c(input: &str) -> nom::IResult<&str, HashMap<String, Vec<(usize, usize)>>> {
        let (input, class) = nom::multi::separated_list1(nom::character::complete::newline, Self::parse_c_one)(input)?;
        Ok((input, class.into_iter().collect()))
    }

    fn parse_t_one(input: &str) -> nom::IResult<&str, Vec<usize>> {
        let sep = nom::sequence::tuple((
                nom::character::complete::space0,
                nom::bytes::complete::tag(","),
                nom::character::complete::space0,
        ));
        nom::multi::separated_list1(sep, nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse()))(input)
    }

    fn parse_t(input: &str) -> nom::IResult<&str, Vec<usize>> {
        let (input, _) = nom::bytes::complete::tag("your ticket:")(input)?;
        let (input, _) = nom::character::complete::newline(input)?;
        Self::parse_t_one(input)
    }

    fn parse_n(input: &str) -> nom::IResult<&str, Vec<Vec<usize>>> {
        let (input, _) = nom::bytes::complete::tag("nearby tickets:")(input)?;
        let (input, _) = nom::character::complete::newline(input)?;
        nom::multi::separated_list0(nom::character::complete::newline, Self::parse_t_one)(input)
    }

    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, class) = Self::parse_c(input)?;
        let (input, _) = nom::multi::many1(nom::character::complete::newline)(input)?;

        let (input, ticket) = Self::parse_t(input)?;
        let (input, _) = nom::multi::many1(nom::character::complete::newline)(input)?;

        let (input, nearby) = Self::parse_n(input)?;
        Ok((input, Self { class, ticket, nearby, }))
    }

    pub fn guess_column(&self) -> Option<HashMap<String, usize>> {
        let mut guess = vec![];
        for (name, verify) in self.class.iter() {
            let mut possible = vec![];
            for guess_idx in 0..self.class.len() {
                let mut is_valid = true;
                for ticket in self.nearby.iter() {
                    let n = ticket[guess_idx];
                    if !verify.iter().any(|(s, e)| *s <= n && n <= *e) {
                        is_valid = false;
                        break;
                    }
                }
                if is_valid {
                    possible.push(guess_idx);
                }
            }
            guess.push((name.to_string(), possible));
        }

        let mut res = HashMap::new();
        loop {
            let mut fin = vec![];
            for (col, a) in guess.iter().filter(|(n, g)| g.len() == 1) {
                let v = a[0];
                res.insert(col.to_string(), v);
                fin.push(v);
            }
            if fin.len() == 0 { return None; }
            for v in fin.into_iter() {
                for (col, gs) in guess.iter_mut() {
                    *gs = gs.iter().filter(|n| **n != v).map(|v| *v).collect();
                }
            }
            if res.len() == self.class.len() {
                break;
            }
        }

        Some(res)
    }
}

pub fn p16() {
    let contents = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

    let contents = r#"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"#;

    let contents = std::fs::read_to_string("./assets/adv16.txt").unwrap();

    let mut messy: TicketPuzzle = TicketPuzzle::parse(&contents).unwrap().1;
    eprintln!("{:?}", messy.error_rate());
    messy.drop_invalid();
    eprintln!("{:?}", messy);
    let mut res = 1;
    for (name, idx) in messy.guess_column().unwrap() {
        if name.starts_with("departure") {
            println!("{} - {}", name, messy.ticket[idx]);
            res *= messy.ticket[idx];
        }
    }
    println!("{}", res);
}

pub fn p17_p1() {
    let contents = r#".#.
..#
###"#;

    let mut active_points: HashSet<(isize, isize, isize)> =
        // contents
        std::fs::read_to_string("./assets/adv17.txt").unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().filter_map(move |(x, v)| match v {
            '#' => Some((x as isize, y as isize, 0)),
            _   => None,
        })).collect();

    for idx in 0..6 {
        let mut max_x = 0; let mut min_x = 0;
        let mut max_y = 0; let mut min_y = 0;
        let mut max_z = 0; let mut min_z = 0;
        for (x, y, z) in active_points.iter() {
            max_x = max_x.max(*x); max_y = max_y.max(*y); max_z = max_z.max(*z);
            min_x = min_x.min(*x); min_y = min_y.min(*y); min_z = min_z.min(*z);
        }

        let mut next = HashSet::new();
        for x in (min_x-1)..(max_x+2) {
            for y in (min_y-1)..(max_y+2) {
                for z in (min_z-1)..(max_z+2) {
                    let is_active = active_points.contains(&(x, y, z));

                    let mut active_count = if is_active { -1 } else { 0 };
                    for rx in [-1, 0, 1].iter() {
                        for ry in [-1, 0, 1].iter() {
                            for rz in [-1, 0, 1].iter() {
                                if active_points.contains(&(x+rx, y+ry, z+rz)) {
                                    active_count += 1;
                                }
                            }
                        }
                    }
                    // eprintln!("checking: {} {} {}", x, y, z);

                    if is_active && (active_count == 2 || active_count == 3) {
                        next.insert((x, y, z));
                    } else if !is_active && active_count == 3 {
                        next.insert((x, y, z));
                    }
                }
            }
        }
        active_points = next;
        println!("idx {}: {:?}", idx, active_points.len());
    }
}

pub fn p17_p2() {
    let contents = r#".#.
..#
###"#;

    let mut active_points: HashSet<(isize, isize, isize, isize)> =
        // contents
        std::fs::read_to_string("./assets/adv17.txt").unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().filter_map(move |(x, v)| match v {
            '#' => Some((x as isize, y as isize, 0, 0)),
            _   => None,
        })).collect();

    for idx in 0..6 {
        let mut max_x = 0; let mut min_x = 0;
        let mut max_y = 0; let mut min_y = 0;
        let mut max_z = 0; let mut min_z = 0;
        let mut max_w = 0; let mut min_w = 0;
        for (x, y, z, w) in active_points.iter() {
            max_x = max_x.max(*x); max_y = max_y.max(*y); max_z = max_z.max(*z); max_w = max_w.max(*w);
            min_x = min_x.min(*x); min_y = min_y.min(*y); min_z = min_z.min(*z); min_w = min_w.min(*w);
        }

        let mut next = HashSet::new();
        for x in (min_x-1)..(max_x+2) {
            for y in (min_y-1)..(max_y+2) {
                for z in (min_z-1)..(max_z+2) {
                    for w in (min_w-1)..(max_w+2) {
                        let is_active = active_points.contains(&(x, y, z, w));

                        let mut active_count = if is_active { -1 } else { 0 };
                        for rx in [-1, 0, 1].iter() {
                            for ry in [-1, 0, 1].iter() {
                                for rz in [-1, 0, 1].iter() {
                                    for rw in [-1, 0, 1].iter() {
                                        if active_points.contains(&(x+rx, y+ry, z+rz, w+rw)) {
                                            active_count += 1;
                                        }
                                    }
                                }
                            }
                        }
                        // eprintln!("checking: {} {} {}", x, y, z);

                        if is_active && (active_count == 2 || active_count == 3) {
                            next.insert((x, y, z, w));
                        } else if !is_active && active_count == 3 {
                            next.insert((x, y, z, w));
                        }
                    }
                }
            }
        }
        active_points = next;
        println!("idx {}: {:?}", idx, active_points.len());
    }
}


#[derive(Debug, Clone)]
pub enum Expr {
    Lit(i32),
    Add(Box<Expr>, Box<Expr>),
    Multi(Box<Expr>, Box<Expr>),
    Paren(Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> i64 {
        match self {
            Expr::Lit(r) => *r as i64,
            Expr::Add(l, r) => l.eval() + r.eval(),
            Expr::Multi(l, r) => l.eval() * r.eval(),
            Expr::Paren(r) => r.eval(),
        }
    }

    // https://github.com/Geal/nom/blob/master/tests/arithmetic_ast.rs
    fn parse_lit(input: &str) -> nom::IResult<&str, Self> {
        let lit = nom::sequence::delimited(
            nom::character::complete::multispace0,
            nom::combinator::map(
                nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse()),
                Self::Lit
            ),
            nom::character::complete::multispace0,
        );
        nom::branch::alt((lit, Self::parse_paren))(input)
    }

    fn parse_paren(input: &str) -> nom::IResult<&str, Self> {
        nom::sequence::delimited(
            nom::character::complete::multispace0,
            nom::sequence::delimited(
                nom::bytes::complete::tag("("),
                nom::combinator::map(Self::parse_expr, |e| Self::Paren(Box::new(e))),
                nom::bytes::complete::tag(")"),
            ),
            nom::character::complete::multispace0,
        )(input)
    }

    fn parse_expr(input: &str) -> nom::IResult<&str, Self> {
        let (input, initial) = Self::parse_lit(input)?;
        let (input, remainder) = nom::multi::many0(
            nom::branch::alt((
                    nom::sequence::tuple((nom::character::complete::char('*'), Self::parse_lit)),
                    nom::sequence::tuple((nom::character::complete::char('+'), Self::parse_lit)),
            ))
        )(input)?;

        let res = remainder.into_iter().fold(initial, |acc, (op, right)| {
            match op {
                '*' => Expr::Multi(Box::new(acc), Box::new(right)),
                '+' => Expr::Add(Box::new(acc), Box::new(right)),
                _ => unreachable!()
            }
        });
        Ok((input, res))
    }

    // precedence: * -> + -> () lit
    fn parse_expr_l0(input: &str) -> nom::IResult<&str, Self> {
        let (input, initial) = Self::parse_expr_l1(input)?;
        let (input, remainder) = nom::multi::many0(
            nom::sequence::tuple((nom::character::complete::char('*'), Self::parse_expr_l1)),
        )(input)?;

        let res = remainder.into_iter().fold(initial, |acc, (op, right)| {
            match op {
                '*' => Expr::Multi(Box::new(acc), Box::new(right)),
                _ => unreachable!()
            }
        });
        Ok((input, res))
    }

    fn parse_expr_l1(input: &str) -> nom::IResult<&str, Self> {
        let (input, initial) = Self::parse_expr_l2(input)?;
        let (input, remainder) = nom::multi::many0(
            nom::sequence::tuple((nom::character::complete::char('+'), Self::parse_expr_l2)),
        )(input)?;

        let res = remainder.into_iter().fold(initial, |acc, (op, right)| {
            match op {
                '+' => Expr::Add(Box::new(acc), Box::new(right)),
                _ => unreachable!()
            }
        });
        Ok((input, res))
    }

    fn parse_expr_l2(input: &str) -> nom::IResult<&str, Self> {
        let paren = nom::sequence::delimited(
            nom::character::complete::multispace0,
            nom::sequence::delimited(
                nom::bytes::complete::tag("("),
                nom::combinator::map(Self::parse_expr_l0, |e| Self::Paren(Box::new(e))),
                nom::bytes::complete::tag(")"),
            ),
            nom::character::complete::multispace0,
        );
        let lit = nom::sequence::delimited(
            nom::character::complete::multispace0,
            nom::combinator::map(
                nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse()),
                Self::Lit
            ),
            nom::character::complete::multispace0,
        );
        nom::branch::alt((lit, paren))(input)
    }
}

pub fn p18() {
    let contents = r#"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
5 + (8 * 3 + 9 + 3 * 4 * 3)
2 * 3 + (4 * 5)"#;
    let exprs: Vec<_> =
        // contents
        std::fs::read_to_string("./assets/adv18.txt").unwrap()
        .lines()
        .map(|line| Expr::parse_expr_l0(line).unwrap().1.eval())
        .collect();
    eprintln!("{:?}", exprs);
    eprintln!("{:?}", exprs.into_iter().sum::<i64>());
}

#[derive(Debug, Clone)]
pub enum Rule {
    Any(Vec<Rule>),
    Uniq(String),
    Seq(Vec<usize>),
}

impl Rule {
    fn parse_uniq(input: &str) -> nom::IResult<&str, Self> {
        let (input, res) = nom::sequence::delimited(
            nom::bytes::complete::tag("\""),
            nom::character::complete::alpha0,
            nom::bytes::complete::tag("\""),
        )(input)?;
        Ok((input, Self::Uniq(res.to_string())))
    }

    fn parse_any(input: &str) -> nom::IResult<&str, Self> {
        let (input, res) = nom::multi::separated_list1(
            nom::bytes::complete::tag(" | "),
            Self::parse_seq
        )(input)?;

        Ok((input, Self::Any(res)))
    }

    fn parse_seq(input: &str) -> nom::IResult<&str, Self> {
        let (input, res) = nom::multi::separated_list1(
            nom::character::complete::space1,
            nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse::<usize>())
        )(input)?;
        Ok((input, Self::Seq(res)))
    }

    fn parse(input: &str) -> nom::IResult<&str, (usize, Self)> {
        let (input, id) = nom::combinator::map_res(
            nom::character::complete::digit1,
            |s: &str| s.parse::<usize>()
        )(input)?;
        let (input, _) = nom::bytes::complete::tag(":")(input)?;
        let (input, _) = nom::character::complete::space0(input)?;

        let (input, res) = nom::branch::alt((
                Self::parse_uniq,
                Self::parse_any,
                Self::parse_seq,
        ))(input)?;

        if input.len() != 0 {
            return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo)));
        }

        Ok((input, (id, res)))
    }

    // pub fn valid<'s>(&self, rules: &HashMap<usize, Rule>, input: &'s str) -> (&'s str, bool) {
    // }

    pub fn valid<'s>(&self, rules: &HashMap<usize, Rule>, input: &'s str) -> (&'s str, bool) {
        eprintln!("{:?} - {}", self, input);
        match self {
            Self::Any(rs) => {
                for r in rs.iter() {
                    let (remain, is_valid) = r.valid(rules, input);
                    if is_valid {
                        return (remain, true);
                    }
                }
                return (input, false);
            },
            Self::Uniq(res) => {
                if input.starts_with(res) {
                    return (&input[res.len()..], true)
                } else {
                    return (input, false)
                }
            },
            Self::Seq(rs) => {
                let mut remain = input;
                for rid in rs.iter() {
                    if let Some(r) = rules.get(rid) {
                        let (res, is_valid) = r.valid(rules, remain);
                        if !is_valid {
                            return (input, false);
                        }
                        remain = res;
                    } else {
                        return (input, false);
                    }
                }
                return (remain, true);
            }
        }
    }

    // pub fn valid_all<'s>(&self, rules: &HashMap<usize, Rule>, input: &'s str) -> bool {
    //     match self {
    //         Self::Uniq(res) => {
    //             return res == input;
    //         },
    //         Self::Seq(rs) => {
    //             let mut input = input;
    //             for rid in rs.iter() {
    //                 if let Some(r) = rules.get(rid) {
    //                     for i in
    //                     let is_valid = r.valid_all(rules, remain);
    //                 }
    //             }


    //         }
    //     }
    // }
}

pub fn valid_rule(rules: &HashMap<usize, Rule>, input: &str) -> bool {
    let r = rules.get(&0).unwrap();
    eprintln!(">>>> {:?}", r);
    let (input, res) = r.valid(rules, input);
    input.len() == 0 && res
}

pub fn p19() {
    let contents = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    // let contents = std::fs::read_to_string("./assets/adv19.txt").unwrap();

    let contents = r#"0: 42 31 | 42 0 31
42: "a"
31: "b"

aabb"#;

    // let contents = contents.replace("8: 42", "8: 42 | 42 8");
    // let contents = contents.replace("11: 42 31", "11: 42 31 | 42 11 31");

    let contents_s: Vec<_> = contents.split("\n\n").collect();
    assert_eq!(contents_s.len(), 2);
    let rules: HashMap<usize, Rule> = contents_s[0]
        .split("\n")
        .map(|s| Rule::parse(s).unwrap().1)
        .collect();

    // eprintln!("{:?}", rules);
    let out: Vec<_> = contents_s[1]
        .split("\n")
        .filter(|s| valid_rule(&rules, s))
        .collect();
    eprintln!("{:?}", out);
    eprintln!("{:?}", out.len());
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Pixel {
    Dot,
    Sharp,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    data: [[Pixel; 10]; 10],
    id: usize,
}

pub struct Border {
    data: [Pixel; 10],
    posi: Vec<usize>
}
pub enum BorderDir {
    Left,
    Right,
    Top,
    Bottom,
}
pub enum Flip {
    N,
    UD,
    LR
}


impl Tile {
    // pub fn borders(&self) -> Vec<(Flip, BorderDir, Border)> {
    // }

    pub fn parse_with_id(input: &str) -> nom::IResult<&str, Self> {
        let (input, _) = nom::bytes::complete::tag("Tile")(input)?;
        let (input, _) = nom::character::complete::space1(input)?;
        let (input, id) = nom::combinator::map_res(
            nom::character::complete::digit1,
            |s: &str| s.parse::<usize>()
        )(input)?;
        let (input, _) = nom::bytes::complete::tag(":")(input)?;
        let (input, _) = nom::character::complete::newline(input)?;
        let (input, data) = Self::parse_data(input)?;
        let (r, _) = nom::combinator::eof(input)?;
        Ok((r, Self { id, data }))
    }

    pub fn parse_pixel(input: &str) -> nom::IResult<&str, Pixel> {
        let dot = nom::bytes::complete::tag(".");
        let sharp = nom::bytes::complete::tag("#");

        nom::branch::alt((
                nom::combinator::value(Pixel::Dot, dot),
                nom::combinator::value(Pixel::Sharp, sharp)
        ))(input)
    }

    pub fn parse_col(input: &str) -> nom::IResult<&str, [Pixel; 10]> {
        let (input, res) = nom::multi::count(Self::parse_pixel, 10)(input)?;
        Ok((input, res.try_into().unwrap()))
    }

    pub fn parse_data(input: &str) -> nom::IResult<&str, [[Pixel; 10]; 10]> {
        let (input, res) = nom::multi::separated_list0(
            nom::character::complete::newline,
            Self::parse_col
        )(input)?;

        if res.len() != 10 {
            return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::ParseTo)));
        }
        Ok((input, res.try_into().unwrap()))
    }

    pub fn left(&self) -> Vec<Pixel> {
        self.data[0].iter().map(|s| *s).collect()
    }

    pub fn top_border(&self) -> Vec<usize> {
        self.data[0].iter().enumerate().flat_map(|(idx, s)|
            if *s == Pixel::Sharp { Some(idx) } else { None }
        ).collect()
    }

    pub fn bottom_border(&self) -> Vec<usize> {
        self.data[9].iter().enumerate().flat_map(|(idx, s)|
            if *s == Pixel::Sharp { Some(idx) } else { None }
        ).collect()
    }

    pub fn left_border(&self) -> Vec<usize> {
        self.data.iter().map(|s| &s[0]).enumerate().flat_map(|(idx, s)|
            if *s == Pixel::Sharp { Some(idx) } else { None }
        ).collect()
    }

    pub fn right_border(&self) -> Vec<usize> {
        self.data.iter().map(|s| &s[9]).enumerate().flat_map(|(idx, s)|
            if *s == Pixel::Sharp { Some(idx) } else { None }
        ).collect()
    }
}

// TODO: too difficult for me now.
pub fn p20() {
    let contents = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;

    let tiles: Vec<_> = contents.split("\n\n").map(|s| Tile::parse_with_id(s).unwrap().1).collect();

    // let mut lefts: HashMap<usize, Vec<usize>> = HashMap::new(); let mut lefts_to: HashMap<Vec<usize>, Vec<usize>> = Default::default();
    // let mut rights: HashMap<usize, Vec<usize>> = HashMap::new(); let mut rights_to: HashMap<Vec<usize>, Vec<usize>> = Default::default();
    // let mut tops: HashMap<usize, Vec<usize>> = HashMap::new(); let mut tops_to: HashMap<Vec<usize>, Vec<usize>> = Default::default();
    // let mut bottoms: HashMap<usize, Vec<usize>> = HashMap::new(); let mut bottoms_to: HashMap<Vec<usize>, Vec<usize>> = Default::default();
    // for tile in tiles.iter() {
    //     lefts.insert(tile.id, tile.left_border()); lefts_to.entry(tile.left_border()).or_default().push(tile.id);
    //     rights.insert(tile.id, tile.right_border()); rights_to.entry(tile.right_border()).or_default().push(tile.id);
    //     tops.insert(tile.id, tile.top_border()); tops_to.entry(tile.top_border()).or_default().push(tile.id);
    //     bottoms.insert(tile.id, tile.bottom_border()); bottoms_to.entry(tile.bottom_border()).or_default().push(tile.id);
    // }

    // let all: Vec<_> = tiles.iter().map(|t| t.id).collect();
    // let mut lefts_match: HashMap<usize, Vec<usize>> = all.iter().map(|i| (*i, all.clone())).collect();
    // eprintln!("{:?}, {:?}", lefts, rights);
    // for _ in 0..1 {
    //     for (lid, target) in lefts_match.iter_mut() {
    //         let left = &lefts[lid];
    //         let r: Vec<_> = target.iter().filter(|r| left == &rights[r]).collect();
    //         eprintln!("{} -> {:?}", lid, r);
    //     }
    // }

    let borders: HashMap<_, _> = tiles.iter().map(|t| {
        (t.id, (t.top_border(), t.right_border(), t.bottom_border(), t.left_border()))
    }).collect();

    let mut graph: HashMap<usize, Vec<usize>> = Default::default();
    for tile in tiles.iter() {
        graph.insert(tile.id, vec![]);
        for tile_ in tiles.iter() {
            if tile.id == tile_.id { continue; }

        }

    }

    eprintln!("{:?}", graph);




    // let mut possi: HashMap<_, _> = HashMap::new();
    // for tile in tiles.iter() {
    //     let left = tile.left_border(); let mut l = vec![];
    //     let right = tile.right_border(); let mut r = vec![];
    //     let top = tile.top_border(); let mut t = vec![];
    //     let bottom = tile.bottom_border(); let mut b = vec![];

    //     for tile_t in tiles.iter() {
    //         if tile_t.id == tile.id { continue; }
    //         if left == tile_t.right_border() { l.push(tile_t.id); }
    //         if right == tile_t.left_border() { r.push(tile_t.id); }
    //         if top == tile_t.bottom_border() { t.push(tile_t.id); }
    //         if bottom == tile_t.top_border() { b.push(tile_t.id); }
    //     }
    //     possi.insert(tile.id, (t, r, b, l));
    // }

    // println!("{:?}", possi);


}
