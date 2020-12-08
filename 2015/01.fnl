(fn readInput [] 
  (with-open [file (assert (io.open "inputs/i01.txt"))]
    (file:read :*a)))

(fn iIter [theIter]
  (var c 0)
  (let [newIter (fn []
    (set c (+ c 1))
    (let [v (theIter)]
      (if (not= v nil)
        (values v c)
        nil ) ))]
  newIter))

(fn solve1 [file]
  (var floor 0)
  (each [c (file:gmatch :.)]
    (match c
      "(" (set floor (+ floor 1)) 
      ")" (set floor (- floor 1)) ) )
  floor )

(fn solve2 [file]
  (var (floor basement) (values 0 nil))
  (each [c i (iIter (file:gmatch :.))]
    (match c
      "(" (set floor (+ floor 1)) 
      ")" (set floor (- floor 1)))
    (when (= floor -1)
      (set basement i)
      (lua "break")
    ) 
  )
  basement 
)



(-> (readInput) (solve1) (print))
(-> (readInput) (solve2) (print))