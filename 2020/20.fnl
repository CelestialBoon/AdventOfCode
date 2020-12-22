(local lume (require :lume))

(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))
(macro fold [[name init] iter-tbl fold-expr] `(do (var ,name ,init) (each ,iter-tbl (set ,name ,fold-expr)) ,name))

(fn range [a b] (var i (- a 1)) (fn ret [] (when (< i b) (set i (+ 1 i)) i)))
(fn merge [t1 t2] (each [k v (pairs t2)] (tset t1 k v)) t1)
(fn isIn? [a bs] (var result false) (each [_ v (pairs bs)] (when (= a v) (set result true) (lua :break))) result)
(fn maxBy [ls compare] (var result nil) (each [k v (pairs ls)] (when (or (not result) (compare v result k)) (set result v))) result)

(fn mod4 [a b] (-> a (+ (or b 0)) (- (if (< 0 a) 1 0)) (% 4) (+ (if (< 0 a) 1 -4))))
(fn order4 [as] (table.sort as) (match as [1 4] (values 4 1) [a b] (values a b)))
(fn missing4 [as] (lume.match [1 2 3 4] #(not (isIn? $ as))))

(fn readParseInput []
  (let [tiles {}]
    (var (id tContent) (values 0 []))
    (each [l (io.lines "inputs/i20")]
      (let [sid (l:match "Tile (%d+):")
            empty (l:match "^$")]
        (if sid (do (set id (tonumber sid)) (set tContent []))
          (not empty) (table.insert tContent l)
          (tset tiles id {: tContent : id}))))
    tiles))

(fn borders [lines]
  (let [borders [ (. lines 1)
          (table.concat (icollect [_ l (ipairs lines)] (l:sub -1 -1)))
          (string.reverse (. lines (length lines)))
          (table.concat (icollect [_ l (lume.ripairs lines)] (l:sub 1 1)))]
        bordersSet {}]
    (each [i b (ipairs borders)]
      (tset borders (- 0 i) (b:reverse)))
    (each [i b (pairs borders)] (tset bordersSet b i))
    {: borders : bordersSet}))

(fn getBorder [t side orien]
  (let [b (mod4 (or orien t.orien) (- side 1))]
    (values (. t.borders b) b)))

(fn getReverseBorder [t side orien]
  (let [b (- 0 (mod4 (or orien t.orien) (- side 1)))]
    (values (. t.borders b) b)))

(fn getOrien [t border side]
  (mod4 (if (= :number (type border)) border (. t.bordersSet border)) (- 1 side)))

(fn allConnections [t1 t2]
  (var (res1 res2) (values nil nil))
  (each [k1 v1 (ipairs t1.borders)] (each [k2 v2 (pairs t2.borders)]
    (when (= v1 v2)
      (set res1 k1)
      (set res2 (math.abs k2))
      (lua :break))))
  (values res1 res2))

(fn possibleOriens [t place]
  (let [sides (icollect [_ side (pairs t.matches)] side)]
    (match (lume.count t.matches)
      2 (let [(min max) (order4 sides)]
          (match place
            :sw [min (- 0 max)]
            :se [max (- 0 min)]
            :nw [(mod4 min -1) (mod4 (- 0 max) -1)]
            :ne [(mod4 max 1) (mod4 (- 0 min) 1)]))
      3 (let [missing (missing4 sides)]
          (match place
            :s [missing (- 0 missing)]
            :n [(mod4 missing 2) (mod4 (- 0 missing) 2)]
            :w [(mod4 missing 1) (mod4 (- 0 missing) 1)]
            :e [(mod4 missing -1) (mod4 (- 0 missing) -1)])))))

(fn explodeString [s]
  (let [cl []]
    (for [i 1 (s:len)] (tset cl i (s:sub i i)))
    cl))

(fn rotateCll [cll orien]
  (let [rotate1 (fn [cll] (let [ncll [] len (length cll)]
          (for [i 1 len] (tset ncll i []))
          (each [i cl (ipairs cll)]
            (each [j c (ipairs cl)] (table.insert (. ncll (- len j -1)) i c)))
          ncll))
        rotate2 (fn [cll]
          (icollect [_ cl (lume.ripairs cll)]
            (icollect [_ c (lume.ripairs cl)] c)))
        flip (fn [cll]
          (icollect [_ cl (ipairs cll)]
            (icollect [_ c (lume.ripairs cl)] c)))]
    (match orien
      1 cll
      2 (rotate1 cll)
      3 (rotate2 cll)
      4 (rotate1 (rotate2 cll))
      -1 (flip cll)
      -2 (flip (rotate1 cll))
      -3 (flip (rotate2 cll))
      -4 (flip (rotate1 (rotate2 cll))))))

(fn rotateTile [tile]
  (rotateCll
    (icollect [_ s (ipairs tile.tContent)] (explodeString s))
    tile.orien))

(fn gridToCll [grid mnum]
  (let [totCll (icollect [_ (range 1 (* 8 mnum))] [])
        gset (fn [i j v] (tset (. totCll i) j v))]
    (each [i tl (ipairs grid)]
      (each [j t (ipairs tl)]
        (let [(oi oj) (values (* 8 (- i 1)) (* 8 (- j 1)))
              cll (rotateTile t)]
          (each [i cl (ipairs cll)]
            (each [j c (ipairs cl)]
              (when (and (< 1 i) (< i 10) (< 1 j) (< j 10)) (gset (+ i oi -1) (+ j oj -1) c)))))))
    totCll))

(fn findDragons [cll]
  (let [gWidth (length (. cll 1))
        coords [[0 0] [0 5] [0 6] [0 11] [0 12] [0 17] [0 18] [0 19] [1 1] [1 4] [1 7] [1 10] [1 13] [1 16] [-1 18]]]
    (fold [sum 0] [i cl (ipairs cll)]
      (+ sum (fold [psum 0] [j c (ipairs cl)]
        (+ psum (if (and
            (< 1 i) (< i gWidth) (<= j (- gWidth 19))
            (lume.all coords (fn [[oi oj]] (-> cll (. (+ i oi)) (. (+ j oj)) (= "#")))))
          1 0)))))))

(fn solve [tiles]
  (fn findFittingTile [left upper]
    (let [(ref dir) (if left (values left 2) (values upper 3))
          (sb b) (getReverseBorder ref dir)
          newId (. ref.sides b)
          newTile (. tiles newId)
          orien (if left (getOrien newTile sb 4) (getOrien newTile sb 1))]
      (tset newTile :orien orien)
      newTile))
  (each [_ t (pairs tiles)]
    (merge t (borders t.tContent))
    (tset t :matches {})
    (tset t :sides {}))
  (each [id1 t1 (pairs tiles)]
    (each [id2 t2 (pairs tiles)]
      (when (< id1 id2)
        (let [(con1 con2) (allConnections t1 t2)]
          (when (not= nil con1)
            (tset t1.matches id2 con1)
            (tset t1.sides con1 id2)
            (tset t1.sides (- 0 con1) id2)
            (tset t2.matches id1 con2)
            (tset t2.sides con2 id1)
            (tset t2.sides (- 0 con2) id1))))))
  (let [corners (lume.filter tiles #(= 2 (lume.count $.matches)))
        grid []
        mnum 12]
    (print (fold [prod 1] [i t (pairs corners)] (* prod (. t :id))))
    (for [i 1 mnum]
      (tset grid i [])
      (for [j 1 mnum]
        (if (and (= i 1) (= j 1))
          (let [firstTile (. corners 1)
                firstOrien (. (possibleOriens firstTile :nw) 1)]
            (tset firstTile :orien firstOrien)
            (table.insert (. grid i) firstTile))
          (let [leftTile (-?> grid (. i) (. (- j 1)))
                upperTile (-?> grid (. (- i 1)) (. j))
                newTile (findFittingTile leftTile upperTile)]
            (table.insert (. grid i) newTile)))))
    (let [cll (gridToCll grid mnum)
          dragonsFound {}]
      (each [_ orien (ipairs [1 2 3 4 -1 -2 -3 -4])]
        (let [cll (rotateCll cll orien)
              dragons (findDragons cll)]
          (tset dragonsFound orien dragons)))
      (let [maxDragons (maxBy dragonsFound #(> $1 $2))
            waterHashes (fold [sum 0] [_ cl (ipairs cll)]
                          (+ sum (fold [psum 0] [_ c (ipairs cl)] (+ psum (if (= c "#") 1 0)))))]
        (print (.. "dragons :" maxDragons " hashes: " waterHashes " water roughness:" (- waterHashes (* maxDragons 15))))))))

(-> (readParseInput) (solve))
