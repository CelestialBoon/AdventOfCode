(macro icollect [iter-tbl value-expr] `(let [tbl# []] (each ,iter-tbl (tset tbl# (+ (length tbl#) 1) ,value-expr)) tbl#))
(macro fold [[name init] iter-tbl fold-expr] `(do (var ,name ,init) (each ,iter-tbl (set ,name ,fold-expr)) ,name))

(local lume (require :lume))

(fn any? [as f] (var res? false) (each [k v (pairs as)] (when (f v k) (set res? true))) res?)
(fn all? [as f] (var res? true) (each [k v (pairs as)] (when (not (f v k)) (set res? false))) res?)
(fn range [a b] (var i (- a 1)) (fn ret [] (when (< i b) (set i (+ 1 i)) i)))
(fn concat [as bs] (each [_ v (ipairs bs)] (table.insert as v)) as)
(fn between [a b c] (and (<= a b) (<= b c)))
(fn copy [as] (let [bs {}] (each [k v (pairs as)] (tset bs k v)) bs))
(fn copyInitLast [as] (let [cas (copy as) last (table.remove cas)] {:init cas : last}))
(fn copyHeadTail [as] (let [cas (copy as) head (table.remove cas 1)] {: head :tail cas}))
(fn copyTset [as k v] (local cas (copy as)) (tset cas k v) cas)
(fn firstBy [as f] (var res nil) (each [i v (ipairs as)] (set res (or res (f v i)))) res)

(fn readParseInput []
  (var myTicket nil)
  (let [rules []
        otherTickets []
        parseTicket (fn [ticket] (icollect [n (ticket:gmatch "%d+")] (tonumber n)))]
    (each [l (io.lines "inputs/i16")]
      (let [(name n1 n2 n3 n4) (l:match "([%a ]+): (%d+)%-(%d+) or (%d+)%-(%d+)")
            mYourTicket (l:match "your ticket:")
            sTicket (l:match "^([%d,]+)$")]
        (if name (table.insert rules {: name :bounds {(tonumber n1) (tonumber n2) (tonumber n3) (tonumber n4)}})
          mYourTicket (set myTicket true)
          (and sTicket (= myTicket true)) (set myTicket (parseTicket sTicket))
          sTicket (table.insert otherTickets (parseTicket sTicket)))))
    {: rules : myTicket : otherTickets}))

(fn solve1 [{: rules : myTicket : otherTickets}]
  (let [validNums {}
        checkTicket #(icollect [_ n (ipairs $)] (if (not (. validNums n)) n))
        otherValidTickets []]
    (each [_ r (ipairs rules)] (each [min max (pairs r.bounds)] (each [i (range min max)]
                                                                  (tset validNums i i))))
    (local invalidNums (checkTicket myTicket))
    (each [_ t (ipairs otherTickets)] (let [invalidns (checkTicket t)]
      (when (= 0 (length invalidns)) (table.insert otherValidTickets t))
      (concat invalidNums invalidns)))
    (print (fold [sum 0] [_ v (ipairs invalidNums)] (+ sum v)))
    {: rules : myTicket : otherValidTickets}))

(fn solve2 [{: rules : myTicket : otherValidTickets}]
  (let [tickets (concat otherValidTickets [myTicket])
        checkRuleOnField (fn [{: name : bounds} i]
          (all? tickets (fn [ticket] (any? bounds #(between $2 (. ticket i) $1)))))
        compileOrder (fn compileOrder [rules fields]
          (if (= 0 (length rules)) fields
            (let [{:head rule :tail newRules} (copyHeadTail (lume.sort rules #(< (length $1.fields) (length $2.fields))))
                  nField (. rule.fields 1)]
              (tset fields nField rule)
              (each [_ r (ipairs newRules)]
                (lume.remove r.fields nField))
              (compileOrder newRules fields))))]
    (each [_ r (ipairs rules)]
      (set r.fields (icollect [i (range 1 (length myTicket))] (if (checkRuleOnField r i) i nil))))
    (local fields (compileOrder rules {}))
    (fold [prod 1] [i n (ipairs myTicket)] 
      (if (string.match (. (. fields i) :name) "^departure") (* prod n) prod))))

(-> (readParseInput) (solve1) (solve2) (print))