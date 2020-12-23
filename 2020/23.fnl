(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))

(fn mod [a m] (if (= a m) 1 (+ a 1)))
(fn range [a b] (var i (- a 1)) (fn ret [] (when (< i b) (set i (+ 1 i)) i)))
(fn isIn? [a bs] (var result false) (each [_ v (pairs bs)] (when (= a v) (set result true) (lua :break))) result)

(let [max 1000000
      maxMoves 10000000
      modm (fn [a] (mod a max))
      firstCups [5 8 3 9 7 6 2 4 1]
      nCups (icollect [i (range 1 max)] (if (< i 10) (. firstCups i) i))
      lltbl []
      move (fn move [curr nMove]
        (when (<= nMove maxMoves)
          (let [m1 curr.next
                m2 m1.next
                m3 m2.next
                succ m3.next
                prev (do (var prev curr.prev) (while (isIn? prev [m1 m2 m3]) (set prev prev.prev)) prev)
                afterPrev prev.next]
            (set curr.next succ)
            (set prev.next m1)
            (set m3.next afterPrev)
            (move succ (+ 1 nMove)))))]
  (var prev nil)
  (for [i 1 max]
    (let [new {:n i : prev}]
      (tset lltbl i new)
      (set prev new)))
  (tset (. lltbl 1) :prev (. lltbl max))
  (each [i n (ipairs nCups)] (tset (. lltbl n) :next (->> i (modm) (. nCups) (. lltbl))))
  (move (. lltbl (. nCups 1)) 1)
  (let [n1 (-> lltbl (. 1) (. :next))
        n2 n1.next]
    (print (* n1.n n2.n))))