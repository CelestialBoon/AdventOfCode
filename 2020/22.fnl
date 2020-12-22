(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))
(macro fold [[name init] iter-tbl fold-expr] `(do (var ,name ,init) (each ,iter-tbl (set ,name ,fold-expr)) ,name))
(macro add1 [a] `(set ,a (+ ,a 1)))

(fn pop [as] (table.remove as 1))
(fn push [as a1 a2] (table.insert as a1) (table.insert as a2))
(fn any? [as f] (var res? false) (each [i v (ipairs as)] (when (f v i) (set res? true))) res?)
(fn all? [as f] (var res? true) (each [i v (ipairs as)] (when (not (f v i)) (set res? false))) res?)

(fn copy [as n] (icollect [i v (ipairs as)] (when (or (not n) (<= i n)) v)))
(fn zipEquals [as bs]
  (var (res i) (values true 0))
  (let [iter1 (ipairs as) iter2 (ipairs bs)]
    (while true
      (let [(i1 v1) (iter1 as i) (i2 v2) (iter2 bs i)]
        (add1 i)
        (when (or (not i1) (not i2))
          (lua :break))
        (when (not= v1 v2) (set res false) (lua :break)))))
  res)
(fn equals [as bs] (and (= (length as) (length bs)) (zipEquals as bs)))

(let [input (with-open [file (assert (io.open "inputs/i22"))]
                (file:read :*a))
      (p1 p2) (input:match "^Player 1:\n(.*)\n\nPlayer 2:(.*)\n$")
      parse (fn [s] (icollect [n (s:gmatch "%d+")] (tonumber n)))
      (cs1 cs2) (values (parse p1) (parse p2))
      calculateScore (fn [deck]
        (fold [sum 0] [i n (ipairs deck)]
          (+ sum (* n (- (length deck) i -1)))))
      round (fn round [cs1 cs2]
        (if (= 0 (length cs1)) cs2
            (= 0 (length cs2)) cs1
            (let [c1 (pop cs1) c2 (pop cs2)]
              (push (if (> c1 c2) (values cs1 c1 c2) (values cs2 c2 c1)))
              (round cs1 cs2))))
      recursiveRound (fn recursiveRound [cs1 cs2 usedDecks]
        (if (= 0 (length cs1)) 2
            (= 0 (length cs2)) 1
            (any? usedDecks (fn [[d1 d2]] (and (equals d1 cs1) (equals d2 cs2)))) 1
          (do (table.insert usedDecks [(copy cs1) (copy cs2)])
            (let [c1 (pop cs1) c2 (pop cs2)]
              (if (or (> c1 (length cs1)) (> c2 (length cs2)))
                (do (push (if (> c1 c2) (values cs1 c1 c2) (values cs2 c2 c1)))
                    (recursiveRound cs1 cs2 usedDecks))
                (do (push (if (= 1 (recursiveRound (copy cs1 c1) (copy cs2 c2) []))
                              (values cs1 c1 c2) (values cs2 c2 c1)))
                    (recursiveRound cs1 cs2 usedDecks)))))))]
  (let [winner (round (copy cs1) (copy cs2))]
    (print (calculateScore winner)))
  (let [d1 (copy cs1) d2 (copy cs2)
        winner (recursiveRound d1 d2 [])]
    (print (calculateScore (if (= winner 1) d1 d2)))))
