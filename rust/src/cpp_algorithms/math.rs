
fn is_armstrong_getdi(mut curr: i32) -> u32 {
    let mut di = 0;
    while curr > 0 {
        curr = curr / 10;
        di += 1;
    }
    di
}

pub fn is_armstrong(num: i32) -> bool {
    let mut sum = 0; let mut curr = num;
    let k = is_armstrong_getdi(num);
    while curr > 0 {
        let rem = curr % 10;
        curr = curr / 10;

        sum = sum + rem.pow(k);
    }

    num == sum
}

pub fn bin_expo(mut a: i32, mut b: i32) -> i32 {
    let mut res = 1;
    while b > 0 {
        if b % 2 > 0 {
            res = res * a;
        }
        a = a * a;
        b = b / 2;
    }
    return res;
}

pub fn binomial(n: usize, mut k: usize) -> usize {
    if k > n / 2 { k = n - k; }
    let mut res: f64 = 1.;

    // [improved sol](https://cp-algorithms.com/combinatorics/binomial-coefficients.html)
    // we can't ensure (n-kk) / (kk+1) is an integer
    for kk in 0..k {
        res = res * (n - kk) as f64;
        res = res / (kk + 1) as f64;
    }
    res.round() as usize
}

fn sum_of_divisor(num: i32) -> i32 {
    let mut sum = 1;
    for idx in 2..(num as f64).sqrt() as i32 + 1{
        if num % idx == 0 {
            sum += idx;
            sum += num / idx;
        }
    }

    sum
}

pub fn are_amicable(x: i32, y: i32) -> bool {
    sum_of_divisor(x) == y && sum_of_divisor(y) == x
}

pub fn is_factorial(mut num: usize) -> bool {
    let mut fac = 2;
    while num >= fac && num % fac == 0 {
        num /= fac;
        fac += 1;
    }

    num == 1
}

pub fn is_prime(num: usize) -> bool {
    if num <= 1 { return false; }
    if num == 2 { return true; }

    for idx in 2..(num as f64).sqrt() as usize + 1{
        if num % idx == 0 {
            return false
        }
    }

    true
}

pub fn double_factorial(mut num: usize) -> usize {
    let mut res = 1;
    while num > 1 {
        res *= num;
        num -= 2;
    }
    res
}

pub fn phi_func(mut num: usize) -> usize {
    let mut result = num;

    for idx in 2..(num as f64).sqrt() as usize + 1 {
        if num % idx == 0 {
            while num % idx == 0 {
                num = num / idx;
            }

            result -= result / idx;
        }
    }

    if num > 1 {
        result -= result / num;
    }

    result
}

pub fn extended_euclid(mut a: usize, mut b: usize, gcd: &mut usize, x: &mut usize, y: &mut usize) {
    // ensure a >= b
    if a < b { std::mem::swap(&mut a, &mut b); }

    if b == 0 {
        *gcd = a;
        *x = 1;
        *y = 0;
    } else {
        extended_euclid(b, a % b, gcd, x, y);
        let temp = *x;
        *x = *y;
        *y = temp - (a / b) * (*y);
    }
}

fn extended_euclid_step(r: &mut usize, r0: &mut usize, quotient: usize) {
    let temp = *r;
    *r = *r0 - quotient * temp;
    *r0 = temp;
}

pub fn extended_euclid_iter(mut a: usize, mut b: usize, gcd: &mut usize, x: &mut usize, y: &mut usize) {
    // ensure a >= b
    if a < b { std::mem::swap(&mut a, &mut b); }

    let mut s = 0; let mut s0 = 1;
    let mut t = 1; let mut t0 = 0;
    let mut r = b; let mut r0 = a;

    while r != 0 {
        let quotient = r0 / r;
        extended_euclid_step(&mut r, &mut r0, quotient);
        extended_euclid_step(&mut s, &mut s0, quotient);
        extended_euclid_step(&mut t, &mut t0, quotient);
    }

    *gcd = r0;
    *x = s0;
    *y = t0;
}

pub fn factorial(mut num: usize) -> usize {
    let mut curr = 1;
    while num > 0 {
        curr *= num;
        num -= 1;
    }
    curr
}

pub fn fast_power_recu(a: usize, b: usize) -> usize {
    if b == 0 { return 1; }
    if b == 1 { return a; }

    let r = fast_power_recu(a, b / 2);
    if b % 2 == 0 {
        r * r
    } else {
        r * r * a
    }
}

pub fn fast_power_linear(mut a: usize, mut b: usize) -> usize {
    if b == 0 { return 1; }
    let mut res = 1;
    while b > 0 {
        if b % 2 > 0 {
            res *= a;
        }

        a = a * a;
        b = b / 2;
    }

    res
}

pub fn fibonacci_recu(n: usize) -> usize {
    if n <= 1 { return n; }
    return fibonacci_recu(n-1) + fibonacci_recu(n-2)
}

pub fn fibonacci_fast(n: usize) -> usize {
    if n <= 1 { return n; }
    if n > 93 { return std::usize::MAX; } // too large for u64

    let mut a = 0; let mut b = 1;
    for _ in 0..n-1 {
        let temp = b;
        b = a + b;
        a = temp;
    }
    b
}

fn fibonacci_multiply(m1: [[usize; 2]; 2], m2: [[usize; 2]; 2]) -> [[usize; 2]; 2] {
    let mut res: [[usize; 2]; 2] = Default::default();
    res[0][0] = m1[0][0] * m2[0][0] + m1[0][1] * m2[1][0];
    res[0][1] = m1[0][0] * m2[0][1] + m1[0][1] * m2[1][1];
    res[1][0] = m1[1][0] * m2[0][0] + m1[1][1] * m2[1][0];
    res[1][1] = m1[1][0] * m2[0][1] + m1[1][1] * m2[1][1];

    res
}

fn fibonacci_power(matrix: [[usize; 2]; 2], n: usize) -> [[usize; 2]; 2]{
    if n <= 1 { return matrix; }
    let r = fibonacci_power(matrix, n / 2);

    if n % 2 == 0 {
        fibonacci_multiply(r, r)
    } else {
        fibonacci_multiply(fibonacci_multiply(r, r), matrix)
    }
}


pub fn fibonacci_matrix(n: usize) -> usize {
    if n == 0 { return 0; }
    let mut matrix = [[1, 1], [1, 0]];
    matrix = fibonacci_power(matrix, n);
    matrix[0][1]
}

fn fibonacci_string_add(a: &str, b: &str) -> String {
    let a: Vec<_> = a.chars().collect();
    let b: Vec<_> = b.chars().collect();

    let al = a.len(); let bl = b.len();
    let mut carry = 0; let mut out = String::new();
    for i in 0..al.max(bl) {
        let ca = if al >= i + 1 {
            a[al-i-1]
        } else {
            '0'
        } as u8;
        let cb = if bl >= i + 1 {
            b[bl-i-1]
        } else {
            '0'
        } as u8 ;

        let mut o = (ca - 48) + (cb - 48) + 48 + carry;
        if o >= 48 + 10 {
            o -= 10;
            carry = 1;
        } else {
            carry = 0;
        }

        out.push(o as char);
    }

    if carry > 0 {
        out.push('1');
    }
    out.chars().rev().collect()
}

pub fn fibonacci_string(n: usize) -> String {
    if n == 0 { return "0".into(); }
    let mut a = "0".to_string();
    let mut b = "1".to_string();

    for _ in 0..n-1 {
        let temp = b.clone();
        b = fibonacci_string_add(&a, &b);
        a = temp;
    }
    b
}

pub fn gcd(num1: usize, num2: usize) -> usize {
    let mut a = num1.max(num2);
    let mut b = num2.min(num1);

    while a % b != 0 {
        let temp = a % b;
        a = temp.max(b);
        b = temp.min(b);
    }
    b

    // if num1 == num2 { return num1; }

    // let mut base_num = 0;
    // let mut previous_remainder = 1;

    // if num1 > num2 {
    //     base_num = num1;
    //     previous_remainder = num2;
    // } else {
    //     base_num = num2;
    //     previous_remainder = num1;
    // }

    // while base_num % previous_remainder != 0 {
    //     let old_base = base_num;
    //     base_num = previous_remainder;
    //     previous_remainder = old_base % previous_remainder;
    // }

    // return previous_remainder;
}

pub fn gcds(nums: &[usize]) -> usize {
    let mut idx: usize = 1;
    let mut gcd = nums[0];
    while idx < nums.len() {
        if nums[idx] % gcd == 0 {
            idx += 1;
        } else {
            gcd = nums[idx] % gcd;
        }
    }
    gcd
}

pub fn integral(mut s: f64, e: f64, func: impl Fn(f64) -> f64, delta: f64) -> f64 {
    let mut res = 0.;
    assert!(s < e);
    let mut n = s + delta;
    while n <= e {
        res = res + delta * (func(s) + func(n)) / 2.0;
        s += delta;
        n += delta;
    }

    res
}

fn find_power_prime(mut k: usize) -> Vec<(usize, usize)> {
    assert!(k > 0);
    let mut res = vec![];
    let mut i = 2;

    while k != 1 {
        if k % i == 0 {
            let mut count = 0;

            while k % i == 0 {
                k = k / i;
                count += 1;
            }

            res.push((i, count));
        }
        i += 1;
    }
    res

}

pub fn find_power(n: usize, k: usize) -> usize {
    let mut res = std::usize::MAX;
    for (p, c) in find_power_prime(k).into_iter() {
        res = res.min(find_power_ofp(n, p) / c)
    }
    res
}

fn find_power_ofp(n: usize, p: usize) -> usize {
    let mut count = 0;
    let mut r = p;

    while r <= n {
        count += n / r;
        r = r * p;
    }
    count
}

pub fn binomial_coef_sum(n: usize) -> usize {
    1 << n
}

pub fn sum_of_digits(mut num: isize) -> isize {
    if num < 0 { num = -num; }
    let mut out = 0;
    while num > 0 {
        out += num % 10;
        num = num / 10;
    }
    out
}

pub fn sqrt_double(a: f64) -> f64 {
    if a > 0. && a < 1. {
        return 1. / sqrt_double(1. / a);
    }

    let mut l = 0.; let mut r = a;
    let epsilon = 1e-12;

    while l <= r {
        let mid = (l + r) / 2.;
        if mid * mid > a {
            r = mid;
        } else {
            if a - mid * mid < epsilon {
                return mid;
            }

            l = mid;
        }
    }

    return -1.;
}

pub fn sieve(n: usize) -> Vec<bool> {
    assert!(n >= 2);
    let mut is_prime = vec![true; n];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..n {
        if is_prime[i] {
            let mut j = i * i;
            while j < n {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    is_prime
}


pub fn power_of_two(n: isize) -> bool {
    return n & (n-1) == 0;
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn sum_of_digits_sample() {
        assert_eq!(sum_of_digits(119765), 29);
        assert_eq!(sum_of_digits(12256), 16);
    }

    #[test]
    fn find_power_sample() {
        assert_eq!(find_power(7, 2), 4);
        assert_eq!(find_power(10, 9), 2);
    }

    #[test]
    fn integral_sample() {
        assert!((integral(3.24, 7.56, |x| x.ln() + x.exp() + x, 0.0001) - 1924.80384023549).abs() <= 0.001)
    }

    #[test]
    fn is_armstrong_sample() {
        assert_eq!(is_armstrong(370), true);
        assert_eq!(is_armstrong(225), false);
        assert_eq!(is_armstrong(-23), false);
        assert_eq!(is_armstrong(153), true);
        assert_eq!(is_armstrong(0), true);
        assert_eq!(is_armstrong(12), false);
    }

    #[test]
    fn binomial_sample() {
        assert_eq!(binomial(1, 1), 1);
        assert_eq!(binomial(57, 57), 1);
        assert_eq!(binomial(6, 3), 20);
        assert_eq!(binomial(10, 5), 252);
        assert_eq!(binomial(20, 10), 184756);
        assert_eq!(binomial(30, 15), 155117520);
        assert_eq!(binomial(40, 20), 137846528820);
        assert_eq!(binomial(50, 25), 126410606437752);
        assert_eq!(binomial(60, 30), 118264581564861424);
        // assert_eq!(binomial(62, 31), 465428353255261088);
    }

    #[test]
    fn are_amicable_sample() {
        assert_eq!(are_amicable(220, 284), true);
        assert_eq!(are_amicable(6368, 6232), true);
        assert_eq!(are_amicable(458, 232), false);
    }

    #[test]
    fn is_factorial_sample() {
        assert_eq!(is_factorial(50), false);
        assert_eq!(is_factorial(720), true);
        assert_eq!(is_factorial(0), false);
        assert_eq!(is_factorial(479001600), true);
    }

    #[test]
    fn is_prime_sample() {
        assert!(!is_prime(50));
        assert!(is_prime(115249));
    }

    #[test]
    fn double_factorial_sample() {
        assert_eq!(double_factorial(5), 15);
        assert_eq!(double_factorial(15), 2027025);
        assert_eq!(double_factorial(0), 1);
    }

    #[test]
    fn phi_func_sample() {
        assert_eq!(phi_func(100), 40);
        assert_eq!(phi_func(1), 1);
        assert_eq!(phi_func(17501), 15120);
        assert_eq!(phi_func(1420), 560);
    }

    #[test]
    fn extended_euclid_sample() {
        let a = 240; let b = 46; let mut gcd = 0; let mut x = 0; let mut y = 0;
        extended_euclid_iter(a, b, &mut gcd, &mut x, &mut y);
        println!("{} {} {}", gcd, x, y);
    }

    #[test]
    fn factorial_sample() {
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(6), 720);
    }

    #[test]
    fn fast_power_sample() {
        let mut rng = rand::thread_rng();
        for _ in 0..20 {
            let a: usize = rng.gen::<usize>() % 20;
            let b: usize = rng.gen::<usize>() % 20;

            assert_eq!(fast_power_linear(a, b), a.pow(b as _));
            assert_eq!(fast_power_recu(a, b), a.pow(b as _));
        }
    }

    #[test]
    fn fibonacci_sample() {
        assert_eq!(fibonacci_matrix(0), 0);
        assert_eq!(fibonacci_matrix(1), 1);
        assert_eq!(fibonacci_matrix(2), 1);
        assert_eq!(fibonacci_matrix(3), 2);
        assert_eq!(fibonacci_matrix(4), 3);
        assert_eq!(fibonacci_matrix(15), 610);
        assert_eq!(fibonacci_matrix(50), 12586269025);

        assert_eq!(fibonacci_recu(0), 0);
        assert_eq!(fibonacci_recu(1), 1);
        assert_eq!(fibonacci_recu(2), 1);
        assert_eq!(fibonacci_recu(3), 2);
        assert_eq!(fibonacci_recu(15), 610);

        assert_eq!(fibonacci_fast(0), 0);
        assert_eq!(fibonacci_fast(1), 1);
        assert_eq!(fibonacci_fast(2), 1);
        assert_eq!(fibonacci_fast(3), 2);
        assert_eq!(fibonacci_fast(15), 610);
        assert_eq!(fibonacci_fast(50), 12586269025);

        assert_eq!(&fibonacci_string(15), "610");
        assert_eq!(&fibonacci_string(50), "12586269025");
    }

    #[test]
    fn gcd_sample() {
        assert_eq!(gcd(312, 221), 13);
        assert_eq!(gcd(289, 204), 17);
        assert_eq!(gcd(5, 1000), 5);

        // assert_eq!(gcds(&vec![5, 1000]), 5);
        // assert_eq!(gcds(&vec![289, 204]), 17);
    }

}
