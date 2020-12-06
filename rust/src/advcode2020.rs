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
