(local lume (require :lume))
(local view (require :fennelview))
(fn pp [x] (print (view x)))

(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))
; (macro collect [iter-tbl key-value-expr] `(let [tbl# {}] (each ,iter-tbl (match ,key-value-expr (k# v#) (tset tbl# k# v#))) tbl#))
(macro fold [[name init] iter-tbl fold-expr] `(do (var ,name ,init) (each ,iter-tbl (set ,name ,fold-expr)) ,name))

(fn merge [t1 t2] (each [k v (pairs t2)] (tset t1 k v)) t1)
(fn isIn? [a bs] (var result false) (each [_ v (pairs bs)] (when (= a v) (set result true) (lua :break))) result)
(fn intersection [as bs] (lume.filter as #(isIn? $ bs)))
; (fn touch [t k] (tset t k (or (. t k) {})))
(fn mod4 [a b] (-> a (+ (or b 0)) (- 1) (% 4) (+ (if (< 0 a) 1 -4))))
(fn order4 [as] (table.sort as) (match as [1 4] (values 4 1) [a b] (values a b)))
(fn missing4 [as] (lume.match [1 2 3 4] #(not (isIn? $ as))))
; (fn range [a b] (var i (- a 1)) (fn ret [] (when (< i b) (set i (+ 1 i)) i)))

(fn readParseInput []
  (let [tiles {}]
    (var (id tContent) (values 0 []))
    (each [l (io.lines "inputs/i20")]
      (let [sid (l:match "Tile (%d+):")
            empty (l:match "^$")
          ]
        (if sid (do (set id (tonumber sid)) (set tContent []))
          (not empty) (table.insert tContent l)
          (tset tiles id {: tContent : id})
        )
      )
    )
    tiles
  )
)

(fn borders [lines]
  (let [borders [ (. lines 1)
          (table.concat (icollect [_ l (ipairs lines)] (l:sub -1 -1)))
          (string.reverse (. lines (length lines)))
          (string.reverse (table.concat (icollect [_ l (ipairs lines)] (l:sub 1 1))))]
        bordersSet {}]
    (each [i b (ipairs borders)]
      (tset borders (- 0 i) (b:reverse))
    )
    (each [i b (pairs borders)] (tset bordersSet b i))
    {: borders : bordersSet}
  )
)

(fn getBorder [t side orien]
  (let [b (mod4 side (- (or orien t.orien) 1))]
    (values (. t.borders b) b)
  )
)

(fn getReverseBorder [t side orien]
  (let [b (- 0 (mod4 side (- (or orien t.orien) 1)))]
    (values (. t.borders b) b)
  )
)

(fn getOrien [t border side]
  (mod4 (if (= :number (type border)) border (t.bordersSet border)) (- 1 side))
)

(fn allConnections [t1 t2]
  (var (res1 res2) (values nil nil))
  (each [k1 v1 (ipairs t1.borders)] (each [k2 v2 (pairs t2.borders)]
    (when (= v1 v2)
      (set res1 [k1 k2])
      (set res2 [(math.abs k2) (* k1 (math.sign k2))])
      (lua :break)
    )
  ))
  (values res1 res2)
)

(fn getConnection [t1 t2]
  (let [matches (. t1.matches t2.id)]
    (if (< 0 t1.orien) 
      matches
      [(- 0 (. matches 1)) (- 0 (. matches 2))]
    )
  )
)

(fn possibleOriens [t place]
  (let [sides (macrodebug (icollect [_ side (pairs t.matches)] side))]
    (match (lume.count t.matches)
      2 (let [(min max) (order4 sides)]
        (match place
          :sw [min (- 0 max)]
          :se [max (- 0 min)]
          :nw [(mod4 min -1) (mod4 (- 0 max) -1)]
          :ne [(mod4 max 1) (mod4 (- 0 min) 1)]
        )
      )
      3 (let [missing (missing4 sides)]
        (match place
          :s [missing (- 0 missing)]
          :n [(mod4 missing 2) (mod4 (- 0 missing) 2)]
          :w [(mod4 missing 1) (mod4 (- 0 missing) 1)]
          :e [(mod4 missing -1) (mod4 (- 0 missing) -1)]
        )
      )
    )
  )
)


(fn solve [tiles]
  (fn findFittingTile [place left upper]
    (let [(ref dir) (if left (values left 2) (values upper 3))
          (sb b) (getReverseBorder ref dir)
          newId (. ref.sides b)
          newTile (. tiles newId)
          oriens []]
          (when left (let [newOrien1 (getOrien newTile (. newTile.matches left.id) 4)
                          newOrien2 (mod4 (- 0 newOrien1) 2)]
            (table.insert oriens [newOrien1 newOrien2])))
          (when upper (let [newOrien3 (getOrien newTile (. newTile.matches upper.id) 1)
                            newOrien4 (- 0 newOrien3)]
            (table.insert oriens [newOrien3 newOrien4])))
          (when place 
            (table.insert oriens (possibleOriens newTile place)))
      (tset newTile :orien (-> (intersection (. oriens 1) (. oriens 2)) (. 1)))
      newTile
    )
  )
  (each [_ t (pairs tiles)]
    (merge t (borders t.tContent))
    (tset t :matches {})
    (tset t :sides {})
  )
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
            (tset t2.sides (- 0 con2) id1)
             ) ) ) ) )

  (let [
        corners (lume.filter tiles #(= 2 (lume.count $.matches)))
        edges (lume.filter tiles #(= 3 (lume.count $.matches)))
        inner (lume.filter tiles #(= 4 (lume.count $.matches)))
        grid []
        ]
    (print (fold [prod 1] [i t (pairs corners)] (* prod (. t :id))))
    (for [i 1 12]
      (tset grid i [])
      (match i
        1 (let [firstTile (. corners 1)
                firstOrien (. (possibleOriens firstTile :nw) 1)
              ]
            (tset firstTile :orien firstOrien)
            (table.insert (. grid i) firstTile)
            (var prevTile firstTile)
          )
          (for [j 2 11]
            (let [newTile (findFittingTile :n prevTile)]
              (table.insert (. grid i) newTile)
              (set prevTile newTile)
            )
          )
          (let [newTile (findFittingTile :ne prevTile)]
            (table.insert (. grid i) newTile)
          )
        12 (do 
            (var (leftTile upTile) (values nil (-> grid (. 11) (. 1))))
            (let [newTile (findFittingTile :sw leftTile upTile)]
              (table.insert (. grid i) newTile)
              (set (leftTile upTile) (values newTile (-> grid (. 11) (. 2))))
            )
            (for [j 2 11]
              (let [newTile (findFittingTile nil leftTile upTile)]
                (table.insert (. grid i newTile))
                (set (leftTile upTile) (values newTile (-> grid (. 11) (. (+ j 1)))))
              )
            )
          _ (let [newTile (findFittingTile :se leftTile upTile)]
              (table.insert (. grid i) newTile)
            )
          )
        _ (do (var (leftTile upTile) (values nil (-> grid (. (- i 1)) (. 1))))
              (let [newTile (findFittingTile :w leftTile upTile)]
                (table.insert (. grid i) newTile)
                (set (leftTile upTile) (values newTile (-> grid (. (- i 1)) (. 2))))
              )
              (for [j 2 11]
                (let [newTile (findFittingTile nil leftTile upTile)]
                  (table.insert (. grid i newTile))
                  (set (leftTile upTile) (values newTile (-> grid (. (- i 1)) (. (+ j 1)))))
                )
              )
            _ (let [newTile (findFittingTile :e leftTile upTile)]
                (table.insert (. grid i) newTile)
              )
          )
      )
    )
  )
)

(-> (readParseInput) (solve))