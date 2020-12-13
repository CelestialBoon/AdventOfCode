(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))
(fn repeat [a num] (let [res []] (for [i 1 num] (tset res i a)) res))

(local view (require :fennelview))
(fn pp [x] (print (view x)))

(fn readParseInput []
  (icollect [l (io.lines "inputs/i06")]
    (let [(com ax ay bx by) (l:match "(%D+) (%d+),(%d+) through (%d+),(%d+)")]
      {:com (match com "turn on" :on "turn off" :off "toggle" :tog)
        :a {:x (+ 1 (tonumber ax)) :y (+ 1 (tonumber ay))}
        :b {:x (+ 1 (tonumber bx)) :y (+ 1 (tonumber by))}
      })
  )
)

(fn solve1 [input]
  (local ls [])
  (for [i 1 1000] (tset ls i (repeat false 1000)))
  (each [_ {: com : a : b} (ipairs input)]
    (for [i a.x b.x]
      (for [j a.y b.y]
        (match com
          :on  (tset (. ls i) j true)
          :off (tset (. ls i) j false)
          :tog (tset (. ls i) j (not (. (. ls i) j))) ) ) ) )
  (var cnt 0)
  (each [_ row (ipairs ls)]
    (each [_ c (ipairs row)]
      (when c (set cnt (+ 1 cnt))) ) )
  cnt )

(fn solve2 [input]
  (local ls [])
  (for [i 1 1000] (tset ls i (repeat 0 1000)))
  (each [_ {: com : a : b} (ipairs input)]
    (for [i a.x b.x]
      (for [j a.y b.y]
        (match com
          :on  (tset (. ls i) j (+ 1 (. (. ls i) j)))
          :off (tset (. ls i) j (math.max 0 (- (. (. ls i) j) 1)))
          :tog (tset (. ls i) j (+ 2 (. (. ls i) j)))))))
  (var cnt 0)
  (each [_ row (ipairs ls)]
    (each [_ n (ipairs row)]
      (set cnt (+ n cnt))))
  cnt )

(-> (readParseInput) (solve1) (print))
(-> (readParseInput) (solve2) (print))