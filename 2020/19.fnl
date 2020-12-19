(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))
(macro fold [[name init] iter-tbl fold-expr] `(do (var ,name ,init) (each ,iter-tbl (set ,name ,fold-expr)) ,name))

(fn copy [as] (let [bs {}] (each [k v (pairs as)] (tset bs k v)) bs))
(fn copyReplace [as i ns] (let [asc (copy as)] (table.remove asc i) (for [j 1 (length ns)] (table.insert asc (+ i j -1) (. ns j))) asc))

(fn readParseInput []
  (let [iter (io.lines "inputs/i19")
    rules []
    signals []
    parseNums (fn [s] (icollect [n (s:gmatch "%d+")] (tonumber n)))]
    (each [l iter]
      (let [(num content) (l:match "^(%d+): \"?(.-)\"?$")]
        (if num
          (tset rules (tonumber num) 
            (if (content:match "%a") content
              (let [(p1 p2) (content:match "([%d ]+) | ([%d ]+)")]
                (if p1
                  [(parseNums p1) (parseNums p2)]
                  [(parseNums content)]))))
          (lua :break))))
    (each [l iter]
      (table.insert signals l))
    {: rules : signals}))

(fn solve [{: rules : signals}]
  (fn checkSig [sig]
    (let [currRules (copy (. rules 0))
          check (fn check [rule i]
        (if (or (> i (sig:len)) (> i (length rule))) 
          false
          (let [rulei (. rule i)]
            (if (= (type (. rules rulei)) :string)
              (if (= (. rules rulei) (sig:sub i i))    
                (if (and (= i (sig:len)) (= i (length rule)))
                  true
                  (check rule (+ i 1)))
                false)
              (do (each [_ r (ipairs (. rules rulei))]
                    (table.insert currRules (copyReplace rule i r)))
                (check (table.remove currRules) i))))))]
      (var matched? false)
      (while (and (< 0 (length currRules)) (not matched?)) (set matched? (check (table.remove currRules) 1)))
      matched?))
  (fold [res 0] [_ sig (ipairs signals)] (+ res (if (checkSig sig) 1 0))))

(fn replaceRules [{: rules : signals}]
  (tset rules 8 [[42] [42 8]])
  (tset rules 11 [[42 31] [42 11 31]])
  {: rules : signals})

(-> (readParseInput) (solve) (print))
(-> (readParseInput) (replaceRules) (solve) (print))
