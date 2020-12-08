(var md5 (require :md5))

(fn search [i] 
  (let [message (md5.sumhexa (.. "yzbqklnj" i))]
    (if (message:match "^00000")
      i
      (search (+ 1 i))
    )
  )
)

(print (search 0))
; (print (md5.sumhexa :pqrstuv1048970))
