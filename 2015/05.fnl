(fn countIter [iter] (var n 0) (each [_ iter] (set n (+ n 1))) n)

(fn solve1 []
  (var nNice 0)
  (each [l (io.lines "inputs/i05")]
    (when (and (not (or (l:match :ab) (l:match :cd) (l:match :pq) (l:match :xy)))
             (<= 3 (countIter (l:gmatch "[aeiou]")))
             (l:match "(.)%1") )
      (set nNice (+ nNice 1)) ) )
  nNice )

(fn solve2 []
  (var nNice 0)
  (each [l (io.lines "inputs/i05")]
    (when (and (l:match "(.).%1") (l:match "(..).*%1"))
      (set nNice (+ nNice 1))))
  nNice 
)

(print (solve1))
(print (solve2))