module Day04
open Helpers
open System.Text.RegularExpressions
open System.Linq

let numberRx = Regex(@"\d+")

let getNumbers (s:string) = seq {for m in numberRx.Matches(s) -> m.Value |> int} |> set
let getSides (s:string) = 
    let parts1 = s.Split([|'|'|])
    let parts2 = parts1.[0].Split([|':'|])
    numberRx.Match(parts2.[0]).Value |> int, getNumbers parts2.[1], getNumbers parts1.[1]

let solve (lines:string[]) = 
    let cards = lines |> Seq.map getSides |> Seq.toArray
    let wins = cards |> Seq.map (fun (i, ws, ns) -> ws.Intersect(ns) |> Seq.length) |> Seq.toArray
    let s1 = wins |> Seq.choose (fun n -> if n = 0 then None else Some <| ipow 2 (n-1)) |> Seq.sum
    //let dict = cards |> Seq.map (fun (i, ws, ns) -> i, (ws, ns)) |> dict
    for i in cards.Length-1 .. -1 .. 0 do
        let baseWins = wins.[i]
        for j in i+1 .. i+baseWins do wins.[i] <- wins.[i] + wins.[j]
    s1 |> int64, (wins.Length + (wins |> Seq.sum)) |> int64
    
    
let example1 = [| "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
; "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"
; "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"
; "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"
; "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"
; "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
|] 


let solution = {
    Solve = solve
    Check = myAssert (solve example1) (13L, 30L)
}












