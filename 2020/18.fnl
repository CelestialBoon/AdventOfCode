(macro fold [[name init] iter-tbl fold-expr] `(do (var ,name ,init) (each ,iter-tbl (set ,name ,fold-expr)) ,name))

(fn opChain1 [s]
  (fold [res (tonumber (s:match "^%d+"))]
        [op n (s:gmatch "([*+]) (%d+)")]
      (match op
        :+ (+ res (tonumber n))
        :* (* res (tonumber n)))))

(fn opChain2 [s]
  (fn sums [s]
    (let [(s1 s2) (s:match "(%d+) %+ (%d+)")]
      (if s1
        (sums (s:gsub "%d+ %+ %d+" (+ (tonumber s1) (tonumber s2)) 1))
        s)))
  (fn prods [s]
    (let [(p1 p2) (s:match "(%d+) %* (%d+)")]
      (if p1
        (prods (s:gsub "%d+ %* %d+" (* (tonumber p1) (tonumber p2)) 1))
        s)))
  (tonumber (prods (sums s))))

(fn solve [opChain]
  (fn resolveLine [s]
    (let [parensContent (s:match "%(([^(]-)%)")]
      (if parensContent
        (resolveLine (s:gsub "%([^(]-%)" (opChain parensContent) 1))
        (opChain s))))
  (fold [sum 0] [l (io.lines "inputs/i18")]
    (+ sum (resolveLine l))))

(-> opChain1 (solve) (print))
(-> opChain2 (solve) (print))