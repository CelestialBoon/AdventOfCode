module Day08
open Helpers
open System.Text.RegularExpressions

let lineRx = Regex @"(\w{3}) = \((\w{3}), (\w{3})\)"

let rec gcd a b = 
    match a, b with
    | (a,0L) -> a
    | (a,b) -> gcd b (a % b)

let mcm a b = a / (gcd a b) * b

let powers = [0;1;2] |> List.map (ipow 26) |> List.toArray
let toInt (s:string) = s.ToCharArray() |> Array.zip powers |> Array.map (fun (a,b) -> a* (int b - int 'A')) |> Array.sum
let match2Int (m:Match) (i:int) = m.Groups.[i].Value |> toInt
let squared26 = ipow 26 2
let destination1 = (ipow 26 3) - 1
let isDestination1 i = i = destination1
let endsInZ i = i <= destination1 && i>= destination1 - squared26

let solvePre (lines:string[]) =
    let directions = lines.[0].ToCharArray() |> Array.map (fun c -> match c with | 'L' -> 0 | 'R' -> 1 | _ -> failwith "Lost our way")
    let paths:(int*int)[] = Array.create (ipow 26 3) (-1,-1)
    lines |> Array.skip 2 |> Array.iter (fun s ->
        let m = lineRx.Match s
        paths.[match2Int m 1] <- (match2Int m 2, match2Int m 3)
    )
    let theAs = [|0..squared26 - 1|] |> Array.filter (fun i -> paths.[i] <> (-1,-1))
    let goAround destCheck sp = 
        let modulo = Array.length directions |> int64
        let mutable arrived = false
        let mutable step = 0L
        let mutable spot = sp
        while not arrived do
            let direction = if directions.[step % modulo |> int] = 0 then fst else snd
            spot <- paths.[spot] |> direction
            step <- step + 1L
            if destCheck spot then arrived <- true
        step
    (goAround, theAs)


let solve1 lines = (solvePre lines |> fst) isDestination1 0
let solve2 lines = 
    let solver, theAs = solvePre lines
    theAs |> Array.map (solver endsInZ) |> Array.reduce mcm

let example1 = [| "LLR" 
; "" 
; "AAA = (BBB, BBB)" 
; "BBB = (AAA, ZZZ)" 
; "ZZZ = (ZZZ, ZZZ)" 
|]

let example2 = [| "LR" 
; "" 
; "AAA = (AAB, XXX)" 
; "AAB = (XXX, AAZ)" 
; "AAZ = (AAB, XXX)" 
; "BBA = (BBB, XXX)" 
; "BBB = (BBC, BBC)" 
; "BBC = (BBZ, BBZ)" 
; "BBZ = (BBB, BBB)" 
; "XXX = (XXX, XXX)" 
|]

let solution = {
    Solve = fun lines -> solve1 lines, solve2 lines
    Check = myAssert (solve1 example1, solve2 example2) (6L,6L)
}
