(local lume (require :lume))

(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))
(macro fold [[name init] iter-tbl fold-expr] `(do (var ,name ,init) (each ,iter-tbl (set ,name ,fold-expr)) ,name))

(fn access [t x y k v]
  (when (not (. t x))
    (tset t x {}))
  (when (not (-> t (. x) (. y)))
    (tset (. t x) y {:tbl t : x : y}))
  (if
    (not= nil v)
      (tset (-> t (. x) (. y)) k v)
    (not= nil k)
      (-?> t (. x) (. y) (. k))
      (-?> t (. x) (. y))))

(fn neighbors [x y]
  [[(+ x 1) y]
  [(- x 1) y]
  [x (+ y 1)]
  [x (- y 1)]
  [(- x 1) (+ y 1)]
  [(+ x 1) (- y 1)]])

(fn getNeighbors [tile]
  (if tile.nbrs tile.nbrs
    (let [nbrs (icollect [_ [x y] (ipairs (neighbors tile.x tile.y))] (access tile.tbl x y))]
      (tset tile :nbrs nbrs)
      nbrs)))

(fn checkFlip [tile]
  (when (not tile.checked?)
    (set tile.checked? true)
    (let [color tile.c
          nblacks (fold [cnt 0] [_ nbr (ipairs (getNeighbors tile))] (+ cnt (if nbr.c 1 0)))]
      (when (or
                (and color (or (= 0 nblacks) (< 2 nblacks)))
                (and (not color) (= 2 nblacks)))
        (set tile.changing? true)))))

(fn flip [tile]
  (set tile.checked? false)
  (when tile.changing?
    (set tile.c (not tile.c))
    (set tile.changing? false)))

(fn countGrid [grid]
  (fold [cnt 0] [_ row (pairs grid)] (+ cnt 
    (or (fold [pcnt 0] [_ tile (pairs row)] (+ pcnt (if tile.c 1 0))) 0))))

(let [lSteps (icollect [l (io.lines "inputs/i24")] (lume.array (l:gmatch "[ns]?[ew]")))
      move (fn move [steps x y]
        (if (= 0 (length steps))
          (values x y)
          (move steps (match (table.remove steps)
            :e  (values (+ x 1) y)
            :w  (values (- x 1) y)
            :ne (values x (+ y 1))
            :sw (values x (- y 1))
            :nw (values (- x 1) (+ y 1))
            :se (values (+ x 1) (- y 1))))))
        hexes {}]
  (each [_ steps (ipairs lSteps)]
    (let [(x y) (move steps 0 0)]
      (access hexes x y :c (not (access hexes x y :c)))))
  (print (countGrid hexes))
  (for [day 1 100]
    (let [currentTiles []]
      (each [_ row (pairs hexes)]
        (each [_ tile (pairs row)]
          (table.insert currentTiles tile)))
      (each [_ tile (ipairs currentTiles)]
        (checkFlip tile)
        (when tile.c (each [_ t (ipairs (getNeighbors tile))] (checkFlip t)))))
    (each [_ row (pairs hexes)]
      (each [_ tile (pairs row)]
        (flip tile))))
  (print (countGrid hexes)))
