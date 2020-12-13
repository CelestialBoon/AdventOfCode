(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))

(fn count [as f] (var cnt 0) (each [i v (ipairs as)] (when (f v i) (set cnt (+ cnt 1)))) cnt)

(local view (require :fennelview))
(fn pp [x] (print (view x)))

(fn readParseInput []
  (icollect [l (io.lines "inputs/i07")]
    (let [(i1 o1) (l:match "^(%w+) %-> (%w+)$")
          (i2 o2) (l:match "^NOT (%w+) %-> (%w+)$")
          (i3 c3 ii3 o3) (l:match "^(%w+) (%w+) (%w+) %-> (%w+)$")
          ]
      (if i1 {:c :PLUG : i1 :o o1}
          i2 {:c :NOT :i1 i2 :o o2}
          i3 {:c c3 :i1 i3 :i2 ii3 :o o3}
))))


(fn solve1 [instrs]
  (local jolts {})
  (fn translateInputs [c]
    (var (i1 i2) (values nil nil))
    (when c.i1 (set i1 (or (tonumber c.i1) (. jolts c.i1) nil)))
    (when c.i2 (set i2 (or (tonumber c.i2) (. jolts c.i2) nil)))
    (values i1 i2)
  )
  (var a? false)
  (while (not a?)
    ;iterate on the list, and see what can be done (what inputs are all there)
    (each [_ i (ipairs instrs)]
      (when (not i.done?)
        (local (i1 i2) (translateInputs i))
        ; (print (view i) i1 i2)
        (match i
          {:c :PLUG : o} (when i1 (set i.done? true) (tset jolts o i1))
          {:c :NOT : o} (when i1 (set i.done? true) (tset jolts o (% (bnot i1) 65536)))
          {:c :LSHIFT : o} (when i1 (set i.done? true) (tset jolts o (% (lshift i1 i2) 65536)))
          {:c :RSHIFT : o} (when i1 (set i.done? true) (tset jolts o (% (rshift i1 i2) 65536)))
          {:c :AND : o} (when (and i1 i2) (set i.done? true) (tset jolts o (% (band i1 i2) 65536)))
          {:c :OR : o} (when (and i1 i2) (set i.done? true) (tset jolts o (% (bor i1 i2) 65536)))
          _ (print (.. "error, instruction invalid: " (view i)))
        )
      )
    )
    ; (print (.. "instructions done: " (count instrs #$1.done?)))
    (set a? jolts.a)
  )
  jolts.a
)

(-> (readParseInput) (solve1) (print))
;change 14146 -> b in the input for part 2