(fn permutations [as]
  (local res [])
  (for [i1 1 8] (for [i2 1 7] (for [i3 1 6] (for [i4 1 5] (for [i5 1 4] (for [i6 1 3] (for [i7 1 2]
    (local asc [(table.unpack as)])
    (table.insert res [(table.remove asc i1) (table.remove asc i2) (table.remove asc i3) (table.remove asc i4) (table.remove asc i5) (table.remove asc i6) (table.remove asc i7) (table.remove asc)])
  )))))))
  res
)

(local routes { 12 66 21 66
  13 28 31 28
  14 60 41 60
  15 34 51 34
  16 34 61 34
  17 3 71 3
  18 108 81 108
  23 22 32 22
  24 12 42 12
  25 91 52 91
  26 121 62 121 
  27 111 72 111
  28 71 82 71 
  34 39 43 39
  35 113 53 113
  36 130 63 130
  37 35 73 35
  38 40 83 40
  45 63 54 63
  46 21 64 21
  47 57 74 57
  48 83 84 83
  56 9 65 9
  57 50 75 50
  58 60 85 60
  67 27 76 27
  68 81 86 81
  78 90 87 90})

(fn solve1 []
  (var min math.huge)
  (each [_ perm (ipairs (permutations [1 2 3 4 5 6 7 8]))]
    (var route 0)
    (each [i c (ipairs perm)]
      (when (< i 8)
        (set route (+ route (. routes (+ (* 10 c) (. perm (+ 1 i))))))
      )
    )
    (when (< route min) (set min route))
  )
  (print min)
)

(fn solve2 []
  (var max 0)
  (each [_ perm (ipairs (permutations [1 2 3 4 5 6 7 8]))]
    (var route 0)
    (each [i c (ipairs perm)]
      (when (< i 8)
        (set route (+ route (. routes (+ (* 10 c) (. perm (+ 1 i))))))
      )
    )
    (when (> route max) (set max route))
  )
  (print max)
)

(solve1)
(solve2)