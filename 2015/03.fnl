(fn readInput [] 
  (with-open [file (assert (io.open "inputs/i03.txt"))]
    (file:read :*a)))

(fn tbllen [tbl]
  (var cnt 0)
  (each [_ _ (pairs tbl)] (set cnt (+ 1 cnt)))
  cnt)

(fn solve1 [input]
  (var (x y) (values 0 0))
  (let [houses {"0,0" {:presents 1}}]
    (each [c (input:gmatch :.)]
      (match c
        :^ (set y (+ y 1))
        :v (set y (- y 1))
        :> (set x (+ x 1))
        :< (set x (- x 1))      
      )
      (let [pos (.. x "," y)
            house (or (. houses pos) {:presents 0})
            ]
        (tset house :presents (+ 1 (. house :presents)))
        (tset houses pos house)
      )
    )
    (tbllen houses)
  )
)

(fn solve2 [input]
  (var (santa robo turn) (values {:x 0 :y 0} {:x 0 :y 0} 0))
  (let [houses {"0,0" {:presents 1}}]
    (each [c (input:gmatch :.)]
      (let [d (if (= 0 (% turn 2)) santa robo)]    
        (match c
          :^ (set d.y (+ d.y 1))
          :v (set d.y (- d.y 1))
          :> (set d.x (+ d.x 1))
          :< (set d.x (- d.x 1))      
        )
        (let [pos (.. d.x "," d.y)
              house (or (. houses pos) {:presents 0})
              ]
          (tset house :presents (+ 1 (. house :presents)))
          (tset houses pos house)
        )
      )
      (set turn (+ turn 1))
    )
    (tbllen houses)
  )
)

(-> (readInput) (solve1) (print))
(-> (readInput) (solve2) (print))