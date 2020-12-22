(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))
(macro collect [iter-tbl key-value-expr] `(let [tbl# {}] (each ,iter-tbl (match ,key-value-expr (k# v#) (tset tbl# k# v#))) tbl#))
(macro fold [[name init] iter-tbl fold-expr] `(do (var ,name ,init) (each ,iter-tbl (set ,name ,fold-expr)) ,name))

(local lume (require :lume))
(local view (require :fennelview))
(fn pp [x] (print (view x)))

(fn isIn? [a bs] (var result false) (each [_ v (pairs bs)] (when (= a v) (set result true) (lua :break))) result)
(fn merge [t1 t2] (each [k v (pairs t2)] (tset t1 k v)) t1)
(fn mergeList [as] (fold [res []] [_ a (ipairs as)] (merge res a)))
(fn only1 [aset] (var res nil) (each [_ v (pairs aset)] (set res v)) res)

(let [allergensList {}
    ingrSet {}
    recipes (icollect [l (io.lines "inputs/i21")]
      (let [(sContains sAllergens) (l:match "( %(contains ([%a ,]+)%))")
            sIngredients (l:sub 1 (- 0 (sContains:len)))
            allergens (icollect [a (sAllergens:gmatch "%a+")] a)
            ingredients (icollect [i (sIngredients:gmatch "%a+")] i)]
        (each [_ a (ipairs allergens)]
          (when (not (. allergensList a))
            (tset allergensList a {:lists []}))
          (table.insert (. (. allergensList a) :lists) ingredients))
        (each [_ i (ipairs ingredients)]
          (tset ingrSet i i))
        {: allergens : ingredients}))]
  (each [a atbl (pairs allergensList)]
  (let [ingrSet (mergeList (icollect [_ l (ipairs atbl.lists)] (collect [_ ingr (ipairs l)] (values ingr ingr))))]
    (tset atbl :set (lume.filter ingrSet (fn [i] (lume.all atbl.lists (fn [l] (isIn? i l)))) true))))
  (let [noAllergens (lume.filter ingrSet (fn [i] (lume.all allergensList (fn [a] (not (isIn? i a.set))))) true)]
  (print (fold [sum 0] [_ i (pairs noAllergens)]
    (+ sum (lume.count recipes (fn [{: ingredients}] (isIn? i ingredients)))))))
  (let [realAllergens {}]
  (while (< 0 (lume.count allergensList))
    (each [a tbl (pairs allergensList)]
      (when (= 1 (lume.count tbl.set))
        (local ingr (only1 tbl.set))
        (tset realAllergens a ingr)
        (tset allergensList a nil)
        (each [_ t (pairs allergensList)]
          (tset t.set ingr nil))
        (lua :break))))
  (pp realAllergens)))
