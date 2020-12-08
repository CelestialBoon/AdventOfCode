(fn parseInputs []
  (let [dims []]
    (each [s (io.lines "inputs/i02.txt")]
      (let [(a b c) (s:match "(%d+)x(%d+)x(%d+)")]
        (table.insert dims [(tonumber a) (tonumber b) (tonumber c)])
      )
    )
    dims
  )
)

(fn solve1 [dims]
  (var total 0)
  (each [_ ds (ipairs dims)]
    (table.sort ds)
    (let [[a b c] ds]
      (set total (+ total (+ (* 3 a b) (* 2 b c) (* 2 a c))))
    )
  )
  total
)

(fn solve2 [dims]
  (var total 0)
  (each [_ ds (ipairs dims)]
    (table.sort ds)
    (let [[a b c] ds]
      (set total (+ total (+ a a b b (* a b c))))
    )
  )
  total
)

(-> (parseInputs) (solve1) (print))
(-> (parseInputs) (solve2) (print))