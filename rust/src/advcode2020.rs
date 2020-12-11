use std::collections::{HashSet, HashMap};

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


