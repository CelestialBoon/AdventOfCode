(macro add1 [a] `(set ,a (+ ,a 1)))

(fn access [t {: x : y : z : w} v]
  (when (not (. t x)) (tset t x []))
  (var current (. t x))
  (when (not (. current y)) (tset current y []))
  (set current (. current y))
  (when (not (. current z)) (tset current z []))
  (set current (. current z))
  (when (not= nil v) (tset current w v))
  (. current w))

(fn inquire [t {: x : y : z : w}]
  (-?> t (. x) (. y) (. z) (. w)))

(fn iIter [theIter]
  (var count 0)
  (fn newIter []
    (add1 count)
    (let [v (theIter)]
      (if (not= v nil)
        (values count v)
        nil))))

(fn neighbors [{: x : y : z : w}]
  (let [res []]
    (for [a -1 1] (for [b -1 1] (for [c -1 1] (for [d -1 1]
      (when (or (not= a 0) (not= b 0) (not= c 0) (not= d 0))
        (table.insert res {:x (+ x a) :y (+ y b) :z (+ z c) :w (+ w d)}))))))
    res))

(fn neighborsCnt [t ns]
  (var tot 0)
  (each [_ n (ipairs ns)] (when (inquire t n) (add1 tot)))
  tot)

(fn readParseInput []
  (let [input "#.#..###\n.#....##\n.###...#\n..####..\n....###.\n##.#.#.#\n..#..##.\n#.....##"
        t []]
    (each [x s (iIter (input:gmatch "[^\n]+"))] (each [y c (iIter (s:gmatch "."))]
      (access t {: x : y :z 0 :w 0} (match c :. false :# true))))
    t))

(fn cycle [t turns]
  (let [wid (length t) len (length (. t 1))]
    (for [turn 1 turns]
      (local changing [])
      (for [x (- 1 turn) (+ wid turn)]
        (for [y (- 1 turn) (+ len turn)]
          (for [z (- 0 turn) turn]
            (for [w (- 0 turn) turn]
              (let [p {: x : y : z : w}
                    onNeigh (neighborsCnt t (neighbors p))]
                (if (inquire t p)
                  (when (or (< onNeigh 2) (> onNeigh 3))
                    (tset changing p false))
                  (when (= onNeigh 3)
                    (tset changing p true))))))))
      (each [p v (pairs changing)]
        (access t p v)))))

(fn countOn [t]
  (var sum 0)
  (each [_ cube (pairs t)] (each [_ plane (pairs cube)] (each [_ line (pairs plane)] (each [_ n (pairs line)]
    (when n (add1 sum))))))
  sum)

(fn solve [t]
  (cycle t 6)
  (countOn t))

(-> (readParseInput) (solve) (print))