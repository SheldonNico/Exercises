use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

pub fn p01() {
    let contents = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
    let contents = std::fs::read_to_string("./assets/adv2022/adv01.txt").unwrap();

    let calories: Vec<Vec<usize>> = contents.trim().split("\n\n").map(
        |lines| lines.split("\n").map(|cc| cc.parse().unwrap()).collect()
    ).collect();
    let mut calories_max: Vec<usize> = calories.into_iter().map(|cc| cc.into_iter().sum::<usize>()).collect();
    eprintln!("max calories: {:?}", calories_max.iter().max());

    calories_max.sort();
    eprintln!("top 3 calories: {:?}", calories_max.iter().rev().take(3).sum::<usize>());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum P02Shape {
    Rock,
    Paper,
    Scissor,
}

impl P02Shape {
    pub fn from_u8(idx: u8) -> Self {
        match idx {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissor,
            _ => unreachable!("primitive integer must between 0-2"),
        }
    }

    pub fn from_u8_rev(slf: u8, score: u8) -> Self {
        match score {
            0 => P02Shape::from_u8(slf).win(),
            1 => P02Shape::from_u8(slf),
            2 => P02Shape::from_u8(slf).lose(),
            _ => unreachable!("primitive integer must between 0-2"),
        }
    }

    pub fn lose(self) -> Self {
        use P02Shape::*;

        match self {
            Rock    => Paper,
            Scissor => Rock,
            Paper   => Scissor,
        }
    }

    pub fn win(self) -> Self {
        use P02Shape::*;

        match self {
            Rock    => Scissor,
            Scissor => Paper,
            Paper   => Rock,
        }
    }

    pub fn round(self, right: Self) -> usize {
        use P02Shape::*;

        let base = ((self as u8) + 1) as usize;
        let score = match (self, right) {
            (l, r) if l      == r => 3,
            (Rock, Scissor)  => 6,
            (Scissor, Paper) => 6,
            (Paper, Rock)    => 6,
            _                => 0,
        };

        base + score
    }
}

pub fn p02() {
    let contents = r#"A Y
B X
C Z"#;
    let contents = std::fs::read_to_string("./assets/adv2022/adv02.txt").unwrap();

    let shapes: Vec<(P02Shape, P02Shape)> = contents.trim().split("\n").map(|line| {
        let (abc, xyz) = line.split_once(" ").unwrap();
        let abc = abc.chars().next().unwrap() as u8 - 'A' as u8;
        let xyz = xyz.chars().next().unwrap() as u8 - 'X' as u8;

        (P02Shape::from_u8(abc), P02Shape::from_u8(xyz))
    }).collect();
    let score: usize = shapes.iter().map(|(other, slf)| {
        slf.round(*other)
    }).sum();

    // eprintln!("{:?}", shapes);
    eprintln!("score: {:?}", score);

    let shapes: Vec<(P02Shape, P02Shape)> = contents.trim().split("\n").map(|line| {
        let (abc, xyz) = line.split_once(" ").unwrap();
        let abc = abc.chars().next().unwrap() as u8 - 'A' as u8;
        let xyz = xyz.chars().next().unwrap() as u8 - 'X' as u8;

        (P02Shape::from_u8(abc), P02Shape::from_u8_rev(abc, xyz))
    }).collect();
    let score: usize = shapes.iter().map(|(other, slf)| {
        slf.round(*other)
    }).sum();
    // eprintln!("{:?}", shapes);
    eprintln!("score: {:?}", score);
}

pub fn p03() {
    let contents = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
    let contents = std::fs::read_to_string("./assets/adv2022/adv03.txt").unwrap();

    let priorities: usize = contents.trim().split("\n").map(|line| {
        let mut items: Vec<usize> = line.trim().chars().map(|c| {
            let idx = if c.is_lowercase() { 
                c as u8 - 'a' as u8
            } else {
                (c as u8 - 'A' as u8) + 26
            };
            idx as usize + 1
        }).collect();
        let len = items.len();
        assert!(len % 2 == 0);
        let cap = len / 2;

        let (left, right) = items.split_at_mut(cap);
        left.sort();
        right.sort();

        let common = left.iter().find(|idx| {
            right.binary_search(idx).is_ok()
        });
        *common.unwrap()
    }).sum();
    eprintln!("priorities: {}", priorities);

    let priorities: Vec<Vec<usize>> = contents.trim().split("\n").map(|line| {
        let items: Vec<usize> = line.trim().chars().map(|c| {
            let idx = if c.is_lowercase() { 
                c as u8 - 'a' as u8
            } else {
                (c as u8 - 'A' as u8) + 26
            };
            idx as usize + 1
        }).collect();
        items
    }).collect();
    assert!(priorities.len() % 3 == 0);
    let mut tot = 0;
    for idx in 0..(priorities.len() / 3) {
        let chunk0: HashSet<usize> = priorities[0+3*idx].iter().map(|p| *p).collect();
        let chunk1: HashSet<usize> = priorities[1+3*idx].iter().map(|p| *p).collect();
        let chunk2: HashSet<usize> = priorities[2+3*idx].iter().map(|p| *p).collect();

        let inter: HashSet<usize> = chunk0.intersection(&chunk1).map(Clone::clone).collect();
        let inter: HashSet<usize> = inter.intersection(&chunk2).map(Clone::clone).collect();
        assert_eq!(inter.len(), 1);

        let common = inter.into_iter().next().unwrap();
        eprintln!("{:?}", common);
        tot += common;
    }
    eprintln!("priorities: {}", tot);
}

fn p04_pair(input: &str) -> nom::IResult<&str, (usize, usize)> {
    let (input, x) = nom::character::complete::u64(input)?;
    let (input, _) = nom::character::complete::char('-')(input)?;
    let (input, y) = nom::character::complete::u64(input)?;
    Ok((input, (x as usize, y as usize)))
}


pub fn p04() {
    let contents = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
    let contents = std::fs::read_to_string("./assets/adv2022/adv04.txt").unwrap();

    let pairs: usize = contents.trim().split("\n").map(|line| {
        let (_, ((x0, y0), _, (x1, y1))) = nom::sequence::tuple((
            p04_pair,
            // nom::character::complete::char(','),
            nom::bytes::complete::tag(","),
            p04_pair,
        ))(line).unwrap();
        (x0, y0, x1, y1)
    }).filter(|(x0, y0, x1, y1)| {
        (x0 <= x1 && y0 >= y1) ||
        (x0 >= x1 && y0 <= y1) 
    }).count();
    eprintln!("pairs: {}", pairs);

    let pairs: usize = contents.trim().split("\n").map(|line| {
        let (_, ((x0, y0), _, (x1, y1))) = nom::sequence::tuple((
            p04_pair,
            // nom::character::complete::char(','),
            nom::bytes::complete::tag(","),
            p04_pair,
        ))(line).unwrap();
        (x0, y0, x1, y1)
    }).filter(|(x0, y0, x1, y1)| {
        !(x0 > y1 || y0 < x1)
    }).count();
    eprintln!("overlap pairs: {}", pairs);
}

fn p05_stack(input: &str) -> nom::IResult<&str, Vec<Vec<Option<char>>>> {
    let bucket = nom::sequence::delimited(
        nom::character::complete::char('['),
        nom::character::complete::anychar,
        nom::character::complete::char(']'),
    );
    let empty = nom::multi::count(
        nom::character::complete::char(' '),
        3
    );

    let cell1 = nom::combinator::map(bucket, |c| Some(c));
    let cell2 = nom::combinator::map(empty, |c| None);
    let cell = nom::branch::alt((cell1, cell2));
    let cells = nom::multi::separated_list1(nom::character::complete::char(' '), cell);
    let (input, cells) = nom::multi::separated_list0(nom::character::complete::newline, cells)(input)?;

    let idx = nom::sequence::delimited(
        nom::character::complete::char(' '),
        nom::character::complete::u8,
        nom::character::complete::char(' '),
    );
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, index) = nom::multi::separated_list0(nom::character::complete::char(' '), idx)(input)?;

    let width = index.len();
    assert!(index == (0..width).map(|w| w as u8 +1).collect::<Vec<u8>>());
    assert!(cells.iter().all(|row| row.len() == width));

    let height = cells.len();
    let mut stacks = Vec::with_capacity(width);
    for column in 0..width {
        let mut stack: Vec<_> = (0..height).into_iter().map(|row| cells[row][column]).collect();
        stack.reverse();
        stacks.push(stack);
    }

    Ok((input, stacks))
}

#[derive(Debug, Clone)]
struct P05Cmd {
    from: usize, 
    to: usize, 
    count: usize,
}

fn p05_cmd(input: &str) -> nom::IResult<&str, Vec<P05Cmd>> {
    let line = |input| -> nom::IResult<&str, P05Cmd> {
        let (input, _) = nom::bytes::complete::tag("move ")(input)?;
        let (input, count) = nom::character::complete::u8(input)?;
        let (input, _) = nom::bytes::complete::tag(" from ")(input)?;
        let (input, from) = nom::character::complete::u8(input)?;
        let (input, _) = nom::bytes::complete::tag(" to ")(input)?;
        let (input, to) = nom::character::complete::u8(input)?;
        Ok((input, P05Cmd { from: from as usize - 1, to: to as usize - 1, count: count as usize }))
    };
    nom::multi::separated_list0(nom::character::complete::newline, line)(input)
}

fn p05_process_step(stack: &mut Vec<Vec<Option<char>>>, from: usize, to: usize, width: &mut usize, height: &mut usize) {
    let from_index = stack[from].iter().take_while(|c| c.is_some()).count();
    let to_index = stack[to].iter().take_while(|c| c.is_some()).count();

    if to_index >= *height {
        stack.iter_mut().map(|column| column.push(None)).for_each(drop);
        *height += 1;
    }

    assert!(from_index >= 1);
    let c = stack[from][from_index-1].take();
    stack[to][to_index] = c;

    // 除了自己受影响，其他列也需要调整
    if stack.iter().all(|column| column[*height-1].is_none()) {
        stack.iter_mut().map(|column| { 
            let c = column.pop().unwrap();
            debug_assert!(c.is_none());
        }).for_each(drop);

        *height -= 1;
    }
}

fn p05_process(stack: &mut Vec<Vec<Option<char>>>, cmd: &P05Cmd, width: &mut usize, height: &mut usize, v2: bool) {
    let P05Cmd { from, to, count } = cmd.clone();

    if v2 {
        for _ in 0..count {
            p05_process_step(stack, from, to, width, height);
        }
        // 只需要把序号重新整理下就可以
        let to_index = stack[to].iter().take_while(|c| c.is_some()).count();
        assert!(to_index >= count);
        stack[to][(to_index-count)..to_index].reverse();
    } else {
        for _ in 0..count {
            p05_process_step(stack, from, to, width, height);
        }
    }
}

fn p05_display(stack: &mut Vec<Vec<Option<char>>>, width: usize, height: usize) {
    for idx in 0..height {
        for jdx in 0..width {
            eprint!("{}", stack[jdx][idx].unwrap_or(' '));
        }
        eprintln!();
    }
}

pub fn p05() {
    let contents = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    let contents = &std::fs::read_to_string("./assets/adv2022/adv05.txt").unwrap();

    let (_, (stack_orig, _, cmds)) = nom::sequence::tuple((
        p05_stack,
        nom::multi::many1(nom::character::complete::newline),
        p05_cmd,
    ))(contents).unwrap();

    let mut stack = stack_orig.clone();
    let mut width = stack.len();
    assert!(width > 0);
    let mut height = stack[0].len();
    assert!(height > 0);
    for cmd in cmds.iter() {
        // eprintln!("{:?}", stack);
        p05_process(&mut stack, cmd, &mut width, &mut height, false);
    }
    // eprintln!("{:?}", stack);
    // eprintln!("{:?}", cmds);
    p05_display(&mut stack, width, height);

    let mut stack = stack_orig.clone();
    let mut width = stack.len();
    assert!(width > 0);
    let mut height = stack[0].len();
    assert!(height > 0);
    for cmd in cmds.iter() {
        // eprintln!("{:?}", stack);
        p05_process(&mut stack, cmd, &mut width, &mut height, true);
    }
    // eprintln!("{:?}", stack);
    // eprintln!("{:?}", cmds);
    p05_display(&mut stack, width, height);
}

pub fn p06() {
    let contents = &std::fs::read_to_string("./assets/adv2022/adv06.txt").unwrap();

    for contents in [ 
        r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#, // has some bugs.
        r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#,
        r#"nppdvjthqldpwncqszvftbrmjlhg"#,
        r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#,
        r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#,
        &std::fs::read_to_string("./assets/adv2022/adv06.txt").unwrap().trim(), 
    ] {

        // let mut buf: [char; 4] = [0 as char; 4];
        // let mut is_marker;
        // for (idx, marker) in contents.chars().enumerate() {
        //     buf.rotate_left(1);
        //     *buf.last_mut().unwrap() = marker;
        //
        //     if idx < 4 { continue; }
        //     is_marker = true;
        //     'is_dup: for idx in 0..buf.len() {
        //         for jdx in idx+1..buf.len() {
        //             if buf[idx] == buf[jdx] {
        //                 is_marker = false;
        //                 break 'is_dup;
        //             }
        //         }
        //     }
        //
        //     if is_marker {
        //         eprintln!("found marker@{}: {:?}", idx+1, contents);
        //         break;
        //     }
        // }


        let characters: Vec<char> = contents.chars().collect();
        let width: usize = 14;

        let mut counts = [0; 26];
        for (idx, character) in characters.iter().enumerate() {
            let first = *character as u8 - 'a' as u8;
            counts[first as usize] += 1;
            if idx < width { continue; }

            if !counts.iter().any(|c| *c > 1) {
                eprintln!("found marker@{}: {:?}", idx, contents);
                break;
            }

            let last = characters[idx - width] as u8 - 'a' as u8;
            counts[last as usize] -= 1;
        }
    }
}


// P07 的难点在于，如果不单纯为了解决问题，我们需要构建一个简易的文件树系统，
// dir 相互移动，需要同时持有子节点和父节点，这显然就到了rust的不擅长领域。有GC
// 的语言，可以疯狂用 gc 做，有shared_ptr 可以用unsafe做，但rust不允许unsafe，
// 这就导致 Arc/Cell 等的大量嵌套。最后太过复杂，转而 Arean 方案。
#[derive(Debug)]
pub enum P07EntryType {
    Dir,
    File, 
    Root
}

#[derive(Debug, Default)]
pub struct P07System {
    nodes_child: Vec<Vec<usize>>,
    nodes_paren: Vec<usize>,
    nodes_count: usize,
    nodes_alive: usize,

    nodes_infos: Vec<P07Info>,
}

#[derive(Debug, Default)]
pub struct P07Info {
    name: String,
    is_dir: bool,
    is_file: bool,
    is_root: bool,
    size_file: Option<usize>,
    size_dir: usize,
}

impl P07System {
    fn root() -> Self {
        let mut slf = Self::default();
        let node_root = slf.new();
        slf.nodes_child[node_root].push(node_root);
        slf.nodes_paren[node_root] = node_root;
        slf.nodes_infos[node_root] = P07Info::new_root();
        slf.nodes_alive = 1;
        slf.nodes_count = 1;
        slf
    }

    pub fn new(&mut self) -> usize {
        assert!(self.nodes_alive == self.nodes_count);
        let node = self.nodes_count;
        self.nodes_count += 1;
        self.nodes_alive += 1;
        self.nodes_child.push(vec![]);
        self.nodes_paren.push(node);
        self.nodes_infos.push(P07Info::default());

        node
    }

    pub fn parse_new_item(&mut self, node: usize, item: P07Info) {
        let mut node_found = None;
        for &child in self.nodes_child[node].iter() {
            if self.nodes_infos[child].name == item.name {
                node_found = Some(child);
                break;
            }
        }
        if node_found.is_none() {
            let node_new = self.new();
            node_found.replace(node_new);
        }
        let node_found = node_found.unwrap();

        self.nodes_infos[node_found] = item;
        self.nodes_paren[node_found] = node;
        self.nodes_child[node].push(node_found);
    }

    pub fn parse_cd(&mut self, paren: usize, name: String) -> usize {
        if name == "/" { return 0; }
        if name == ".." { return self.nodes_paren[paren]; }

        for &node in self.nodes_child[paren].iter() {
            let info = &self.nodes_infos[node];
            if info.name == name { return node; }
        }

        let node = self.new();
        self.nodes_paren[node] = paren;
        self.nodes_infos[node] = P07Info::new_dir(name);
        return node;
    }
}

impl P07Info {
    fn new_dir(name: impl Into<String>) -> Self {
        let mut slf = Self::default();
        slf.name = name.into();
        slf.is_dir = true;
        slf
    }

    fn new_root() -> Self {
        let mut slf = Self::new_dir("/");
        slf.is_root = true;
        slf
    }

    fn new_file(name: impl Into<String>, size: usize) -> Self {
        let mut slf = Self::default();
        slf.name = name.into();
        slf.is_file = true;
        slf.size_file = Some(size);
        slf
    }
}

pub enum P07Cmd {
    Cd(String),
    Ls,
}

fn p07_parse_cmd(input: &str) -> nom::IResult<&str, P07Cmd> {
    let (input, _) = nom::bytes::complete::tag("$ ")(input)?;
    let cd = nom::sequence::preceded(
        nom::bytes::complete::tag("cd "),
        nom::multi::many1(nom::character::complete::satisfy(|c| c.is_alphanumeric() || c == '/' || c == '.')),
    );
    let ls = nom::bytes::complete::tag("ls");
    let (input, cmd) = nom::branch::alt((
        nom::combinator::map(cd, |ss: Vec<char>| P07Cmd::Cd(ss.into_iter().collect())),
        nom::combinator::map(ls, |_| P07Cmd::Ls),
    ))(input)?;
    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, cmd))
}

fn p07_parse_ls(input: &str) -> nom::IResult<&str, Vec<P07Info>> {
    let dir = nom::sequence::preceded(
        nom::bytes::complete::tag("dir "),  
        nom::multi::many1(nom::character::complete::satisfy(|c| c.is_alphanumeric() || c == '/')),
    );
    let file = nom::sequence::tuple((
        nom::character::complete::u64::<&str, _>,
        nom::character::complete::space1,
        nom::multi::many1(nom::character::complete::satisfy(|c| c.is_alphanumeric() || c == '.')),
    ));

    let dir = nom::combinator::map(dir, |name| {
        let mut info = P07Info::default();
        let name: String = name.into_iter().collect();
        info.is_dir = true;
        info.is_root = name == "/";
        info.name = name;
        info
    });
    let file = nom::combinator::map(file, |(fsize, _, fname)| {
        let mut info = P07Info::default();
        info.name = fname.into_iter().collect();
        info.is_file = true;
        info.size_file = Some(fsize as usize);
        info
    });

    let dir_or_file = nom::branch::alt((dir, file));
    let (input, out) = nom::multi::separated_list0(nom::character::complete::newline, dir_or_file)(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    Ok((input, out))
}

fn p07_with_cmd(dir: &mut P07System, node: usize, stdout: &str) {
    if stdout.is_empty() { return; }
    let (stdout, cmd) = p07_parse_cmd(stdout).unwrap();
    match cmd {
        P07Cmd::Cd(folder) => {
            let node = dir.parse_cd(node, folder);
            p07_with_cmd(dir, node, stdout);
        },
        P07Cmd::Ls => {
            let (stdout, items) = p07_parse_ls(stdout).unwrap();
            for item in items {
                dir.parse_new_item(node, item)
            }
            p07_with_cmd(dir, node, stdout);
        }
    }
}

fn p07_walk_display(dir: &P07System, node: usize, tab: usize) {
    let print_space = || { for _ in 0..tab { eprint!("    "); } };
    if dir.nodes_infos[node].is_dir {
        print_space(); eprintln!("- {} (dir, {})", dir.nodes_infos[node].name, dir.nodes_infos[node].size_dir);

        for &child in dir.nodes_child[node].iter() {
            if child == node { continue; }
            p07_walk_display(dir, child, tab+1);
        }
    } else if dir.nodes_infos[node].is_file {
        print_space(); eprintln!("- {} (file, size={})", dir.nodes_infos[node].name, dir.nodes_infos[node].size_file.unwrap());
    }
}

fn p07_walk_fill_size(dir: &mut P07System, node: usize) -> usize {
    if dir.nodes_infos[node].is_dir {
        let mut size_tot = 0;
        for &child in dir.nodes_child[node].clone().iter() {
            if child == node { continue; }
            size_tot += p07_walk_fill_size(dir, child);
        }
        dir.nodes_infos[node].size_dir = size_tot;
        return size_tot;
    } else if dir.nodes_infos[node].is_file { 
        dir.nodes_infos[node].size_file.unwrap()
    } else {
        0
    }
}

fn p07_walk_find_small_folder(dir: &P07System, node: usize, sum: &mut usize) {
    if dir.nodes_infos[node].is_dir {
        if dir.nodes_infos[node].size_dir < 100000 { *sum += dir.nodes_infos[node].size_dir; }
        for &child in dir.nodes_child[node].iter() {
            if child == node { continue; }
            p07_walk_find_small_folder(dir, child, sum);
        }
    }
}

fn p07_walk_find_smallest_big_folder(dir: &P07System, node: usize, threshold: usize, big_folders: &mut Vec<(usize, usize)>) {
    if dir.nodes_infos[node].is_dir {
        if dir.nodes_infos[node].size_dir >= threshold { big_folders.push((dir.nodes_infos[node].size_dir, node)); }
        for &child in dir.nodes_child[node].iter() {
            if child == node { continue; }
            p07_walk_find_smallest_big_folder(dir, child, threshold, big_folders);
        }
    }
}

pub fn p07() {
    let contents = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;


    // note the last newline must be used.
    let contents = &std::fs::read_to_string("./assets/adv2022/adv07.txt").unwrap();
    let mut root = P07System::root();
    p07_with_cmd(&mut root, 0, contents);

    p07_walk_fill_size(&mut root, 0);
    p07_walk_display(&root, 0, 0);

    let mut sum = 0;
    p07_walk_find_small_folder(&root, 0, &mut sum);
    eprintln!("All small folder: {}", sum);

    let mut big_folders = vec![];
    let threshold = 30000000 - (70000000 - root.nodes_infos[0].size_dir);
    p07_walk_find_smallest_big_folder(&root, 0, threshold, &mut big_folders);
    big_folders.sort();
    eprintln!("{} => {:?}", threshold, big_folders);
}

fn p08_parse_height(input: &str) -> nom::IResult<&str, Vec<Vec<usize>>> {
    nom::multi::separated_list1(
        nom::character::complete::newline, 
        nom::multi::many1(nom::combinator::map(nom::character::complete::satisfy(|c: char| c.is_digit(10)), |v| (v as u8 - '0' as u8) as usize))
    )(input)
}

fn p08_walk_visable(tree: &Vec<Vec<usize>>, visable: &mut Vec<Vec<bool>>, height: usize, width: usize, idx: usize, jdx: usize) -> bool {
    true
}

pub fn p08() {
    let contents = r#"30373
25512
65332
33549
35390"#;
    
    let contents = &std::fs::read_to_string("./assets/adv2022/adv08.txt").unwrap();

    let (_, trees) = p08_parse_height(&contents).unwrap();
    let height = trees.len();
    assert!(height > 0);
    let width = trees[0].len();
    assert!(width > 0);

    let mut visable = vec![vec![false; width]; height];
    let mut max_left_to_right = vec![vec![-1isize; width]; height];
    let mut max_top_to_bottom = vec![vec![-1isize; width]; height];
    let mut max_right_to_left = vec![vec![-1isize; width]; height];
    let mut max_bottom_to_top = vec![vec![-1isize; width]; height];
    for idx in 0..height {
        max_left_to_right[idx][1] = trees[idx][0] as isize;
        for jdx in 2..width {
            max_left_to_right[idx][jdx] = max_left_to_right[idx][jdx-1].max( trees[idx][jdx-1] as isize );
        }
    }
    for jdx in 0..width {
        max_top_to_bottom[1][jdx] = trees[0][jdx] as isize;
        for idx in 2..height {
            max_top_to_bottom[idx][jdx] = max_top_to_bottom[idx-1][jdx].max( trees[idx-1][jdx] as isize );
        }
    }
    for idx in 0..height {
        max_right_to_left[idx][width-2] = trees[idx][width-1] as isize;
        for jdx in (0..(width-2)).rev() {
            max_right_to_left[idx][jdx] = max_right_to_left[idx][jdx+1].max( trees[idx][jdx+1] as isize);
        }
    }
    for jdx in 0..width {
        max_bottom_to_top[height-2][jdx] = trees[height-1][jdx] as isize;
        for idx in (0..(height-2)).rev() {
            max_bottom_to_top[idx][jdx] = max_bottom_to_top[idx+1][jdx].max(trees[idx+1][jdx] as isize);
        }
    }

    for idx in 0..height {
        for jdx in 0..width {
            if idx == 0 || jdx == 0 || idx == height - 1 || jdx == width - 1 { 
                visable[idx][jdx] = true;
            } else {
                let height = trees[idx][jdx];
                #[allow(unused_parens)]
                if (
                    height as isize > max_left_to_right[idx][jdx] ||
                    height as isize > max_right_to_left[idx][jdx] ||
                    height as isize > max_top_to_bottom[idx][jdx] ||
                    height as isize > max_bottom_to_top[idx][jdx]
                ) {
                    visable[idx][jdx] = true;
                }
            }
        }
    }
    // eprintln!("{:?}", trees);
    // eprintln!("{:?}", visable);
    let mut count = 0;
    for idx in 0..height {
        for jdx in 0..width {
            if visable[idx][jdx] { count += 1; }
        }
    }
    eprintln!("Tot: {}", count);

    let mut max_left_to_right = vec![vec![1usize; width]; height];
    let mut max_top_to_bottom = vec![vec![1usize; width]; height];
    let mut max_right_to_left = vec![vec![1usize; width]; height];
    let mut max_bottom_to_top = vec![vec![1usize; width]; height];
    for idx in 0..height {
        for jdx in 1..width {
            let mut next = max_left_to_right[idx][jdx];
            while jdx > next && trees[idx][jdx-next] < trees[idx][jdx] {
                max_left_to_right[idx][jdx] += max_left_to_right[idx][jdx-next];
                next = max_left_to_right[idx][jdx];
            }
        }
    }
    for jdx in 0..width {
        for idx in 1..height {
            let mut next = max_top_to_bottom[idx][jdx];
            while idx > next && trees[idx-next][jdx] < trees[idx][jdx] {
                max_top_to_bottom[idx][jdx] += max_top_to_bottom[idx-next][jdx];
                next = max_top_to_bottom[idx][jdx];
            }
        }
    }
    for idx in 0..height {
        for jdx in (0..(width-1)).rev() {
            let mut next = max_right_to_left[idx][jdx];
            while jdx + next < width - 1 && trees[idx][jdx+next] < trees[idx][jdx] {
                max_right_to_left[idx][jdx] += max_right_to_left[idx][jdx+next];
                next = max_right_to_left[idx][jdx];
            }
        }
    }
    for jdx in 0..width {
        for idx in (1..(height-1)).rev() {
            let mut next = max_bottom_to_top[idx][jdx];
            while idx + next < height - 1 && trees[idx+next][jdx] < trees[idx][jdx] {
                max_bottom_to_top[idx][jdx] += max_bottom_to_top[idx+next][jdx];
                next = max_bottom_to_top[idx][jdx];
            }
        }
    }

    let mut score = 1;
    for idx in 0..height {
        for jdx in 0..width {
            let lr = max_left_to_right[idx][jdx];
            let tb = max_top_to_bottom[idx][jdx];
            let rl = max_right_to_left[idx][jdx];
            let bt = max_bottom_to_top[idx][jdx];
            if idx == 0 || jdx == 0 || idx == height - 1 || jdx == width - 1 { 
            } else {
                score = score.max(lr * tb *rl * bt);
            }
            // print!("({} {} {} {}) ", tb, lr, bt, rl);
        }
        // println!();
    }
    eprintln!("Ideal: {}", score);
}

#[derive(Debug, Copy, Clone)]
enum P09Motion { R, U,L,D }

fn p09_parse_motions(input: &str) -> nom::IResult<&str, Vec<(P09Motion, usize)>> {
    use P09Motion::*;

    let motion = nom::branch::alt((
        nom::combinator::value(U, nom::character::complete::char::<&str, _>('U')),
        nom::combinator::value(D, nom::character::complete::char::<&str, _>('D')),
        nom::combinator::value(L, nom::character::complete::char::<&str, _>('L')),
        nom::combinator::value(R, nom::character::complete::char::<&str, _>('R')),
    ));
    nom::multi::separated_list0(
        nom::character::complete::newline, 
        nom::sequence::tuple((
            nom::sequence::terminated(motion, nom::character::complete::space1),
            nom::combinator::map(nom::character::complete::u64, |s| s as usize),
        )),
    )(input)
}

#[derive(Debug, Default)]
struct P07Position {
    hx: isize,
    hy: isize,
    tx: isize,
    ty: isize,
}

impl P07Position {
    pub fn walk(&mut self, motion: P09Motion) {
        use P09Motion::*;

        match motion {
            L => { self.hx -= 1; },
            R => { self.hx += 1; },
            U => { self.hy += 1; },
            D => { self.hy -= 1; },
        }

        if self.ty + 1 < self.hy {
            self.ty = self.ty + 1;
            self.tx = self.hx;
        } 

        if self.ty > self.hy + 1 {
            self.ty = self.ty - 1;
            self.tx = self.hx;
        }

        if self.tx > self.hx + 1 {
            self.tx = self.tx - 1;
            self.ty = self.hy;
        }

        if self.tx + 1 < self.hx {
            self.tx = self.tx + 1;
            self.ty = self.hy;
        }
    }
}

fn p09_motion(motion: P09Motion, (hx, hy): &mut (isize, isize)) {
    use P09Motion::*;

    match motion {
        L => { *hx -= 1; },
        R => { *hx += 1; },
        U => { *hy += 1; },
        D => { *hy -= 1; },
    }
}

fn p09_walk((hx, hy): &mut (isize, isize), (tx, ty): &mut (isize, isize)) {
    let hsize = *hx - *tx; let vsize = *hy - *ty;
    // 先排除相邻点位，剩下的规则就简单多了
    if hsize.abs() + vsize.abs() <= 1 { return; }
    if hsize.abs() == 1 && vsize.abs() == 1 { return; }

    if hsize > 0 { *tx += 1; }
    if hsize < 0 { *tx -= 1; }
    if vsize > 0 { *ty += 1; }
    if vsize < 0 { *ty -= 1; }
}

fn p09_display_knots(knots: &[(isize, isize)]) {
    let mut x0 = 0; let mut x1 = 0;
    let mut y0 = 0; let mut y1 = 0;
    for &(px, py) in knots.iter() {
        x0 = x0.min(px); x1 = x1.max(px);
        y0 = y0.min(py); y1 = y1.max(py);
    }
    let mut bars = vec![vec![9999; (x1-x0) as usize + 1]; (y1-y0) as usize + 1];
    bars[(0-y0) as usize][(0-x0) as usize] = 9998;
    for (idx, (px, py)) in knots.iter().enumerate() {
        bars[(py-y0) as usize][(px-x0) as usize] = idx.min(bars[(py-y0) as usize][(px-x0) as usize]);
    }
    bars.reverse();
    // eprintln!("{:?}", (x0, y0, x1, y1));
    for bar in bars.into_iter() {
        for b in bar.into_iter() {
            if b <= 9 {
                print!("{}", b);
            }  else if b == 9998 {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn p09_display<'a>(points_iter: impl Iterator<Item=&'a (isize, isize)>) {
    let mut points: Vec<(isize, isize)> = vec![];
    let mut x0 = 0; let mut x1 = 0;
    let mut y0 = 0; let mut y1 = 0;
    for &(px, py) in points_iter {
        x0 = x0.min(px); x1 = x1.max(px);
        y0 = y0.min(py); y1 = y1.max(py);
        points.push((px, py));
    }
    let mut bars = vec![vec![false; (x1-x0) as usize + 1]; (y1-y0) as usize + 1];
    for (px, py) in points.into_iter() {
        bars[(py-y0) as usize][(px-x0) as usize] = true;
    }
    bars.reverse();
    // eprintln!("{:?}", (x0, y0, x1, y1));
    for bar in bars.into_iter() {
        for b in bar.into_iter() {
            if !b { print!("."); } else { print!("#"); }
        }
        println!();
    }
}

pub fn p09() {
    let contents = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
    let contents = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    let contents = &std::fs::read_to_string("./assets/adv2022/adv09.txt").unwrap();

    let (input, motions) = p09_parse_motions(contents).unwrap();
    // eprintln!("{:?}", motions);

    let mut start = P07Position::default();
    let mut tails = HashSet::new();
    tails.insert((start.tx, start.ty));
    for &(md, mx) in motions.iter() {
        for _ in 0..mx {
            start.walk(md);
            tails.insert((start.tx, start.ty));
        }
    }
    // eprintln!("{}: {:?}", tails.len(), tails);
    eprintln!("{}", tails.len());

    const KNOT_LEN: usize = 10;
    let mut knots: [(isize, isize); KNOT_LEN] = [(0, 0); KNOT_LEN];
    let mut tails = HashSet::new();
    tails.insert(knots.last().cloned().unwrap());
    for (idx, &(md, mx)) in motions.iter().enumerate() {
        for _ in 0..mx {
            p09_motion(md, knots.first_mut().unwrap());
            for idx in 1..KNOT_LEN {
                let (left, right) = knots.split_at_mut(idx);
                let head = left.last_mut().unwrap();
                let tail = right.first_mut().unwrap();
                p09_walk(head, tail);
            }
            tails.insert(knots.last().cloned().unwrap());
        }
        // p09_display_knots(&knots);
        // eprintln!();

    }
    // eprintln!("{}: {:?}", tails.len(), tails);
    eprintln!("{}", tails.len());
    // p09_display(tails.iter());
}

pub fn p10() {
    let contents = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    let contents = &std::fs::read_to_string("./assets/adv2022/adv10.txt").unwrap();

    let (_, instructions) = p10_parse_instruction(contents).unwrap();
    // eprintln!("{:?}", instructions);

    let mut startx: isize = 1; let mut cycle: usize = 0;
    let mut values: Vec<isize> = vec![];
    for &instr in instructions.iter() {
        match instr {
            P10Instruction::Noop => { 
                values.push(startx);
                cycle += 1; 
            },
            P10Instruction::Addx(x) => {
                values.push(startx);
                startx += x;
                values.push(startx);
                cycle += 2;
            }
        }
    }
    // eprintln!("{:?}", values);
    let mut sum = 0;
    for idx in 0.. {
        let offset = 20 + idx * 40 - 1;
        if offset > values.len() { break; }
        let part = values[offset-1] * (offset as isize + 1);
        sum += part;
        eprintln!("{:?} {}", &values[offset-4..offset], part);
    }
    eprintln!("Finalize: {}", sum);

    
    let mut cursor = 0;
    let mut line = vec![];
    let mut sprite = 1;
    let add_symbol = |idx: isize, sprite: isize| -> char {
        if idx >= sprite - 1 && idx <= sprite + 1 { '#' } else { '.' }
    };
    for &instr in instructions.iter() {
        match instr {
            P10Instruction::Noop => { 
                line.push(add_symbol((cursor % 40) as isize, sprite));
                cursor += 1;
            },
            P10Instruction::Addx(x) => {
                line.push(add_symbol((cursor % 40) as isize, sprite));
                cursor += 1;
                line.push(add_symbol((cursor % 40) as isize, sprite));
                sprite += x;
                cursor += 1;
            }
        }
    }

    for idx in 0..line.len() {
        eprint!("{}", line[idx]);
        if (idx+1) % 40 == 0 { eprintln!(); }
    }
}

#[derive(Clone, Copy, Debug)]
enum P10Instruction { Addx(isize), Noop }

fn p10_parse_instruction(input: &str) -> nom::IResult<&str, Vec<P10Instruction>> {
    let noop = nom::bytes::complete::tag("noop");
    let addx = nom::sequence::tuple(( 
            nom::bytes::complete::tag("addx"),
            nom::character::complete::space1,
            nom::character::complete::i64,
    ));

    let instruction = nom::branch::alt((
            nom::combinator::map(addx, |(_, _, x)| P10Instruction::Addx(x as isize)),
            nom::combinator::value(P10Instruction::Noop, noop)
        ));
    nom::multi::separated_list1(
        nom::character::complete::newline,
        instruction
    )(input)
}

pub fn p11() {
    let contents = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    let contents = &std::fs::read_to_string("./assets/adv2022/adv11.txt").unwrap();

    let (left, monkeys_orig) = p11_parse(contents).unwrap();
    let mut monkeys = monkeys_orig.clone();
    eprintln!("{:?}", left);
    eprintln!("{:?}", monkeys);

    for _ in 0..20 {
        p11_round(&mut monkeys);

        for mk in monkeys.iter() {
            eprintln!("Monkey {}: {:?}", mk.id, mk.items);
        }
        eprintln!();
    }

    eprintln!("{:?}", Vec::from_iter(monkeys.iter().map(|mk| mk.inspects_counter)));

    ////////////////////////////////////////////////////////////////////////////
    let mut monkeys = monkeys_orig.clone();
    let primitives: Vec<usize> = monkeys.iter().map(|mk| mk.test).collect();
    for mk in monkeys.iter_mut() {
        for &item in mk.items.iter() {
            mk.rems.push(primitives.iter().map(|&p| (p, item % p)).collect())
        }
    }

    for idx in 0..10000 {
        p11_round2(&mut monkeys);

        if idx == 0 || idx == 19 || (idx + 1) % 1000 == 0 {
            eprint!("ROUND {} ", idx+1);
            eprintln!("{:?}", Vec::from_iter(monkeys.iter().map(|mk| mk.inspects_counter)));
            eprintln!();
        }
    }
    let mut out = Vec::from_iter(monkeys.iter().map(|mk| mk.inspects_counter));
    out.sort(); out.reverse();
    eprintln!("{:?}: {}", out, out[0] * out[1]);
}

fn p11_parse_monkey(input: &str) -> nom::IResult<&str, P11Monkey> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{newline, space1, space0};
    use nom::sequence::tuple;
    use nom::combinator::map;

    let (input, _) = tag("Monkey ")(input)?;
    let (input, id) = nom::character::complete::u64(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = newline(input)?;

    ////////////////////////////////////////////////////////////////////////////
    let (input, _) = space1(input)?;
    let (input, _) = tag("Starting items: ")(input)?;
    let (input, items) = nom::multi::separated_list0(tuple((tag(","), space0)), nom::character::complete::u64)(input)?;
    let (input, _) = newline(input)?;

    ////////////////////////////////////////////////////////////////////////////
    let (input, _) = space1(input)?;
    let (input, _) = tag("Operation: new = old")(input)?;
    let (input, _) = space0(input)?;

    let op_plus = tuple((tag("+"), space0, nom::character::complete::i64));
    let op_minus = tuple((tag("-"), space0, nom::character::complete::i64));
    let op_mul = tuple((tag("*"), space0, nom::character::complete::i64));
    // let op_div = tuple((tag("/"), space0, nom::character::complete::i64));
    let op_square = tuple((tag("*"), space0, tag("old")));

    let (input, op) = nom::branch::alt((
        map(op_plus, |(_, _, x)| P11Operation::Add(x as isize)),
        map(op_minus, |(_, _, x)| P11Operation::Add(-x as isize)),
        map(op_mul, |(_, _, x)| P11Operation::Mul(x as isize)),
        map(op_square, |(_, _, _)| P11Operation::Square),
    ))(input)?;
    let (input, _) = newline(input)?;

    ////////////////////////////////////////////////////////////////////////////
    let (input, _) = space1(input)?;
    let (input, _) = tag("Test: divisible by ")(input)?;
    let (input, test) = nom::character::complete::u64(input)?;
    let (input, _) = newline(input)?;

    ////////////////////////////////////////////////////////////////////////////
    let (input, _) = space1(input)?;
    let (input, _) = tag("If true: throw to monkey ")(input)?;
    let (input, true_to) = nom::character::complete::u64(input)?;
    let (input, _) = newline(input)?;

    ////////////////////////////////////////////////////////////////////////////
    let (input, _) = space1(input)?;
    let (input, _) = tag("If false: throw to monkey ")(input)?;
    let (input, false_to) = nom::character::complete::u64(input)?;

    let m = P11Monkey {
        id: id as usize,
        items: items.into_iter().map(|x| x as usize).collect(),
        operation: op,
        test: test as usize,
        true_to: true_to as usize,
        false_to: false_to as usize,

        inspects_counter: 0,
        rems: Default::default(),
    };
    Ok((input, m))
}

fn p11_parse(input: &str) -> nom::IResult<&str, Vec<P11Monkey>> {
    nom::multi::separated_list0(
        nom::multi::many1(nom::character::complete::newline),
        p11_parse_monkey,
    )(input)
}

#[derive(Debug, Clone, Copy)]
enum P11Operation {
    Add(isize),
    Mul(isize),
    Square,
}

impl P11Operation {
    fn update(&self, old: usize) -> usize {
        match self {
            P11Operation::Add(y) => (old as isize + y) as usize,
            P11Operation::Mul(y) => (old as isize * y) as usize,
            P11Operation::Square => old * old,
        }
    }
}

#[derive(Debug, Clone)]
struct P11Monkey {
    id: usize,
    items: Vec<usize>,
    operation: P11Operation,
    test: usize,
    true_to: usize,
    false_to: usize,

    inspects_counter: usize,
    rems: Vec<HashMap<usize, usize>>,
}

fn p11_round(monkeys: &mut Vec<P11Monkey>) {
    for idx in 0..monkeys.len() {
        for item in std::mem::replace(&mut monkeys[idx].items, Default::default()).into_iter() {
            monkeys[idx].inspects_counter += 1;
            let item = monkeys[idx].operation.update(item);
            let item = item / 3;
            let ydx = if item % monkeys[idx].test == 0 {
                monkeys[idx].true_to
            } else { 
                monkeys[idx].false_to
            };
            monkeys[ydx].items.push(item);
        }
    }
}

fn p11_round2(monkeys: &mut Vec<P11Monkey>) {
    for idx in 0..monkeys.len() {
        for mut item_base in std::mem::replace(&mut monkeys[idx].rems, Default::default()).into_iter() {
            monkeys[idx].inspects_counter += 1;

            for (prim, rem) in item_base.iter_mut() {
                *rem = (monkeys[idx].operation.update(*rem)) % prim;
            }
            let is_divisable = *item_base.get(&monkeys[idx].test).unwrap() == 0;

            // let item = item / 3;
            let ydx = if is_divisable {
                monkeys[idx].true_to
            } else { 
                monkeys[idx].false_to
            };

            monkeys[ydx].rems.push(item_base);
        }
    }
}

pub fn p12_parse(input: &str) -> nom::IResult<&str, ((usize, usize), (usize, usize), Vec<Vec<usize>>)> {
    let (input, maze) = nom::multi::separated_list1(
        nom::character::complete::newline, 
        nom::multi::many1(nom::character::complete::satisfy(|c| c.is_alphabetic()))
    )(input)?;
    let (input, _) = nom::combinator::opt(nom::character::complete::newline)(input)?;
    nom::combinator::eof(input)?;

    let height = maze.len();
    assert!(height > 0);
    let width = maze[0].len();
    assert!(width > 0);

    let mut out = vec![vec![0; width]; height];
    let mut sx = 0; let mut sy = 0;
    let mut ex = 0; let mut ey = 0;
    for idx in 0..height {
        for jdx in 0..width {
            let c = maze[idx][jdx];
            if c == 'S' {
                sx = jdx;
                sy = idx;
                out[idx][jdx] = ('a' as u8 - 'a' as u8) as usize;
            } else if c == 'E' {
                ex = jdx;
                ey = idx;
                out[idx][jdx] = ('z' as u8 - 'a' as u8) as usize;
            } else {
                out[idx][jdx] = (c as u8 - 'a' as u8) as usize;
            }
        }
    }

    Ok((input, ((sx, sy), (ex, ey), out)))
}

pub fn p12() {
    let contents = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    let contents = &std::fs::read_to_string("./assets/adv2022/adv12.txt").unwrap();

    let (_, (start, stop, maze)) = p12_parse(contents).unwrap();
    // eprintln!("{:?} {:?} {:?}", start, stop, maze);

    let height = maze.len();
    assert!(height > 0);
    let width = maze[0].len();
    assert!(width > 0);

    let mut stack = vec![vec![-1; width]; height];
    let mut points = vec![start]; let mut level = 0; stack[start.1][start.0] = level;
    while points.len() > 0 && !points.iter().any(|&p| p == stop) {
        level += 1;
        p12_decent(height, width, &maze, level, &mut points, &mut stack, false);
    }

    // for line in stack.iter() {
    //     for x in line.iter() {
    //         print!("{: >3}", x);
    //     }
    //     println!();
    // }
    eprintln!("Found: {}", stack[stop.1][stop.0]);

    let mut stack = vec![vec![-1; width]; height];
    let mut points = vec![stop]; let mut level = 0; stack[stop.1][stop.0] = level;
    while points.len() > 0 && !points.iter().any(|&p| p == start) {
        level += 1;
        p12_decent(height, width, &maze, level, &mut points, &mut stack, true);
    }

    let mut available = vec![];
    for (idx, line) in stack.iter().enumerate() {
        for (jdx, x) in line.iter().enumerate() {
            // print!("{: >3}", x);
            if maze[idx][jdx] == 0 && *x >= 0 {
                available.push((x, (idx, jdx)));
            }
        }
        // println!();
    }
    available.sort();
    // eprintln!("Found: {}", stack[start.1][start.0]);
    eprintln!("Available: {:?}", available);
}

fn p12_decent(
    height: usize,
    width: usize,
    maze: &Vec<Vec<usize>>,
    level: isize,
    points: &mut Vec<(usize, usize)>,
    stack: &mut Vec<Vec<isize>>,
    reversed: bool,
) {
    for (sx, sy) in std::mem::replace(points, Default::default()).into_iter() {
        for (vx, vy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = sx as isize + vx;
            let ny = sy as isize + vy;
            if nx < 0 || ny < 0 || nx >= (width as isize) || ny >= (height as isize) { continue; }
            let nx = nx as usize; let ny = ny as usize;

            if stack[ny][nx] >= 0 { continue; }
            if reversed {
                if !(maze[ny][nx] + 1 >= maze[sy][sx]) { continue; }
            } else {
                if !(maze[ny][nx] <= maze[sy][sx] + 1) { continue; }
            }

            stack[ny][nx] = level;
            points.push((nx, ny));
        } 
    }
}

fn p12_search(
    height: usize,
    width: usize,
    maze: &Vec<Vec<usize>>,
    (ex, ey): (usize, usize),
    (sx, sy): (usize, usize),
    stack: &mut Vec<Vec<isize>>,
    traces: &mut Vec<(usize, usize)>,
) -> isize {
    if ex == sx && ey == sy { 
        // eprintln!("Found: {:?} {}", traces, traces.len());
        return 0; 
    }
    if stack[sy][sx] >= 0 { return stack[sy][sx]; }
    if stack[sy][sx] == -2 { return stack[sy][sx]; }

    let mut opt = isize::MAX;
    for (vx, vy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let nx = sx as isize + vx;
        let ny = sy as isize + vy;
        if nx < 0 || ny < 0 || nx >= (width as isize) || ny >= (height as isize) { continue; }
        let nx = nx as usize; let ny = ny as usize;
        if traces.iter().any(|old| *old == (nx, ny)) { continue; }

        if maze[ny][nx] + 1 >= maze[sy][sx] {
            let remain = traces.len();
            traces.push((sx, sy));
            let mem = p12_search(height, width, maze, (ex, ey), (nx, ny), stack, traces);
            traces.truncate(remain);

            if mem == -2 { continue; }

            assert!(mem != -1);
            opt = opt.min(mem + 1);
        }
    } 

    if opt == isize::MAX {
        stack[sy][sx] = -2;
    } else {
        stack[sy][sx] = opt;
    }

    stack[sy][sx]
}

pub fn p13() {
    let contents = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    let contents = &std::fs::read_to_string("./assets/adv2022/adv13.txt").unwrap();

    let (_, items) = p13_parse(contents).unwrap();
    let mut sum = 0;
    for (idx, (left, right)) in items.iter().enumerate() {
        if left < right {
            sum += idx + 1;
            dbg!(sum);
        }
        // eprintln!();
    }
    eprintln!("{:?}", sum);

    let mut signals: Vec<_> = items.clone().into_iter().map(|(a, b)| [a, b]).into_iter().flatten().collect();
    let s0 = p13_parse_list_or_item("[[2]]").unwrap().1;
    let s1 = p13_parse_list_or_item("[[6]]").unwrap().1;
    signals.push(s0.clone());
    signals.push(s1.clone());
    signals.sort();
    
    // for s in signals.iter() { println!("{}", s); }
    let mut prod = 1;

    for (idx, s) in signals.iter().enumerate() {
        #[allow(unused_parens)]
        if s == &s0 || s == &s1 {
            prod *= (idx + 1);
            eprintln!("idx: {}", idx + 1)
        }
    }
    eprintln!("Product of signal: {}", prod);
}

impl std::cmp::PartialOrd for P13Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use P13Item::*;
        // eprintln!("{} vs {}", self, other);

        match (self, other) {
            (Item(left), Item(right)) => left.partial_cmp(right),
            (List(left), List(right)) => {
                let mut offset = 0;
                loop {
                    if left[offset..].len() == 0 && right[offset..].len() == 0 {
                        return Some(std::cmp::Ordering::Equal);
                    } else if left[offset..].len() == 0 {
                        return Some(std::cmp::Ordering::Less);
                    } else if right[offset..].len() == 0 {
                        return Some(std::cmp::Ordering::Greater)
                    } else {
                        match left[offset].partial_cmp(&right[offset]) {
                            None => {},
                            Some(std::cmp::Ordering::Equal) => {},
                            Some(v) => return Some(v),
                        }
                    }
                    offset += 1;
                }
            },
            (Item(left), List(right)) => { 
                List(vec![Item(*left)]).partial_cmp(other)
            }
            (List(left), Item(right)) => {
                self.partial_cmp(&List(vec![Item(*right)]))
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Ord)]
enum P13Item {
    List(Vec<P13Item>),
    Item(usize),
}

impl std::fmt::Debug for P13Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for P13Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            P13Item::Item(v) => v.fmt(f),
            P13Item::List(vs) => {
                write!(f, "[")?;
                for v in vs.iter() { v.fmt(f)?; write!(f, ", ")?; }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

fn p13_parse_item(input: &str) -> nom::IResult<&str, P13Item> {
    nom::combinator::map(nom::character::complete::u64, |v| P13Item::Item(v as usize))(input)
}

fn p13_parse_list(input: &str) -> nom::IResult<&str, P13Item> {
    nom::sequence::delimited(
        nom::character::complete::char('['), 
        nom::combinator::map(
            nom::multi::separated_list0(nom::character::complete::char(','), p13_parse_list_or_item),
            |vs| P13Item::List(vs)
        ),
        nom::character::complete::char(']'), 
    )(input)
}

fn p13_parse_list_or_item(input: &str) -> nom::IResult<&str, P13Item> {
    nom::branch::alt((p13_parse_item, p13_parse_list))(input)
}

fn p13_parse(input: &str) -> nom::IResult<&str, Vec<(P13Item, P13Item)>> {
    let pair = nom::sequence::tuple((p13_parse_list_or_item, nom::character::complete::newline, p13_parse_list_or_item));
    nom::multi::separated_list0(
        nom::multi::count(nom::character::complete::newline, 2),
        nom::combinator::map(pair, |(a, _, b)| (a, b)),
    )(input)
}

pub fn p14() {
    let contents = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    let contents = &std::fs::read_to_string("./assets/adv2022/adv14.txt").unwrap();

    let (_, rocks) = p14_parse(contents).unwrap();
    let source = (500, 0);
    let ((x0, y0), (x1, y1)) = p14_locate( rocks.iter().flatten().chain( std::iter::once(&source)) ).unwrap();
    let width = (x1 - x0 + 1) as usize;
    let height = (y1 - y0 + 1) as usize;

    let mut cave_orig = vec![vec![0; width]; height];
    for line in rocks.iter() {
        for (&(rx0, ry0), &(rx1, ry1)) in line.iter().zip(line.iter().skip(1)) {
            let points: Vec<(isize, isize)> = if rx0 == rx1 {
                if ry1 >= ry0 {
                    (ry0..ry1+1).map(|y| (rx0, y)).collect()
                } else {
                    (ry1..ry0+1).map(|y| (rx0, y)).collect()
                }
            } else {
                assert_eq!(ry0, ry1);
                if rx1 >= rx0 {
                    (rx0..rx1+1).map(|x| (x, ry0)).collect()
                } else {
                    (rx1..rx0+1).map(|x| (x, ry0)).collect()
                }
            };

            for (rx, ry) in points.into_iter() {
                cave_orig[(ry-y0) as usize][(rx- x0) as usize] = 1
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    let mut cave = cave_orig.clone();
    let mut count = 0;
    while let Some(rest) = p14_make_sand(&mut cave, width, height, (x0, y0), source) {
        eprintln!("rest@{:?}", rest);
        count += 1;
        // assert!(count <= 15);
    }
    eprintln!("Rested: {}", count);

    eprintln!("==================================");
    ////////////////////////////////////////////////////////////////////////////
    let mut count = 0;

    let mut cave = P12Cave::new(
        cave_orig.clone(),
        width,
        height,
        x0,
        y0,
        source.0,
        source.1,

        2,
    );

    while let Some(rest) = cave.step() {
        // eprintln!("rest@{:?}", rest);
        count += 1;
        // assert!(count <= 30);
    }
    eprintln!("Rested: {}", count + 1);
}

fn p14_locate<'a>(points: impl Iterator<Item = &'a (isize, isize)>) -> Option<((isize, isize), (isize, isize))> {
    let mut x0 = isize::MAX; let mut y0 = isize::MAX;
    let mut x1 = isize::MIN; let mut y1 = isize::MIN;
    let mut at_least_one_point = false;
    for &(x, y) in points {
        at_least_one_point = true;
        x0 = x0.min(x); x1 = x1.max(x);
        y0 = y0.min(y); y1 = y1.max(y);
    }

    if at_least_one_point {
        Some(((x0, y0), (x1, y1)))
    } else {
        None
    }
}

fn p14_parse(input: &str) -> nom::IResult<&str, Vec<Vec<(isize, isize)>>> {
    let point = nom::sequence::tuple((
        nom::character::complete::i64,
        nom::character::complete::char(','),
        nom::character::complete::i64,
    ));
    let point = nom::combinator::map(point, |(a, _, b)| (a as isize, b as isize));
    let row = nom::multi::separated_list0(
        nom::sequence::tuple((
                nom::character::complete::space0, nom::bytes::complete::tag("->"), nom::character::complete::space0
        )),
        point
    );
    nom::multi::separated_list1(
        nom::character::complete::newline, 
        row,
    )(input)
}

fn p14_make_sand(
    cave: &mut Vec<Vec<usize>>, width: usize, height: usize, (x0, y0): (isize, isize), (mut sx, mut sy): (isize, isize)
) -> Option<(isize, isize)> {
    loop {
        let mut is_rest =  true; 
        for (vx, vy) in [(0, 1), (-1, 1), (1, 1)] {
            let nx = sx + vx; let ny = sy + vy;
            if nx < x0 || nx >= x0 + width as isize || ny < y0 || ny >= y0 + height as isize { return None; }

            if cave[(ny-y0) as usize][(nx-x0) as usize] == 0 {
                sx = nx; sy = ny;
                is_rest = false;
                break;
            }
        }

        if is_rest {
            cave[(sy-y0) as usize][(sx-x0) as usize] = 1;
            return Some((sx, sy));
        }
    }
}

struct P12Cave {
    filled: Vec<Vec<usize>>, 
    width: usize,
    height: usize, 
    center_x: isize,
    center_y: isize,
    source_x: isize,
    source_y: isize,

    limit_y: isize,
}

impl P12Cave {
    pub fn new(
        filled: Vec<Vec<usize>>, 
        width: usize,
        height: usize, 
        center_x: isize,
        center_y: isize,
        source_x: isize,
        source_y: isize,

        limit_y: usize,
    ) -> Self {
        let mut slf = P12Cave { filled, width, height, center_x, center_y, source_x, source_y, limit_y: 0, };
        slf.limit_y = slf.center_y + (slf.height as isize) - 1 + (limit_y as isize);
        slf
    }

    pub fn adjust_with(&mut self, (nx, ny): (isize, isize)) {
        if nx >= self.center_x + (self.width as isize) {
            self.extend_right(nx - (self.center_x + (self.width as isize)) + 1)
        }

        if nx < self.center_x {
            self.extend_left(self.center_x - nx);
        }

        if ny >= self.center_y + (self.height as isize) {
            self.extend_down(ny - (self.center_y + (self.height as isize)) + 1);
        }

        if ny < self.center_y {
            self.extend_up(self.center_y-ny);
        }
    }

    pub fn step(&mut self) -> Option<(isize, isize)> {
        let (mut sx, mut sy) = (self.source_x, self.source_y);

        loop {
            let mut is_rest =  true; 
            for (vx, vy) in [(0, 1), (-1, 1), (1, 1)] {
                let nx = sx + vx; let ny = sy + vy;
                self.adjust_with((nx, ny));

                if ny < self.limit_y && self.filled[(ny-self.center_y) as usize][(nx-self.center_x) as usize] == 0 {
                    sx = nx; sy = ny;
                    is_rest = false;
                    break;
                }
            }

            if is_rest {
                self.filled[(sy-self.center_y) as usize][(sx-self.center_x) as usize] = 1;
                if (sx, sy) == (self.source_x, self.source_y) {
                    return None;
                } else {
                    return Some((sx, sy));
                }
            }
        }
    }

    fn extend_right(&mut self, extend: isize) {
        assert!(extend > 0);
        self.width += 1;
        for row in self.filled.iter_mut() {
            row.push(0);
        }
    }

    fn extend_left(&mut self, extend: isize) {
        assert!(extend > 0);
        self.center_x -= 1;
        for row in self.filled.iter_mut() {
            row.insert(0, 0);
        }
    }

    fn extend_down(&mut self, extend: isize) {
        assert!(extend > 0);
        self.height += 1;
        self.filled.push(vec![0; self.width]);
    }

    fn extend_up(&mut self, extend: isize) {
        assert!(extend > 0);
        self.center_y -= 1;
        self.filled.insert(0, vec![0; self.width]);
    }
}

fn p15_parse(input: &str) -> nom::IResult<&str, Vec<((isize, isize), (isize, isize))>> {
    let line = nom::combinator::map(
        nom::sequence::tuple((
            nom::bytes::complete::tag("Sensor at x="),
            nom::character::complete::i64,
            nom::bytes::complete::tag(", y="),
            nom::character::complete::i64,
            nom::bytes::complete::tag(": closest beacon is at x="),
            nom::character::complete::i64,
            nom::bytes::complete::tag(", y="),
            nom::character::complete::i64,
        )),
        |(_, sx, _, sy, _, bx, _, by)| ((sx as isize, sy as isize), (bx as isize, by as isize))
    );

    nom::multi::separated_list0(
        nom::character::complete::newline, 
        line
    )(input)
}

pub fn p15() {
    let contents = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
    let contents = &std::fs::read_to_string("./assets/adv2022/adv15.txt").unwrap();

    let (_, sensors) = p15_parse(contents).unwrap();
    let ((x0, y0), (x1, y1)) = p15_window(sensors.iter().map(|(p1, p2)| [p1, p2]).flatten());

    // Too slow...
    if false {
        let mut map = P15Map::new(x0, y0, x1, y1);

        for &(spos, bpos) in sensors.iter() {
            map.add_closest_beacon(spos, bpos);
        }

        eprintln!("Beacons: {}", map.beacons.iter().flatten().filter(|&&v| v == 3).count());
        eprintln!("Beacons@10: {}", map.beacons[map.locate((0, 10)).unwrap().1].iter().filter(|&&v| v == 3).count());
        map.display();
    }


    let (mut segments, blocked) = p15_search(10, &sensors);
    segments.sort();
    eprintln!("{:?}", segments);
    eprintln!(
        "Count: {}", 
        segments.iter().map(|(x, y)| y - x).sum::<isize>() - (blocked.len() as isize)
    );

    println!("===========================================");
    // const MAX_LEN: isize = 20;
    const MAX_LEN: isize = 4000000;
    for yy in 0..MAX_LEN {
        let (segments, blocked) = p15_search_inside(yy, &sensors, 0, MAX_LEN+1);
        let not_possible = segments.iter().map(|(x, y)| y - x).sum::<isize>();
        // eprintln!("row@{}: {}", yy, not_possible);
        if not_possible <= MAX_LEN {
            let possible: Vec<_> = (0..MAX_LEN+1).into_iter().filter(|&xx| { 
                segments.iter().all(|&(x, y)| !(x <= xx && xx < y))
            }).collect();
            eprintln!("Found within range {}: {:?} x {} = {}[0]", MAX_LEN, possible, yy, yy + possible[0] * 4000000);
        }
    }
}

struct P15Map {
    beacons: Vec<Vec<u8>>,
    x0: isize,
    y0: isize,
    width: usize,
    height: usize,
}

impl P15Map {
    fn new(x0: isize, y0: isize, x1: isize, y1: isize) -> Self {
        let width = (x1 - x0) as usize + 1;
        let height = (y1 - y0) as usize + 1;

        let beacons = vec![vec![0; width]; height];
        Self { beacons, x0, y0, width, height }
    }

    fn locate(&self, point: (isize, isize)) -> Option<(usize, usize)> {
        let (px, py) = point;
        let x = px - self.x0;
        let y = py - self.y0;

        if x >= 0 && x < (self.width as isize) && y >= 0 && y < (self.height as isize) {
            Some((x as usize, y as usize)) 
        } else { 
            None
        }
    }

    fn adjust_left(&mut self, offset: isize) {
        assert!(offset > 0);
        self.x0 -= offset;
        self.width += offset as usize;
        for row in self.beacons.iter_mut() { for _ in 0..offset { row.insert(0, 0); } }
    }

    fn adjust_right(&mut self, offset: isize) {
        assert!(offset > 0);
        self.width += offset as usize;
        for row in self.beacons.iter_mut() { for _ in 0..offset { row.push(0); } }
    }

    fn adjust_down(&mut self, offset: isize) {
        assert!(offset > 0);
        self.height += offset as usize;
        for _ in 0..offset { self.beacons.push(vec![0; self.width]); }
    }

    fn adjust_up(&mut self, offset: isize) {
        assert!(offset > 0);
        self.y0 -= offset;
        self.height += offset as usize;
        for _ in 0..offset { self.beacons.insert(0, vec![0; self.width]); }
    }

    fn adjust(&mut self, point: (isize, isize)) -> bool {
        let (px, py) = point;
        let x = px - self.x0;
        let y = py - self.y0;

        let mut is_adjusted = false;

        if x < 0 { is_adjusted = true; self.adjust_left(-x); }
        if x >= (self.width as isize) { is_adjusted = true; self.adjust_right(x - self.width as isize + 1)}
        if y < 0 { is_adjusted = true; self.adjust_up(-y); }
        if y >= self.height as isize { is_adjusted = true; self.adjust_down(y - self.height as isize + 1); }

        is_adjusted
    }

    fn add_closest_beacon(&mut self, spos: (isize, isize), bpos: (isize, isize)) {
        let (xs, ys) = self.locate(spos).unwrap();
        let (xb, yb) = self.locate(bpos).unwrap();
        self.beacons[ys][xs] = 1;
        self.beacons[yb][xb] = 2;

        let dist_max = (xb as isize - xs as isize).abs() + (yb as isize - ys as isize).abs();
        for dist in 1..dist_max+2 {
            // eprint!("{}: ", dist);
            for (vx, vy) in (0..dist).zip((0..dist).rev()) {
                for (dir_x, dir_y) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                    let xn = spos.0 + vx * dir_x; let yn = spos.1 + vy * dir_y;

                    // eprint!("({} {}) ", xn, yn);
                    self.adjust((xn, yn));
                    let (xn_offset, yn_offset) = self.locate((xn, yn)).unwrap();
                    if self.beacons[yn_offset][xn_offset] == 0 {
                        self.beacons[yn_offset][xn_offset] = 3;
                    }
                }
            }
            // eprintln!();
        }

        // panic!("{:?} {:?}", spos, bpos);
    }

    fn display(&self) {
        for idx in 0..self.height {
            for jdx in 0..self.width {
                let s = match self.beacons[idx][jdx] {
                    3 => 'x',
                    2 => 'B',
                    1 => 'S',
                    _ => '.',
                };
                print!("{}", s);
            }
            println!();
        }
    }
}

fn p15_add_segment(segments: &mut Vec<(isize, isize)>, left: isize, right: isize) {
    // dbg!(left, right);
    let mut ori = vec![(left, right)];
    for &(x0, y0) in segments.iter() {
        // eprintln!("{:?} vs {:?}", ori, (x0, y0));
        for (left, right) in std::mem::replace(&mut ori, Default::default()) {
            // if x0 >= right || y0 <= left { continue; } // no interaction
            // 不相交
            if left >= y0 || right <= x0 {
                ori.push((left, right));
            } 
            if left >= x0 && right <= y0 {

            }
            if left < x0 && right > x0 {
                ori.push((left, x0));
            }
            if right > y0 && left < y0 {
                ori.push((y0, right))
            }
        }
    }
    segments.append(&mut ori);
    // eprintln!("??? {:?}", segments);
}

fn p15_window<'a>(points_iter: impl Iterator<Item=&'a (isize, isize)>) -> ((isize, isize), (isize, isize)) {
    let mut x0 = isize::MAX; let mut y0 = isize::MAX;
    let mut x1 = isize::MIN; let mut y1 = isize::MIN;
    for &(px, py) in points_iter {
        x0 = x0.min(px); y0 = y0.min(py);
        x1 = x1.max(px); y1 = y1.max(py); 
    }

    assert_ne!(x0, isize::MAX, "no point found in the iterator?");
    ((x0, y0), (x1, y1))
}

fn p15_search(yy: isize, sensors: &[((isize, isize), (isize, isize))] ) -> (Vec<(isize, isize)>, HashSet<(isize, isize)>) {
    let mut segments: Vec<(isize, isize)> = vec![];
    for &((xs, ys), (xb, yb)) in sensors.iter() {
        let dist = (xb - xs).abs() + (yb - ys).abs();
        let y_dist = (ys - yy).abs();
        if y_dist > dist { continue; }
        let x_left = dist - y_dist;
        p15_add_segment(&mut segments, xs-x_left, xs+x_left+1);
    }

    let blocked = sensors.iter().map(|(s, b)| [s, b]).flatten().filter(|&&(x, y)| y == yy).map(Clone::clone).collect::<HashSet<_>>();
    (segments, blocked)
}

fn p15_search_inside(yy: isize, sensors: &[((isize, isize), (isize, isize))], limit_x0: isize, limit_x1: isize) -> (Vec<(isize, isize)>, HashSet<(isize, isize)>) {
    let mut segments: Vec<(isize, isize)> = vec![];
    for &((xs, ys), (xb, yb)) in sensors.iter() {
        let dist = (xb - xs).abs() + (yb - ys).abs();
        let y_dist = (ys - yy).abs();
        if y_dist > dist { continue; }
        let x_left = dist - y_dist;
        let x0_within_limt = (xs-x_left).max(limit_x0);
        let x1_within_limit = (xs+x_left+1).min(limit_x1);
        if x0_within_limt >= x1_within_limit { continue; }
        p15_add_segment(&mut segments, x0_within_limt, x1_within_limit);
    }

    let blocked = sensors.iter().map(|(s, b)| [s, b]).flatten().filter(|&&(x, y)| y == yy).map(Clone::clone).collect::<HashSet<_>>();
    (segments, blocked)
}

#[derive(Debug, Clone)]
pub struct P16Valve {
    name: String,
    to: Vec<String>,
    rate: usize,
}

#[derive(Debug)]
pub struct P16ValveMap {
    names_to_idx: HashMap<String, usize>,
    idx_to_names: Vec<String>,

    count: usize,
    rates: Vec<usize>,
    to: Vec<Vec<usize>>,
    shortest_dist: Vec<Vec<isize>>,
}

impl P16ValveMap {
    fn new(valves: &[P16Valve]) -> Self {
        let mut idx_to_names = vec![];
        let mut names_to_idx: HashMap<String, usize> = Default::default();
        let mut rates = vec![];
        let mut to = vec![];
        for (idx, valve) in valves.iter().enumerate() {
            idx_to_names.push(valve.name.clone());
            names_to_idx.insert(valve.name.clone(), idx);
            rates.push(valve.rate);
            to.push(Vec::with_capacity(valve.to.len()));
        }
        for (idx, valve) in valves.iter().enumerate() {
            for name in valve.to.iter() {
                to[idx].push(*names_to_idx.get(name).unwrap());
            }
        }

        // eprintln!(">>>>>");
        // for idx in 0..to.len() {
        //     for jdx in to[idx].iter() {
        //         eprintln!("{}>{}", idx, jdx);
        //     }
        // }
        // eprintln!(">>>>>");

        let count = names_to_idx.len();
        let shortest_dist = Self::search(&to);

        Self { names_to_idx, idx_to_names, count, rates, to, shortest_dist, }
    }

    fn search(graph: &Vec<Vec<usize>>) -> Vec<Vec<isize>> {
        let width = graph.len();
        let mut dist: Vec<Vec<isize>> = vec![vec![-1; width]; width];

        for idx in 0..width { dist[idx][idx] = 0; }
        
        // 1. 注意这是双向图
        for count in 0.. {
            let mut is_finished = true;

            for idx in 0..width {
                for jdx in 0..width {
                    if dist[idx][jdx] == count {
                        for &ndx in graph[jdx].iter() {
                            if dist[idx][ndx] < 0 {
                                // assert!(dist[idx][jdx] >= count);
                                dist[idx][ndx] = count + 1;// dist[idx][jdx] + 1;
                                is_finished = false; // find updates
                            }
                        }

                    }
                }
            }


            // for idx in 0..dist.len() {
            //     for jdx in 0..width {
            //         eprint!("{: >3}", dist[idx][jdx]);
            //     }
            //     eprintln!();
            // }
            //
            // eprintln!();
            // eprintln!();

            if is_finished { break; }
        }


        for idx in 0..width {
            for jdx in idx..width {
                if dist[idx][jdx] != dist[jdx][idx] {
                    for idx in 0..dist.len() {
                        for jdx in 0..width {
                            eprint!("{: >3}", dist[idx][jdx]);
                        }
                        eprintln!();
                    }

                    panic!();
                }
            }
        }

        dist
    }
}

impl P16Valve {
    fn parse(input: &str) -> nom::IResult<&str, P16Valve> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{alphanumeric1, space0};
        // let (input, _) = tag("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB")(input)?;
        let (input, _) = tag("Valve ")(input)?;
        let (input, name) = alphanumeric1(input)?;
        let (input, _) = tag(" has flow rate=")(input)?;
        let (input, rate) = nom::character::complete::u64(input)?;
        let (input, _) = tag("; ")(input)?;
        let (input, _) = nom::branch::alt((tag("tunnels lead"), tag("tunnel leads")))(input)?;
        let (input, _) = tag(" to ")(input)?;
        let (input, _) = nom::branch::alt((tag("valves "), tag("valve ")))(input)?;
        let (input, to) = nom::multi::separated_list1(
            nom::sequence::tuple((tag(","), space0)), alphanumeric1
        )(input)?;
        Ok((input, P16Valve {
            name: name.to_owned(),
            to: to.into_iter().map(ToOwned::to_owned).collect(),
            rate: rate as usize,
        }))
    }
}

fn p16_parse(input: &str) -> nom::IResult<&str, Vec<P16Valve>> {
    nom::multi::separated_list0(
        nom::character::complete::newline, 
        P16Valve::parse
    )(input)
}

pub fn p16() {
    let contents = r#"Valve LA has flow rate=22; tunnels lead to valves KA, MA
Valve MA has flow rate=24; tunnels lead to valves LA, NA
Valve NA has flow rate=26; tunnels lead to valves MA, OA
Valve OA has flow rate=28; tunnels lead to valves NA, PA
Valve PA has flow rate=30; tunnels lead to valves OA
Valve AA has flow rate=0; tunnels lead to valves BA
Valve BA has flow rate=2; tunnels lead to valves AA, CA
Valve CA has flow rate=4; tunnels lead to valves BA, DA
Valve DA has flow rate=6; tunnels lead to valves CA, EA
Valve EA has flow rate=8; tunnels lead to valves DA, FA
Valve FA has flow rate=10; tunnels lead to valves EA, GA
Valve GA has flow rate=12; tunnels lead to valves FA, HA
Valve HA has flow rate=14; tunnels lead to valves GA, IA
Valve IA has flow rate=16; tunnels lead to valves HA, JA
Valve JA has flow rate=18; tunnels lead to valves IA, KA
Valve KA has flow rate=20; tunnels lead to valves JA, LA"#;
// Part 1: 2640 [o]
// 2640 |AA|FA|GA|HA|IA|JA|KA|LA|MA|NA|OA|PA
// Part 2: 2670
// 1240 |AA|DA|EA|FA|GA|HA|IA|JA|CA
// 1430 |AA|KA|LA|MA|NA|OA|PA

    let contents = r#"Valve AA has flow rate=0; tunnels lead to valves BA
Valve BA has flow rate=1; tunnels lead to valves AA, CA
Valve CA has flow rate=4; tunnels lead to valves BA, DA
Valve DA has flow rate=9; tunnels lead to valves CA, EA
Valve EA has flow rate=16; tunnels lead to valves DA, FA
Valve FA has flow rate=25; tunnels lead to valves EA, GA
Valve GA has flow rate=36; tunnels lead to valves FA, HA
Valve HA has flow rate=49; tunnels lead to valves GA, IA
Valve IA has flow rate=64; tunnels lead to valves HA, JA
Valve JA has flow rate=81; tunnels lead to valves IA, KA
Valve KA has flow rate=100; tunnels lead to valves JA, LA
Valve LA has flow rate=121; tunnels lead to valves KA, MA
Valve MA has flow rate=144; tunnels lead to valves LA, NA
Valve NA has flow rate=169; tunnels lead to valves MA, OA
Valve OA has flow rate=196; tunnels lead to valves NA, PA
Valve PA has flow rate=225; tunnels lead to valves OA"#;
// Part 1: 13468 [o]
// 13468 |AA|IA|JA|KA|LA|MA|NA|OA|PA
// Part 2: 12887 [o]
// 4857 |AA|FA|GA|HA|IA|JA|KA|EA|DA
// 8030 |AA|LA|MA|NA|OA|PA

//     let contents = r#"Valve BA has flow rate=2; tunnels lead to valves AA, CA
// Valve CA has flow rate=10; tunnels lead to valves BA, DA
// Valve DA has flow rate=2; tunnels lead to valves CA, EA
// Valve EA has flow rate=10; tunnels lead to valves DA, FA
// Valve FA has flow rate=2; tunnels lead to valves EA, GA
// Valve GA has flow rate=10; tunnels lead to valves FA, HA
// Valve HA has flow rate=2; tunnels lead to valves GA, IA
// Valve IA has flow rate=10; tunnels lead to valves HA, JA
// Valve JA has flow rate=2; tunnels lead to valves IA, KA
// Valve KA has flow rate=10; tunnels lead to valves JA, LA
// Valve LA has flow rate=2; tunnels lead to valves KA, MA
// Valve MA has flow rate=10; tunnels lead to valves LA, NA
// Valve NA has flow rate=2; tunnels lead to valves MA, OA
// Valve OA has flow rate=10; tunnels lead to valves NA, PA
// Valve PA has flow rate=2; tunnels lead to valves OA, AA
// Valve AA has flow rate=0; tunnels lead to valves BA, PA"#;
// // Part 1: 1288
// // 1288 |AA|CA|EA|GA|IA|KA|MA|NA|OA|PA|BA
// // Part 2: 1484
// // 794 |AA|CA|EA|GA|IA|HA|FA|DA
// // 690 |AA|OA|MA|KA|JA|LA|NA|PA|BA

//     let contents = r#"Valve AK has flow rate=100; tunnels lead to valves AJ, AW, AX, AY, AZ
// Valve AW has flow rate=10; tunnels lead to valves AK
// Valve AX has flow rate=10; tunnels lead to valves AK
// Valve AY has flow rate=10; tunnels lead to valves AK
// Valve AZ has flow rate=10; tunnels lead to valves AK
// Valve BB has flow rate=0; tunnels lead to valves AA, BC
// Valve BC has flow rate=0; tunnels lead to valves BB, BD
// Valve BD has flow rate=0; tunnels lead to valves BC, BE
// Valve BE has flow rate=0; tunnels lead to valves BD, BF
// Valve BF has flow rate=0; tunnels lead to valves BE, BG
// Valve BG has flow rate=0; tunnels lead to valves BF, BH
// Valve BH has flow rate=0; tunnels lead to valves BG, BI
// Valve BI has flow rate=0; tunnels lead to valves BH, BJ
// Valve BJ has flow rate=0; tunnels lead to valves BI, BK
// Valve BK has flow rate=100; tunnels lead to valves BJ, BW, BX, BY, BZ
// Valve BW has flow rate=10; tunnels lead to valves BK
// Valve BX has flow rate=10; tunnels lead to valves BK
// Valve BY has flow rate=10; tunnels lead to valves BK
// Valve BZ has flow rate=10; tunnels lead to valves BK
// Valve CB has flow rate=0; tunnels lead to valves AA, CC
// Valve CC has flow rate=0; tunnels lead to valves CB, CD
// Valve CD has flow rate=0; tunnels lead to valves CC, CE
// Valve CE has flow rate=0; tunnels lead to valves CD, CF
// Valve CF has flow rate=0; tunnels lead to valves CE, CG
// Valve CG has flow rate=0; tunnels lead to valves CF, CH
// Valve CH has flow rate=0; tunnels lead to valves CG, CI
// Valve CI has flow rate=0; tunnels lead to valves CH, CJ
// Valve CJ has flow rate=0; tunnels lead to valves CI, CK
// Valve CK has flow rate=100; tunnels lead to valves CJ, CW, CX, CY, CZ
// Valve CW has flow rate=10; tunnels lead to valves CK
// Valve CX has flow rate=10; tunnels lead to valves CK
// Valve CY has flow rate=10; tunnels lead to valves CK
// Valve CZ has flow rate=10; tunnels lead to valves CK
// Valve AA has flow rate=0; tunnels lead to valves AB, BB, CB
// Valve AB has flow rate=0; tunnels lead to valves AA, AC
// Valve AC has flow rate=0; tunnels lead to valves AB, AD
// Valve AD has flow rate=0; tunnels lead to valves AC, AE
// Valve AE has flow rate=0; tunnels lead to valves AD, AF
// Valve AF has flow rate=0; tunnels lead to valves AE, AG
// Valve AG has flow rate=0; tunnels lead to valves AF, AH
// Valve AH has flow rate=0; tunnels lead to valves AG, AI
// Valve AI has flow rate=0; tunnels lead to valves AH, AJ
// Valve AJ has flow rate=0; tunnels lead to valves AI, AK"#;
// // Part 1: 2400
// // 2400 |AA|CK|CX|CZ|CY|CW
// // Part 2: 3680
// // 1840 |AA|AK|AW|AX|AY|AZ
// // 1840 |AA|CK|CZ|CX|CY|CW
//     let contents = r#"Valve AW has flow rate=0; tunnels lead to valves LG, TL
// Valve OM has flow rate=0; tunnels lead to valves XK, IM
// Valve BG has flow rate=0; tunnels lead to valves MP, SB
// Valve XB has flow rate=0; tunnels lead to valves MA, TL
// Valve CD has flow rate=0; tunnels lead to valves VL, OF
// Valve VF has flow rate=0; tunnels lead to valves CS, XK
// Valve HK has flow rate=0; tunnels lead to valves RL, QB
// Valve QN has flow rate=0; tunnels lead to valves IV, QR
// Valve OF has flow rate=4; tunnels lead to valves TQ, CD, IR, IM, JE
// Valve QB has flow rate=14; tunnels lead to valves HK, XE, CS, VO
// Valve ZE has flow rate=7; tunnels lead to valves JB, NC, SE, OI
// Valve OW has flow rate=0; tunnels lead to valves MB, JB
// Valve MA has flow rate=0; tunnels lead to valves XB, MB
// Valve MP has flow rate=0; tunnels lead to valves VK, BG
// Valve UE has flow rate=9; tunnels lead to valves ZM, RZ, WI, HO, FO
// Valve QR has flow rate=24; tunnel leads to valve QN
// Valve TQ has flow rate=0; tunnels lead to valves OF, AA
// Valve SE has flow rate=0; tunnels lead to valves ZE, ZZ
// Valve AQ has flow rate=20; tunnel leads to valve CX
// Valve XE has flow rate=0; tunnels lead to valves JQ, QB
// Valve DC has flow rate=8; tunnels lead to valves ZD, MJ, RZ
// Valve ZM has flow rate=0; tunnels lead to valves YJ, UE
// Valve VK has flow rate=21; tunnel leads to valve MP
// Valve VR has flow rate=0; tunnels lead to valves WV, PS
// Valve BH has flow rate=0; tunnels lead to valves AA, MB
// Valve ZR has flow rate=0; tunnels lead to valves LG, AI
// Valve JE has flow rate=0; tunnels lead to valves OF, HO
// Valve IR has flow rate=0; tunnels lead to valves IV, OF
// Valve FO has flow rate=0; tunnels lead to valves XQ, UE
// Valve AA has flow rate=0; tunnels lead to valves NC, VY, BH, TQ, YJ
// Valve ZZ has flow rate=0; tunnels lead to valves SE, TL
// Valve XQ has flow rate=0; tunnels lead to valves IV, FO
// Valve WI has flow rate=0; tunnels lead to valves UE, VO
// Valve VY has flow rate=0; tunnels lead to valves AA, LG
// Valve XK has flow rate=15; tunnels lead to valves VF, OM, ZD
// Valve CX has flow rate=0; tunnels lead to valves AQ, MB
// Valve JQ has flow rate=0; tunnels lead to valves XE, IV
// Valve LG has flow rate=3; tunnels lead to valves VY, PS, ZR, AW, OI
// Valve JB has flow rate=0; tunnels lead to valves ZE, OW
// Valve OI has flow rate=0; tunnels lead to valves ZE, LG
// Valve YJ has flow rate=0; tunnels lead to valves ZM, AA
// Valve NC has flow rate=0; tunnels lead to valves AA, ZE
// Valve KR has flow rate=0; tunnels lead to valves SB, MJ
// Valve MB has flow rate=17; tunnels lead to valves CX, BH, AI, OW, MA
// Valve AI has flow rate=0; tunnels lead to valves ZR, MB
// Valve TL has flow rate=16; tunnels lead to valves ZZ, XB, AW
// Valve RL has flow rate=0; tunnels lead to valves WV, HK
// Valve CS has flow rate=0; tunnels lead to valves VF, QB
// Valve WV has flow rate=25; tunnels lead to valves RL, VL, VR
// Valve ZD has flow rate=0; tunnels lead to valves XK, DC
// Valve IV has flow rate=23; tunnels lead to valves XQ, IR, JQ, QN
// Valve PS has flow rate=0; tunnels lead to valves VR, LG
// Valve RZ has flow rate=0; tunnels lead to valves DC, UE
// Valve VO has flow rate=0; tunnels lead to valves WI, QB
// Valve MJ has flow rate=0; tunnels lead to valves DC, KR
// Valve IM has flow rate=0; tunnels lead to valves OM, OF
// Valve VL has flow rate=0; tunnels lead to valves CD, WV
// Valve SB has flow rate=18; tunnels lead to valves BG, KR
// Valve HO has flow rate=0; tunnels lead to valves JE, UE"#;
// // Part 1: 1789

//     // my example
//     let contents = r#"Valve BB has flow rate=13; tunnels lead to valves CC, AA
// Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
// Valve CC has flow rate=2; tunnels lead to valves DD, BB
// Valve EE has flow rate=3; tunnels lead to valves FF, DD
// Valve FF has flow rate=0; tunnels lead to valves EE, GG
// Valve GG has flow rate=0; tunnels lead to valves FF, HH
// Valve HH has flow rate=22; tunnel leads to valve GG
// Valve II has flow rate=0; tunnels lead to valves AA, JJ
// Valve JJ has flow rate=21; tunnel leads to valve II"#;

    // let contents = &std::fs::read_to_string("./assets/adv2022/adv16.txt").unwrap();
    let (_, valves) = p16_parse(contents).unwrap();

    let vmap = P16ValveMap::new(&valves);
    // eprintln!("{:?}", vmap);

    let unopened: HashSet<usize> = vmap.rates.iter().enumerate().filter(|(idx, r)| **r > 0).map(|(idx, _)| idx).collect();
    let (out, traces) = p16_search(&vmap, 30, *vmap.names_to_idx.get("AA").unwrap(), 0, unopened, Some(vec![]));
    eprintln!("Best energy: {}", out);

    // eprintln!("{:?}", vmap.shortest_dist[vmap.names_to_idx.get("YX").cloned().unwrap()].iter().enumerate().filter_map(|(idx, step)| if vmap.rates[idx] > 0 { Some((&vmap.idx_to_names[idx], vmap.rates[idx], step)) } else { None }).collect::<Vec<_>>());
    if let Some(traces) = traces {
        for &(sdx, ndx, dist, delta, left) in traces.iter() {
            eprint!("({} -> {} {} {}*{}={}) ", vmap.idx_to_names[sdx], vmap.idx_to_names[ndx], dist, left, vmap.rates[ndx], delta);
        }
        eprintln!();
    }

    eprintln!("================================");
    let unopened: Vec<usize> = vmap.rates.iter().enumerate().filter(|(idx, r)| **r > 0).map(|(idx, _)| idx).collect();
    let aa = *vmap.names_to_idx.get("AA").unwrap();
    let (out, state) = p16_search_par2(&vmap, 0, P16StatePar2::new(unopened, 26, aa));
    eprintln!("Best energy: {}", out);

    // eprintln!("{:?}", vmap.shortest_dist[vmap.names_to_idx.get("YX").cloned().unwrap()].iter().enumerate().filter_map(|(idx, step)| if vmap.rates[idx] > 0 { Some((&vmap.idx_to_names[idx], vmap.rates[idx], step)) } else { None }).collect::<Vec<_>>());
    if let Some(traces) = &state.traces {
        for &(who, sdx, ndx, dist, delta, left) in traces.iter() {
            eprint!("(#{} {} -> {} {} {}*{}={}) ", who, vmap.idx_to_names[sdx], vmap.idx_to_names[ndx], dist, left, vmap.rates[ndx], delta);
        }
        eprintln!();
    }
}

type P16Trace = Vec<(usize, usize, usize, usize, usize)>;
pub fn p16_search(
    vmap: &P16ValveMap, minutes: usize, last: usize, energy: usize, unopened: HashSet<usize>, traces: Option<P16Trace>
) -> (usize, Option<P16Trace>) {
    // eprintln!(">>> {} {} {}", minutes, last, energy);
    if minutes == 0 || unopened.len() == 0 { return (energy, traces); }

    let mut best_energy = energy; let mut best_traces = traces.clone();
    for &ndx in unopened.iter() {
        let dist = vmap.shortest_dist[last][ndx] as usize;
        if minutes < dist+1 { continue; }
        let mut unopened = unopened.clone();
        unopened.remove(&ndx);
        let delta = vmap.rates[ndx] * (minutes - dist - 1);

        let mut traces = traces.clone();
        if let Some(t) = &mut traces { t.push((last, ndx, dist, delta, minutes-dist-1)); }

        let (e0, traces) = p16_search(vmap, minutes-dist-1, ndx, energy + delta, unopened, traces);
        if e0 > best_energy {
            best_energy = e0;
            best_traces = traces;
        }
    }

    (best_energy, best_traces)
}

#[derive(Debug, Clone)]
struct P16StatePar2 {
    players: [(usize, usize); 2],

    unopened: Vec<usize>,
    traces: Option<P16Trace2>,
}

impl P16StatePar2 {
    fn new(unopened: Vec<usize>, minute: usize, start: usize) -> Self {
        Self {
            players: [(minute, start); 2],
            unopened,

            traces: None,
        }
    }
    fn enable_trace(mut self) -> Self{
        self.traces = Some(vec![]);
        self
    }
}

type P16Trace2 = Vec<(usize, usize, usize, usize, usize, usize)>;
// still not performant. fail to run test cast 2-5 (zero indexed). 
// TODO: need optimization.
fn p16_search_par2(
    vmap: &P16ValveMap,
    energy: usize,
    state: P16StatePar2,
) -> (usize, P16StatePar2) {
    if state.unopened.len() == 0 || state.players.iter().all(|&(minute, _)| minute == 0) { return (energy, state); }

    let mut idx = state.players.len() + 1;
    for (p, &(m, _)) in state.players.iter().enumerate() {
        if m > 0 { idx = p; break; }
    }
    debug_assert!(idx < state.players.len());
    let (minute, last) = state.players[idx];

    let mut best_energy = energy;
    let mut best_state = state.clone();

    if idx < state.players.len() - 1 {
        let mut state = state.clone();
        state.players[idx] = (0, last);
        let (new_energy, new_state) = p16_search_par2(vmap, energy + 0, state);

        if new_energy > best_energy { best_energy = new_energy; best_state = new_state; }
    }

    let mut is_touched = false;
    for (ndx_index, &ndx) in state.unopened.iter().enumerate() {
        let dist = vmap.shortest_dist[last][ndx] as usize;
        if minute < dist+1 { continue; }
        is_touched = true;

        let mut state = state.clone();
        let left = minute - dist - 1;
        let delta = vmap.rates[ndx] * left;
        state.players[idx] = (left, ndx);
        state.unopened.remove(ndx_index);
        if let Some(t) = &mut state.traces {
            t.push((idx, last, ndx, dist, delta, left));
        }
        let (new_energy, new_state) = p16_search_par2(vmap, energy + delta, state);
        
        if new_energy > best_energy { best_energy = new_energy; best_state = new_state; }
    }

    if !is_touched {
        let mut state = state.clone();
        state.players[idx] = (0, last);
        return p16_search_par2(vmap, energy, state)
    } else {
        // eprint!("{}: ", best_energy);
        // for &(who, sdx, ndx, dist, delta, left) in best_state.traces.as_ref().unwrap().iter() {
        //     eprint!("(#{} {} -> {}) ", who, vmap.idx_to_names[sdx], vmap.idx_to_names[ndx], );
        // }
        // eprintln!();
        (best_energy, best_state)
    }

    // let [(minute0, last0), (minute1, last1)] = state.players;
    // let mut best_energy = energy;
    // let mut best_state = state.clone();
    //
    // for &ndx0 in state.unopened.iter() {
    //     for &ndx1 in state.unopened.iter() {
    //         if ndx0 == ndx1 { continue; } // go to same node
    //         // if ndx0 == last0 && ndx1 == last1 { continue; } // no one goes next.
    //
    //         let dist0 = vmap.shortest_dist[last0][ndx0] as usize;
    //         let dist1 = vmap.shortest_dist[last1][ndx1] as usize;
    //         if minute0 < dist0 + 1 { continue; }
    //         if minute1 < dist1 + 1 { continue; }
    //
    //         // // (x, _) -> (_, x) -> 一定在上一次已经一次运动了
    //         // if !moved0 && ndx1 == last1 { continue }
    //         // if !moved1 && ndx0 == last0 { continue }
    //
    //         let mut left0 = minute0; let mut left1 = minute1; let mut delta = 0;
    //         // let mut next_moved0 = false; let mut next_moved1 = false;
    //
    //         if ndx0 != last0 { 
    //             // next_moved0 = true;
    //             left0 = minute0-dist0-1;
    //             delta += vmap.rates[ndx0] * left0;
    //         }
    //         if ndx1 != last1 {
    //             // next_moved1 = false;
    //             left1 = minute1-dist1-1;
    //             delta += vmap.rates[ndx1] * left1;
    //         }
    //
    //         let mut state = state.clone();
    //         state.players = [(left0, ndx0), (left1, ndx1)];
    //         state.unopened = state.unopened.into_iter().filter(|&u| {
    //             u != ndx0 && u != ndx1 && 
    //             (vmap.shortest_dist[ndx0][u] + 1 < (left0 as isize) || vmap.shortest_dist[ndx1][u] + 1 < (left1 as isize))
    //         }).collect();
    //         let (new_energy, new_state) = p16_search_par2(vmap, energy + delta, state);
    //
    //         if new_energy > best_energy {
    //             best_energy = new_energy;
    //             best_state = new_state;
    //         }
    //     }
    // }
    //
    // (best_energy, best_state)
}

fn p17_parse(input: &str) -> nom::IResult<&str, Vec<i8>> {
    nom::multi::many0(nom::branch::alt((
        nom::combinator::value(1, nom::character::complete::char::<&str, _>('>')),
        nom::combinator::value(-1, nom::character::complete::char('<')),
    )))(input)
}

#[derive(Debug, Clone)]
struct P17Rock {
    height: usize,
    width: usize,
    points: Vec<(isize, isize)>,
}

impl P17Rock {
    fn parse(input: &str) -> Self {
        let lines: Vec<Vec<_>> = input.split("\n").map(|line| line.chars().collect()).collect();
        let height = lines.len();
        let width = lines[0].len();

        let mut points = vec![];
        for (idx, line) in lines.iter().rev().enumerate() {
            for (jdx, &symbol) in line.iter().enumerate() {
                if symbol == '#' {
                    points.push((jdx as isize, idx as isize));
                }
            }
        }
        Self {
            width,
            height,
            points,
        }
    }
}

pub fn p17() {
    let contents = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;
    let patterns = r#"####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##"#;

    // let contents = &std::fs::read_to_string("./assets/adv2022/adv17.txt").unwrap();
    let jets: Vec<i8> = p17_parse(contents).unwrap().1;

    let patterns: Vec<_> = patterns.split("\n\n").map(|s| P17Rock::parse(s)).collect();
    let mut stack = vec![]; 
    let mut idx_pattern = 0;
    let mut idx_jet = 0;
    let mut disappear = 0;
    let max_count = 2022;
    // let max_count = 1000000000000usize;
    // let max_count = 10;
    // 针对第一题，disappear 的优化都没有必要
    let timer = std::time::Instant::now();
    for idx in 0..max_count {
        // if idx % 10000000 == 0 { eprintln!("{:?} ????", chrono::Local::now()); }
        p17_fall(&jets, &patterns, &mut idx_jet, &mut idx_pattern, &mut stack, &mut disappear);
    }
    eprintln!("Tall: {}", stack.len() + disappear);
    eprintln!("Elapshed: {:?}", timer.elapsed());

    eprintln!("=======================");
    // 我们不需要维护整个stack，只需要维护最上面的四个
    let mut idx_pattern = 0;
    let mut idx_jet = 0;
    let mut stack = [[-1; P17SHAPEHEIGHT]; P17WIDTH];
    // let max_count = 2022;
    // let max_count = 1000000000000usize;
    // let max_count = 5;
    // let mut stack_inited = stack.clone();
    // let mut is_first = true;
    let mut floor = 0;
    // fail to find iteration pattern
    // 由于 pattern 和 jets 都是周期重复的，所以一定在某个时间后，整体重置了，但跑了几个测试没有发现
    // 重复的现象
    let timer = std::time::Instant::now();
    for idx in 0..max_count {
        // if idx % 10000000 == 0 { eprintln!("{:?} ???? {:?}", chrono::Local::now(), stack); }
        p17_fall_fast(&jets, &patterns, &mut idx_jet, &mut idx_pattern, &mut stack, &mut floor);

        // if !is_first && stack_inited == stack {
        //     println!("{: >5}: {:?} {}", idx, stack, idx_jet);
        // }
        //
        // if is_first && stack.iter().flatten().all(|&y| y >= 0) {
        //     println!("{: >5}({}): {:?} {}", idx, floor, stack, idx_jet);
        //     stack_inited = stack;
        //     is_first = false;
        // }

        // eprintln!("{:?}", stack);
        // eprintln!();
    }
    eprintln!("Tall: {}", stack.iter().flatten().max().unwrap() + (floor) + 1);
    eprintln!("Elapshed: {:?}", timer.elapsed());

}

const P17WIDTH: usize = 7;
const P17GAP: usize = 3;
const P17XOFFSET: usize = 2;
const P17SHAPEHEIGHT: usize = 4;
fn p17_fall(
    jets: &[i8], patterns: &[P17Rock], idx_jet: &mut usize, idx_pattern: &mut usize, stack: &mut Vec<[u8; P17WIDTH]>, disappear: &mut usize
) {
    let mut rock = patterns[*idx_pattern % patterns.len()].clone();
    *idx_pattern += 1;
    // eprintln!("Init@{}: {:?}", *idx_pattern - 1, rock);

    let heighest = stack.len();
    rock.points = rock.points.into_iter().map(|(x, y)| (
        x + P17XOFFSET as isize, y + heighest as isize + P17GAP as isize
    )).collect();

    loop { 
        let is_stop = p17_step(stack, disappear, &mut rock, jets[*idx_jet % jets.len()]);
        *idx_jet += 1;
        if !is_stop { break; }
    }

    // eprintln!("Final: {:?}", rock);
    // eprintln!();

    for &(px, py) in rock.points.iter() {
        if py >= stack.len() as isize {
            for _ in 0..(py - stack.len() as isize + 1) { stack.push([0; P17WIDTH]); }
        }
        stack[py as usize][px as usize] = 1;
    }

    // calculate disappear
    let mut max_row = stack.len();
    for column in 0..P17WIDTH {
        for row in (0..stack.len()).rev() {
            if stack[row][column] == 1 || row == 0 {
                max_row = max_row.min(row);
                break;
            }
        }
    }

    let height = stack.len();
    if max_row > 0 && max_row < stack.len() {
        *disappear += max_row;
        stack.rotate_left(max_row);
        stack.truncate(height - max_row);
    }
    // eprintln!("{} {}", max_row, stack.len());
}

fn p17_step(stack: &mut Vec<[u8; 7]>, disappear: &mut usize, rock: &mut P17Rock, jet: i8) -> bool {
    let mut is_moved = false;

    if rock.points.iter().all(|&(x, y)| {
        let nx =  x + (jet as isize);
        nx < P17WIDTH as isize && nx >= 0 && (y >= stack.len() as isize || stack[y as usize][nx as usize] != 1)
    }) {
        rock.points = rock.points.clone().into_iter().map(|(x, y)| { (x + (jet as isize), y) }).collect();
    }

    if rock.points.iter().all(|&(x, y)| {
        let ny  = y - 1;
        ny >= 0 && (ny >= stack.len() as isize || stack[ny as usize][x as usize] != 1)
    }) {
        rock.points = rock.points.clone().into_iter().map(|(x, y)| { (x, y - 1) }).collect();
        is_moved = true;
    }

    // eprintln!("{} {:?}", jet, rock);

    is_moved
}

fn p17_fall_fast(
    jets: &[i8], patterns: &[P17Rock], idx_jet: &mut usize, idx_pattern: &mut usize, 
    stack: &mut [[isize; P17SHAPEHEIGHT]; P17WIDTH], floor: &mut isize,
) {
    let mut rock = patterns[*idx_pattern % patterns.len()].clone();
    *idx_pattern += 1;
    // eprintln!("Init@{}: {:?}", *idx_pattern - 1, rock);

    let heighest = stack.iter().flatten().max().unwrap() + 1;
    rock.points = rock.points.into_iter().map(|(x, y)| (
        x + P17XOFFSET as isize, y + heighest as isize + P17GAP as isize
    )).collect();

    loop { 
        let is_stop = p17_step_fast(stack, &mut rock.points, jets[*idx_jet % jets.len()]);
        *idx_jet += 1;
        if !is_stop { break; }
    }
    // eprintln!("{:?}", rock);

    for column in 0..P17WIDTH {
        for &(px, py) in rock.points.iter() {
            if px != column as isize { continue; }
            if py > stack[column].last().cloned().unwrap() {
                stack[column].rotate_right(1);
                stack[column][0] = py; // 不断用最新的point去堆叠
            }
        }
    }

    let lowest = stack.iter().flatten().min().cloned().unwrap();
    if lowest > 0 {
        stack.iter_mut().for_each(|row| row.iter_mut().for_each(|y| *y -= lowest));
        *floor += lowest;
    }
}

fn p17_step_fast(
    stack: &mut [[isize; P17SHAPEHEIGHT]; P17WIDTH], rocks: &mut Vec<(isize, isize)>, jet: i8
) -> bool {
    if rocks.iter().all(|&(x, y)| {
        let nx =  x + (jet as isize);
        nx < P17WIDTH as isize && nx >= 0 && stack[nx as usize].iter().all(|&sy| sy != y)
    }) {
        *rocks = rocks.clone().into_iter().map(|(x, y)| { (x + (jet as isize), y) }).collect();
    }
        
    let points: Vec<_> = rocks.clone().into_iter().map(|(x, y)| { (x, y - 1) }).collect();
    if points.iter().all(|&(x, y)| x >= 0 && stack[x as usize].iter().all(|&sy| sy != y)) {
        *rocks = points.into_iter().filter(|&(x, y)| y > stack[x as usize].last().cloned().unwrap()).collect();
        !(rocks.len() == 0)
    } else {
        false
    }
}

fn p18_parse(input: &str) -> nom::IResult<&str, Vec<(isize, isize, isize)>> {
    let point = nom::sequence::tuple((
            nom::character::complete::i64,
            nom::character::complete::char(','),
            nom::character::complete::i64,
            nom::character::complete::char(','),
            nom::character::complete::i64,
    ));

    // NOTE: 为什么 +1？参考p18里的注释
    nom::multi::separated_list0(
        nom::character::complete::newline, 
        nom::combinator::map(point, |(x, _, y, _, z)| (x as isize + 1, y as isize + 1, z as isize + 1)),
    )(input)
}

pub fn p18() {
    let contents = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

    let contents = &std::fs::read_to_string("./assets/adv2022/adv18.txt").unwrap();
    let cubes = p18_parse(contents).unwrap().1;
    
    let outside = p18_calculate_cube_surface(&cubes);
    eprintln!("{:?}", outside);
    // 需要把内部的环找出来，三维里面的闭环如何去计算？

    // // 如果边界设定不足以包裹所有点，那么有可能有通路
    // NOTE: 这里我们开始没有校验，因为答案已经正确了，我们运气好没有遇到截面上的闭环
    // 为了实现简单，我们直接在 parse 阶段 +1，否则每个点的坐标都index需要平移一下，改动太大
    debug_assert!(*cubes.iter().map(|(x, y, z)| x).min().unwrap() > 0);
    debug_assert!(*cubes.iter().map(|(x, y, z)| y).min().unwrap() > 0);
    debug_assert!(*cubes.iter().map(|(x, y, z)| z).min().unwrap() > 0);

    let len_x = *cubes.iter().map(|(x, y, z)| x).max().unwrap() as usize + 1;
    let len_y = *cubes.iter().map(|(x, y, z)| y).max().unwrap() as usize + 1;
    let len_z = *cubes.iter().map(|(x, y, z)| z).max().unwrap() as usize + 1;
    eprintln!("Area {} x {} x {}", len_x, len_y, len_z);

    // 0 -> internal; 1 -> outside ; 2 -> cube; 
    let mut grid: Vec<Vec<Vec<u8>>> = vec![vec![vec![0; len_z]; len_y]; len_x];
    ////////////////////////////////////////////////////////////////////////////
    for &(x, y, z) in cubes.iter() { grid[x as usize][y as usize][z as usize] = 2; }
    let start = (0, 0, 0);
    assert!(grid[start.0][start.1][start.2] == 0);
    let mut points = vec![start];
    grid[start.0][start.1][start.2] = 1;

    while points.len() > 0 { p18_search(&mut grid, &mut points, len_x, len_y, len_z); }

    ////////////////////////////////////////////////////////////////////////////
    let mut count_outside = 0;
    let mut points_internal = vec![];
    for xindex in 0..len_x {
        for yindex in 0..len_y {
            for zindex in 0..len_z {
                match grid[xindex][yindex][zindex] {
                    1 => { count_outside += 1; }
                    0 => {
                        points_internal.push((xindex as isize, yindex as isize, zindex as isize));
                    },
                    _ => {},
                }
            }
        }
    }
    let count_internal = len_x * len_y * len_z - count_outside - cubes.len();
    eprintln!("Internal: {}", count_internal);
    eprintln!("points: {:?}", points_internal);
    let inside = p18_calculate_cube_surface(&points_internal);
    eprintln!("Inside = {}, Answer = {}", inside, outside - inside);
}

fn p18_calculate_cube_surface(cubes: &[(isize, isize, isize)]) -> usize {
    let mut interact = 0;
    for idx in 0..cubes.len() {
        for jdx in (idx+1)..cubes.len() {
            let (x0, y0, z0) = cubes[idx];
            let (x1, y1, z1) = cubes[jdx];

            if (x0 - x1).abs() + (y0 - y1).abs() + (z0 - z1).abs() == 1 {
                interact += 1;
            }
        }
    }
    cubes.len() * 6 - 2*interact
}

fn p18_search(grid: &mut Vec<Vec<Vec<u8>>>, points: &mut Vec<(usize, usize, usize)>, len_x: usize, len_y: usize, len_z: usize) {
    for (x, y, z) in std::mem::replace(points, Default::default()) {
        debug_assert!(grid[x][y][z] == 1);

        for (vx, vy, vz) in [
            ( 1, 0,   0),
            (-1, 0,   0),
            ( 0, 1,   0),
            ( 0, -1,  0),
            ( 0,  0,  1),
            ( 0,  0, -1),
        ] {
            let nx = x as isize + vx;
            let ny = y as isize + vy;
            let nz = z as isize + vz;
            if (nx >= 0) && (nx < len_x as isize) && (ny >= 0) && (ny < len_y as isize) && (nz >= 0) && (nz < len_z as isize) {
                let nx = nx as usize; let ny = ny as usize; let nz = nz as usize;
                match grid[nx][ny][nz] {
                    0 => { grid[nx][ny][nz] = 1; points.push((nx, ny, nz)); },
                    1 => { }
                    2 => { }
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct P19Blueprint {
    idx: usize,
    ore: usize,
    clay: usize,
    obsidian0: usize,
    obsidian1: usize,
    geode0: usize,
    geode1: usize,
}

impl P19Blueprint {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::u32;

        let separator = |input| {
            nom::sequence::tuple((
                nom::character::complete::space0,
                nom::combinator::opt(nom::character::complete::newline),
                nom::character::complete::space0,
            ))(input)
        };

        let (input, _) = tag("Blueprint ")(input)?;
        let (input, idx) = u32(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = separator(input)?;

        let (input, _) = tag("Each ore robot costs ")(input)?;
        let (input, ore) = u32(input)?;
        let (input, _) = tag(" ore.")(input)?;
        let (input, _) = separator(input)?;

        let (input, _) = tag("Each clay robot costs ")(input)?;
        let (input, clay) = u32(input)?;
        let (input, _) = tag(" ore.")(input)?;
        let (input, _) = separator(input)?;

        let (input, _) = tag("Each obsidian robot costs ")(input)?;
        let (input, obsidian0) = u32(input)?;
        let (input, _) = tag(" ore and ")(input)?;
        let (input, obsidian1) = u32(input)?;
        let (input, _) = tag(" clay.")(input)?;
        let (input, _) = separator(input)?;

        let (input, _) = tag("Each geode robot costs ")(input)?;
        let (input, geode0) = u32(input)?;
        let (input, _) = tag(" ore and ")(input)?;
        let (input, geode1) = u32(input)?;
        let (input, _) = tag(" obsidian.")(input)?;

        Ok((input, Self {
            idx: idx as usize,
            ore: ore as usize,
            clay: clay as usize,
            obsidian0: obsidian0 as usize,
            obsidian1: obsidian1 as usize,
            geode0: geode0 as usize,
            geode1: geode1 as usize,
        }))
    }
}

fn p19_parse(input: &str) -> nom::IResult<&str, Vec<P19Blueprint>> {
    nom::multi::separated_list0(
        nom::multi::many1(nom::character::complete::newline), 
        P19Blueprint::parse,
    )(input)
}

#[derive(Debug, Clone, Default)]
pub struct P19State {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,

    bot_ore: usize,
    bot_clay: usize,
    bot_obsidian: usize,
    bot_geode: usize,
}

impl P19State {
    fn new() -> Self {
        let mut slf = Self::default();
        slf.bot_ore += 1;
        slf
    }
    
    fn produce(&mut self, gap: usize) {
        self.ore += self.bot_ore * gap;
        self.clay += self.bot_clay * gap;
        self.obsidian += self.bot_obsidian * gap;
        self.geode += self.bot_geode * gap;
    }
}

pub fn p19() {
    let contents = r#"Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian."#;

    // let contents = &std::fs::read_to_string("./assets/adv2022/adv19.txt").unwrap();

    let blueprints = p19_parse(contents).unwrap().1;
    // eprintln!("{:?}", blueprints);

    let mut sum = 0;
    for (idx, blueprint) in blueprints.iter().enumerate() {
        // 每一步必须要生产一个机器，否则搜索域太大了
        // 只生产而不生产机器的话，只有两种情况：
        // - 直到最终都没有足够资源任何一个生产机器
        // - 可以生产某个机器但选择放弃积累原料，那么其一定产出小于生产了某个机器，因为后者
        // 至少会产生最终目标 geode 导致最终绩效更好

        // let state = p19_search_dfs(blueprint, 24, P19State::new());
        let state = p19_search_bfs(blueprint, 24);
        eprintln!("state@{}: {:?}", idx, state);
        sum += (idx + 1) * state;
    }
    eprintln!("Sum of qulity: {}", sum);
}

fn p19_search_bfs(blueprint: &P19Blueprint, minute: usize) -> usize {
    // 搜索所有 path 可选项太多了，我们需要实时排除潜在的最差结果
    let mut states = vec![P19State::new()]; 
    let mut threshold_obsidian = 0;
    let mut threshold_geode = 0;
    for count in 0..minute {
        dbg!(states.len());
        // assert_ne!(count, 3);
        for mut state in std::mem::replace(&mut states, Default::default()) {
            threshold_obsidian = threshold_obsidian.max(state.bot_obsidian);
            // if state.bot_obsidian < threshold_obsidian { continue; }

            threshold_geode = threshold_geode.max(state.bot_geode);
            if state.bot_geode < threshold_geode { continue; }

            if state.ore >= blueprint.ore {
                let mut state = state.clone();
                state.ore -= blueprint.ore;
                state.produce(1);
                state.bot_ore += 1;
                states.push(state);
            }

            if state.ore >= blueprint.clay {
                let mut state = state.clone();
                state.ore -= blueprint.clay;
                state.produce(1);
                state.bot_clay += 1;
                states.push(state);
            }

            if state.ore >= blueprint.obsidian0 && state.clay >= blueprint.obsidian1 {
                let mut state = state.clone();
                state.ore -= blueprint.obsidian0;
                state.clay -= blueprint.obsidian1;
                state.produce(1);
                state.bot_obsidian += 1;
                states.push(state);
            }

            if state.ore >= blueprint.geode0 && state.obsidian >= blueprint.geode1 {
                let mut state = state.clone();
                state.ore -= blueprint.geode0;
                state.obsidian -= blueprint.geode1;
                state.produce(1);
                state.bot_geode += 1;
                // eprintln!("Got one geode: {:?}", state);
                states.push(state);
            }

            state.produce(1);
            states.push(state);
        }
    }

    states.into_iter().map(|s| s.geode).max().unwrap()
}

fn p19_search_dfs(blueprint: &P19Blueprint, minute: usize, state: P19State) -> P19State {
    if minute == 0 { return state; }
    let mut best_state = state.clone();
    best_state.produce(1);

    // check first
    if state.bot_ore > 0 {
        let mut state = state.clone();
        let mut minute = minute;
        while minute > 0 && state.ore < blueprint.ore { 
            minute -= 1;
            state.produce(1); 
        }
        if state.ore >= blueprint.ore {
            state.ore -= blueprint.ore;
            state.bot_ore += 1;
            let other = p19_search_dfs(blueprint, minute, state);
            if other.geode > best_state.geode {
                best_state = other;
            }
        }
    }

    if state.bot_ore > 0 {
        let mut state = state.clone();
        let mut minute = minute;
        while minute > 0 && state.ore < blueprint.clay { 
            minute -= 1;
            state.produce(1); 
        }
        if state.ore >= blueprint.clay {
            state.ore -= blueprint.clay;
            state.bot_clay += 1;
            let other = p19_search_dfs(blueprint, minute, state);
            if other.geode > best_state.geode {
                best_state = other;
            }
        }
    }

    if state.bot_ore > 0 && state.bot_clay > 0 {
        let mut state = state.clone();
        let mut minute = minute;
        while minute > 0 && state.ore < blueprint.obsidian0 && state.clay < blueprint.obsidian1 { 
            minute -= 1;
            state.produce(1); 
        }
        if state.ore >= blueprint.obsidian0 && state.clay >= blueprint.obsidian1 {
            state.ore -= blueprint.obsidian0;
            state.clay -= blueprint.obsidian1;
            state.bot_obsidian += 1;

            let other = p19_search_dfs(blueprint, minute, state);
            if other.geode > best_state.geode {
                best_state = other;
            }
        }
    }

    if state.bot_ore > 0 && state.bot_obsidian > 0 {
        let mut state = state.clone();
        let mut minute = minute;
        while minute > 0 && state.ore < blueprint.geode0 && state.obsidian < blueprint.geode1 { 
            minute -= 1;
            state.produce(1); 
        }
        if state.ore >= blueprint.geode0 && state.obsidian >= blueprint.geode1 {
            state.ore -= blueprint.geode0;
            state.obsidian -= blueprint.geode1;

            state.bot_geode += 1;
            let other = p19_search_dfs(blueprint, minute, state);
            if other.geode > best_state.geode {
                best_state = other;
            }
        }

    }
    // eprintln!("{} {:?}", minute, best_state);

    best_state
}


pub fn p20() {
    let contents = r#"1
2
-3
3
-2
0
4"#;

//     let contents = r#"9
// 2
// -3
// 3
// -2
// 0
// 4"#;

    let contents = &std::fs::read_to_string("./assets/adv2022/adv20.txt").unwrap();
    // for num in -10..10i32 {
    //     eprintln!("{: >5} => {}", num, (num).rem_euclid(5i32));
    // }
    // todo!();

    // let moves: Vec<isize> = contents.trim().split("\n").map(|n| n.parse().unwrap()).collect();
    // let mut positions: Vec<usize> = (0..moves.len()).collect();
    // let mut positions_wtf: Vec<usize> = (0..moves.len()).collect();
    //
    // eprintln!("Length@ {}", positions.len());
    // for idx in 0..3 {
    //     p20_move_fucked(&mut positions, &moves, idx);
    //     p20_move_wtf(&mut positions_wtf, &moves, idx);
    //     if positions != positions_wtf {
    //         eprintln!("on {}: {}", idx, moves[idx]);
    //
    //         eprintln!("{:?}", positions);
    //         eprintln!("{:?}", positions_wtf);
    //
    //         let curr = positions.iter().map(|&ix| moves[ix]).collect::<Vec<_>>();
    //         let curr_wtf = positions_wtf.iter().map(|&ix| moves[ix]).collect::<Vec<_>>();
    //
    //         eprintln!("{:?}", curr);
    //         eprintln!("{:?}", curr_wtf);
    //
    //         eprintln!();
    //         todo!();
    //     }
    // }
    // todo!();
    //

    // 不知道是不是语义上产生了偏差，我并没有在题干中读出 wtf 的操作逻辑，其歧义的地方在于，当
    // move 的元素超过list的长度是，wrapping是否要保留原来的元素，而题干的例子并没有涉及到该种
    // 情形，由此我们无法知晓：
    //
    // 有效的例子： [9, 2, -3, 3, -2, 0, 4] 移动第一个元素时候，
    // - 我的结果（错误）：[2, -3, 9, 3, -2, 0, 4] 
    // - 其他人的结果（正确）：[2, -3, 3, 9, -2, 0, 4]
    // 可以看到9向右侧移动9格子时候，是跳过自身的。
    // 从我的角度看这很难理解，如果是ring的话；但显然大部分人都理解了这个道理
    //
    // 参考：https://www.reddit.com/r/adventofcode/comments/zv0j7b/2022_day_20_part1_fails_to_work_on_real_data/
    // 这里还是要以环为核心，移动10 必须等于 移动1操作10次，如果不把原来的元素扣掉，那么移动距离=长度，就没有变化
    let moves: Vec<isize> = contents.trim().split("\n").map(|n| n.parse().unwrap()).collect();
    let mut positions: Vec<usize> = (0..moves.len()).collect();
    for idx in 0..positions.len() {
        p20_move_fucked_fixed(&mut positions, &moves, idx);
        // p20_move_wtf(&mut positions, &moves, idx);

        if idx <= 50 {
            let curr = positions.iter().map(|&ix| moves[ix]).collect::<Vec<_>>();
            if curr.len() < 50 {
                eprintln!("{: >3}@{: >3}: {:?}", idx, moves[idx % moves.len()], curr);
                if (idx + 1) % positions.len() == 0 { eprintln!(); }
            }
        } 

    }

    let mut sum_of_groves = 0;
    let curr = positions.iter().map(|&ix| moves[ix]).collect::<Vec<_>>();
    let pos = curr.iter().position(|&v| v == 0).unwrap();
    for idx in [1000, 2000, 3000] {
        let found = (pos + idx) % positions.len();
        eprintln!("{}: {}  {}", idx, positions[found], moves[positions[found]]);
        sum_of_groves += moves[positions[found]];
    }
    eprintln!("sum_of_groves: {}", sum_of_groves);

    eprintln!("==================");
    let key = 811589153;
    let moves: Vec<isize> = moves.into_iter().map(|x| x*key).collect();

    let mut positions: Vec<usize> = (0..moves.len()).collect();
    for idx in 0..(positions.len() * 10) {
        p20_move_wtf(&mut positions, &moves, idx);
        // p20_move_fucked_fixed(&mut positions, &moves, idx);

        if (idx +1) % positions.len() == 0 {
            let curr = positions.iter().map(|&ix| moves[ix]).collect::<Vec<_>>();
            if curr.len() < 50 {
                eprintln!("{: >3}@{: >3}: {:?}", idx, moves[idx % moves.len()], curr);
                if (idx + 1) % positions.len() == 0 { eprintln!(); }
            }
        } 
    }
    let mut sum_of_groves = 0;
    let curr = positions.iter().map(|&ix| moves[ix]).collect::<Vec<_>>();
    let pos = curr.iter().position(|&v| v == 0).unwrap();
    for idx in [1000, 2000, 3000] {
        let found = (pos + idx) % positions.len();
        eprintln!("{}: {}  {}", idx, positions[found], moves[positions[found]]);
        sum_of_groves += moves[positions[found]];
    }
    eprintln!("sum_of_groves: {}", sum_of_groves);
}

fn p20_move2(positions: &mut [usize], moves: &[isize], idx: usize) {
    let idx_now = positions.iter().position(|&w| w == (idx % positions.len())).unwrap();
    let movement = moves[idx % moves.len()];
    if movement > 0 {
        let movement = (movement as usize) % moves.len();
        positions.rotate_left(idx_now);
        positions[..movement+1].rotate_left(1);
    } else if movement < 0 {
        let position_len = positions.len();
        // eprintln!("???: {} {} {}", moves[idx], position_len, position_len - idx_now);
        positions.rotate_right(position_len - idx_now - 1);
        let movement = (- (movement % (position_len as isize))) as usize;
        positions[(position_len - movement - 1)..position_len].rotate_right(1);
    }
}

fn p20_move_wtf(positions: &mut [usize], moves: &[isize], idx: usize) {
    let idx_now = positions.iter().position(|&w| w == (idx % positions.len())).unwrap();
    let movement = moves[idx % moves.len()];
    if movement == 0 { return; }

    let idx_to = (idx_now as isize + movement).rem_euclid(positions.len() as isize - 1) as usize;
    match idx_now.cmp(&idx_to) {
        std::cmp::Ordering::Less => {
            positions[idx_now..idx_to+1].rotate_left(1);
        },
        std::cmp::Ordering::Greater => {
            positions[idx_to..idx_now+1].rotate_right(1);
        },
        _ => {}
    }
}

fn p20_move_fucked_fixed(positions: &mut [usize], moves: &[isize], idx: usize) {
    let idx_now = positions.iter().position(|&w| w == (idx % positions.len())).unwrap();
    let movement = moves[idx % moves.len()];
    if movement == 0 { return; }

    let idx_to = (idx_now as isize + movement).rem_euclid(positions.len() as isize - 1) as usize;
    // 先把 idx_now 的元素取出来，我们idx_now的位置(如果要还原的话)还是 idx_now
    // 在剩下的位置里面需要插入到 idx_to (前面) 里面

    // // 好理解的方式：
    // // 这段比较好理解， p20_move_wtf 中比较男理解
    // let wtf = positions.remove(idx_now);
    // positions.insert(idx_to, wtf);

    // 这段不好理解：在右侧由于本身占一格，需要offset一下
    if idx_now <= idx_to {
        // 要把 idx_now 插入到 idx_to + 1 的位置
        positions[idx_now .. idx_to + 1].rotate_left(1)
    } else {
        // 要把 idx_now 插入 idx_to 的位置
        positions[idx_to .. idx_now + 1].rotate_right(1)
    }
}

fn p20_move_fucked(positions: &mut [usize], moves: &[isize], idx: usize) {
    let idx_now = positions.iter().position(|&w| w == (idx % positions.len())).unwrap();
    let movement = moves[idx % moves.len()];
    if movement == 0 { return; }

    let idx_to = (idx_now as isize + movement).rem_euclid(positions.len() as isize) as usize;
    if movement > 0 {
        if idx_to >= idx_now {
            positions[idx_now..idx_to+1].rotate_left(1);
        } else {
            positions[idx_to + 1 .. idx_now + 1].rotate_right(1);
        }
    } else {
        if idx_to <= idx_now {
            positions[idx_to .. idx_now + 1].rotate_right(1);
        } else {
            positions[idx_now .. idx_to].rotate_left(1);
        }
    }

    // let idx_to = (idx_now as isize + movement).rem_euclid(positions.len() as isize - 1) as usize;
    // match idx_now.cmp(&idx_to) {
    //     std::cmp::Ordering::Less => {
    //         positions[idx_now..idx_to+1].rotate_left(1);
    //     },
    //     std::cmp::Ordering::Greater => {
    //         positions[idx_to..idx_now+1].rotate_right(1);
    //     },
    //     _ => {}
    // }


    // let idx_to = (
    //     (idx_now as isize) + movement + if movement >= 0 { 1 } else { -1 }
    // ).rem_euclid(positions.len() as isize);
    // let idx_to = idx_to as usize;
    //
    // // 方向很重要，与 np.searchsorted 类似
    // if movement > 0 {
    //     // 右侧
    //     if idx_now <= idx_to {
    //         // [idx_now, idx_to)
    //         positions[idx_now .. idx_to].rotate_left(1);
    //     } else {
    //         // [idx_to, idx_now]
    //         positions[idx_to .. idx_now + 1].rotate_right(1);
    //     }
    // } else {
    //     // 左侧
    //     if idx_to <= idx_now {
    //         // (idx, idx_now]
    //         positions[idx_to + 1 .. idx_now + 1].rotate_right(1);
    //     } else {
    //         // [idx_now, idx_to]
    //         positions[idx_now .. idx_to + 1].rotate_left(1);
    //     }
    // }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum P21MonkeyOp {
    Add, Sub, Div, Mul,
}

impl P21MonkeyOp {
    fn rev(self) -> Self {
        use P21MonkeyOp::*;
        match self {
            Add => Sub,
            Sub => Add,
            Div => Mul,
            Mul => Div,
        }
    }
}

#[derive(Debug, Clone)]
enum P21MonkeyValue {
    Yell(isize),
    Option {
        left: String,
        right: String,
        op: P21MonkeyOp
    }
}

#[derive(Debug, Clone)]
struct P21Monkey {
    name: String,
    value: P21MonkeyValue
}

impl P21Monkey {
    fn is_yell(&self) -> bool {
        matches!(self.value, P21MonkeyValue::Yell(_))
    }

    fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::character::complete::{alphanumeric1, space0};
        use nom::bytes::complete::tag;
        use nom::combinator::{map, value};
        use nom::sequence::tuple;
        
        let (input, name) = alphanumeric1(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = space0(input)?;
        let op = nom::branch::alt((
            value(P21MonkeyOp::Add, tag::<&str, &str, _>("+")),
            value(P21MonkeyOp::Sub, tag("-")),
            value(P21MonkeyOp::Div, tag("/")),
            value(P21MonkeyOp::Mul, tag("*")),
        ));

        let (input, value) = nom::branch::alt((
            map(nom::character::complete::i64, |v| P21MonkeyValue::Yell(v as isize)),
            map(
                tuple((alphanumeric1, space0, op, space0, alphanumeric1)),
                |(left, _, op, _, right)| P21MonkeyValue::Option { left: left.to_owned(), right: right.to_owned(), op }
            ),
        ))(input)?;
        Ok((input, Self { name: name.to_owned(), value }))
    }
}


fn p21_parse(input: &str) -> nom::IResult<&str, Vec<P21Monkey>> {
    nom::multi::separated_list0(nom::character::complete::newline, P21Monkey::parse)(input)
}

pub fn p21() {
    let contents = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;

    let wtf = r#"
humn: ptdq + dvpt = 298 + 3 = 301
dvpt: 3
ptdq: lgvd / ljgn = 596 / 2 = 298
lgvd: cczh - sllz = 600 - 4 = 596
ljgn: 2
sllz: 4
cczh: pppw * lfqf = 150 * 4 = 600
pppw: sjmn + 0   = 150
lfqf: 4
sjmn: drzm * dbpl = 150
dbpl: 5
drzm: hmdt - zczc = 30
zczc: 2
hmdt: 32"#;

    let contents = &std::fs::read_to_string("./assets/adv2022/adv21.txt").unwrap();
    let monkeys = p21_parse(contents).unwrap().1;
    // dbg!(monkeys);
    p21_solve(monkeys.clone());

    eprintln!("=================");
    let mut unfound = vec!["humn"];
    let mut monkeys_rev = vec![];
    let mut used = vec![false; monkeys.len()];
    while unfound.len() > 0 {
        let name = unfound.pop().unwrap();
        for idx in 0..monkeys.len() {
            if used[idx] { continue; }
            let m = &monkeys[idx];
            if m.name == "humn" {

            } else {
                if m.name == name {
                    used[idx] = true;
                    monkeys_rev.push(m.clone());
                    break;
                } else if let P21MonkeyValue::Option { left, right, op } = &m.value {
                    let op = if m.name == "root" { P21MonkeyOp::Sub } else { *op };
                    if left == name {
                        monkeys_rev.push(P21Monkey {
                            name: name.to_owned(),
                            value: P21MonkeyValue::Option { 
                                left: m.name.to_owned(),
                                right: right.clone(),
                                op: op.rev()
                            }
                        });
                        unfound.push(right);
                        unfound.push(&m.name);
                        used[idx] = true;
                        break;
                    } else if right == name {
                        let op_rev = match op {
                            P21MonkeyOp::Add => P21MonkeyOp::Sub,
                            P21MonkeyOp::Sub => P21MonkeyOp::Sub,
                            P21MonkeyOp::Div => P21MonkeyOp::Div,
                            P21MonkeyOp::Mul => P21MonkeyOp::Div,
                        };
                        let (left0, right0) = match op {
                            P21MonkeyOp::Add | P21MonkeyOp::Mul => {
                                (m.name.to_owned(), left.clone())
                            },
                            _ => {
                                (left.clone(), m.name.to_owned())
                            }
                        };
                        monkeys_rev.push(P21Monkey {
                            name: name.to_owned(),
                            value: P21MonkeyValue::Option { 
                                left: left0, right: right0, op: op_rev,
                            }
                        });
                        unfound.push(left);
                        unfound.push(&m.name);
                        used[idx] = true;
                        break;
                    }
                }
            }
        }
    }
    for idx in 0..monkeys.len() {
        if used[idx] { continue; }
        if monkeys[idx].name == "root" || monkeys[idx].name == "humn" { continue; }
        monkeys_rev.push(monkeys[idx].clone());
        // if let P21MonkeyValue::Yell(_) = &monkeys[idx].value {
        // }
    }
    monkeys_rev.push(P21Monkey {
        name: "root".to_owned(),
        value: P21MonkeyValue::Yell(0)
    });
    // dbg!(&monkeys_rev);
    p21_solve(monkeys_rev.clone());
}



fn p21_solve(mut monkeys: Vec<P21Monkey>) {
    let name_to_index: HashMap<String, usize> = monkeys.iter().enumerate().map(|(idx, m)| (m.name.clone(), idx)).collect();

    while monkeys.iter().any(|m| !m.is_yell()) {
        let mut is_updated = false;
        for idx in 0..monkeys.len() {
            match &monkeys[idx].value {
                P21MonkeyValue::Yell(_) => { continue; },
                P21MonkeyValue::Option { left, right, op } => {
                    let left = *name_to_index.get(left).unwrap();
                    let right = *name_to_index.get(right).unwrap();
                    match (&monkeys[left].value, &monkeys[right].value) {
                        (P21MonkeyValue::Yell(left), P21MonkeyValue::Yell(right)) => {
                            let out = match op {
                                P21MonkeyOp::Add => { left + right }
                                P21MonkeyOp::Sub => { left - right }
                                P21MonkeyOp::Div => { left / right }
                                P21MonkeyOp::Mul => { left * right }
                            };
                            eprintln!("{} -> {}", monkeys[idx].name, out);
                            monkeys[idx].value = P21MonkeyValue::Yell(out);
                            is_updated = true;
                        },
                        _ => { continue; }
                    }
                }
            }
        }

        assert!(is_updated, "no update in this loop");
    }
}

pub fn p22() {
    let contents = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;
    // let contents = &std::fs::read_to_string("./assets/adv2022/adv22.txt").unwrap();

    // -1 -> empty
    // 0  -> space
    // 1  -> tile
    let (_, (blocks, instructions)) = p22_parse(contents).unwrap();
    let height = blocks.len(); assert!(height > 0);
    let width = blocks[0].len(); assert!(width > 0);
    let limits_x: Vec<(usize, usize)> = (0..height).map(|ny| {
        (
            blocks[ny as usize].iter().position(|&y| y >= 0).unwrap(),
            width - 1 - blocks[ny as usize].iter().rev().position(|&y| y >= 0).unwrap(),
        )
    }).collect();
    let limits_y: Vec<(usize, usize)> = (0..width).map(|nx| {
        (
            blocks.iter().map(|row| &row[nx as usize]).position(|&y| y >= 0).unwrap(),
            height - 1 - blocks.iter().rev().map(|row| &row[nx as usize]).position(|&y| y >= 0).unwrap()
        )
    }).collect();


    let mut ypos = 0;
    let mut xpos = blocks[0].iter().position(|&x| x == 0).unwrap();
    let mut face = 0;
    for &instr in instructions.iter() {
        p22_update(
            &mut xpos, &mut ypos, &mut face, instr, 
            &blocks, height, width, &limits_x, &limits_y
        );
    }
    dbg!(xpos, ypos, face);
    eprintln!("Score: {}", 1000 * (ypos + 1) + 4 * (xpos + 1) + face as usize);

    eprintln!("=================================");
    // 第二部分很简单，跟第一部分不同只是替换了边界点的移动逻辑。但最复杂的地方在于如何去恢复这样的拼凑环节，
    // 肉眼可以一眼完成，但怎么在程序里面去搜索？
    // 选择通过选择和镜像把数据折叠成一样
    p22_display(&blocks, height, width);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum P22Move {
    Forward(usize),
    L,
    R,
}

fn p22_parse(input: &str) -> nom::IResult<&str, (Vec<Vec<i8>>, Vec<P22Move>)> {
    use nom::combinator::{map, value};
    let (input, mut blocks) = nom::multi::separated_list1(
        nom::character::complete::newline,
        nom::multi::many1(nom::branch::alt((
            value(-1, nom::character::complete::char(' ')),
            value( 0, nom::character::complete::char('.')),
            value( 1, nom::character::complete::char('#')),
        )))
    )(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, instructions) = nom::multi::many1(
        nom::branch::alt((
            value(P22Move::L, nom::character::complete::char('L')),
            value(P22Move::R, nom::character::complete::char('R')),
            map(nom::character::complete::u64, |x| P22Move::Forward(x as usize))
        ))
    )(input)?;
    // let height = blocks.len();
    let width = blocks.iter().map(|row| row.len()).max().unwrap();
    blocks.iter_mut().for_each(|row| row.resize(width, -1));

    Ok((input, (blocks, instructions)))
}

fn p22_update(
    xpos: &mut usize, ypos: &mut usize, face: &mut i8, instr: P22Move, 
    blocks: &Vec<Vec<i8>>, height: usize, width: usize, limits_x: &[(usize, usize)], limits_y: &[(usize, usize)]
) {
    // eprintln!("{} {} {:?}", xpos, ypos, instr);
    match instr {
        P22Move::Forward(step) => {
            for _ in 0..step {
                let (mut nx, mut ny) = match *face {
                    0 => (*xpos as isize + 1, *ypos as isize),
                    1 => (*xpos as isize    , *ypos as isize + 1),
                    2 => (*xpos as isize - 1, *ypos as isize),
                    3 => (*xpos as isize    , *ypos as isize - 1),
                    _ => unreachable!(),
                };
                if ny == *ypos as isize {
                    if nx < limits_x[ny as usize].0 as isize {
                        nx = limits_x[ny as usize].1 as isize;
                    }
                    if nx > limits_x[ny as usize].1 as isize {
                        nx = limits_x[ny as usize].0 as isize;
                    }
                }
                if nx == *xpos as isize {
                    if ny < limits_y[nx as usize].0 as isize {
                        ny = limits_y[nx as usize].1 as isize;
                    }
                    if ny > limits_y[nx as usize].1 as isize {
                        ny = limits_y[nx as usize].0 as isize;
                    }
                }

                // if nx == -1 { 
                //     let limit = blocks[ny as usize].iter().rev().position(|&y| y >= 0).unwrap();
                //     nx = (width - 1 - limit) as isize;
                // }
                // if ny == -1 {
                //     let limit = blocks.iter().rev().map(|row| &row[nx as usize]).position(|&y| y >= 0).unwrap();
                //     ny = (height - 1 - limit) as isize;
                // }
                // if nx == width as isize {
                //     nx = blocks[ny as usize].iter().position(|&y| y >= 0).unwrap() as isize;
                // }
                // if ny == height as isize {
                //     nx = blocks.iter().map(|row| &row[nx as usize]).position(|&y| y >= 0).unwrap() as isize;
                // }
                
                // eprintln!(">>> {} {} {} {}", nx, ny, width, height);
                // if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                    let block = blocks[ny as usize][nx as usize];
                    if block == 0 { 
                        *xpos = nx as usize;
                        *ypos = ny as usize;
                    } else {
                        break;
                    }
                // } else {
                //     break;
                // }
            }
        },
        P22Move::L => {
            *face -= 1;
            *face = (*face).rem_euclid(4);
        },
        P22Move::R => {
            *face += 1;
            *face = (*face).rem_euclid(4);
        },
    }
}

fn p22_display(blocks: &Vec<Vec<i8>>, height: usize, width: usize) {
    for y in 0..height {
        for x in 0..width {
            match blocks[y][x] {
                0 => print!("."),
                1 => print!("#"),
                _ => print!(" "),
            }
        }
        println!();
    }
}

pub fn p23() {
    let contents = r#".....
..##.
..#..
.....
..##.
....."#;

    let contents = r#"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
.............."#;

    let contents = &std::fs::read_to_string("./assets/adv2022/adv23.txt").unwrap();
    
    let (_, points) = p23_parse(contents).unwrap();
    let mut blocks: HashSet<(isize, isize)> = Default::default();
    for y in 0..points.len() {
        for x in 0..points[0].len() {
            if points[y][x] == 1 {
                blocks.insert((x as isize, y as isize));
            }
        }
    }
    // eprintln!("{:?}", blocks);

    // p23_display(&blocks);
    // eprintln!();

    let mut blocks_part1 = blocks.clone();
    for idx in 0..10 {
        p23_iter(&mut blocks_part1, idx);
        // p23_display(&blocks_part1);
        // eprintln!();
    }

    eprintln!("Score: {}", p23_score(&blocks_part1));
    eprintln!("===========");
    let mut blocks_part2 = blocks.clone();
    for idx in 0.. {
        if !p23_iter(&mut blocks_part2, idx) {
            eprintln!("Finished@ {}", idx + 1);
            break;
        }
        // eprintln!("{:?}", blocks_part2);
        // p23_display(&blocks_part2);
        // eprintln!();
        // dbg!(idx);
    }
    
}

fn p23_score(blocks: &HashSet<(isize, isize)>) -> isize {
    let xmin = blocks.iter().map(|&(x, y)| x).min().unwrap();
    let xmax = blocks.iter().map(|&(x, y)| x).max().unwrap();
    let ymin = blocks.iter().map(|&(x, y)| y).min().unwrap();
    let ymax = blocks.iter().map(|&(x, y)| y).max().unwrap();

    (xmax - xmin + 1) * (ymax - ymin + 1) - (blocks.len() as isize)
}

fn p23_display(blocks: &HashSet<(isize, isize)>) {
    let xmin = blocks.iter().map(|&(x, y)| x).min().unwrap();
    let xmax = blocks.iter().map(|&(x, y)| x).max().unwrap();
    let ymin = blocks.iter().map(|&(x, y)| y).min().unwrap();
    let ymax = blocks.iter().map(|&(x, y)| y).max().unwrap();

    for y in ymin..ymax+1 {
        for x in xmin..xmax+1 {
            if blocks.contains(&(x, y)) {
                eprint!("#")
            } else {
                eprint!(".")
            }
        }
        eprintln!();
    }
}

fn p23_parse(input: &str) -> nom::IResult<&str, Vec<Vec<i8>>> {
    nom::multi::separated_list0(
        nom::character::complete::newline, 
        nom::multi::many1(nom::branch::alt((
            nom::combinator::value(0, nom::character::complete::char('.')),
            nom::combinator::value(1, nom::character::complete::char('#')),
        )))
    )(input)
}

fn p23_iter(blocks: &mut HashSet<(isize, isize)>, idx: usize) -> bool {
    let mut proposes = Vec::with_capacity(blocks.len());
    const VXY: [(isize, isize); 4] = [ (0, -1), (0, 1), (-1, 0), (1, 0), ];

    for &(x, y) in blocks.iter() {
        let mut px = x;  let mut py = y;

        let mut is_movable = false;
        for xx in -1..1+1 {
            for yy in -1..1+1 {
                if !(xx == 0 && yy == 0) && blocks.contains(&(x+xx, y + yy)){
                    is_movable = true;
                    break;
                }
            }
        }

        if is_movable {
            for shift in 0..VXY.len() {
                let (vx, vy) = VXY[ (shift + idx) % VXY.len() ];
                let nx = x + vx; let ny = y + vy;
                let mut adjacent = Vec::with_capacity(3);
                adjacent.push((nx, ny));
                if vx == 0 {
                    adjacent.push((x + 1, ny));
                    adjacent.push((x - 1, ny));
                }
                if vy == 0 {
                    adjacent.push((nx, ny + 1));
                    adjacent.push((nx, ny - 1));
                }
                // eprintln!("{:?}", adjacent);
                debug_assert_eq!(adjacent.len(), 3);

                if adjacent.into_iter().all(|(x, y)| !blocks.contains(&(x, y))) {
                    px = nx; py = ny;
                    break;
                }
            }
        }

        proposes.push(((x, y), (px, py)));
    }
    // eprintln!("{:?}", blocks);
    // eprintln!("{:?}", proposes);

    let mut proposes_map : HashMap<(isize, isize), usize> = HashMap::with_capacity(proposes.len());
    for (_, ppos) in proposes.iter() {
        let count = proposes_map.entry(ppos.clone()).or_insert(0);
        *count += 1;
    }
    
    blocks.clear();
    let mut is_moved = false;
    for ((x, y), (px, py)) in proposes.into_iter() {
        if (x, y) != (px, py) { is_moved = true; }
        if *proposes_map.get(&(px, py)).unwrap() > 1 {
            blocks.insert((x, y));
        } else {
            blocks.insert((px, py));
        }
    }
    is_moved
}

fn p24_parse(input: &str) -> nom::IResult<&str, Vec<Vec<i8>>> {
    nom::multi::separated_list1(
        nom::character::complete::newline, 
        nom::multi::many1(nom::branch::alt((
            nom::combinator::value(0, nom::character::complete::char('>')),
            nom::combinator::value(1, nom::character::complete::char('v')),
            nom::combinator::value(2, nom::character::complete::char('<')),
            nom::combinator::value(3, nom::character::complete::char('^')),
            nom::combinator::value(-1, nom::character::complete::char('.')),
            nom::combinator::value(-2, nom::character::complete::char('#')),
        )))
    )(input)
}

fn p24_display(blizzards: &Vec<Vec<i8>>, height: usize, width: usize) {
    for y in 0..height {
        for x in 0..width {
            match blizzards[y][x] {
                -1 => eprint!("."),
                -2 => eprint!("#"),
                0 => eprint!(">"),
                1 => eprint!("v"),
                2 => eprint!("<"),
                3 => eprint!("^"),
                _ => unreachable!()
            }
        }
        eprintln!();
    }
}

#[derive(Debug, Clone, Default)]
struct P24Blizzards {
    height: usize,
    width: usize,
    left: Vec<(isize, isize)>,
    right: Vec<(isize, isize)>,
    up: Vec<(isize, isize)>,
    down: Vec<(isize, isize)>,
}

impl P24Blizzards {
    fn new(blizzards: &Vec<Vec<i8>>) -> Self {
        let mut slf: Self = Default::default();
        slf.height = blizzards.len() - 2;
        slf.width = blizzards[0].len() - 2;
        for y in 1..blizzards.len()-1 {
            for x in 1..blizzards[0].len()-1 {
                let pos = (x as isize - 1, y as isize - 1);
                match blizzards[y][x] {
                    0 => { slf.right.push(pos) }
                    1 => { slf.down.push(pos) }
                    2 => { slf.left.push(pos) }
                    3 => { slf.up.push(pos) }
                    _ => { }
                }
            }
        }
        slf
    }

    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x as isize, y as isize);
                let count = self.right.iter()
                    .chain(self.up.iter())
                    .chain(self.left.iter())
                    .chain(self.down.iter())
                    .filter(|&&x| x == pos).count();
                if count > 0 {
                    eprint!("{}", count);
                } else {
                    eprint!(".");
                }
            }
            eprintln!();
        }
    }

    fn step(&mut self) {
        // dbg!(self.height, self.width);
        for (pos, (vx, vy)) in self.right.iter_mut().zip(std::iter::repeat((1, 0)))
            .chain( self.up.iter_mut().zip(std::iter::repeat((0, -1))) )
            .chain( self.left.iter_mut().zip(std::iter::repeat((-1, 0))) )
            .chain( self.down.iter_mut().zip(std::iter::repeat((0, 1))) )
        {
            let (x, y) = *pos;
            let mut nx = x + vx; let mut ny = y + vy;
            if nx < 0 { nx = self.width as isize - 1; }
            if nx >= self.width as isize { nx = 0; }
            if ny < 0 { ny = self.height as isize - 1; }
            if ny >= self.height as isize { ny = 0; }
            // eprintln!("{:?} vs {:?}", pos, (nx, ny));
            *pos = (nx, ny);
        }
    }

    fn contains(&self, point: (isize, isize)) -> bool {
        self.right.iter()
            .chain(self.left.iter())
            .chain(self.up.iter())
            .chain(self.down.iter())
            .any(|&pos| pos == point)
    }
}

pub fn p24() {
    let contents = r#"#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#"#;
    let contents = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;

    let contents = &std::fs::read_to_string("./assets/adv2022/adv24.txt").unwrap();

    let (_, blizzards_orig) = p24_parse(contents).unwrap();
    // let height = blizzards.len(); debug_assert!(height > 0);
    // let width = blizzards[0].len(); debug_assert!(width > 0);
    //
    // p24_display(&blizzards, height, width);

    let mut blizzards = P24Blizzards::new(&blizzards_orig);
    // blizzards.display();
    // for _ in 0..18 { blizzards.step(); }
    // eprintln!();
    // blizzards.display();

    let stop = (blizzards.width as isize - 1, blizzards.height as isize); let start = (0, -1);
    let shortest = p24_search_bfs(&mut blizzards, start, stop);
    eprintln!("{}", shortest);
    eprintln!("=====================");

    let mut blizzards = P24Blizzards::new(&blizzards_orig);
    let stop = (blizzards.width as isize - 1, blizzards.height as isize); let start = (0, -1);
    let shortest1 = p24_search_bfs(&mut blizzards, start, stop);
    dbg!(shortest1);
    let shortest2 = p24_search_bfs(&mut blizzards, stop, start);
    dbg!(shortest2);
    let shortest3 = p24_search_bfs(&mut blizzards, start, stop);
    dbg!(shortest3);
    dbg!(shortest1 + shortest2 + shortest3);
}

fn p24_search_bfs(blizzards: &mut P24Blizzards, start: (isize, isize), stop: (isize, isize)) -> usize {
    let mut reached: HashSet<_> = vec![start].into_iter().collect();
    for count in 1.. {
        blizzards.step();

        for (sx, sy) in std::mem::replace(&mut reached, Default::default()).into_iter() {
            for (vx, vy) in [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)] {
                let nx = sx + vx; let ny = sy + vy;
                if (nx, ny) == stop { return count; }

                // 这个条件和下面的条件不重合
                if (nx, ny) == start { reached.insert((nx, ny)); }
                // eprintln!(">>> {:?}", (nx, ny));
                if nx >= 0 && nx < blizzards.width as isize && ny >= 0 && ny < blizzards.height as isize {
                    if !blizzards.contains((nx, ny)) {
                        reached.insert((nx, ny));
                    }
                }
            }
        }
        // eprintln!("{}: {:?}", count, reached);
        assert!(reached.len() > 0);
    }

    unreachable!();
}

pub fn p25() {
    let contents = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;
    let contents = &std::fs::read_to_string("./assets/adv2022/adv25.txt").unwrap();

    let mut sum = 0;
    for num in contents.trim().split("\n") {
        let mut out = 0;
        for (idx, digit) in num.chars().enumerate() {
            out *= 5;
            out += match digit {
                '1' => 1,
                '2' => 2,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => unreachable!()
            };
        }
        sum += out;
        eprintln!("{: >20} - {}", num, out);
    }

    eprintln!("Sum of all: {}", sum);
    eprintln!("Sum of all: {:?}", p25_encode(sum));
}

fn p25_encode(num: isize) -> String {
    const DIGITS: [char; 5] = ['=', '-', '0', '1', '2'];
    // 字首只能是 [1, 2] = [3, 4]
    // 其余位置为 ['=', '-', '0', '1', '2'] = [0, 1, 2, 3, 4]
    let mut base = 2; let mut count = 0;
    while base < num {
        base = base * 5 + 2;
        count += 1;
    }
    // count 是 0 为index
    // 2   -> 0
    // 22  -> 1
    // 222 -> 2

    // xn = 5^0 + 5^1 + 5^2 ... + 5^n 
    // 5 * xn + 5^0 = xn + 5^(n+1)
    // xn = ( 5^(n+1) - 1 ) / 4
    let left = (5_isize.pow(count) - 1) / 4 ;
    let left = left * -2;
    assert_eq!((5_isize.pow(count) - 1) % 4, 0);
    let one_base = 5_isize.pow(count) + left;
    let two_base = 2 * 5_isize.pow(count) + left;

    let mut out = vec![];
    let mut offset = if num < two_base {
        out.push(DIGITS[3]);
        num - one_base
    } else {
        out.push(DIGITS[4]);
        num - two_base
    };

    for idx in (0..count).rev() {
        let base = 5_isize.pow(idx as u32);
        let digit = offset / base;
        assert!(digit >= 0 && digit <= 4, "digit invalid: {}, offset={}, base={}", digit, offset, base);
        out.push(DIGITS[digit as usize]);
        offset = offset % base;
    }
    out.into_iter().collect()
}
