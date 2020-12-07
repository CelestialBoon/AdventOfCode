; it has to contain that number of chars
; which means it's an exam at the char level
; each line they are given a min, a max, and a char, and there's to examine the chars int there
; so how do you check for chars in lua?
; string to chars, and then compare the two chars (ints)
(local lume (require :lume))
; (local view (require :fennelview))

(fn xor [a b]
  (and (or a b) (not (and a b)))
)

(fn read-input [filename]
  (with-open [file (assert (io.open filename))]
    (file:read :*a)))

(fn strtochars [str] 
  (let [result []]
    (for [i 1 (length str)]
      (table.insert result (string.byte str i))
    )
    result
  )
)

; "1-4 n: nnnnn"
(fn decodeline [line]
  (local (min max letter passw) (string.match line "^(%d+)-(%d+) (%l): (%l+)$"))
  (values (tonumber min) (tonumber max) (string.byte letter) (strtochars passw))
)

(fn checkline1 [min max num nums]
  (local cnt (lume.count nums (fn c [n] (= n num))))
  (and (<= cnt max) (>= cnt min))
)

(fn checkline2 [first second num nums]
  (let [{first num1 second num2} nums]
    (xor (= num1 num) (= num2 num))
  )
)

(fn solution [solver]
  (var count 0)
  (each [entry (string.gmatch (read-input "inputs/i02.txt") "[^\n]+")]
    (when (solver (decodeline entry))
      (set count (+ count 1))
    )
  )
  (print count)
)

(solution checkline1)
(solution checkline2)