(fn pow [a b m] (var res 1) (for [_ 1 b] (set res (% (* res a) m))) res)

(let [m 20201227
      door 13233401
      card 6552760]
  (var (i dp cp curr) (values 1 nil nil 7))
  (while (or (not dp) (not cp))
    (set i (+ i 1))
    (set curr (% (* 7 curr) m))
    (when (= door curr) (set dp i))
    (when (= card curr) (set cp i))
  )
  (print dp cp (pow door cp m))
)