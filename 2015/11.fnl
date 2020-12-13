(local view (require :fennelview))
(fn pp [x] (print (view x)))

(fn solve1 []
  (fn increment [cs i] (let [c (. cs i)] (tset cs i (+ c (if (or (= c 105) (= c 111) (= c 108)) 2 1)))))
  (fn tostr [cs] (string.char (. cs 1) (. cs 2) (. cs 3) (. cs 4) (. cs 5) (. cs 6) (. cs 7) (. cs 8)))
  (fn nPass [cs]
    (local len (length cs))
    (increment cs len)
    (for [i (length cs) 1 -1]
      (when (= 123 (. cs i))
        (do (tset cs i 97) (increment cs (- i 1))))
    )
    cs
  )
  (fn hasTriplet [cs]
    (fn check [cs c cnt]
      (if (= cnt 3) true
          (= (length cs) 0) false
          (let [d (table.remove cs)]
            (if (not c) (check cs d 1)
                (= d (- c 1)) (check cs d (+ cnt 1))
                (check cs d 1)))))
    (check [(table.unpack cs)])
  )
  (fn nextValidPass [cs]
    (let [cs (nPass cs)
          s (string.char (table.unpack cs))
    ]
      (if (and (hasTriplet cs) (s:match "(%w)%1.*(%w)%2"))
        (values s (view cs))
        (nextValidPass cs)
      )
    )
  )
  (print (nextValidPass [104 101 112 120 99 114 114 113]))
  (print (nextValidPass [104 101 112 120 120 121 122 122]))
)

(solve1)