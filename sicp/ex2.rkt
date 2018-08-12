#lang racket
(require math/number-theory)
;;; Chapter 2 Building abstract with data
(define (ex2.1)
  (define (make-rat n d)
    (let* ([n (if (> d 0) n (- n))]
           [d (if (> d 0) d (- d))]
           [g (gcd n d)])
      (cons (/ n g) (/ d g))))

  (define numer car)
  (define denom cdr)
  (define (add-rat x y)
    (make-rat (+ (* (numer x) (denom y)) (* (denom x) (numer y))) (* (denom x) (denom y))))
  (define (sub-rat x y)
    (make-rat (- (* (numer x) (denom y)) (* (denom x) (numer y))) (* (denom x) (denom y))))
  (define (mul-rat x y)
    (make-rat (* (numer x) (numer y)) (* (denom x) (denom y))))
  (define (div-rat x y)
    (make-rat (* (numer x) (denom y)) (* (denom x) (numer y))))
  (define (equal-rat? x y)
    (equal? (* (numer x) (denom y)) (* (denom x) (numer y))))
  (define (print-rat x)
    (newline)
    (display (numer x))
    (display "/")
    (display (denom x)))
  (define one-half (make-rat 1 2))
  (print-rat one-half)
  (define one-third (make-rat 1 3))
  (print-rat (add-rat one-half one-third))
  (print-rat (mul-rat one-half one-third))
  (print-rat (add-rat one-third one-third))

  ;; ex 2.1
  (print-rat (make-rat 1 2))
  (print-rat (make-rat 1 -2))
  (print-rat (make-rat -1 -2))
  (print-rat (make-rat 0 -2))
  (newline)
  )
(ex2.1)

(define (ex2.2)
  (define (make-point x y) (cons x y))
  (define x-point car)
  (define y-point cdr)
  (define (make-segment p1 p2) (cons p1 p2))
  (define start-segment car)
  (define end-segment cdr)
  (define (print-point p)
    (newline)
    (display "(")
    (display (x-point p))
    (display ",")
    (display (y-point p))
    (display ")")
    )

  (define (midpoint-segment p)
    (make-point (/ (+ (x-point (start-segment p)) (x-point (end-segment p))) 2)
                (/ (+ (y-point (start-segment p)) (y-point (end-segment p))) 2)
                )
    )
  (print-point (make-point 1 2))
  (print-point (midpoint-segment (make-segment (make-point 1 2) (make-point 3 7))))
  (newline)
  )
(ex2.2)

(define (ex2.3)
  (define (make-point x y) (cons x y))
  (define x-point car)
  (define y-point cdr)
  (define (make-segment p1 p2) (cons p1 p2))
  (define start-segment car)
  (define end-segment cdr)
  (define (print-point p)
    (newline)
    (display "(")
    (display (x-point p))
    (display ",")
    (display (y-point p))
    (display ")")
    )

  (define (make-rect p1 p2)
    (cond [(and (< (x-point p1) (x-point p2)) (< (y-point p1) (y-point p2))) (cons p1 p2)]
          [(and (> (x-point p1) (x-point p2)) (> (y-point p1) (y-point p2))) (cons p2 p1)]
          [else (error "rect construct by left bottom point and right top point")]
          ))
  (define point-lb car)
  (define point-rt cdr)
  (define (point-lt rect)
    (make-point (x-point (point-lb rect)) (y-point (point-rt rect))))
  (define (point-rb rect)
    (make-point (x-point (point-rt rect)) (y-point (point-rt rect))))
  (define (width rect)
    (- (x-point (point-rt rect)) (x-point (point-lb rect))))
  (define (height rect)
    (- (y-point (point-rt rect)) (y-point (point-lb rect))))

  (define (peri rect)
    (* 2 (+ (width rect) (height rect)))
    )
  (define (area rect)
    (* (width rect) (height rect))
    )

  (define rect (make-rect (make-point 0 0) (make-point 3 4)))
  (display (area rect))
  (newline)
  (display (peri rect))
  (newline)
  ;; another way: start point with height and width
  )
(ex2.3)

(define (ex2.4)
  (define (cons x y)
    (lambda (m) (m x y)))
  (define (car z) (z (lambda (p q) p)) )
  (define (cdr z) (z (lambda (p q) q)) )

  (newline)
  (display (car (cons 1 2)))
  (display " ")
  (display (cdr (cons 1 2)))
  (newline)
  )
(ex2.4)

(define (ex2.5)
  (define (cons x y)
    ;; x y must be non negative
    (* (expt 2 x) (expt 3 y)))
  (define (finding-factor m n)
    (if (= 0 (remainder m n))
        (+ 1 (finding-factor (/ m n) n))
        0)
    )
  (define (car z)
    (finding-factor z 2) )
  (define (cdr z)
    (finding-factor z 3) )

  (newline)
  (display (car (cons 1 2)))
  (display " ")
  (display (cdr (cons 1 2)))
  (newline)
  )
(ex2.5)

(define (ex2.6)
  (define zero (lambda (f) (lambda (x) x)))
  (define (add-1 n)
    (lambda (f) (lambda (x) (f ((n f) x))))
    )
  ;; (define one (add-1 zero))
  ;; (define two (add-1 one))
  (define one (lambda (f) (lambda (x) (f (f x)))))
  (define two (lambda (f) (lambda (x) (f (f (f (f x)))))))
  (newline)
  (display "use lambda function to define positive integer")
  (newline)
  )
(ex2.6)

(define (ex2.7)
  (define (make-interval x y)
    (cons x y))
  (define lower-bound car)
  (define upper-bound cdr)
  (define (add-interval x y)
    (make-interval (+ (lower-bound x) (lower-bound y))
                   (+ (upper-bound x) (upper-bound y)))
    )
  (define (mul-interval x y)
    (let ([p1 (* (lower-bound x) (lower-bound y))]
          [p2 (* (lower-bound x) (upper-bound y))]
          [p3 (* (upper-bound x) (lower-bound y))]
          [p4 (* (upper-bound x) (upper-bound y))])
      (make-interval (min p1 p2 p3 p4) (max p1 p2 p3 p4)))
    )
  (define (div-interval x y)
    (mul-interval x
                  (make-interval (/ 1.0 (upper-bound y)) (/ 1.0 (lower-bound y)))))
  (define (sub-interval x y)
    (add-interval x
                  (make-interval (- (upper-bound y)) (- (lower-bound y))))
    )
  (newline)
  (display (div-interval (make-interval 1 2) (make-interval 3 4)))
  (newline)
  (display (add-interval (make-interval 1 2) (make-interval 3 4)))
  (newline)
  (display (sub-interval (make-interval 1 2) (make-interval 3 4)))
  (newline)
  )
(ex2.7)

;; ex2.9
;; (a, b) + (c, d) => (a+c, b+d)
;; (b-a)/2 + (d-c)/2 => (b+d-a-c)/2 = (b-a)/2 + (d-c)/2
;; multiplication
;; (a, b) * (c, d) => (min(ac, ad, bc, bd), max(ac, ad, bc, bd))
;; (b-a)/2 * (d-c)/2 not equal to? (bd - ac) / 2 if all a, b, c, d is greater than 0
;; the same applies to division

;; ex2.10
;; (define (div-interval x y)
;;   (if (<= (* (upper-bound y) (lower-bound)) 0)
;;       (mul-interval x
;;                     (make-interval (/ 1.0 (upper-bound y)) (/ 1.0 (lower-bound y))))
;;       )
;;   )
;; (define (mul-interval x y)
;;   (let ([a (lower-bound x)]
;;         [b (upper-bound x)]
;;         [c (lower-bound y)]
;;         [d (upper-bound y)])
;;     (cond [(and (positive? a) (positive? c)) (make-interval (* a c) (* b d))]
;;           [(and (negative? b) (positive? c)) (make-interval (* a d) (* b c))]
;;           [(and (negative? b) (negative? d)) (make-interval (* b d) (* a c))]
;;           [(and (positive? a) (negative? d)) (make-interval (* b c) (* a d))]
;;           [else (make-interval (min (* a c) (* a d) (* b c) (* b d))
;;                                (max (* a c) (* a d) (* b c) (* b d)))]))
;;   )

;; ex2.11
;; (define (make-center-width c w)
;;   (make-interval (- c w) (+ c w)))
;; (define (center i)
;;   (/ (+ (lower-bound i) (upper-bound i)) 2))
;; (define (width i)
;;   (/ (- (upper-bound i) (lower-bound i)) 2))
;; (define (make-center-percentage c t)
;;   (make-center-width c (* c t)))
;; (define (percentage i)
;;   (/ (width i) (center i)))

;; ex2.13
;; spproximate percentage tolerance of product is the sum of percentage tolerance of two interval
;; But it's just approximately not exactly the same

;; ex2.14
(define (ex2.14)
  (define (make-interval x y)
    (cons x y))
  (define lower-bound car)
  (define upper-bound cdr)
  (define (make-center-width c w)
    (make-interval (- c w) (+ c w)))
  (define (center i)
    (/ (+ (lower-bound i) (upper-bound i)) 2))
  (define (width i)
    (/ (- (upper-bound i) (lower-bound i)) 2))
  (define (make-center-percentage c t)
    (make-center-width c (* c t)))
  (define (percentage i)
    (/ (width i) (center i)))
  (define (add-interval x y)
    (make-interval (+ (lower-bound x) (lower-bound y))
                   (+ (upper-bound x) (upper-bound y)))
    )
  (define (mul-interval x y)
    (let ([p1 (* (lower-bound x) (lower-bound y))]
          [p2 (* (lower-bound x) (upper-bound y))]
          [p3 (* (upper-bound x) (lower-bound y))]
          [p4 (* (upper-bound x) (upper-bound y))])
      (make-interval (min p1 p2 p3 p4) (max p1 p2 p3 p4)))
    )
  (define (div-interval x y)
    (mul-interval x
                  (make-interval (/ 1.0 (upper-bound y)) (/ 1.0 (lower-bound y)))))
  (define (sub-interval x y)
    (add-interval x
                  (make-interval (- (upper-bound y)) (- (lower-bound y))))
    )
  (newline)
  (let ([a (make-center-percentage 1 0.0000001)]
        [b (make-center-percentage 1 0.1)]
        [one (make-center-percentage 1 0)]
        ;; [one (make-interval 1 1)]
        )
    (display (div-interval (mul-interval a b) (add-interval a b)))
    (newline)
    (display (div-interval one (add-interval (div-interval one a) (div-interval one b))))
    (newline)
    (display (div-interval a a))
    (newline)
    (display (div-interval b b))
    )
  (newline)
  )
(ex2.14)
;; causing a/a is not real 1

;; ex2.15
;; the problem lays in the div-interval definition
;; 1 / interval (2, 3) ->  (1/3, 1/2) right
;; 1 / interval (-2, 2) -> (1/2, -1/2) wrong
;; 1/interval make interval not valid when interval has 0 inside
;; this is the ral problem. so whatever par1, par2 all should be wrong

;; ex2.16
;; i don't think this is possible
;; causine a/a if a is interval, then a/a should have bound by definition, which is not equal to one
;; while one is derived from multiple equaltion one * any one = any one. That's impossible by definition.
;; Like i said in ex2.15, the real problem of interval is it's not complete itself.

;;; Hierarchical Data and the Closure Property
(define (ex2.17)
  (define (last-pair li)
    (let ([a (car li)]
          [b (cdr li)])
      (if (null? b) a (last-pair b)))
    )
  (newline)
  (display (last-pair (list 1 2 3 4)))
  (newline)
  (display (last-pair (list 1)))
  ;; (newline)
  ;; (display(last-pair empty))
  (newline)
  )
(ex2.17)

(define (ex2.18)
  (define (reverse li)
    (if (null? li)
        li
        (append (reverse (cdr li)) (list (car li))))
    )
  (newline)
  (display (reverse (list 1 2 3 4)))
  (newline)
  )
(ex2.18)

(define (ex2.19)
  (define us-coins (list 50 25 10 5 1))
  (define uk-coins (list 100 50 20 10 5 2 1 0.5))
  (define (cc amount coin-values)
    (cond [(= amount 0) 1]
          [(or (< amount 0) (null? coin-values)) 0]
          [else (+ (cc amount (cdr coin-values))
                   (cc (- amount (car coin-values)) coin-values))]))

  (define (same-parity w . t)
    (if (even? w)
        (cons w (filter even? t))
        (cons w (filter odd? t)))
    )
  (displayln (cc 100 us-coins))
  (displayln (same-parity 1 2 3 4 5 6 7))
  (displayln (same-parity 2 3 4 5 6 7))
  )
(ex2.19)

(define (ex2.27)
  (define (reverse li)
    (cond [(null? li) li]
          [(pair? (car li)) (append (reverse (cdr li)) (list (reverse (car li))))]
          [else (append (reverse (cdr li)) (list (car li)))]
          )
    )

  (define (fringe li)
    (cond [(null? li) li]
          [(pair? (car li)) (append (fringe (car li)) (fringe (cdr li)))]
          [else (append (list (car li)) (fringe (cdr li)))])
    )
  (displayln (reverse (list 1 2 3)))
  (displayln (reverse (list (list 1 2) (list 3 4))))
  (displayln (fringe (list (list 1 2) (list 3 4))))
  )
(ex2.27)

(define (ex2.29)
  (define (make-mobile left right) (list left right))
  (define mobile? list?)
  (define (make-branch length structure) (list length structure))
  (define left-branch car)
  (define right-branch cadr)
  (define branch-length car)
  (define branch-structure cadr)
  (define (total-weight mobile)
    (let* ([l (branch-structure (left-branch mobile))]
           [r (branch-structure (right-branch mobile))]
           [l-t (if (mobile? l) (total-weight l) l)]
           [r-t (if (mobile? r) (total-weight r) r)])
      (+ l-t r-t))
    )
  (define (torque branch)
    (* (branch-length branch)
       (let ([s (branch-structure branch)])
         (if (mobile? s) (total-weight s) s))))
  (define (branch-balanced? branch)
    (= (torque (left-branch branch))
       (torque (right-branch branch))))
  (define (balanced? mobile)
    (let* ([left (left-branch mobile)]
           [right (right-branch mobile)]
           [left-structure (branch-structure left)]
           [right-structure (branch-structure right)])
      (and (branch-balanced? mobile)
           (if (mobile? left-structure) (balanced? left-structure) #t)
           (if (mobile? right-structure) (balanced? right-structure) #t))))

  (define m1 (make-mobile (make-branch 2 3) (make-branch 2 3)))
  (define m2 (make-mobile (make-branch 3 6) (make-branch 2 m1)))
  (displayln (balanced? m1))
  (displayln (balanced? m2))
  )
(ex2.29)

(define (ex3.33)
  (define (map-m f sequence)
    (foldr (lambda (x acc) (cons (f x) acc)) empty sequence))
  (displayln (map-m (lambda (x) (* x x)) (list 1 2 3 4)))
  (define (append-m seq1 seq2)
    (foldr cons seq2 seq1))
  (displayln (append-m (list 1 2 3) (list 4 5 6)))
  (define (length-m sequence)
    (foldr (lambda (x acc) (+ acc 1)) 0 sequence))
  (displayln (length-m (list 1 2 3 4 3)))

  (define (horner-eval x coefficients)
    (foldr (lambda (a acc) (+ (* acc x) a)) 0 coefficients))
  (displayln (horner-eval 2 (list 1 3 0 5 0 1)))

  (define (zip seq1 seq2)
    (cond [(empty? seq1) empty]
          [(empty? seq2) empty]
          [(cons (cons (car seq1) (car seq2)) (zip (cdr seq1) (cdr seq2)))])
    )
  (define (foldr-n op init seqs)
    ;; (if (empty? (car seqs))
    ;;     empty
    ;;     (cons (foldr op init (map car seqs))
    ;;           (foldr-n op init (map cdr seqs)))
    ;;     )
    (if (empty? (cdr seqs))
        (map (lambda (x) (op x init)) (car seqs))
        (map (lambda (pair) (op (car pair) (cdr pair))) (zip (car seqs) (foldr-n op init (cdr seqs)))))
    )
  (displayln (foldr-n + 0 (list (list 1 2 3) (list 4 5 6) (list 7 8 9) (list 10 11 12))))
  (displayln (zip (list 1 2 3) (list 4 5 6)))

  (define (dot-product v w)
    (foldr + 0 (map * v w)))
  (displayln (dot-product (list 1 2 3) (list 1 2 3)))
  (define (matrix-*-vector m v)
    ;; (foldr (lambda (a acc) (cons (dot-product v a) acc)) empty m)
    (map (lambda (v2) (dot-product v v2)) m)
    )
  (define (matrix-*-matrix m n)
    ;; (foldr (lambda (a acc) (cons (matrix-*-vector (transpose n) a) acc)) empty m)
    (map (lambda (v) (matrix-*-vector (transpose n) v)) m)
    )
  (define (transpose m)
    (if (empty? (car m))
        empty
        (cons (map car m)
              (transpose (map cdr m))))
    )
  (let ([v1 (list 1 2)]
        [v2 (list 4 5)]
        [m1 (list (list 1 2) (list 4 5) (list 5 1))]
        [m2 (list (list 1 2 3 4 5) (list 6 7 8 9 10))])
    (displayln (dot-product v1 v2))
    (displayln (matrix-*-vector m1 v2))
    (displayln (matrix-*-matrix m1 m2))
    (displayln (transpose m1))
    )

  (displayln (foldr / 1 (list 1 2 3)))
  (displayln (foldl / 1 (list 1 2 3)))
  (displayln (foldr list empty (list 1 2 3)))
  (displayln (foldl list empty (list 1 2 3)))
  ;; foldr and foldl will produce the same values if operation on a and acc will
  ;; generate the same type of acc likely, the acc should not be more ~complex~
  ;; a, not have order in its structure like list, array
  (define (reverse-l sequence)
    (foldl cons empty sequence))
  (displayln (reverse-l (list 1 2 3)))
  (define (reverse-r sequence)
    (foldr (lambda (a acc) (append acc (list a))) empty sequence))
  (displayln (reverse-r (list 1 2 3)))
  )
(ex3.33)

(define (ex2.40)
  ;; (define (flatten-1 s)
  ;;   (foldr (lambda (a acc) (append acc a)) empty s))
  ;; (define (permutations s)
  ;;   (if (empty? s)
  ;;       (list empty)
  ;;       (flatten-1 (map (lambda (x)
  ;;                         (map (lambda (ac) (cons x ac))
  ;;                              (permutations (filter (lambda (ss) (not (= ss x))) s)))) s))
  ;;       )
  ;;   )
  ;; (permutations (list 1 2 3))
  (define (unique-pairs n)
    (foldr append empty (map (lambda (i) (map (lambda (j) (cons i j)) (range 1 i))) (range 2 (+ n 1)))))
  (displayln (filter (lambda (v) (prime? (+ (car v) (cdr v)))) (unique-pairs 7)))

  (define (concate vs)
    (foldr append empty vs))
  (define (find-ordered-triples-equal-x n x)
    (define (p v)
      (let ([v1 (car v)]
            [v2 (cadr v)]
            [v3 (caddr v)])
        (and (< v1 v2) (< v2 v3) (= x (+ v1 v2 v3)))))

    (define (iter i1 . is)
      (if (empty? is)
          (map (lambda (i) (list i)) i1)
          (let ([old-iter (apply iter is)])
            (concate (map (lambda (i) (map (lambda (ii) (cons i ii)) old-iter)) i1)))
          )
      )
    (newline)
    (filter p (iter (range 1 n) (range 2 n) (range 3 n)))
    )
  (displayln (find-ordered-triples-equal-x 20 15))

  ;; TODO Here ex2.42 ex2.43
  (define (queens board-size)
    (define (queens-on-board k)
      (define empty-board (build-list board-size
                                      (lambda (v) (build-list board-size
                                                              (lambda (v) 0)))))
      (define (safe? board)
        (let* ([new-col (car board)]
               [rest (cdr board)]
               [p (lambda (col) (= 1 (apply max (map + col new-col))))])
          (foldr (lambda (a acc) (and a acc)) #t (map p rest))))
      (define (adjoin-position row queens)
        (let* ([rest-queens (map (lambda (old-col) (cons 0 old-col)) queens)]
               [max-len (if (empty? rest-queens) 1 (length (car rest-queens)))])
          (if (<= max-len row)
              (error "not possible, should not happen")
              (cons (build-list max-len
                                (lambda (row-id) (if (= row-id row) 1 0)))
                    rest-queens))))

      (if (= k 0)
          (list empty-board)
          (filter (lambda (positions) (safe? positions))
                  (concate (map (lambda (rest-of-queens)
                                  (map (lambda (new-row) (adjoin-position new-row rest-of-queens))
                                       (range 0 board-size)))
                                (queens-on-board (- k 1)))))
          )
      )
    (queens-on-board board-size)
    )
  (map displayln (queens 2))
  )
(ex2.40)

(define (ex2.44)
  ;; (define (up-split painter n)
  ;;   (if (= n 0)
  ;;       painter
  ;;       (let ([smaller (up-split painter (- n 1))])
  ;;         (below painter (beside smaller smaller)))))

  ;; (define (split opOnSingle opOnTwo)
  ;;   (define (split-re painter n)
  ;;     (if (= n 0) painter
  ;;         (let ([smaller (split-re painter (- n 1))])
  ;;           (opOnSingle painter (opOnTwo smaller smaller))
  ;;           )))
  ;;   split-re
  ;;   )
  (define (frame-coord-map frame)
    (lambda (v)
      (add-vect (origin-frame frame)
                (add-vect (scale-vect (xcor-vect v) (edge1-frame frame))
                          (scale-vect (ycor-vect v) (edge2-frame frame))))))
  (define (make-frame origin edge1 edge2)
    (cons origin (cons edge1 edge2)))
  (define (origin-frame frame) (car frame))
  (define (edge1-frame frame) (cadr frame))
  (define (edge2-frame frame) (cddr frame))

  (define (make-vect x y)
    (cons x y))
  (define xcor-vect car)
  (define ycor-vect cdr)
  (define (add-vect v1 v2)
    (make-vect (+ (xcor-vect v1) (xcor-vect v2))
               (+ (ycor-vect v1) (ycor-vect v2))))
  (define (sub-vect v1 v2)
    (make-vect (- (xcor-vect v1) (xcor-vect v2))
               (- (ycor-vect v1) (ycor-vect v2))))
  (define (scale-vect f v)
    (make-vect (* f (xcor-vect v)) (* f (ycor-vect v))))

  ;; ex2.47 easy

  ;; ex2.48
  (define make-segment cons)
  (define start-segment car)
  (define end-segment cdr)

  (define (draw-line s e) (error "not implement"))

  (define (segments->painter segment-list)
    (lambda (frame)
      (for-each (lambda (segment)
                  (draw-line ((frame-coord-map frame) (start-segment segment))
                             ((frame-coord-map frame) (end-segment segment))))
                segment-list)))
  (define (draw-outline-frame frame)
    (let* ([o (origin-frame frame)]
           [e1 (edge1-frame frame)]
           [e2 (edge2-frame frame)]
           [vlb o]
           [vlt (add-vect o e2)]
           [vrb (add-vect o e1)]
           [vrt (add-vect vrb e2)])
      (segments->painter (list (make-segment vlb vlt)
                               (make-segment vlt vrt)
                               (make-segment vrt vrb)
                               (make-segment vrb vlb)))
      )
    )
  (define (draw-X-on-frame frame)
    (let* ([o (origin-frame frame)]
           [e1 (edge1-frame frame)]
           [e2 (edge2-frame frame)]
           [vlb o]
           [vlt (add-vect o e2)]
           [vrb (add-vect o e1)]
           [vrt (add-vect vrb e2)])
      (segments->painter (list (make-segment vlb vrt)
                               (make-segment vlt vrb)))
      )
    )
  (define (draw-diamond-on-frame frame)
    (let* ([o (origin-frame frame)]
           [e1 (edge1-frame frame)]
           [e2 (edge2-frame frame)]
           [vlb (add-vect o (scale-vect 0.5 e1))]
           [vlt (add-vect (add-vect o e1) (scale-vect 0.5 e2))]
           [vrb (add-vect o (scale-vect 0.5 e2))]
           [vrt (add-vect (add-vect o e2) (scale-vect 0.5 e1))]
           )
      (segments->painter (list (make-segment vlb vlt)
                               (make-segment vlt vrt)
                               (make-segment vrt vrb)
                               (make-segment vrb vlb)))
      )
    )

  (define (transform-painter painter origin corner1 corner2)
    (lambda (frame)
      (let* ([m (frame-coord-map frame)]
             [new-origin (m origin)]
             [new-corner1 (m corner1)]
             [new-corner2 (m corner2)])
        (painter (make-frame new-origin
                             (sub-vect new-corner1 new-origin)
                             (sub-vect new-corner2 new-origin))))))

  (define (flip-vert painter)
    (transform-painter painter
                       (make-vect 0.0 1.0)
                       (make-vect 1.0 1.0)
                       (make-vect 0.0 0.0)))
  (define (shrink-to-upper-right painter)
    (transform-painter painter
                       (make-vect 0.5 0.5)
                       (make-vect 1.0 0.5)
                       (make-vect 0.5 1.0)))
  (define (rotate90 painter)
    (transform-painter painter
                       (make-vect 9.0 1.0)
                       (make-vect 0.0 0.0)
                       (make-vect 1.0 1.0))
    )
  (define (squash-inwards painter)
    (transform-painter painter
                       (make-vect 0.0 0.0)
                       (make-vect 0.65 0.35)
                       (make-vect 0.35 0.65)))


  (define (beside painter1 painter2)
    (let ([painter-left (transform-painter painter1
                                           (make-vect 0.0 0.0)
                                           (make-vect 0.0 0.5)
                                           (make-vect 0.0 1.0))]
          [painter-right (transform-painter painter2
                                            (make-vect 0.5 0.0)
                                            (make-vect 1.0 0.0)
                                            (make-vect 0.5 1.0))])
      (lambda (frame)
        (painter-left frame)
        (painter-right frame)
        ))
    )

  ;; ex2.50
  (define (flip-horiz painter)
    (transform-painter painter
                       (make-vect 1.0 0.0)
                       (make-vect 0.0 0.0)
                       (make-vect 1.0 1.0)))
  (define (rotate180 painter)
    (transform-painter painter
                       (make-vect 1.0 1.0)
                       (make-vect 0.0 1.0)
                       (make-vect 1.0 0.0)))
  (define (rotate270 painter)
    (transform-painter painter
                       (make-vect 1.0 0.0)
                       (make-vect 1.0 1.0)
                       (make-vect 0.0 0.0)))

  ;; ex2.51
  (define (below painter1 painter2)
    (let ([painter-bottom (transform-painter painter1
                                            (make-vect 0.0 0.0)
                                            (make-vect 1.0 0.0)
                                            (make-vect 0.0 0.5))]
          [painter-top (transform-painter painter2
                                          (make-vect 0.0 0.5)
                                          (make-vect 1.0 0.5)
                                          (make-vect 0.0 1.0))])
      (lambda (frame)
        (painter-top frame)
        (painter-bottom frame))))

  (define (below-complex painter1 painter2)
    (rotate90 (beside (rotate270 painter1) (rotate270 painter2))))

  ;; ex2.52 TODO here
  (displayln "Just to make ex2.44 syntax correctly")
  )
(ex2.44)

;;; Symbolic Data
(define (ex2.53)
  (displayln (list 'a 'b 'c))

  ;; ex2.44
  ;; I don't know what it means ???
  (define (equal?-quote a b)
    (cond [(and (symbol? a) (symbol? b)) (equal? a b)]
          [(and (empty? a) (empty? b)) #t]
          [(not (or (empty? a) (empty? b))) (and (equal? (car a) (car b))
                                                 (equal?-quote (cdr a) (cdr b)))]
          [else #f])
    )
  (displayln (equal?-quote 'a 'b))
  (displayln (equal?-quote 'a 'a))
  (displayln (equal?-quote '(this is a list) '(this is a list)))
  (displayln (equal?-quote '(this is a list) '(this (is a) list)))
  ;; ex2.55
  (displayln (car ''abracadabra))
  (displayln (car '(quote abracadabra)))

  (define (deriv exp var)
    (cond [(number? exp) 0]
          [(variable? exp) (if (same-variable? exp var) 1 0)]
          [(sum? exp) (make-sum (deriv (addend exp) var)
                                (deriv (augend exp) var))]
          [(product? exp) (make-sum
                           (make-product (multiplier exp)
                                         (deriv (multiplicand exp) var))
                           (make-product (deriv (multiplier exp) var)
                                         (multiplicand exp)))]
<<<<<<< HEAD
          [(exponentiation? exp) (make-product
                                  (make-product (exponent exp) (make-exponentiation (base exp) (- (exponent exp) 1)))
                                  (deriv (base exp) var))]
=======
>>>>>>> f44e85aa1afa76465915025f5af4c99b40a9fcea
          [else (error "unknown expression" exp)])
    )
  (define (variable? x) (symbol? x))
  (define (same-variable? v1 v2)
    (and (variable? v1) (variable? v2) (eq? v1 v2)))
  (define (=number? v a)
    (and (number? v) (= v a)))
  (define (make-sum a1 a2)
    (cond [(=number? a1 0) a2]
          [(=number? a2 0) a1]
          [(and (number? a1) (number? a2)) (+ a1 a2)]
          [else (list '+ a1 a2)]))
  (define (make-product m1 m2)
    (cond [(or (=number? m1 0) (=number? m2 0)) 0]
          [(=number? m1 1) m2]
          [(=number? m2 1) m1]
          [(and (number? m1) (number? m2)) (* m1 m2)]
          [else (list '* m1 m2)]))
  (define (sum? exp) (and (pair? exp) (eq? (car exp) '+)))
  (define (addend s) (cadr s))
  (define (augend s) (caddr s))
  (define (product? exp) (and (pair? exp) (eq? (car exp) '*)))
  (define (multiplier p) (cadr p))
  (define (multiplicand p) (caddr p))
  (displayln (deriv '(+ x 3) 'x))
  (displayln (deriv '(* x y) 'x))
  (displayln (deriv '(* (* x y) (+ x 3)) 'x))

<<<<<<< HEAD
  ;; ex2.56
  (define (make-exponentiation base exponent)
    (cond [(=number? exponent 0) 1]
          [(=number? exponent 1) base]
          [(and (number? base) (number? exponent)) (expt base exponent)]
          [else (list 'expt base exponent)])
    )
  (define (exponentiation? exp) (and (pair? exp) (eq? (car exp) 'expt)))
  (define (base e) (cadr e))
  (define (exponent e) (caddr e))
  (displayln (deriv '(expt x 1) 'x))

  ;; ex2.57 TODO HERE
  ;; (define (make-sum-l a1 . a2)
  ;;   (cond [(empty? a2) a1]
  ;;         [(=number? a1 0) a2]
  ;;         [else (list '+ a1 (apply make-sum-l a2))]))

  ;; (define (make-product-l m1 . m2)
  ;;   (cond [(empty? m2) m1]
  ;;         [(=number? m1 0) 0] ))
  )
(ex2.53)

(define (ex2.59)
  (define (element-of-set? x set)
    (cond [(empty? set) #f]
          [(equal? x (car set)) #t]
          [else (element-of-set? x (cdr set))]))
  (define (adjoin-set x set)
    (if (element-of-set? x set)
        set
        (cons x set)))
  (define (intersection-set set1 set2)
    (cond [(or (empty? set1) (empty? set2)) empty]
          [(element-of-set? (car set1) set2) (cons (car set1) (intersection-set (cdr set1) set2))]
          [else (intersection-set (cdr set1) set2)]))
  (define (union-set set1 set2)
    (cond [(empty? set1) set2]
          [(empty? set2) set1]
          [(element-of-set? (car set1) set2) (union-set (cdr set1) set2)]
          [else (cons (car set1) (union-set (cdr set1) set2))]))

  (displayln (element-of-set? 1 (intersection-set (list 1 2 3) (list 1 1 2))))
  )
(ex2.59)

(define (ex2.60)
  (define (element-of-set? x set)
    (cond [(empty? set) #f]
          [(equal? x (car set)) #t]
          [else (element-of-set? x (cdr set))]))
  (define (adjoin-set x set)
    (cons x set))
  (define (intersection-set set1 set2)
    (cond [(or (empty? set1) (empty? set2)) empty]
          [(element-of-set? (car set1) set2) (cons (car set1) (intersection-set (cdr set1) set2))]
          [else (intersection-set (cdr set1) set2)]))
  (define (union-set set1 set2)
    (append set1 set2))
  (displayln (element-of-set? 1 (intersection-set (list 1 2 3) (list 1 1 2))))
  ;; this representation is useful when intersection-set not used frequently, while union used most time
  )
(ex2.60)

(define (ex2.61)
  (define (element-of-set? x set)
    (cond [(empty? set) #f]
          [(equal? x (car set)) #t]
          [(< x (car set) #f)]
          [else (element-of-set? x (cdr set))]))
  (define (intersection-set set1 set2)
    (if (or (empty? set1) (empty? set2))
        empty
        (let ([x1 (car set1)] [x2 (car set2)] [y1 (cdr set1)] [y2 (cdr set2)])
          (cond [(= x1 x2) (cons x1 (intersection-set y1 y2))]
                [(< x1 x2) (intersection-set y1 set2)]
                [else (intersection-set set1 y2)]))))
  (define (adjoin-set x set)
    (cond [(empty? set) (cons x empty)]
          [(= x (car set)) set]
          [(< x (car set)) (cons x set)]
          [else (cons (car set) (adjoin-set x (cdr set)))]))
  (define (union-set set1 set2)
    (cond [(empty? set1) set2]
          [(empty? set2) set1]
          [else (let ([x1 (car set1)] [x2 (car set2)] [y1 (cdr set1)] [y2 (cdr set2)])
                  (cond [(= x1 x2) (cons x1 (union-set y1 y2))]
                        [(< x1 x2) (cons x1 (intersection-set y1 set2))]
                        [else (cons x1 (intersection-set set1 y2))]))]))
  (displayln (element-of-set? 1 (intersection-set (list 1 2 3) (list 1 1 2))))
  )
(ex2.61)

(define (ex2.63)
  (define (entry tree) (car tree))
  (define (left-branch tree) (cadr tree))
  (define (right-branch tree) (caddr tree))
  (define (make-tree entry left right)
    (list entry left right))

  (define (element-of-set? x set)
    (cond [(empty? set) #f]
          [(= x (entry set)) #t]
          [(< x (entry set)) (element-of-set? x (left-branch set))]
          [else (element-of-set? x (right-branch set))]))
  (define (adjoin-set x set)
    (cond [(empty? set) (make-tree x empty empty)]
          [(= x (car set)) set]
          [(< x (car set)) (make-tree (entry set)
                                      (adjoin-set x (left-branch set))
                                      (right-branch set))]
          [else (make-tree (entry set)
                           (left-branch set)
                           (adjoin-set x (right-branch set)))]))

  (define (tree->list-1 tree)
    (if (empty? tree)
        empty
        (append (tree->list-1 (left-branch tree))
                (cons (entry tree)
                      (tree->list-1 (right-branch tree))))))
  (define (tree->list-2 tree)
    (define (copy-to-list tree result-list)
      (if (empty? tree)
          result-list
          (copy-to-list (left-branch tree)
                        (cons (entry tree)
                              (copy-to-list (right-branch tree) result-list)))))
    (copy-to-list tree empty))
  (displayln (tree->list-1 (make-tree 1
                                      (make-tree 3 (make-tree 4 empty empty) (make-tree 5 empty empty))
                                      (make-tree 2 (make-tree 6 empty empty) (make-tree 7 (make-tree 8 empty empty) empty)))))
  (displayln (tree->list-2 (make-tree 1
                                      (make-tree 3 (make-tree 4 empty empty) (make-tree 5 empty empty))
                                      (make-tree 2 (make-tree 6 empty empty) (make-tree 7 (make-tree 8 empty empty) empty)))))
  ;; the result is the same
  (displayln (tree->list-1 (make-tree 7
                                      (make-tree 3 (make-tree 1 empty empty) (make-tree 5 empty empty))
                                      (make-tree 9 empty (make-tree 11 empty empty)))))
  (displayln (tree->list-1 (make-tree 3
                                      (make-tree 1 empty empty)
                                      (make-tree 7
                                                 (make-tree 5 empty empty)
                                                 (make-tree 9
                                                            empty
                                                            (make-tree 11 empty empty))))))
  (displayln (tree->list-1 (make-tree 5
                                      (make-tree 3
                                                 (make-tree 1 empty empty)
                                                 empty)
                                      (make-tree 9
                                                 (make-tree 7 empty empty)
                                                 (make-tree 11 empty empty)))))

  (displayln (tree->list-2 (make-tree 7
                                      (make-tree 3 (make-tree 1 empty empty) (make-tree 5 empty empty))
                                      (make-tree 9 empty (make-tree 11 empty empty)))))
  (displayln (tree->list-2 (make-tree 3
                                      (make-tree 1 empty empty)
                                      (make-tree 7
                                                 (make-tree 5 empty empty)
                                                 (make-tree 9
                                                            empty
                                                            (make-tree 11 empty empty))))))
  (displayln (tree->list-2 (make-tree 5
                                      (make-tree 3
                                                 (make-tree 1 empty empty)
                                                 empty)
                                      (make-tree 9
                                                 (make-tree 7 empty empty)
                                                 (make-tree 11 empty empty)))))
  ;; the speed mainly differ in append vs foldr, the second is more naive to contruct list, which
  ;; means faster

  (define (list->tree elements)
    (car (partial-tree elements (length elements))))
  (define (partial-tree elts n)
    (if (= n 0)
        (cons empty elts)
        (let* ([left-size (quotient (- n 1) 2)]
               [left-result (partial-tree elts left-size)]
               [left-tree (car left-result)]
               [non-left-elts (cdr left-result)]
               [right-size (- n (+ left-size 1))]
               [this-entry (car non-left-elts)]
               [right-result (partial-tree (cdr non-left-elts) right-size)]
               [right-tree (car right-result)]
               [remainning-elts (cdr right-result)])
          (cons (make-tree this-entry left-tree right-tree)
                remainning-elts))))
  (displayln (list->tree (list 1 3 5 7 9 11)))
  ;; O(n)
  (define (union-set set1 set2)
    (define (union-ordered-list l1 l2)
      (cond [(empty? l1) l2]
            [(empty? l2) l1]
            [else (let* ([a1 (car l1)]
                         [d1 (cdr l1)]
                         [a2 (car l2)]
                         [d2 (cdr l2)])
                    (cond [(= a1 a2) (cons a1 (union-ordered-list d1 d2))]
                          [(< a1 a2) (cons a1 (union-ordered-list d1 l2))]
                          [else (cons a2 (union-ordered-list l1 d2))]))]))
    (let* ([l1 (tree->list-2 set1)]
           [l2 (tree->list-2 set2)])
      (list->tree (union-ordered-list l1 l2))))
  (define (intersection-set set1 set2)
    (define (intersection-ordered-list l1 l2)
      (if (or (empty? l1) (empty? l2))
          empty
          (let* ([a1 (car l1)]
                 [d1 (cdr l1)]
                 [a2 (car l2)]
                 [d2 (cdr l2)])
            (cond [(= a1 a2) (cons a1 (intersection-ordered-list d1 d2))]
                  [(< a1 a2) (intersection-ordered-list d1 l2)]
                  [else (intersection-ordered-list l1 d2)]))))
    (let* ([l1 (tree->list-2 set1)]
           [l2 (tree->list-2 set2)])
      (list->tree (intersection-ordered-list l1 l2))))
  (displayln (element-of-set? 1 (intersection-set (list->tree (list 1 2 3))
                                                  (list->tree (list 1 1 2)))))
  (displayln (tree->list-1 (intersection-set (list->tree (list 1 2 3 4 5 6 7))
                                             (list->tree (list 1 3 4 10 12 34)))))
  ;; ex2.66 easy
 )
(ex2.63)

(define (ex2.67)
  (define (make-leaf symbol weight) (list 'leaf symbol weight))
  (define (leaf? object) (eq? (car object) 'leaf))
  (define symbol-leaf cadr)
  (define weight-leaf caddr)
  (define (make-code-tree left right)
    (list left
          right
          (append (symbols left) (symbols right))
          (+ (weight left) (weight right))))
  (define (left-branch tree) (car tree))
  (define (right-branch tree) (cadr tree))
  (define (symbols tree)
    (if (leaf? tree)
        (list (symbol-leaf tree))
        (caddr tree)))
  (define (weight tree)
    (if (leaf? tree)
        (weight-leaf tree)
        (cadddr tree)))
  (define (decode bits tree)
    (define (decode-1 bits current-branch)
      (if (empty? bits)
          empty
          (let ([next-branch (choose-branch (car bits) current-branch)])
            (if (leaf? next-branch)
                (cons (symbol-leaf next-branch)
                      (decode-1 (cdr bits) tree))
                (decode-1 (cdr bits) next-branch)))))
    (decode-1 bits tree))
  (define (choose-branch bit branch)
    (cond [(= bit 1) (right-branch branch)]
          [(= bit 0) (left-branch branch)]
          [else (error "bad bit: choose-branch" bit)]))

  (define (adjoin-set tree set)
    (cond [(empty? set) (list tree)]
          [(< (weight tree) (weight (car set))) (cons tree set)]
          [else (cons (car set) (adjoin-set tree (cdr set)))]))
  (define (make-leaf-set pairs)
    (if (empty? pairs)
        empty
        (let ([pair (car pairs)]
              [rpairs (cdr pairs)])
          (adjoin-set (make-leaf (car pair) (cadr pair))
                      (make-leaf-set rpairs)))))
  (define sample-tree
    (make-code-tree (make-leaf 'A 4)
                    (make-code-tree (make-leaf 'B 2)
                                    (make-code-tree (make-leaf 'D 1)
                                                    (make-leaf 'C 1)))))
  (define sample-message (list 0 1 1 0 0 1 0 1 0 1 1 1 0))
  ;; ex2.67
  (displayln sample-tree)
  (displayln (decode sample-message sample-tree))

  ;; ex2.68
  (define (encode message tree)
    (define (encode-symbol sym tree)
      (define (lookup key dict)
        (cond [(empty? dict) (error "key not found:" key)]
              [(eq? key (car (car dict))) (cdr (car dict))]
              [else (lookup key (cdr dict))]))
      (define (makeDict current id)
        (if (leaf? current)
            (list (cons (symbol-leaf current) id))
            (append (makeDict (left-branch current) (append id (list 0)))
                    (makeDict (right-branch current) (append id (list 1))))))
      (define treeDict (makeDict tree empty))

      (lookup sym treeDict)
      )

    (if (empty? message)
        empty
        (append (encode-symbol (car message) tree)
                (encode (cdr message) tree)))
    )

  (displayln (encode (list 'A 'D 'A 'B 'B 'C 'A) sample-tree))
  (displayln sample-message)

  ;; ex2.69
  (define (generate-huffman-tree pairs)
    (successive-merge (make-leaf-set pairs)))
  (define (successive-merge leaf-set)
    (cond [(empty? leaf-set) empty]
          [(empty? (cdr leaf-set)) (error "pair numbers must be even")]
          [else (let* ([l (car leaf-set)]
                       [r (cadr leaf-set)]
                       [rest (cddr leaf-set)]
                       [out (make-code-tree l r)])
                  (if (empty? rest)
                      out
                      (successive-merge (adjoin-set out rest))))])
    )
  (displayln (generate-huffman-tree (list (list 'A 4) (list 'B 2) (list 'C 1) (list 'D 1))))
  (displayln sample-tree)

  ;; ex2.70
  (define song-tree (generate-huffman-tree (list (list 'na 16) (list 'yip 9) (list 'sha 3)
                                                 (list 'a 2) (list 'get 2) (list 'job 2)
                                                 (list 'wah 1) (list 'boom 1))))
  (define song-code (encode
                     (list 'get 'a 'job
                           'sha 'na 'na 'na 'na 'na 'na 'na 'na
                           'get 'a 'job
                           'sha 'na 'na 'na 'na 'na 'na 'na 'na
                           'wah 'yip 'yip 'yip 'yip 'yip 'yip 'yip 'yip 'yip
                           'sha 'boom)
                     song-tree))
  (define song-message (decode song-code song-tree))
  (displayln song-tree)
  (displayln song-code)
  (displayln song-message)
  ;; song-code is 84, where fixed length representation would need at least 2**4*36=576

  ;; ex2.71
  ;; if node is n, the maximum symbol represendt is 2**(n-1)+1
  ;; so the answer is log(n-1, 2)+1 bits
  ;; the most frequenct symbol alawys need just 1 bits

  ;; ex2.72
  ;; maybe O(n**2) or O(nlog(n))
  )
(ex2.67)

(define (ex2.73)
  ;; ex2.73
  ;; a. data type are '+ '- '* '/ etc. function is deriv
  ;;    the derive function must return a function that works on the same argument.
  ;;    number? or variable? can be expressed in lambda form. But this will causing another
  ;;    problem, the recursion of deriv will never stop. The number? and variable? are the
  ;;    stop condition of deriv
  ;; b.
  ;; (define (install-deriv-package)
  ;;   ;; internal
  ;;   (define (make e1 e2) (cons e1 e2))
  ;;   (define (make e1 e2) (cons e1 e2))
  ;;   (define (deriv args var)
  ;;     (make (deriv (car args) var)
  ;;           (deriv (cadr args) var)))

  ;;   ;; interface
  ;;   (define (tag exp) (attach-tag 'sum exp))
  ;;   (put 'deriv '(sum) deriv)
  ;;   (put 'make-sum 'sum (lambda (e1 e2) (tag (make e1 e2))))
  ;;   'done
  ;;   )
  ;; (define (install-product-package)
  ;;   ;; internal
  ;;   (define (make e1 e2) (cons e1 e2))
  ;;   (define (deriv args var)
  ;;     (make-sum (deriv (car args) var)
  ;;               (deriv (cadr args) var)))
  ;;     ;; ?? how to handle this?? TODO

  ;;   ;; interface
  ;;   (define (tag exp) (attach-tag 'product exp))
  ;;   (put 'deriv '(product) deriv)
  ;;   (put 'make-product 'product (lambda (e1 e2) (tag (make e1 e2))))
  ;;   'done
  ;;   )
  ;; ex2.78 ======================================================================
  ;; (define (attach-tag type-tag contents)
  ;;   (cons type-tag contents))
  ;; (define (type-tag datum)
  ;;   (if (pair? datum)
  ;;       (car datum)
  ;;       (error "Bad tagged datum" datum)))
  ;; (define (contents datum)
  ;;   (if (pair? datum)
  ;;       (cdr datum)
  ;;       (error "Bad tagged datum: contents" datum)))
  ;; (define (attach-tag type-tag contents)
  ;;   (cons type-tag contents))
  ;; (define (type-tag datum)
  ;;   (if (pair? datum)
  ;;       (car datum)
  ;;       'default))
  ;; (define (contents datum)
  ;;   (if (pair? datum)
  ;;       (cdr datum)
  ;;       datum))

  ;; ;; data directed
  ;; (define (apply-generic op . args)
  ;;   (let* ([type-tags (map type-tag args)]
  ;;          [proc (get op type-tags)])
  ;;     (if proc
  ;;         (apply proc (map contents args))
  ;;         (error "no method for types" type-tags))))
  ;; ;; message passing
  ;; ;; (define (apply-generic op arg) (arg op))


  ;; ;; Generic Arithmetic Operations
  ;; (define (add x y) (apply-generic 'add x y))
  ;; (define (sub x y) (apply-generic 'sub x y))
  ;; (define (mul x y) (apply-generic 'mul x y))
  ;; (define (div x y) (apply-generic 'div x y))
  ;; (define (equ? x y) (apply-generic 'equ? x y))
  ;; (define (=zero? x) (apply-generic '=zero? x))

  ;; (define (install-scheme-number-package)
  ;;   (define (tag x) (attach-tag 'scheme-number x))
  ;;   (put 'add '(scheme-number scheme-number) (lambda (x y) (tag (+ x y))))
  ;;   (put 'sub '(scheme-number scheme-number) (lambda (x y) (tag (- x y))))
  ;;   (put 'mul '(scheme-number scheme-number) (lambda (x y) (tag (* x y))))
  ;;   (put 'div '(scheme-number scheme-number) (lambda (x y) (tag (/ x y))))
  ;;   (put 'equ? '(scheme-number scheme-number) (lambda (x y) (eq? x y)))
  ;;   (put '=zero? 'scheme-number (lambda (x) (= x 0)))
  ;;   (put 'make 'scheme-number (lambda (x) (tag x)))
  ;;   'done)
  ;; (define (make-scheme-number n) ((get 'make 'scheme-number) n))

  ;; (define (install-rational-package)
  ;;   ;; internal
  ;;   (define (numer x) (car x))
  ;;   (define (denom x) (cdr x))
  ;;   (define (make-rat n d)
  ;;     (let ([g (gcd n d)])
  ;;       (cons (/ n g) (/ d g))))
  ;;   (define (add-rat x y)
  ;;     (make-rat (+ (* (numer x) (denom y))
  ;;                  (* (numer y) (denom x)))
  ;;               (* (denom x) (denom y))))
  ;;   (define (sub-rat x y)
  ;;     (make-rat (- (* (numer x) (denom y))
  ;;                  (* (numer y) (denom x)))
  ;;               (* (denom x) (denom y))))
  ;;   (define (mul-rat x y)
  ;;     (make-rat (* (numer x) (numer y)) (* (denom x) (denom y))))
  ;;   (define (div-rat x y)
  ;;     (make-rat (* (numer x) (denom y)) (* (denom x) (numer y))))

  ;;   ;; interface
  ;;   (define (tag x) (attach-tag 'rational x))
  ;;   (put 'add '(rational rational) (lambda (x y) (tag (add-rat x y))))
  ;;   (put 'sub '(rational rational) (lambda (x y) (tag (sub-rat x y))))
  ;;   (put 'mul '(rational rational) (lambda (x y) (tag (mul-rat x y))))
  ;;   (put 'div '(rational rational) (lambda (x y) (tag (div-rat x y))))
  ;;   (put 'equ? '(rational rational) (lambda (x y) (and (= (numer x) (numer y))
  ;;                                                      (= (denom x) (denom y)))))
  ;;   (put '=zero? 'rational (lambda x (= 0 (numer x))))
  ;;   (put 'make 'rational (lambda (n d) (tag (make-rat n d))))
  ;;   'done
  ;;   )
  ;; (define (make-rational n d) ((get 'make 'rational) n d))

  ;; (define (install-rectangular-package)
  ;;   ;; internal
  ;;   (define (make-from-real-imag x y) (cons x y))
  ;;   (define (real-part z) (car z))
  ;;   (define (imag-part z) (cdr z))
  ;;   (define (magnitude z)
  ;;     (let ([x (real-part z)]
  ;;           [y (imag-part z)])
  ;;       (sqrt (+ (* x x) (* y y)))))
  ;;   (define (angle z)
  ;;     (let ([x (real-part z)]
  ;;           [y (imag-part z)])
  ;;       (atan y x)))
  ;;   ;; interface
  ;;   (define (tag x) (attach-tag 'rectangular x))
  ;;   (put 'real-part 'rectangular real-part)
  ;;   (put 'imag-part 'rectangular imag-part)
  ;;   (put 'magnitude 'rectangular magnitude)
  ;;   (put 'angle 'rectangular angle)
  ;;   (put 'make-from-real-imag 'rectangular (lambda (x y) (tag (make-from-real-imag x y))))
  ;;   'done
  ;;   )
  ;; (define (install-polar-package)
  ;;   ;; internal
  ;;   (define (make-from-mag-ang r a) (cons r a))
  ;;   (define (magnitude z) (car z))
  ;;   (define (angle z) (cdr z))
  ;;   (define (real-part z) (* (magnitude z) (cos a)))
  ;;   (define (imag-part z) (* (magnitude z) (sin a)))

  ;;   ;; interface
  ;;   (define (tag x) (attach-tag 'polar x))
  ;;   (put 'real-part 'polar real-part)
  ;;   (put 'imag-part 'polar imag-part)
  ;;   (put 'magnitude 'polar magnitude)
  ;;   (put 'angle 'polar angle)
  ;;   (put 'make-from-mag-ang 'polar (lambda (r a) (tag (make-from-mag-ang r a))))
  ;;   'done
  ;;   )
  ;; (define (install-complex-package)
  ;;   ;; importing
  ;;   (define (make-from-real-imag x y) ((get 'make-from-real-imag 'rectangular) x y))
  ;;   (define (make-from-mag-ang r a) ((get 'make-from-mag-ang 'polar) r a))
  ;;   ;; internal
  ;;   (define (add-complex z1 z2)
  ;;     (make-from-real-imag (+ (real-part z1) (real-part z2))
  ;;                          (+ (imag-part z1) (imag-part z2))))
  ;;   (define (sub-complex z1 z2)
  ;;     (make-from-real-imag (- (real-part z1) (real-part z2))
  ;;                          (- (imag-part z1) (imag-part z2))))
  ;;   (define (mul-complex z1 z2)
  ;;     (make-from-mag-ang (* (magnitude z1) (magnitude z2))
  ;;                        (+ (angle z1) (angle z2))))
  ;;   (define (div-complex z1 z2)
  ;;     (make-from-mag-ang (/ (magnitude z1) (magnitude z2))
  ;;                        (- (angle z1) (angle z2))))
  ;;   (define (real-part z) (apply-generic 'real-part z))
  ;;   (define (imag-part z) (apply-generic 'imag-part z))
  ;;   (define (magnitude z) (apply-generic 'magnitude z))
  ;;   (define (angle z) (apply-generic 'angle z))
  ;;   (define (equal-complex z1 z2) (and (= (real-part z1) (real-part z2))
  ;;                                      (= (imag-part z1) (imag-part z2))))
  ;;   ;; interface
  ;;   (define (tag x) (attach-tag 'complex x))
  ;;   (put 'add '(complex complex) (lambda (z1 z2) (tag (add-complex z1 z2))))
  ;;   (put 'sub '(complex complex) (lambda (z1 z2) (tag (sub-complex z1 z2))))
  ;;   (put 'mul '(complex complex) (lambda (z1 z2) (tag (mul-complex z1 z2))))
  ;;   (put 'div '(complex complex) (lambda (z1 z2) (tag (div-complex z1 z2))))
  ;;   (put 'equ? '(complex complex) (lambda (z1 z2) (equal-complex z1 z2)))
  ;;   (put '=zero? 'complex (lambda (z) (= 0 (magnitude z))))

  ;;   (put 'make-from-real-imag 'complex (lambda (x y) (tag (make-from-real-imag x y))))
  ;;   (put 'make-from-mag-ang 'complex (lambda (x y) (tag (make-from-mag-ang x y))))
  ;;   ;; ex2.77
  ;;   (put 'real-part 'complex real-part)
  ;;   (put 'imag-part 'complex imag-part)
  ;;   (put 'magnitude 'complex magnitude)
  ;;   (put 'angle 'complex angle)
  ;;   ;; apply-generic was invoke once
  ;;   'done)
  ;; (define make-from-real-imag (get 'make-from-real-imag 'complex))
  ;; (define make-from-mag-ang (get 'make-from-mag-ang 'complex))

  ;; ex2.77
  (displayln "all just pseudocode -.-!!")
  )
(ex2.73)

(define (ex2.81)
  (define (apply-generic op . args)
    (let* ([type-tags (map type-tag args)]
           [proc (get op type-tags)])
      (if proc
          (apply proc (map contents args))
          (if (= (length args) 2)
              (let* ([type1 (car type-tags)]
                     [type2 (cadr type-tags)]
                     [v1 (car args)]
                     [v2 (cadr args)]
                     [t1->t2 (get-coercion type1 type2)]
                     [t2->t2 (get-coercion type2 type1)])
                (cond [(t1->t2) (apply-generic op (list (t1->t2 v1) v2))]
                      [(t2->t1) (apply-generic op (list v1 (t2->t1 v2)))]
                      [else (error "not find method for types" type-tags)]))
              (error "not find method for types" type-tags)))))
  ;; ex2.81
  ;; a. see into above definition: proc is None, then goto recusion. apply-generic
  ;; will get the same arguments, which causing a endless loop forever.
  ;; b. no, something like that should not be done. It's the end case to end if clause.
  ;; c.
    (let* ([type-tags (map type-tag args)]
           [proc (get op type-tags)])
      (if proc
          (apply proc (map contents args))
          (if (= (length args) 2)
              (let* ([type1 (car type-tags)]
                     [type2 (cadr type-tags)]
                     [v1 (car args)]
                     [v2 (cadr args)]
                     [t1->t2 (get-coercion type1 type2)]
                     [t2->t2 (get-coercion type2 type1)])
                (cond [(eq? type1 type2) (error "not implement for insde type" type1)]
                      [(t1->t2) (apply-generic op (list (t1->t2 v1) v2))]
                      [(t2->t1) (apply-generic op (list v1 (t2->t1 v2)))]
                      [else (error "not find method for types" type-tags)]))
              (error "not find method for types" type-tags))))
  ;; TODO Here @page 273
  )


(ex2.81)
=======
  )
(ex2.53)
>>>>>>> f44e85aa1afa76465915025f5af4c99b40a9fcea
