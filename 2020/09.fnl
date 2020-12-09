(fn pop [as] (table.remove as 1))
(fn push [as b] (table.insert as b))
(fn any [as f] (var res? false) (each [i v (ipairs as)] (when (f v i) (set res? true))) res?)

(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))

(fn readInput [] (io.lines "inputs/i09"))

(fn gatherSums [preamble]
  (let [result []]
    (each [i v (ipairs preamble)]
      (each [j w (ipairs preamble)]
        (when (not= i j)
          (push result (+ v w)))))
    result))

(fn solve1 [iter]
  (let [preamble [] 
        gatherSums (fn gatherSums [preamble]
          (let [result []]
            (each [i v (ipairs preamble)]
              (each [j w (ipairs preamble)]
                (when (not= i j)
                  (push result (+ v w)))))
            result))
        checkline (fn checkLine [iter]
          (let [newNumber (tonumber (iter))
                sums (gatherSums preamble)]
            (if (= 25 (length preamble))
              (if (any sums #(= $1 newNumber))
                (do
                  (pop preamble)
                  (push preamble newNumber)
                  (checkLine iter))
                newNumber)
              (do
                (push preamble newNumber)
                (checkLine iter)))))]
  (checkLine iter)))

(fn solve2 [invalidNum]
  (var result nil)
  (let [list (icollect [line (readInput)] (tonumber line))
        compileSum (fn compileSum [n sumList sum target]
          (let [num (. list n)
                newSum (+ sum num)]
            (push sumList num)
            (if (= newSum target) (values true sumList)
                (> newSum target) (values false nil)
                                  (compileSum (+ n 1) sumList newSum target))))]
    (each [i v (ipairs list)]
      (local (sum? sumList) (compileSum i [] 0 invalidNum))
      (when sum?
        (table.sort sumList)
        (set result (+ (. sumList 1) (. sumList (length sumList)))) 
        (lua :break))))
  result)

(-> (readInput) (solve1) (print))
(-> (readInput) (solve1) (solve2) (print))
