(fn strtochars [str] 
  (let [result []]
    (for [i 1 (length str)]
      (table.insert result (string.sub str i i))
    )
    result
  )
)

(fn ilines [file]
  (let [liter (io.lines file)]
    (var c 0)
    (fn iter []
      (set c (+ c 1))
      (local line (liter))
      (if (not= line nil)
        (values c line)
        nil))))

(macro walk [p c inc]
  `(do
    (when (= tree (. chars ,p))
      (set ,c (+ ,c 1))
    )
    (set ,p (+ 1 (% (+ ,p (- ,inc 1)) len)))
  )
)

(fn solution []
  (local (len tree) (values 31 "#"))
  (var (p1 p2 p3 p4 p5) (values 1 1 1 1 1))
  (var (c1 c2 c3 c4 c5) (values 0 0 0 0 0))
  (each [i line (ilines "inputs/i03.txt")]
    (local chars (strtochars line))
    (walk p1 c1 1)
    (walk p2 c2 3)
    (walk p3 c3 5)
    (walk p4 c4 7)
    (when (= 1 (% i 2))
      (walk p5 c5 1)
    )
  )
  (print c1 c2 c3 c4 c5 (* c1 c2 c3 c4 c5))
)

(solution)