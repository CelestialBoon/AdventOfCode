(local lume (require :lume))

(macro icollect [iter-tbl value-expr]
  `(let [tbl# []]
     (each ,iter-tbl
       (tset tbl# (+ (length tbl#) 1) ,value-expr))
     tbl#))

(fn read-file []
  (with-open [file (assert (io.open "inputs/i06"))]
    (file:read :*a)))

(fn strtochars [str] 
  (icollect [c (str:gmatch :.)] c))

(fn strtotable [str]
  (let [res {}]
    (each [c (str:gmatch :.)]
      (tset res c c))
  res))

(fn intersection [a b]
  (let [c {}]
    (each [k v (pairs a)]
      (when (. b k) (tset c k v)))
    c))

(fn tbllen [tbl]
  (var cnt 0)
  (each [_ _ (pairs tbl)] (set cnt (+ 1 cnt)))
  cnt)

(fn solve1 [input]
  (var sum 0)
  (let [rInput (string.gsub input "\n(\n*)" "%1")]
    (each [line (rInput:gmatch "%w+")]
      (set sum (+ sum (length (lume.unique (strtochars line)))))))
  sum)

(fn solve2 [input]
  (var sum 0)
  (let [rInput (string.gsub input "\n(\n*)" "-%1")]
    (each [group (rInput:gmatch "%g+")]
      (let [persons (icollect [p (group:gmatch "%w+")] (strtotable p))]
        (set sum (+ sum (tbllen (lume.reduce persons intersection)))))))
  sum)

(-> (read-file) (solve1) (print))
(-> (read-file) (solve2) (print))