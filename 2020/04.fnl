(macro icollect [iter-tbl value-expr]
  `(let [tbl# []]
     (each ,iter-tbl
       (tset tbl# (+ (length tbl#) 1) ,value-expr))
     tbl#))

(fn read-file []
  (with-open [file (assert (io.open "inputs/i04"))]
    (file:read :*a)))

(fn parse [input]
  (let [passports []
    rInput (string.gsub input "\n(\n*)" " %1")]
    (each [line (string.gmatch rInput "[%g ]+")]
      (let [passport {}]
        (each [k v (string.gmatch line "(%w+):([%w#]+)")]
          (tset passport k v))
        (table.insert passports passport)
      ))
    passports
))

(fn equals [a bs]
  (var result false)
  (each [_ v (ipairs bs)]
    (when (= a v) (set result true)))
  result
)

(fn check-valid1 [passports]
  (icollect [_ p (ipairs passports)]
    (when (and (. p :byr) (. p :iyr) (. p :eyr) 
        (. p :hgt) (. p :hcl) (. p :ecl) (. p :pid))
      p)
))

(fn check-valid2 [passports]
  (icollect [_ p (ipairs passports)]
    (let [
        byr (tonumber (. p :byr))
        iyr (tonumber (. p :iyr))
        eyr (tonumber (. p :eyr))
        (hgt unit) (string.match (. p :hgt) "^(%d+)(%a+)$")
        hcl (string.match (. p :hcl) "^#[%da-f]+$")
        ecl (. p :ecl)
        pid (string.match (. p :pid) "^%d+$")]
      (when (and byr iyr eyr hgt unit hcl ecl pid
              (<= 1920 byr) (<= byr 2002)
              (<= 2010 iyr) (<= iyr 2020)
              (<= 2020 eyr) (<= eyr 2030)
              (let [hgt (tonumber hgt)]
                (match unit
                  :cm (and (<= 150 hgt) (<= hgt 193))
                  :in (and (<= 59 hgt) (<= hgt 76))
                  _ false))
              (= 7 (hcl:len))
              (equals ecl [:amb :blu :brn :gry :grn :hzl :oth])
              (= 9 (pid:len))
            )
        p)
)))

(-> (read-file) (parse) (check-valid1) (length) (print))
(-> (read-file) (parse) (check-valid1) (check-valid2) (length) (print))
