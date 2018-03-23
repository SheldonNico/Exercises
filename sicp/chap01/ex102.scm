;; Linear Recursion and Iteration
(define (factorial n)
  (if (= n 1) 1 (* n (factorial (- n 1)))))
(factorial 100)

(define (factorial n)
  (define (factorial-iter n prod iter)
    (if (> iter n) prod (factorial-iter n (* prod iter) (+ 1 iter))))
  (factorial-iter n 1 1))
(factorial 1)


;; ex1.9
(define (+ a b)
  (if (= a 0) b (inc (+ (dec a) b))))
;; (+ 4 5)
;; (inc (+ 3 5))
;; (inc (inc (+ 2 5)))
;; (inc (inc (inc (+ 1 5))))
;; (inc (inc (inc (inc (+ 0 5)))))
;; (inc (inc (inc (inc 5)))) => 9
;; recursive process
(define (+ a b)
  (if (= a 0) b (+ (dec a) (inc b))))
;; (+ 4 5)
;; (+ 3 6)
;; (+ 2 7)
;; (+ 1 8)
;; (+ 0 9)
;; 9
;; iterative process

;; ex1.10
(define (A x y)
  (cond ((= y 0) 0)
        ((= x 0) (* 2 y))
        ((= y 1) 2)
        (else (A (- x 1) (A x (- y 1))))))
(A 1 10)
(A 2 4)
(A 3 3)

(define (f n) (A 0 n)) ;; 2*n
(define (g n) (A 1 n)) ;; 2^n
(define (h n) (A 2 n)) ;; xn = 2^(xn-1)



;; Tree recursion
(define (fib n)
  (cond ((= n 0) 0)
        ((= n 1) 1)
        (else (+ (fib (- n 1))
                 (fib (- n 2))))))
(fib 3)

(define (fib n)
  (define (fib-iter start end count)
    (if (= count 0) start (fib-iter end (+ start end) (- count 1))))
  (fib-iter 0 1 n))
(fib 50)

(define (count-change amount)
  (define (first-denomination kinds-of-coins)
    (cond ((= kinds-of-coins 1) 1)
          ((= kinds-of-coins 2) 5)
          ((= kinds-of-coins 3) 10)
          ((= kinds-of-coins 4) 25)
          ((= kinds-of-coins 5) 50)))
  (define (cc amount kinds-of-coins)
    (cond ((= amount 0) 1)
          ((or (< amount 0) (= kinds-of-coins 0)) 0)
          (else (+ (cc amount (- kinds-of-coins 1))
                   (cc (- amount (first-denomination kinds-of-coins))
                       kinds-of-coins)))))
  (cc amount 5))
(count-change 100)

;; ex1.11
(define (f n)
  (cond ((< n 3) n)
        (else (+ (f (- n 1))
                 (* 2 (f (- n 2)))
                 (* 3 (f (- n 3)))))))
(map f '(1 2 3 4 5 6 7 8 9))


(define (f n)
  (define (f-iter a b c count)
    (cond ((= count 3) c)
          ((< count 3) count)
          (else (f-iter b c (+ c (* 2 b) (* 3 a)) (- count 1)))))
  (f-iter 1 2 3 n))
(map f '(1 2 3 4 5 6 7 8 9))

;; ex1.12
(define (tri m n)
  (cond ((or (= 1 n) (= n m) (= m 1)) 1)
        (else (+ (tri (- m 1) n)
                 (tri (- m 1) (- n 1))))))
(tri 3 2)

;; ex1.13
;; just use mathematical induction method


;; Orders of Growth
;; ex1.14
;; (count-change 100) = (cc 100 5)
;; (cc 100 4) + (cc 50 5)
;; (cc 100 3) + (cc 50 4) + (cc 50 4) + (cc 0 5)
;; (cc 100 2) + (cc 50 3) + (cc 50 3) + (cc 25 4) + (cc 50 3) + (cc 25 4) + 1
;; ...
;; increase as 2^n
;; f(n, 5) = f(n-50, 5) + f(n, 4)
;; f(n, 4) = f(n-25, 4) + f(n, 3)
;; f(n, 3) = f(n-10, 3) + f(n, 2)
;; f(n, 2) = f(n-5, 2)  + f(n, 1)
;; f(n, 1) = f(n-1, 1) + f(n, 0) = f(n-1, 1) + 0
;; f(n, 1) = O(1)
;; f(n, 2) = O(1) * n/5 = O(n)
;; f(n, 3) = O(n) * n/10
;; f(n, 4) = O(n^2) * n/25
;; f(n, 5) = O(n^3) * n/50
;; order of count-change = O(n^4)
;; steps: O(n^4)
;; space: O(n)

;; ex1.15
(define (sine angle)
  (define (cube x) (* x x x))
  (define (improve x)
    (- (* 3 x)
       (* 4 (cube x))))

  (if (< (abs angle) 0.1)
      angle
      (improve (sine (/ angle 3.0)))))

(define pi (* 4 (atan 1)))
(sine (/ pi 60))
;; the same number when 12.5 / 3^n < 0.1
;; steps: O(log(10a)/log(3))
;; space: the same


;; Expt
(define (expt b n)
  (if (= n 0) 1 (* b (expt b (- n 1)))))

(define (expt-fast b n)
  (cond ((= n 0) 1)
        ((even? n) (square (expt-fast b (/ n 2))))
        (else (* b (expt-fast b (- n 1))))))
(expt-fast 3 4)
;; ex1.16
(define (expt-fast-iter b n)
  (define (iter a b n)
    (cond ((= n 1) (* a b))
          ((even? n) (iter a (square b) (/ n 2)))
          (else (iter (* a b) b (- n 1)))))
  (iter 1 b n))
(expt-fast-iter 3 4)

;; ex1.17
(define (mult-fast a b)
  (define (double a) (* 2 a))
  (define (halve b) (/ b 2))
  (cond ((= b 1) a)
        ((even? b) (mult-fast (double a) (halve b)))
        (else (+ a (mult-fast a (- b 1))))))
(mult-fast 5 8)

;; ex1.18
(define (mult-fast-iter a b)
  (define (double a) (* 2 a))
  (define (halve b) (/ b 2))
  (define (iter st a b)
    (cond ((= b 1) (+ st a))
          ((even? b) (iter st (double a) (halve b)))
          (else (iter (+ st a) a (- b 1)))))
  (iter 0 a b))
(mult-fast-iter 4 33)

;; ex1.19
;; T(p, q)^2 = T(q^2+p^2, q^2+p*q*2)
(define (fib-fast n)
  (define (halve n) (/ n 2))
  (define (square n) (* n n))
  (define (nth n list)
    (if (< n 1) (car list) (nth (- n 1) (cdr list))))
  (define (T p q pair)
    (let ((a (nth 0 pair))
          (b (nth 1 pair)))
      (list (+ (* b q) (* a q) (* a p)) (+ (* b p) (* a q)))))
  (define (iter p q pair n)
    (cond ((= n 1) (T p q pair))
          ((even? n) (let* ((sq (square q))
                            (newp (+ (square p) sq))
                            (newq (+ sq (* p q 2))))
                       (iter newp newq pair (halve n))))
          (else (T p q (iter p q pair (- n 1))))))
  (nth 1 (iter 0 1 (list 1 0) (+ n 1))))
(map fib-fast (list 0 1 2 3 4 5 6 7 8 9 10))

(define (fib n)
  (define (fib-iter a b p q count)
    (cond ((= count 0) b)
          ((even? count)
           (fib-iter a b (+ (square q) (square p)) (+ (square q) (* p q 2)) (/ count 2)))
          (else (fib-iter (+ (* b q) (* a q) (* a p))
                          (+ (* b p) (* q a))
                          p
                          q
                          (- count 1)))))
  (fib-iter 1 0 0 1 n))
(map fib (list 0 1 2 3 4 5 6 7 8))

(define (fib n)
  (define (fib-iter a b p q count)
    (cond ((= count 0) b)
          ((even? count) (fib-iter a b (+ (* p p) (* q q)) (+ (* q q) (* p q 2)) (/ count 2)))
          (else (fib-iter (+ (* b q) (* a q) (* a p))
                          (+ (* b p) (* a q))
                          p
                          q
                          (- count 1)))))
  (fib-iter 1 0 0 1 n))
(map fib (list 0 1 2 3 4 5 6 7 8))

;; Greaest common divisors
;; gcd(a, b) = gcd(b, mod(a, b)), a >= b
(define (gcd a b)
  (if (= b 0)
      a
      (gcd b (remainder a b))))
(gcd 3 7)
(gcd 27 18)
;; ex1.20
;; applicative order
;; (gcd 206 40)
;; call remainder -> (gcd 40 6)
;; call remainder -> (gcd 6 4)
;; call remainder -> (gcd 4 2)
;; call remainder -> (gcd 2 0)
;; 2 <<= 4 times
;; normal order
;; (gcd 206 40)
;; (gcd 40 (remainder 206 40))
;; (if (= (remainder 206 40) 0) 40 (gcd 40 (remainder 40 (remainder 206 40)))
;; too much. but clearly it's much more than applicative order

;; Finde prime
(define (smallest-divisor n)
  (define (find-divisor m n)
    (cond ((> (square n) m) m)
          ((= (remainder m n) 0) n)
          (else (find-divisor m (+ n 1)))))
  (find-divisor n 2))
(define (prime? n) (= (smallest-divisor n) n))
(map prime? '(1 2 3 4 5 6 7 8 9 10))


(define (expmod base exp m)
  (cond ((= exp 0) 1)
        ((even? exp) (remainder (square (expmod base (/ exp 2) m)) m))
        (else (remainder (* base (expmod base (- exp 1) m)) m))))


;; a^n % n == a % n = a (cuz a < n)
(define (fermat-test n)
  (define (try-it a)
    (= (expmod a n n) a))
  (try-it (+ 1 (random (- n 1)))))

(define (fast-prime? n times)
  (cond ((= times 0) true)
        ((fermat-test n) (fast-prime? n (- times 1)))
        (else false)))

;; ex1.21
(define (smallest-divisor n)
  (define (find-divisor m n)
    (cond ((> (square n) m) m)
          ((= (remainder m n) 0) n)
          (else (find-divisor m (+ n 1)))))
  (find-divisor n 2))

(smallest-divisor 199)
(smallest-divisor 1999)
(smallest-divisor 19999)


;; ex1.22
(define (smallest-divisor n)
  (define (find-divisor m n)
    (cond ((> (square n) m) m)
          ((= (remainder m n) 0) n)
          (else (find-divisor m (+ n 1)))))
  (find-divisor n 2))
(define (prime? n) (= (smallest-divisor n) n))

(define (timed-prime-test n)
  (newline)
  (display n)
  (start-prime-test n (runtime)))
(define (start-prime-test n start-time)
  (if (prime? n)
      (report-prime (- (runtime) start-time))))
(define (report-prime elapsed-time)
  (display " *** ")
  (display elapsed-time))

(define (search-for-primes start)
  (define (test num count)
    (cond ((= count 3) (newline) (display "finished"))
          ((prime? num) (newline) (display num) (test (+ num 1) (+ count 1)))
          (else (test (+ num 1) count))))
  (test start 0))
(search-for-primes 1000) ;; 1009 1013 1019
(search-for-primes 10000) ;; 10007 10009 10037
(search-for-primes 1000000) ;; 1000003 1000033 1000037
(map timed-prime-test (list 1009 1013 1019 10007 10009 10037 1000003 1000033 1000037))
(search-for-primes 100000000) ;; 1000003 1000033 1000037
(search-for-primes 10000000000) ;; 1000003 1000033 1000037
(search-for-primes 1000000000000) ;; 1000003 1000033 1000037
(map timed-prime-test (list 100000007 100000037 100000039 10000000019 10000000033 10000000061 1000000000039 1000000000061 1000000000063))
;; yes increase by 10

;; ex1.23
(define (next n)
  (if (= n 2) 3 (+ n 2)))
(define (smallest-divisor n)
  (define (find-divisor m n)
    (cond ((> (square n) m) m)
          ((= (remainder m n) 0) n)
          (else (find-divisor m (next n)))))
  (find-divisor n 2))
(map timed-prime-test (list 100000007 100000037 100000039 10000000019 10000000033 10000000061 1000000000039 1000000000061 1000000000063))
;; no not double, more likely sqrt(2)
;; Since O(sqrt(n)) -> O(sqrt(n/2))

;; ex1.24
(define (expmod base exp m)
  (cond ((= exp 0) 1)
        ((even? exp) (remainder (square (expmod base (/ exp 2) m)) m))
        (else (remainder (* base (expmod base (- exp 1) m)) m))))
(define (fermat-test n)
  (define (try-it a)
    (= (expmod a n n) a))
  (try-it (+ 1 (random (- n 1)))))

(define (fast-prime? n times)
  (cond ((= times 0) true)
        ((fermat-test n) (fast-prime? n (- times 1)))
        (else false)))

(define (timed-prime-test n)
  (newline)
  (display n)
  (start-prime-test n (runtime)))
(define (start-prime-test n start-time)
  (if (fast-prime? n 100000)
      (report-prime (- (runtime) start-time))))
(define (report-prime elapsed-time)
  (display " *** ")
  (display elapsed-time))

(map timed-prime-test (list 100000007 100000037 100000039 10000000019 10000000033 10000000061 1000000000039 1000000000061 1000000000063))
;; yes the are increased by the same length as expected
;; there are some discrepancy: when n is small, the O(log(n)) will affected by smaller numbers
;; and fast-prime? is depended on which times it choose, if times is small, all didn't passed the
;; final test, it will still return #t
;; when you make times as small as 10, you will notice it immediately.

;; ex1.25
;; there is no optimization here.
;; the original one was a result of fermat's little theorm. it reduce a lot of work
;; in the calculation of exponent.

;; ex1.26
(define (expmod base exp m)
  (cond ((= exp 0) 1)
        ((even? exp) (remainder (square (expmod base (/ exp 2) m)) m))
        (else (remainder (* base (expmod base (- exp 1) m)) m))))

(define (expmod base exp m)
  (cond ((= exp 0) 1)
        ((even? exp) (* (square (expmod base (/ exp 2) m) (expmod base (/ exp 2) m)) m))
        (else (remainder (* base (expmod base (- exp 1) m)) m))))
;; evaluation strategy determine the first one is better.
;; the L.R's code will calculate the same expmod experession in each recursion, which will increase rapidly.
;; f(n) = 2*f(n/2) + O(1) -> f = O(n)
;; f(n) = f(n/2) + O(1) -> f = O(log(n))

;; ex1.27
