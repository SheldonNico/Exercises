use std::iter::Iterator;

pub fn thirt(n: i64) -> i64 {
    const REMINDERS: [i64; 6] = [1%13, 10%13, 100%13, 1_000%13, 10_000%13, 100_000%13];

    let sum = REMINDERS.into_iter().cycle().scan(n, |acc, a| {
        if *acc <= 0 {
            None
        } else {
            let o = Some( (*acc % 10) * a );
            *acc = *acc / 10;
            o
        }
    }).sum();

    if sum == n {
        sum
    } else {
        thirt(sum)
    }
}

pub fn part_list(arr: Vec<&str>) -> String {
    let mut out = String::new();

    for i in 1..arr.len() {
        out.push('(');
        out.push_str(&arr[..i].join(" "));
        out.push_str(", ");
        out.push_str(&arr[i..].join(" "));
        out.push(')');
    }

    out
}

fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

fn seven(n: i64) -> (i64, i32) {
    let mut n = n;
    let mut count = 0;
    while n > 99 {
        count += 1;
        n = (n/10) - (n%10)*2;
    }
    (n, count)
}

fn row_weights(array: Vec<u32>) -> (u32, u32) {
    array.chunks(2).fold( (0, 0), |acc, xy| {
        (acc.0+xy[0], acc.1+xy.get(1).unwrap_or(&0))
    })
}

fn brain_luck(code: &str, stdin: Vec<u8>) -> Vec<u8> { 
    let mut cpoint = 0;
    let mut dpoint = 0;
    let mut buffer = vec![0];
    let mut stdin = stdin.into_iter();
    let mut stdout = Vec::new();

    let codes: Vec<char> = code.chars().collect();

    while let Some(c) = codes.get(cpoint) { 
        match c {
            '>' => { 
                dpoint += 1;
                if dpoint >= buffer.len() {
                    buffer.resize(dpoint+1, 0);
                }
            },
            '<' => { 
                if dpoint > 0 {
                    dpoint -= 1;
                } else {
                    buffer.insert(0, 0);
                }
            },
            '-' => { 
                if buffer[dpoint] > 0 {
                    buffer[dpoint] -= 1;
                } else {
                    buffer[dpoint] = 0
                }
            },
            '+' => { 
                if buffer[dpoint] < 255 {
                    buffer[dpoint] += 1;
                } else {
                    buffer[dpoint] = 0;
                }
            },
            ',' => { 
                if let Some(i) = stdin.next() {
                    buffer[dpoint] = i;
                } else {
                    break;
                }
            },
            '[' => { 
                if buffer[dpoint] == 0 {
                    let mut counter = 1;
                    for c in codes[cpoint+1..].iter() {
                        cpoint += 1;
                        if *c == ']' && counter == 1 {
                            break; 
                        } else if *c == ']' {
                            counter -= 1;
                        } else if *c == '[' {
                            counter += 1;
                        }
                    }
                }
            },
            ']' => { 
                if buffer[dpoint] != 0 {
                    let mut counter = 1;
                    for c in codes[..cpoint].iter().rev() {
                        cpoint -= 1;
                        if *c == '[' && counter == 1 {
                            break;
                        } else if *c == '[' {
                            counter -= 1;
                        } else if *c == ']' {
                            counter += 1;
                        }
                    }
                }
            },
            '.' => {
                stdout.push(buffer[dpoint]);
            },
            _ => {  },
        }
        cpoint += 1;
    }

  stdout
}

fn step(g: u64, m: u64, n: u64) -> Option<(u64, u64)> {
    if n <= g { return None; }
    let mut primes: Vec<u64> = vec![2];

    'outer: for p in (3..( (n as f64).sqrt() as u64 + 1 )) {
        'inner: for v in primes.iter().take_while(|&&v| v < ((p as f64).sqrt() as u64 + 1)) {
            if p % v == 0 {
                continue 'outer;
            }
        }
        primes.push(p);
    }

    let is_prime = |num: u64| {
        for p in primes.iter().take_while(|&&v| v < ((num as f64).sqrt() as u64 + 1)) {
            if num % p == 0 {
                return false;
            }
        }
        return true;
    };

    for min in (m..=(n-g)) {
        let max = min + g;
        if is_prime(max) && is_prime(min) {
            return Some( (min, max) );
        }
    }

    None
}

use regex::Regex;
use std::borrow::Borrow;
fn phone(dir: &str, num: &str) -> String {
    // TODO: too ugly
    let num_re = Regex::new(&format!("\\+{}", num)).unwrap();
    let addr_re = Regex::new(r"<.*>").unwrap();

    for line in dir.split("\n") {
        let size_p: isize = num_re.find_iter(line).map(|_| 1).sum();
        let size_a: isize = addr_re.find_iter(line).map(|_| 1).sum();
        if size_a == 1 && size_p == 1{ 
            let addr = addr_re.find(line).unwrap();
            let num = num_re.find(line).unwrap();
            let addr = &line[addr.start()..addr.end()];
            let num = &line[num.start()..num.end()];
        }


    }
    String::new()
}

fn stati(strg: &str) -> String {
    if strg.len() == 0 { return "".to_string(); }
    let mut result : Vec<isize> = vec![];
    for line in strg.split(",") {
        if let [ h, m, s ] = line.split("|").map(|s| s.trim().parse::<isize>().expect("format not right") ).collect::<Vec<isize>>().as_slice() {
            result.push( h*60*60 + m*60 + s);
        }
    }

    let range_ = result.iter().max().unwrap() - result.iter().min().unwrap();
    let mean_ = {
        let s: isize = result.iter().sum();
        s as f64 / (result.len() as f64) 
    } as isize;
    let median_ = {
        let mut result = result.clone();
        result.sort();
        let middle = result.len() / 2;
        if result.len() % 2 == 0{
            (result[middle]+result[middle-1]) / 2
        } else {
            result[middle]
        }
    };

    format!("Range: {:02}|{:02}|{:02} Average: {:02}|{:02}|{:02} Median: {:02}|{:02}|{:02}",
            range_ / 3600, (range_ % 3600) / 60, (range_ % 60),
            mean_ / 3600, (mean_ % 3600) / 60, (mean_ % 60),
            median_ / 3600, (median_ % 3600) / 60, (median_ % 60),
            )
}

fn parenthesis(n: isize) -> Vec<String> {
    fn dfs(n: isize, left: isize, right: isize, res: &mut Vec<String>, paren: &mut String) {
        if (n == left) && (n == right) {
            res.push( paren.to_string() );
            return;
        } else if left < n {
            let mut p = paren.clone();
            p.push('(');
            dfs(n, left+1, right, res, &mut p);

            if right < left {
                paren.push(')');
                dfs(n, left, right+1, res, paren);
            }
        } else {
            paren.push(')');
            dfs(n, left, right+1, res, paren);
        }
    }

    let mut result = Vec::new();
    dfs(n, 0, 0, &mut result, &mut String::new());

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thirt() {
        assert_eq!(thirt(1234567), 87);
        assert_eq!(thirt(8529), 79);
        assert_eq!(thirt(85299258), 31);
        assert_eq!(thirt(5634), 57);
    }

    #[test]
    fn test_part_list() {
        assert_eq!(part_list(vec!["I", "wish", "I", "hadn't", "come"]),
                "(I, wish I hadn't come)(I wish, I hadn't come)(I wish I, hadn't come)(I wish I hadn't, come)");
        assert_eq!(part_list(vec!["cdIw", "tzIy", "xDu", "rThG"]), 
            "(cdIw, tzIy xDu rThG)(cdIw tzIy, xDu rThG)(cdIw tzIy xDu, rThG)");
        
    }

    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1,2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1,2,2], vec![1]), vec![2,2]);
        assert_eq!(array_diff(vec![1,2,2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1,2,2], vec![]), vec![1,2,2]);
        assert_eq!(array_diff(vec![], vec![1,2]), vec![]);
    }

    #[test]
    fn test_seven() {
        assert_eq!(seven(477557101), (28, 7));
        assert_eq!(seven(371), (35, 1));
        assert_eq!(seven(477557102), (47, 7));
    }

    #[test]
    fn test_row_weights() {
        assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
        assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
        assert_eq!(row_weights(vec![80]), (80,0));
    }

    #[test]
    fn test_brain_luck() {
        assert_eq!( String::from_utf8( brain_luck(",+[-.,+]", "Codewars".to_string().into_bytes()) ).unwrap(), "Codewars" );
        assert_eq!( brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![9, 8]), vec![72] );
    }

    #[test]
    fn test_step() {
        //assert_eq!( step(2, 100, 110), Some((101, 103)) );
        assert_eq!( step(4, 100, 110), Some((103, 107)) );
        assert_eq!( step(8, 30000, 100000), Some((30089, 30097)) );
        assert_eq!( step(11,30000,100000), None );
        assert_eq!( step(2,10000000,11000000), Some((10000139, 10000141)) );
    }

    #[test]
    fn test_phone() {
        let content = r#"/+1-541-754-3010 156 Alphand_St. <J Steeve>
133, Green, Rd. <E Kustur> NY-56423 ;+1-541-914-3010;
+1-541-984-3012 <P Reed> /PO Box 530; Pollocksville, NC-28573
:+1-321-512-2222 <Paul Dive> Sequoia Alley PQ-67209
+1-741-984-3090 <Peter Reedgrave> _Chicago
:+1-921-333-2222 <Anna Stevens> Haramburu_Street AA-67209
+1-111-544-8973 <Peter Pan> LA
+1-921-512-2222 <Wilfrid Stevens> Wild Street AA-67209
<Peter Gone> LA ?+1-121-544-8974 
<R Steell> Quora Street AB-47209 +1-481-512-2222!
<Arthur Clarke> San Antonio $+1-121-504-8974 TT-45120
<Ray Chandler> Teliman Pk. !+1-681-512-2222! AB-47209,
<Sophia Loren> +1-421-674-8974 Bern TP-46017
<Peter O'Brien> High Street +1-908-512-2222; CC-47209
<Anastasia> +48-421-674-8974 Via Quirinal Roma
<P Salinger> Main Street, +1-098-512-2222, Denver
<C Powel> *+19-421-674-8974 Chateau des Fosses Strasbourg F-68000
<Bernard Deltheil> +1-498-512-2222; Mount Av.  Eldorado
+1-099-500-8000 <Peter Crush> Labrador Bd.
+1-931-512-4855 <William Saurin> Bison Street CQ-23071
<P Salinge> Main Street, +1-098-512-2222, Denve
/+5-541-754-3010 156 Alphandria_Street. <Jr Part>
1333, Green, Road <F Fulgur> NW-46423 ;+6-541-914-3010!
+5-541-984-3012 <Peter Reeves> /PO Box 5300; Albertville, SC-28573
:+5-321-512-2222 <Paulo Divino> Boulder Alley ZQ-87209
+3-741-984-3090 <F Flanaghan> _Chicago Av.
:+3-921-333-2222 <Roland Scorsini> Bellevue_Street DA-67209
+8-111-544-8973 <Laurence Pantow> SA
+8-921-512-2222 <Raymond Stevenson> Joly Street EE-67209
<John Freeland> Mantow ?+2-121-544-8974
<Robert Mitch> Eleonore Street QB-87209 +2-481-512-2222?
<Arthur Paternos> San Antonio $+7-121-504-8974 TT-45121
<Ray Charles> Stevenson Pk. !+7-681-512-2222! CB-47209,
<JP Gorce> +9-421-674-8974 New-Bern TP-16017
<P McDon> Revolution Street +2-908-512-2222; PP-47209
<Elizabeth Corber> +8-421-674-8974 Via Papa Roma
<C Saborn> Main Street, +15-098-512-2222, Boulder
<Colin Marshall> *+9-421-674-8974 Edinburgh UK
<Bernard Povit> +3-498-512-2222; Hill Av.  Cameron
+12-099-500-8000 <Pete Highman> Ontario Bd.
+8-931-512-4855 <W Mount> Oxford Street CQ-23071
<Donald Drinkaw> Moon Street, +3-098-512-2222, Peterville
"#;
    //assert_eq!(phone(content, "48-421-674-8974"), "Phone => 48-421-674-8974, Name => Anastasia, Address => Via Quirinal Roma")
    }

    #[test]
    fn test_stati() {
        assert_eq!(stati("01|15|59, 1|47|16, 01|17|20, 1|32|34, 2|17|17"), "Range: 01|01|18 Average: 01|38|05 Median: 01|32|34");
        assert_eq!(stati("02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|17|17, 2|22|00, 2|31|41"), "Range: 00|31|17 Average: 02|26|18 Median: 02|22|00");
    }

    #[test]
    fn test_parens() {
        assert_eq!(parenthesis(1), vec!["()".to_string()]);
        assert_eq!(parenthesis(2), vec!["(())".to_string(), "()()".to_string()]);
        assert_eq!(parenthesis(3), vec!["((()))".to_string(), "(()())".to_string(), "(())()".to_string(), "()(())".to_string(), "()()()".to_string()]);
    }
}


