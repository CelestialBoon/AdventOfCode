(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))

(fn firstMultipleAbove [p target n]
  (let [n (or n 1) mul (* p n)]
    (if (<= target mul) mul (firstMultipleAbove p target (+ n 1)))))

(fn solve1 []
  (let [target 1001171
        busIds [17 41 37 367 19 23 29 613 13]
        arrivals (icollect [_ v (ipairs busIds)] {:p v :t (firstMultipleAbove v target)})] 
    (var min {:t math.huge})
    (each [_ {: p : t} (pairs arrivals)]
      (when (< t min.t) (set min {: p : t})))
    (print (* min.p (- min.t target)))))

(fn prodExcept [list n] (var res 1) (each [_ v (ipairs list)] (when (not= v n) (set res (* v res)))) res)

(fn inverseMod [n p]
  (var res 1)
  (for [i 1 (- p 2)]
    (set res (% (* res n) p)))
  res)

; chinese reminder theorem!!!
(fn solve2 []
  (let [busMods {17  0 
                 41  -7
                 37  -11
                 367 -17
                 19  2 
                 23  6 
                 29  12 
                 613 -48
                 13  4  }
        busIds (icollect [p _ (pairs busMods)] p)
        totalProd (prodExcept busIds) ]
    (var sol 0)
    (each [p n (pairs busMods)]
      (local otherProd (prodExcept busIds p))
      (set sol (+ sol (* n otherProd (inverseMod otherProd p)))))
    (print (% sol totalProd))))

(solve1)
(solve2)
