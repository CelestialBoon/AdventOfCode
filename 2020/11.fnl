(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))

(fn any [as f] (var res? false) (each [i v (ipairs as)] (when (f v i) (set res? true))) res?)
(fn fold [as acc f] (var res acc) (each [i v (ipairs as)] (set res (f res v i))) res)
(fn repeat [a num] (let [res []] (for [i 1 num] (tset res i a)) res))

(fn printSeats [seats]
  (each [i ss (ipairs seats)]
    (print (table.concat (icollect [_ s (ipairs ss)] s.s)))))

(fn readParseInput []
  (let [seats (icollect [line (io.lines "inputs/i11")]
          (icollect [s (line:gmatch :.)] {: s}))
        lenx (length (. seats 1))
        leny (length seats)]
    (for [i 1 leny]
      (table.insert (. seats i) {:s :.})
      (table.insert (. seats i) 1 {:s :.}))
    (table.insert seats (repeat {:s :.} (+ 2 lenx)))
    (table.insert seats 1 (repeat {:s :.} (+ 2 lenx)))
    seats))

(fn adjacents [seats i j]
  [(. (. seats (+ i 1)) (+ j 1))
   (. (. seats (+ i 1)) j)
   (. (. seats (+ i 1)) (- j 1))
   (. (. seats i) (+ j 1))
   (. (. seats i) (- j 1))
   (. (. seats (- i 1)) (+ j 1))
   (. (. seats (- i 1)) j)
   (. (. seats (- i 1)) (- j 1))])

(fn firstSeens [seats i j]
  (let [lenx (length (. seats 1))
        leny (length seats)
        walk (fn walk [i j directions]
          (let [[x y] directions]
            (if (and (match x
                        -1 (< 1 j)
                        0 true
                        1 (< j lenx))
                     (match y
                        -1 (< 1 i)
                        0 true
                        1 (< i leny)))
              (let [wseat (. (. seats (+ i y)) (+ j x))]
                (match wseat.s
                  :# wseat
                  :L wseat
                  :. (walk (+ i y) (+ j x) directions)))
              (. (. seats i) j))))]
    (icollect [_ directions (ipairs [[-1 -1] [-1 0] [-1 +1] [0 -1] [0 +1] [+1 -1] [+1 0] [+1 +1]])]
      (walk i j directions))))

(fn checkSeats [seats checker switchNum]
  (each [i ss (ipairs seats)]
    (each [j seat (ipairs ss)]
      (match seat
        {:s :#} (when (-> (icollect [_ as (ipairs (checker seats i j))] (string.match as.s :#))
                          (length)
                          (>= switchNum))
            (set seat.changing? true))
        {:s :L} (when (-> (icollect [_ as (ipairs (checker seats i j))] (string.match as.s :#))
                          (length)
                          (= 0))
            (set seat.changing? true))
        {:s :.} nil
        _ (error "seat isn't properly set"))))
  seats)

(fn changeSeats [seats]
  (each [i ss (ipairs seats)]
    (each [j seat (ipairs ss)]
      (when seat.changing?
        (set seat.s (match seat.s
          :# :L
          :L :#
          :. (error "changing floor"))))
      (set seat.changing? false))))

(fn solve [seats policy switchNum]
  (var anyChanging? true)
  (while anyChanging?
    (checkSeats seats policy switchNum)
    (set anyChanging? (any seats (fn [ss] (any ss #$1.changing?))))
    (changeSeats seats)
    ; (printSeats seats)
  )
  (fold seats 0 (fn [acc ss] (+ acc (length (icollect [_ s (ipairs ss)] (string.match s.s :#)))))))

(-> (readParseInput) (solve adjacents 4) (print))
(-> (readParseInput) (solve firstSeens 5) (print))
