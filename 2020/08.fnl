(macro icollect [iter-tbl value-expr]
  `(let [tbl# []]
     (each ,iter-tbl
       (tset tbl# (+ (length tbl#) 1) ,value-expr))
     tbl#))

(fn readParse []
  (icollect [l (io.lines "inputs/i08")]
    (let [(instr num) (l:match "(%w+) ([-+%d]+)")]
      {: instr :num (tonumber num)})))

(fn exec [instructions nline acc]
  (let [{nline instruction} instructions
        len (length instructions)]
    (if (> nline len)
      (values false acc)
      (let [{: instr : num : visited?} instruction]
        (if visited? 
          (values true acc)
          (do
            (tset (. instructions nline) :visited? true)
            (match instr
              :nop (exec instructions (+ nline 1) acc)
              :acc (exec instructions (+ nline 1) (+ acc num))
              :jmp (exec instructions (+ nline num) acc))))))))

(fn solve1 [instructions]
  (exec instructions 1 0))

(fn solve2 [instructions]
  (let [
      switchInst (fn [n]
        (let [pivot (. instructions n)]
          (match (. pivot :instr)
            :jmp (do (tset pivot :instr :nop) true)
            :nop (do (tset pivot :instr :jmp) true)
            _ false)))
      execChange (fn execChange [n]
        (when (> n 1) (switchInst (- n 1)))
        (each [_ v (ipairs instructions)] (tset v :visited? false))
        (let [(looped? acc) (if (switchInst n)
                              (exec instructions 1 0)
                              (execChange (+ 1 n)))]
          (if looped? 
            (execChange (+ 1 n))
            (values looped? acc))))]
    (execChange 1)))

(-> (readParse) (solve1) (print))
(-> (readParse) (solve2) (print))
