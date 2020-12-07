(macro icollect [iter-tbl value-expr]
  `(let [tbl# []]
     (each ,iter-tbl
       (tset tbl# (+ (length tbl#) 1) ,value-expr))
     tbl#))

(fn maxBy [ls compare] 
  (var result nil)
  (each [_ v (ipairs ls)]
    (when (or (not result) (compare v result))
    (set result v)
  ))
  result
)

(fn open-parse []
  (icollect [line (io.lines "inputs/i05.txt")]
    (let [binl (string.gsub (line:gsub "[FL]" "0") "[BR]" "1")
          id (tonumber binl 2)]
    id
)))

(fn solution1 [seats]
  (maxBy seats #(> $1 $2)))

(fn solution2 [seats]
  (let [ordering []
        previousSeats []
        minSeat (maxBy seats #(< $1 $2))]
    (each [_ id (ipairs seats)] (tset ordering id id))
    (for [i 1 (- minSeat 1)] (tset ordering i i))
    (each [_ i (ipairs ordering)] (tset previousSeats i i))
    (+ (length previousSeats) 1)
))

(-> (open-parse) (solution1) (print))
(-> (open-parse) (solution2) (print))
