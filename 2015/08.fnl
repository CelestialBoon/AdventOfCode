(local view (require :fennelview))
(fn pp [x] (print (view x)))

(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))
(fn count [iter f] (var cnt 0) (each [k v iter] (when (f v k) (set cnt (+ cnt 1)))) cnt)


(fn solve1 []
  (var cnt 0)
  (each [l (io.lines "inputs/i08")]
    (let [(l1 n1) (l:gsub "\\\\" "")
          (l2 n2) (l1:gsub "\\\"" "")
          (l3 n3) (l2:gsub "\\x%x%x" "")]
      (set cnt (+ cnt n1 n2 (* 3 n3) 2))))
  (print cnt)
)

(fn solve2 []
  (var cnt 0)
  (each [l (io.lines "inputs/i08")]
    (let [(l1 n1) (l:gsub "\\" "")
          (l2 n2) (l1:gsub "\"" "")]
      (set cnt (+ cnt n1 n2 2))))
  (print cnt)
)

(solve1)
(solve2)