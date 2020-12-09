(fn tbllen [tbl]
  (var cnt 0)
  (each [_ _ (pairs tbl)] (set cnt (+ 1 cnt)))
  cnt)

(fn parseInput []
  (let [bagContents {}]
    (each [line (io.lines "inputs/i07")]
      (when line 
        (let [(firstPart outColor) (line:match "^((.*) bags contain )")
              inBags {}]
          (each [inNum inColor (string.gmatch (line:sub (+ 1 (firstPart:len))) "(%d+) ([%w ]+) bags?[,.]")]
            (tset inBags inColor (tonumber inNum)))
          (tset bagContents outColor inBags))))
    bagContents))

(fn reverse [bagContents]
  (let [reverseBags {}]
    (each [color _ (pairs bagContents)] (tset reverseBags color {}))
    (each [outColor ins (pairs bagContents)]
      (each [inColor inNumber (pairs ins)]
        (tset (. reverseBags inColor) outColor inNumber)))
    reverseBags))

(fn solve1 [reverseBags]
  (let [result {}
        searchColors (fn searchColors [color] 
          (each [col _ (pairs (. reverseBags color))]
            (when (not (. result col))
              (tset result col col)
              (searchColors col)
            )))]
    (searchColors "shiny gold")
    (tbllen result)))

(fn solve2 [bagContents]
  (var total 0)
  (let [fillBag (fn fillBag [outColor qty]
          (each [inColor num (pairs (. bagContents outColor))]
            (let [bagqty (* num qty)]
              (set total (+ total bagqty))
              (fillBag inColor bagqty))))]
    (fillBag "shiny gold" 1))
  total)

(-> (parseInput) (reverse) (solve1) (print))
(-> (parseInput) (solve2) (print))
