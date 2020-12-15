(fn solve1 []
  (let [spoken [13 16 0 12 15]
        lastSpoken []
        limit 30000000]
    (var lastNumber 1)
    (for [i 1 (length spoken)]
      (tset lastSpoken (. spoken i) i))
    (for [i (+ 1 (length spoken)) (- limit 1)]
      (let [prevNum lastNumber]
        (if (. lastSpoken prevNum)
          (set lastNumber (- i (. lastSpoken prevNum)))
          (set lastNumber 0))
        (tset lastSpoken prevNum i)
        (table.insert spoken prevNum)))
    (print lastNumber)))

(solve1)