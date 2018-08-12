#lang racket
(require racket/mpair)
(define (ex3.1)
  (define (make-account balance)
    (define (withdraw amount)
      (if (>= balance amount)
          (begin (set! balance (- balance amount))
                 balance)
          "Insufficient funds"))
    (define (deposit amount)
      (set! balance (+ balance amount))
      balance)
    (define (dispatch m)
      (cond [(eq? m 'withdraw) withdraw]
            [(eq? m 'deposit) deposit]
            [else (error "Unknown request: make-account")]))

    dispatch)
  (define acc (make-account 100))
  (displayln ((acc 'withdraw) 10))
  (displayln ((acc 'withdraw) 10))
  (displayln ((acc 'deposit) 10))

  ;; ex3.1
  (define (make-accumulator base)
    (lambda (new) (begin (set! base (+ base new))
                         base))
    )
  (define A (make-accumulator 5))
  (displayln (A 10))
  (displayln (A 10))

  ;; ex3.2
  (define (make-monitored f)
    (define times-called 0)
    (define (dispatch sym . rest)
      (if (eq? sym 'how-many-calls?)
          times-called
          (begin (set! times-called (+ times-called 1))
                 (apply f (cons sym rest)))))
    dispatch)
  (define s (make-monitored sqrt))
  (s 100)
  (displayln (s 'how-many-calls?))

  ;; ex3.3 / ex3.4
  (define (make-account-new balance passwd)
    (define consecutive-tries 0)
    (define (call-the-cops) (displayln "Calling police ..."))
    (define (withdraw amount)
      (if (>= balance amount)
          (begin (set! balance (- balance amount))
                 balance)
          "Insufficient funds"))
    (define (deposit amount)
      (set! balance (+ balance amount))
      balance)
    (define (dispatch pwd m)
      (if (eq? pwd passwd)
          (begin (set! consecutive-tries 0)
            (cond [(eq? m 'withdraw) withdraw]
                  [(eq? m 'deposit) deposit]
                  [else (error "Unknown request: make-account")]))
          (begin (set! consecutive-tries (+ consecutive-tries 1))
                 (if (> consecutive-tries 7)
                     (call-the-cops)
                     (lambda (amount) "Incorrect password"))))
      )

    dispatch)
  (define acc2 (make-account-new 100 'secret-password))
  (displayln ((acc2 'secret-password 'withdraw) 10))
  (displayln ((acc2 'secret-password 'withdraw) 10))
  (displayln ((acc2 'some-other-password 'deposit) 10))
  (displayln ((acc2 'some-other-password 'deposit) 10))
  (displayln ((acc2 'some-other-password 'deposit) 10))
  (displayln ((acc2 'some-other-password 'deposit) 10))
  (displayln ((acc2 'some-other-password 'deposit) 10))
  (displayln ((acc2 'some-other-password 'deposit) 10))
  (displayln ((acc2 'secret-password 'withdraw) 10))
  (displayln ((acc2 'some-other-password 'deposit) 10))
  (displayln ((acc2 'some-other-password 'deposit) 10))
  (displayln ((acc2 'some-other-password 'deposit) 10))

  ;; monte-carlo
  (define (monte-carlo trials experiment)
    (define (iter trails-remainning passed)
      (cond [(= 0 trails-remainning) (/ passed trials)]
            [(experiment) (iter (- trails-remainning 1) (+ passed 1))]
            [else (iter (- trails-remainning 1) passed)]))
    (iter trials 0)
    )
  (define (cesaro-test) (= (gcd (random 10000) (random 1000)) 1))
  (displayln (sqrt (/ 6.0 (monte-carlo 1000 cesaro-test))))

  (define (monte-carlo-random trials experiment initial)
    (define (iter trails-remainning passed seed)
      (if (= 0 trails-remainning)
          (/ passed trials)
          (let ([test (car (experiment seed))]
                [new-seed (cdr (experiment seed))]
                [trails-remainning-re (- trails-remainning 1)])
            (if test
                (iter trails-remainning-re (+ passed 1) new-seed)
                (iter trails-remainning-re passed new-seed)))))
    (iter trials 0 initial))
  (define (cesaro-test-update seed)
    (let* ([x1 (random-update seed)]
           [x2 (random-update x1)])
      (car (= (gcd x1 x2) 1) x2)))
  (define (random-update seed)
    (error "pusuedocode"))

  ;; ex3.5
  (define (estimate-integral predict x1 y1 x2 y2 trials)
    (define (random-range low high)
      (+ low (* (- high low) (random))))
    (define (iter trials-remainning passed)
      (let ([x3 (random-range x1 x2)]
            [y3 (random-range y1 y2)])
        (cond [(= trials-remainning 0) (/ passed trials)]
              [(predict x3 y3) (iter (- trials-remainning 1) (+ passed 1))]
              [else (iter (- trials-remainning 1) passed)])))
    (iter trials 0)
    )
  (define (in-unit-circle x y)
    (< (+ (* x x) (* y y)) 1))
  (displayln (* 4.0 (estimate-integral in-unit-circle -1 -1 1 1 10000)))

  ;; ex3.6
  (define (make-rand seed)
    (define (random-update seed) (error "should availble by base"))
    (define (dispatch sym . res)
      (cond [(and (eq? sym 'generate) (empty? res))
             (let ([out (random-update seed)])
               (set! seed out)
               out)]
            [(and (eq? sym 'reset) (empty? (cdr res)))
             (set! seed (cdr res))])
      )
    dispatch)
  (displayln "test")

  (define (factorial n)
    (let ([product 1] [counter 1])
      (define (iter)
        (if (> counter n)
            product
            (begin (set! product (* counter product))
                   (set! counter (+ counter 1))
                   (iter))))
      (iter)))
  (displayln (factorial 10))

  ;; ex3.7

  (define (make-account-renew balance passwd)
    (define consecutive-tries 0)
    (define (call-the-cops) (displayln "Calling police ..."))
    (define (withdraw amount)
      (if (>= balance amount)
          (begin (set! balance (- balance amount))
                 balance)
          "Insufficient funds"))
    (define (get-balance) balance)
    (define (deposit amount)
      (set! balance (+ balance amount))
      balance)
    (define (dispatch pwd m)
      (if (eq? pwd passwd)
          (begin (set! consecutive-tries 0)
            (cond [(eq? m 'withdraw) withdraw]
                  [(eq? m 'deposit) deposit]
                  [(eq? m 'get-balance) get-balance]
                  [else (error "Unknown request: make-account")]))
          (begin (set! consecutive-tries (+ consecutive-tries 1))
                 (if (> consecutive-tries 7)
                     (call-the-cops)
                     (lambda (amount) "Incorrect password"))))
      )

    dispatch)
  (define (make-joint acc passwd newpasswd)
    (let ([balance ((acc passwd 'get-balance))])
      (make-account-renew balance newpasswd)))
  ;; the only state variable for account is balance.


  ;; ex3.8
  (define variable-for-f 'first-called)
  (define (f v)
    (if (= variable-for-f 'first-call)
        v
        (begin (set! variable-for-f 'called) 0)))

  ;; ex3.9
  ;; recursive call will look the function in outter space


  (displayln "ex3.1")
  )

(ex3.1)


(define (ex3.12)
  ;; ex 3.12
  ;; response1: (list 'b)
  ;; response2: (list 'b 'c 'd)

  ;; ex 3.13
  ;; endless loop forever. causing empty is never inside this list.


  ;; ex3.14
  ;; mystery reverse list inplace
  ;; w is inversed version of origin v
  ;; v is (list (car v)), it is setting at first loop and never change

  ;; ex3.17
  (define (count-pairs x)
    (define (count arg already)
      (cond [(and (pair? arg) (not (member arg already)))
             (+ (count (car arg) (cons arg already))
                (count (cdr arg) (cons arg already))
                1)]
            [else 0]))

    (count x empty)
    )

  ;; ex3.18
  (define (check-cycle li)
    (define (check arg visited)
      (displayln arg)
      (cond [(empty? arg) #f]
            [(member arg visited) #t]
            [(not (pair? arg)) #f]
            [(or (check (car arg) (cons arg visited))
                 (check (cdr arg) (cons arg visited))) #f]
            [else #f]
            ))
    (check li empty)
    )
  (displayln (check-cycle (list 1 2 3)))

  ;; ex3.19 TODO
  )
(ex3.12)

(define (ex3.20)
  (define (cons x y)
    (define (set-x! v) (set! x v))
    (define (set-y! v) (set! y v))
    (define (dispatch m)
      (cond [(eq? m 'car) x]
            [(eq? m 'cdr) y]
            [(eq? m 'set-car!) set-x!]
            [(eq? m 'set-cdr!) set-y!]
            [(eq? m 'displayln) (println (list x y))]
            [else (error "Undefined operation: cons" m)]))
    dispatch)
  (define z (cons 1 2))
  (z 'displayln)
  ((z 'set-car!) 99)
  (z 'displayln)
  )
(ex3.20)

(define (ex3.21)
  (define (front-ptr queue) (mcar queue))
  (define (rear-ptr queue) (mcdr queue))
  (define (set-front-ptr! queue item) (set-mcar! queue item))
  (define (set-rear-ptr! queue item) (set-mcdr! queue item))
  (define (empty-queue? queue) (empty? (front-ptr queue)))
  (define (make-queue) (mcons empty empty))
  (define (front-queue queue)
    (if (empty-queue? queue)
        (error "Front called with an empty queue")
        (front-ptr (front-ptr queue))))
  (define (insert-queue! queue item)
    (let ([new-pair (mcons item empty)])
      (cond [(empty-queue? queue)
             (set-front-ptr! queue new-pair)
             (set-rear-ptr! queue new-pair)
             queue]
            [else
             (set-rear-ptr! (rear-ptr queue) new-pair)
             (set-rear-ptr! queue new-pair)
             queue])))
  (define (delete-queue! queue)
    (cond [(empty-queue? queue) (error "DELETE! called with an empty queue")]
          [else (set-front-ptr! queue (mcdr (front-ptr queue)))
                queue]))
  (define q1 (make-queue))
  (displayln (insert-queue! q1 'a))
  (displayln (insert-queue! q1 'b))
  (displayln (insert-queue! q1 'c))
  (displayln (front-queue q1))
  (displayln (delete-queue! q1))
  (displayln (delete-queue! q1))

  (define (make-queue-state)
    (define queue (mcons empty empty))
    (define (empty-queue?) (empty? (mcar queue)))
    (define (front-queue)
      (if (empty-queue?)
          (error "FRONT-QUEUE on empty queue")
          (mcar (mcar queue))))
    (define (insert-queue item)
      (let ([newpair (mcons item empty)])
        (if (empty-queue?)
            (begin (set-mcar! queue newpair)
                   (set-mcdr! queue newpair)
                   queue)
            (begin (set-mcdr! (mcdr queue) newpair)
                   (set-mcdr! queue newpair)
                   queue))))
    (define (delete-queue)
      (if (empty-queue?)
          (error "DELETE-QUEUE on empty queue")
          (begin (set-mcar! queue (mcdr (mcar queue)))
                 queue)))

    (define (tostring)
      (format "~v3" queue))

    (define (dispatch m)
      (cond [(eq? m 'empty-queue?) empty-queue?]
            [(eq? m 'front-queue) front-queue]
            [(eq? m 'tostring) tostring]
            [(eq? m 'insert-queue) insert-queue]
            [(eq? m 'delete-queue) delete-queue]
            [else (error "not recongnized function for queue")]))
    dispatch)
  (define q2 (make-queue-state))
  (displayln ((q2 'empty-queue?)))
  (displayln ((q2 'insert-queue) 'a))
  (displayln ((q2 'insert-queue) 'b))
  (displayln ((q2 'insert-queue) 'c))
  (displayln ((q2 'delete-queue)))
  (displayln ((q2 'delete-queue)))

  ;; =================== TODO herer =================================
  (define (make-deque)
    ;; cons can get now and post easily, but not remainning previous data
    (define (make-ptr pre val pos)
      (define (dispatch m)
        (define (set-pre! pre-new) (set! pre pre-new))
        (define (set-val! val-new) (set! val val-new))
        (define (set-pos! pos-new) (set! pos pos-new))
        (cond [(eq? m 'val) val]
              [(eq? m 'pre) pre]
              [(eq? m 'pos) pos]
              [(eq? m 'set-pre!) set-pre!]
              [(eq? m 'set-pos!) set-pos!]
              [else (error "not found operation")]))
      dispatch)
    (define nil 'nil)
    (define (nil? ptr) (eq? ptr nil))

    (define deque (mcons nil (mcons nil nil)))

    (define (empty-deque?) (nil? (mcar deque)))
    (define (front-deque)
      (if (empty-deque?)
          (error "empty deque has no front")
          (mcar (mcdr deque))))
    (define (rear-deque)
      (if (empty-deque?)
          (error "empty deque has no front")
          (mcdr (mcdr deque))))
    (define (rear-insert-deque! item)
      (let ([new-ptr (make-ptr nil item nil)])
        (if (empty-deque?)
            (begin (set-mcar! deque new-ptr)
                   (set-mcar! (mcdr deque) new-ptr)
                   (set-mcdr! (mcdr deque) new-ptr)
                   deque)
            (begin (((mcar (mcdr deque)) 'set-pos!) new-ptr)
                   (set-mcar! (mcdr deque) new-ptr)
                   deque))))
    (define (front-insert-deque! item)
      (let ([new-ptr (make-ptr nil item nil)])
        (if (empty-deque?)
            (begin (set-mcar! deque new-ptr)
                   (set-mcar! (mcdr deque) new-ptr)
                   (set-mcdr! (mcdr deque) new-ptr)
                   deque)
            (begin (((mcdr (mcdr deque)) 'set-pre!) new-ptr)
                   (set-mcdr! (mcdr deque) new-ptr)
                   deque))))
    (define (front-delete-deque!)
      (if (empty-deque?)
          (error "can not delete front of empty deque")
          (begin
            (let ([first (mcar (mcdr deque))]
                  [old (first 'pos)]
                  )
              (begin ()))

            (set-mcar! (mcdr deque) )
                 deque)))
    (define (rear-delete-deque!)
      (if (empty-deque?)
          (error "can not delete rear of empty deque")
          (begin (set-mcdr! (mcdr deque) ((mcdr (mcdr deque))'pre))
                 deque)))

    (define (to-list)
      (define (to-list mlist)
        (if (nil? mlist)
            empty
            (append (to-list (mlist 'pre)) (list (mlist 'val)) (to-list (mlist 'pos)))))
      (to-list (mcar deque)))
    (define (dispatch m)
      (cond [(eq? m 'empty-deque?) empty-deque?]
            [(eq? m 'front-deque) front-deque]
            [(eq? m 'rear-deque) rear-deque]
            [(eq? m 'front-insert-deque!) front-insert-deque!]
            [(eq? m 'rear-insert-deque!) rear-insert-deque!]
            [(eq? m 'front-delete-deque!) front-delete-deque!]
            [(eq? m 'rear-delete-deque!) rear-delete-deque!]
            [(eq? m 'to-list) to-list]
            [else (error "not recongnized function for deque")]))

    dispatch)
  (define q3 (make-deque))
  ((q3 'front-insert-deque!) 'a)
  (displayln ((q3 'to-list)))
  ((q3 'front-insert-deque!) 'b)
  (displayln ((q3 'to-list)))
  ((q3 'rear-insert-deque!) 'c)
  (displayln ((q3 'to-list)))
  ((q3 'rear-insert-deque!) 'd)
  (displayln ((q3 'to-list)))
  ((q3 'front-delete-deque!))
  (displayln ((q3 'to-list)))
  ;; (println ((((q3 'front-insert-deque!) 'a) 'to-list)))



  ;; (displayln ((q2 'insert-queue) 'a))
  ;; (displayln ((q2 'insert-queue) 'b))
  ;; (displayln ((q2 'insert-queue) 'c))
  ;; (displayln ((q2 'delete-queue)))
  ;; (displayln ((q2 'delete-queue)))

  (displayln "test")
  )
(ex3.21)
