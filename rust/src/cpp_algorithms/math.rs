
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


#[cfg(test)]
mod tests {
    use super::*;

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
}
