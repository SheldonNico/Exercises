#lang racket
;; Importang: this is not runable, don't even try
;; ex1.1
10
(+ 5 3 4)
(- 9 1)
(/ 6 2)
(+ (* 2 4) (- 4 6))
(define a 3)
(define b (+ a 1))
(+ a b (* a b))
(= a b)
(if (and (> b a) (< b (* a b)))
    b
    a)
(cond ((= a 4) 6)
      ((= b 4) (+ 6 7 a))
      (else 25))
(+ 2 (if (> b a) b a))
(* (cond ((> a b) a)
         ((< a b) b)
         (else -1))
   (+ a 1))

;; ex1.2
(/ (+ 5 4 (- 2 (- 3 (+ 6 (/ 4 5)))))
   (* 3 (- 6 2) (- 2 7)))

;; ex1.3
(define (square x) (* x x))
(define (squaresum-of-two-largest a b c)
  (cond ((and (<= a b) (<= a c)) (+ (square b) (square c)))
        ((and (<= b a) (<= b c)) (+ (square a) (square c)))
        ((else (+ (square a) (square b)))))
  )

;; ex1.4
(define (a-plus-abs-b a b)
  ((if (> b 0) + -) a b))
;; if b is greater than 0, producer is + which plus a, b
;; otherwise a - b; essentially, it works as its name

;; ex1.5
(define (p) (p))
(define (test x y)
  (if (= x 0) 0 y))
(test 0 (p))
;; actuall p is an endless recursion, scheme will evaluate p forever
;; if in a normal-order, the last expression will work just fine, cause
;; (p) will never need to be calculated. but in an applicative order
;; system it will stuck forever untill you kill it in system.

(define (sqrt-iter guess x)
  (if (good-enough? guess x)
      guess
      (sqrt-iter (improve guess x) x)))
(define (improve guess x)
  (average guess (/ x guess)))
(define (average x y) (/ (+ x y) 2))
(define (good-enough? guess x)
  (< (abs (- (square guess) x)) 0.001))
(define (square x) (* x x))
(define (sqrt x) (sqrt-iter 1.0 x))

(sqrt 9)
(sqrt (+ 100 37))
(sqrt (+ (sqrt 2) (sqrt 3)))
(square (sqrt 1000))

;; ex1.6
;; it won't work. sqrt-iter will loop forever in calling itself. cause new-if
;; will evaluate all its arguments, while if will calculate the last only if predicate
;; gives false. So there is a lazyness problem. Actually, I don't think there is
;; any way to define `if` by yourself in lisp, this bug exist in the evaluate
;; strategy.

;; ex1.7
;; example
(define (test-sqrt x)
  (abs (- x (square (sqrt x)))))
(test-sqrt 1000)
(test-sqrt 0.1)
(test-sqrt 0.01)
(test-sqrt 0.001)

(define (better-enough? new old)
  (< (abs (- new old)) (/ old 1000)))
(define (sqrt-iter-better guess old x)
  (if (better-enough? guess old)
      guess
      (sqrt-iter-better (improve guess x) guess x)))
(define (improve guess x)
  (average guess (/ x guess)))
(define (sqrt-better x)
  (sqrt-iter-better (improve 1.0 x) 1.0 x))

(define (test-sqrt x)
  (abs (- x (square (sqrt-better x)))))
(test-sqrt 1000)
(test-sqrt 0.1)
(test-sqrt 0.01)
(test-sqrt 0.001)

;; ex1.8
(define (cube-iter new old x)
  (if (cube-test? new old x)
      new
      (cube-iter (cube-improve new x) new x)))
(define (cube-test? new old x)
  (< (abs (- new old)) (/ old 1000)))
(define (cube-improve old x)
  (/ (+ (/ x (* old old)) (* 2 old)) 3))
(define (cube x) (cube-iter (cube-improve 1.0 x) 1.0 x))

(cube 8)
(cube 1000)
(cube 13)


(define (cube x)
  (define (cube-iter new old)
    (if (cube-test? new old)
        new
        (cube-iter (cube-improve new) new)))
  (define (cube-test? new old)
    (< (abs (- new old)) (/ old 1000)))
  (define (cube-improve old)
    (/ (+ (/ x (* old old)) (* 2 old)) 3))

  (cube-iter (cube-improve 1.0) 1.0))
;; Linear Recursion and Iteration
(define (factorial n)
  (if (= n 1) 1 (* n (factorial (- n 1)))))
(factorial 100)

(define (factorial n)
  (define (factorial-iter n prod iter)
    (if (> iter n) prod (factorial-iter n (* prod iter) (+ 1 iter))))
  (factorial-iter n 1 1))
(factorial 3)


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

;;; Formulating Abstractions
;; ex1.29
(define (sum-seri term next a b)
  (if (> a b)
      0
      (+ (term a) (sum-seri term next (next a) b))))
(define (integral-simpson f a b n)
  (define h (/ (- b a) n))
  (define (inc n) (+ n 1))
  (define (term k)
    (define xk (+ a (* k h)))
    (define yk (f xk))
    (if (even? k) (* 2 yk) (* 4 yk))
    )
  (* (/ h 3)
     (+ (sum-seri term inc 1 (- n 1)) (f a) (f b)))
  )

;; ex1.30
(define (sum-iter term next a b)
  (define (iter a result)
    (if (> a b)
        result
        (iter (next a) (+ result (term a)))))
  (iter a 0))
(define (sum-rec term next a b)
  (if (> a b)
      0
      (+ (term a) (sum-rec term next (next a) b)))
  )
(define (cube-sum a b) (sum-iter cube inc a b))
(cube-sum 7 10)
(define (cube-sum a b) (sum-rec cube inc a b))
(cube-sum 7 10)

;; ex1.31
(define (product-rec term next a b)
  (if (> a b)
      1
      (* (term a) (product-rec term next (next a) b))))

(define (product-iter term next a b)
  (define (iter a result)
    (if (> a b)
        result
        (iter (next a) (* result (term a)))))
  (iter a 1))

(define (identity a) a)
(define (inc a) (+ a 1))
(define (fac b) (product-iter identity inc 1 b))
(fac 3)
(define (fac b) (product-rec identity inc 1 b))
(fac 3)

(define (approxi-pi n)
  (define (term k)
    (define upper (+ k 1))
    (define lower k)
    (if (odd? k) (/ upper lower) (/ lower upper))
    )

  (product-rec term inc 2 n)
  )
(* 4.0 (approxi-pi 1000))

;; ex1.32
(define (accumulate-rec combiner null-value term a next b)
  (if (> a b)
      null-value
      (combiner (term a) (accumulate-rec combiner null-value term (next a) next b))))

(define (accumulate-iter combiner null-value term a next b)
  (define (iter a result)
    (if (> a b)
        result
        (iter (next a) (combiner result (term a)))))
  (iter a null-value))

(let* ([identity (lambda (n) n)]
       [inc (lambda (n) (+ n 1))]
       [sum (lambda (a b) (accumulate-rec + 0 identity a inc b))])
  (sum 2 10))
(let* ([identity (lambda (n) n)]
       [inc (lambda (n) (+ n 1))]
       [sum (lambda (a b) (accumulate-iter + 0 identity a inc b))])
  (sum 2 10))

;; ex1.33
(define (accumulate-filter-rec combiner null-value term a next b predict)
  (cond [(> a b) null-value]
        [(predict a) (combiner (term a) (accumulate-filter-rec combiner null-value term (next a) next b predict))]
        [else (accumulate-filter-rec combiner null-value term (next a) next b predict)]
        )
  )
(require (only-in math/number-theory prime?))
(let* ([identity (lambda (n) n)]
       [inc (lambda (n) (+ n 1))]
       [square (lambda (n) (* n n))]
       )
  (accumulate-filter-rec + 0 square 0 inc 4 prime?))

(let* ([identity (lambda (n) n)]
       [inc (lambda (n) (+ n 1))]
       [square (lambda (n) (* n n))]
       [num 12]
       [rela-prime? (lambda (n) (= 1 (gcd num n)))]
       )
  (accumulate-filter-rec * 1 identity 1 inc num rela-prime?))

;; Constructing Procedures usign lambda
;; ex1.34
(define (f g) (g 2))
(f (lambda (x) (* x x)))
(f f) ;; => (f f) => (f 2) => (2 2)
;; the import thing here is inside f, it's already know f itself.
;; this is how recursion works

(define (rec n)
  (if (< n 0)
      0
      (+ 1 (rec (- n 1)))))

;; Procedures as General Methods
(define tolerance 0.00001)
(define (half-interval-meghod f a b)
  (define (search f neg-point pos-point)
    (define (small-enough? val) (< (abs val) tolerance))
    (let* ([midpoint (/ (+ neg-point pos-point) 2)]
           [val (f midpoint)])
      (cond [(small-enough? val) midpoint]
            [(< (f midpoint) 0) (search f midpoint pos-point)]
            [(> (f midpoint) 0) (search f neg-point midpoint)])))
  (let ([aval (f a)] [bval (f b)])
    (cond [(and (positive? aval) (negative? bval)) (search f b a)]
          [(and (positive? bval) (negative? aval)) (search f a b)]
          [(= aval 0) a]
          [(= bval 0) b]
          [else (error "values of a b have the same sign")])
    )
  )
(half-interval-meghod sin 0.7 -0.1)
(half-interval-meghod sin 2.0 4.0)
(half-interval-meghod sin 6.0 7.0)
(half-interval-meghod (lambda (x) (- (* x x x) (* 2 x) 3)) 1.0 2.0)

(define (fixed-point f first-guess)
  (let ([next-guess (f first-guess)])
    (cond [(< (abs (- next-guess first-guess)) tolerance) next-guess]
          [else (fixed-point f next-guess)]))
  )
(fixed-point (lambda (n) (+ (- n) 1)) 0.0)
(fixed-point cos 1.0)
(fixed-point (lambda (x) (+ (sin x) (cos x))) 1.0)

(define (sqrt x)
  (fixed-point (lambda (y) (/ x y)) 0.0))
(define (sqrt x)
  (fixed-point (lambda (y) (+ (/ (- x 1) (+ y 1)) 1)) 0.0))
(sqrt 5)
(sqrt 9)

;; ex1.35
;; phi^2 = phi + 1 => phi = 1 + 1 / phi obviously
(fixed-point (lambda (phi) (+ 1 (/ 1 phi))) 0.0)

;; ex1.36
(define tolerance 0.00001)
(define (fixed-point f first-guess)
  (display first-guess)
  (newline)
  (let ([next-guess (f first-guess)])
    (cond [(< (abs (- next-guess first-guess)) tolerance) next-guess]
          [else (fixed-point f next-guess)]))
  )
(fixed-point cos 1.0)
(fixed-point (lambda (x) (/ (log 1000) (log x))) 2.0)
(fixed-point (lambda (x) (/ (+ x (/ (log 1000) (log x))) 2)) 2.0)
;; average dumping make converage much fast

;; ex1.37
(define (cont-frac n d k)
  (define k-1 (- k 1))
  (define (frac-sum-re val)
    (cond [(= val 0) (/ (n k) (d k))]
          [(<= val k-1) (/ (n (- k val)) (+ (d (- k val)) (frac-sum-re (- val 1))))]
          [else (error "not supported input")]))
  (frac-sum-re k-1))

(define 1/phi 0.6180339887)
(cont-frac (lambda (i) 1.0) (lambda (i) 1.0) 11) ;; need k at least 11

(define (cont-frac n d k)
  (define k-1 (- k 1))
  (define (frac-sum-iter now result)
    (cond [(= now k) result]
          [else (frac-sum-iter (+ now 1) (/ (n (- k now)) (+ (d (- k now)) result)))]))
  (frac-sum-iter 0 0))
(cont-frac (lambda (i) 1.0) (lambda (i) 1.0) 11) ;; need k at least 11

;; ex1.38
(define (cont-frac n d k)
  (define k-1 (- k 1))
  (define (frac-sum-re val)
    (cond [(= val 0) (/ (n k) (d k))]
          [(<= val k-1) (/ (n (- k val)) (+ (d (- k val)) (frac-sum-re (- val 1))))]
          [else (error "not supported input")]))
  (frac-sum-re k-1))
(define e-2 (- (exp 1) 2))
(cont-frac (lambda (i) 1.0)
           (lambda (i)
             (let-values ([(mo re) (quotient/remainder i 3)])
               (if (= re 2) (* 2 (+ mo))
                   1)))
           30)

;; ex1.39
;; not the same cont-frac as ex1.37
(define (cont-frac n d k)
  (define k-1 (- k 1))
  (define (frac-sum-re val)
    (cond [(= val 0) (/ (n k) (d k))]
          [(<= val k-1) (/ (n (- k val)) (- (d (- k val)) (frac-sum-re (- val 1))))]
          [else (error "not supported input")]))
  (frac-sum-re k-1))

(define (tan-cf x k)
  (/ (cont-frac (lambda (i) (expt x 2))
                (lambda (i) (- (* 2 i) 1))
                k)
     x)
  )
(tan-cf 12.0 30)
(tan 12)

;; Procedures as returned values
(define (average-damp f)
  (lambda (x) (/ (+ x (f x)) 2)))
((average-damp (lambda (x) (* x x))) 10)

(define tolerance 0.00001)
(define (fixed-point f first-guess)
  (let ([next-guess (f first-guess)])
    (cond [(< (abs (- next-guess first-guess)) tolerance) next-guess]
          [else (fixed-point f next-guess)])))
(define (sqrt x)
  (fixed-point (average-damp (lambda (y) (/ x y))) 1.02))
(sqrt 4)

(define (cube-root x)
  (fixed-point (average-damp (lambda (y) (/ x (* y y)))) 1.02))
(cube-root 27)

(define (deriv g)
  (define dx 0.00001)
  (lambda (x) (/ (- (g (+ x dx)) (g x)) dx)))

(define (cube x) (* x x x))
((deriv cube) 5)

(define (newton-transform f)
  (lambda (x) (- x (/ (f x) ((deriv f) x)))))
(define (newtons-method f guess)
  (fixed-point (newton-transform f) guess))
(newtons-method cube 3.9)
(newtons-method sin 7)
(define (sqrt x) (newtons-method (lambda (y) (- (* y y) x)) 1.0))
(sqrt 16)

(define (fixed-point-of-transform g transform guess)
  (fixed-point (transform g) guess))
(define (sqrt x)
  (fixed-point-of-transform (lambda (y) (/ x y))
                            average-damp
                            1.0))

(sqrt 9)
(define (sqrt x)
  (fixed-point-of-transform (lambda (y) (- (* y y) x)) newton-transform 1.0))
(sqrt 9)

;; ex1.40
(define (cubic a b c)
  (lambda (x) (+ (* x x x) (* a x x) (* b x) c)))
(newtons-method (cubic 1 2 3) 1)

;; ex1.41
(define (inc n) (+ n 1))
(define (double f)
  (lambda (n) (f (f n))))
(((double double) inc) 5)
(((double (double double)) inc) 5)
(((double (double (double double))) inc) 5)

;; ex1.42
(define (compose f g)
  (lambda (x) (f (g x))))
(define (inc n) (+ n 1))
(define (square n) (* n n))
((compose square inc) 6)

;; ex1.43
(define (repeated f n)
  (if (= n 1)
      f
      (compose f (repeated f (- n 1))))
  )
((repeated square 2) 5)
((repeated inc 10) 5)

;; ex1.44
(define (smooth f)
  (define dx 0.0001)
  (lambda (x) (/ (+ (f (- x dx)) (f x) (f (+ x dx))) 3)))
(define (smooth-n f n)
  ((repeated smooth n) f))
((smooth sin) 0.1)
((smooth-n sin 10) 0.1)

;; ex1.45
(define (fixed-point f first-guess)
  (define tolerance 0.00001)
  (display first-guess)
  (display " ")
  (let ([next-guess (f first-guess)])
    (cond [(< (abs (- next-guess first-guess)) tolerance) next-guess]
          [else (fixed-point f next-guess)])))

(define (fixed-point-of-transform g transform guess)
  (fixed-point (transform g) guess))
(define (average-damp f)
  (lambda (x) (/ (+ x (f x)) 2)))
(require racket/sandbox)

(define (find-root n guess k x)
  (call-with-deep-time-limit
   0.01
   (lambda () (fixed-point-of-transform (lambda (y) (/ x (expt y (- n 1))))
                                        (repeated average-damp k)
                                        guess))

   ))

(find-root 2 1.0 1 4)
(find-root 3 1.0 1 8)
(find-root 4 1.0 2 16)
(find-root 5 1.0 2 32)
(find-root 6 1.0 2 64)
(find-root 7 1.0 2 128)
(find-root 8 1.0 3 256)
(find-root 9 1.0 3 512)
(find-root 10 1.0 3 1024)
(find-root 11 1.0 3 2048)
(find-root 12 1.0 3 4096)
(find-root 13 1.0 3 8192)
(find-root 14 1.0 3 16384)
(find-root 15 1.0 3 32768)
(find-root 16 1.0 4 65536)
;; obvisouly just k = ceil(log(2, n))
(define (find-root n guess x)
  (if (<= n 1)
      x
      (fixed-point-of-transform (lambda (y) (/ x (expt y (- n 1))))
                                (repeated average-damp (floor (log n 2)))
                                guess)
      )
  )
(find-root 1102 1.0 40)


;; ex1.46
(define (iterative-imporve good-enough? next-guess)
  (define (iter guess)
    (if (good-enough? guess)
        guess
        (iter (next-guess guess)))
    )
  iter
  )

(define (sqrt x guess)
  ((iterative-imporve (lambda (n) (< (abs (- (* n n) x)) 0.0001))
                      (lambda (y) (/ (+ y (/ x y)) 2))) guess)
  )
(sqrt 4 1.0)
(sqrt 81 1.0)

(define (fixed-point f guess)
  ((iterative-imporve (lambda (v) (< (abs (- (f v) v)) 0.0001))
                      f)
   guess)
  )
(fixed-point sin 1)
(fixed-point sin 2)
