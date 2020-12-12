(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))

(fn readInput []
  (icollect [l (io.lines "inputs/i12")]
    (let [(w d) (l:match "(%w)(%d+)")]
      {: w :n (tonumber d)})))

(fn solve1 [input]
  (var (orien x y) (values 0 0 0))
  (each [_ {: w : n} (ipairs input)]
    (let [mov (match w
          :R (do (set orien (% (+ orien (/ n 90)) 4)) nil)
          :L (do (set orien (% (- orien (/ n 90)) 4)) nil)
          :F (match orien 0 :E 1 :S 2 :W 3 :N)
          c c)]
      (match mov
        :N (set y (+ y n))
        :S (set y (- y n))
        :E (set x (+ x n))
        :W (set x (- x n))
        _ nil)))
  (+ (math.abs x) (math.abs y)))

(fn rotate [deg point]
  (fn rotate1 [am point]
    (let [{: x : y} point]
      (match am
        0 point
        a (rotate1 (- a 1) {:x y :y (- 0 x)}))))
  (rotate1 (% (/ deg 90) 4) point))

(fn solve2 [input]
  (var (wp pos) (values {:x 10 :y 1} {:x 0 :y 0}))
  (each [_ {: w : n} (ipairs input)]
    (match w
      :N (set wp.y (+ wp.y n))
      :S (set wp.y (- wp.y n))
      :E (set wp.x (+ wp.x n))
      :W (set wp.x (- wp.x n))
      :L (set wp (rotate (- 0 n) wp))
      :R (set wp (rotate n wp))
      :F (set pos {:x (+ pos.x (* n wp.x)) :y (+ pos.y (* n wp.y))})))
  (+ (math.abs pos.x) (math.abs pos.y)))

(-> (readInput) (solve1) (print))
(-> (readInput) (solve2) (print))