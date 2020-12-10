(fn cons [a bs] (table.insert bs a) bs)
(fn prod [list] (var res 1) (each [_ v (ipairs list)] (set res (* v res))) res)

(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))

(fn readParse [] (icollect [line (io.lines "inputs/i10")] (tonumber line)))

(fn solve1 [adapters]
  (table.sort adapters)
  (let [len (length adapters)
        walk (fn walk [i last dif1s dif3s]
          (if (> i len) (values dif1s dif3s)
            (let [new (. adapters i)]
              (match (- new last)
                3 (walk (+ 1 i) new dif1s (cons last dif3s))
                1 (walk (+ 1 i) new (cons last dif1s) dif3s)
                _ (walk (+ 1 i) new dif1s dif3s)))))
        (dif1s dif3s) (walk 1 0 [] [])]
    (values (* (length dif1s) (+ 1 (length dif3s))) adapters dif3s)))

(fn solve2 [_ adapters dif3s]
  (let [tots []
        max (. adapters (length adapters))
        adapSet {}
        walk (fn walk [min max]
          (var pTot 0)
          (fn w [j]
            (when (< j max)
              (when (< 0 pTot) (set pTot (- pTot 1)))
              (when (. adapSet (+ j 1)) (set pTot (+ pTot 1)) (w (+ 1 j)))
              (when (. adapSet (+ j 2)) (set pTot (+ pTot 1)) (w (+ 2 j)))
              (when (. adapSet (+ j 3)) (set pTot (+ pTot 1)) (w (+ 3 j)))))
          (w min)
          pTot)]
    (each [_ v (ipairs adapters)] (tset adapSet v v))
    (table.insert dif3s max)
    (var last nil)
    (each [i max (ipairs dif3s)]
      (local min (or last 0))
      (when (< min max)
        (table.insert tots (walk min max)))
      (set last (+ max 3)))
    (prod tots)))

( -> (readParse) (solve1) (print))
( -> (readParse) (solve1) (solve2) (print))