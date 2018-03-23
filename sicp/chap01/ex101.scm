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
