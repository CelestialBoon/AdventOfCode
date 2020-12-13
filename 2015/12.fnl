(local f (with-open [file (assert (io.open "inputs/i12"))]
    (file:read :*a)))

(var tot 0)
(each [m (f:gmatch "%-?%d+")]
  (set tot (+ tot (tonumber m)))
)
(print tot)
; input is already pre-processed to give result 2

; (set tot 0)
; (each [m (string.gmatch (string.gsub 
;     (string.gsub (f:gsub ",\"red\"" "") "%[\"red\",?" "[") "{.-%b{}.-:\"red\".-%b{}.-}" "") "%-?%d+")]
;   (set tot (+ tot (tonumber m)))
; )
; (print tot)
