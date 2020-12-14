(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))
(macro fold [[name init] iter-tbl fold-expr] `(do (var ,name ,init) (each ,iter-tbl (set ,name ,fold-expr)) ,name))

(fn map [f as] (icollect [i v (ipairs as)] (f v i)))
(fn range [a b] (var i (- a 1)) (fn ret [] (when (< i b) (set i (+ 1 i)) i)))

(fn copy [as] (let [bs {}] (each [k v (pairs as)] (tset bs k v)) bs))

; number to binary chars to merging with mask to number again
(fn strToMask [smask] (icollect [c (string.gmatch (smask:reverse) :.)] c))

(fn numToBinary [len num]
  (var n num)
  (let [res []]
    (for [i 1 len]
      (table.insert res (% n 2))
      (set n (math.floor (/ n 2))))
    res))

(fn binToNum [bin] (tonumber (string.reverse (table.concat bin)) 2))

(fn solve1 []
  (fn applyMask [mask bnum]
    (each [i v (pairs mask)] (when (not= v :X) (tset bnum i v))) bnum)
  (var (memory mask) (values {} {}))
  (each [l (io.lines "inputs/i14")]
    (let [smask (l:match "mask = (%w+)")
          (addr num) (l:match "mem%[(%d+)%] = (%d+)")]
      (if smask (set mask (strToMask smask))
        (->> num (numToBinary 36) (applyMask mask) (binToNum) (tset memory addr)))))
  (print (fold [sum 0] [k v (pairs memory)] (+ sum v))))

(fn solve2 []
  (fn applyMask [mask num]
    (let [xs [] nums []]
      (each [i c (ipairs mask)]
        (match c
          :X (table.insert xs i)
          :1 (tset num i 1)))
      (local lenxs (length xs))
      (icollect [n (range 0 (- (^ 2 lenxs) 1))]
        (let [numc (copy num)]
          (each [i v (ipairs (numToBinary lenxs n))]
            (tset numc (. xs i) v))
        numc))))
  (var (memory mask) (values {} {}))
  (each [l (io.lines "inputs/i14")]
    (let [smask (l:match "mask = (%w+)")
          (address num) (l:match "mem%[(%d+)%] = (%d+)") 
          n (tonumber num)]
      (if smask (set mask (strToMask smask))
        (->> address (numToBinary 36) (applyMask mask) (map binToNum) (map #(tset memory $1 n))))))
  (print (fold [sum 0] [k v (pairs memory)] (+ sum v))))

(solve1)
(solve2)