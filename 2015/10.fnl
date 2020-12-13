(fn inserts [as b c] (table.insert as b) (table.insert as c))

(fn flip [as]
  (let [len (length as) res []]
    (for [i len 1 -1]
      (table.insert res (. as i)) )
    res ) )

(fn solve1 []
  (fn process [ins outs cnt prev]
    (if (= (length ins) 0) (do (inserts outs cnt prev) outs)
      (let [new (table.remove ins)]
        (if (not prev) (process ins outs 1 new)
            (not= new prev) (do (inserts outs cnt prev) (process ins outs 1 new))
            (= new prev) (process ins outs (+ cnt 1) prev) ) ) ) )
  (var res [1 3 2 1 1 3 1 1 1 2])
  (for [i 1 50]
    (print i)
    (set res (process (flip res) [])) )
  (print (length res)) )

(solve1)