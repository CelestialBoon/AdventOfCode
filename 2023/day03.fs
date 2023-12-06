module Day03
open Helpers
open System.Text.RegularExpressions

let numberRx = Regex(@"\d+")
let symbolRx = Regex(@"[^0-9.]")

let theMatches (lines:string[]) = 
    let li = lines |> Seq.indexed
    let numberMatches = li |> Seq.collect (fun (i, s) -> seq { for m in numberRx.Matches(s) -> ((m.Index, i), m)})
    let symbolMatches = li |> Seq.collect (fun (i, s) -> seq { for m in symbolRx.Matches(s) -> ((m.Index, i), m)}) |> dict |> System.Collections.Generic.Dictionary
    numberMatches, symbolMatches

let getSymbol (symbolmatches:System.Collections.Generic.Dictionary<(int*int), Match>) (m:(int*int)*Match) =
    let (x, y) = fst m
    let len = (snd m).Length
    let minX = x-1
    let maxX = x+len
    let possibleCoords = seq {for i in minX..maxX -> (i, y-1)} |> Seq.append [(minX, y); (maxX, y)] |> Seq.append <| seq {for i in minX..maxX -> (i, y+1)} 
    possibleCoords |> Seq.tryFind(fun c -> symbolmatches.ContainsKey(c)) |> Option.map (fun c -> (c, symbolmatches.[c]))


let solve (lines:string[]) = 
    let numbers, symbols = theMatches lines
    let s1 = numbers |> Seq.filter (fun n -> Option.isSome <| getSymbol symbols n) |> Seq.map(fun n -> (snd n).Value |> int) |> Seq.sum
    let s2 = 
        numbers |> Seq.choose (fun n -> getSymbol symbols n |> Option.filter (fun (c,s) -> s.Value = "*") |> Option.map (fun s -> fst s, snd n)) |> Seq.groupBy (fun (c, n) -> c) |> Seq.choose (fun a ->
            let mats = snd a |> Seq.map snd |> Seq.toList
            if mats.Length = 2 then Some (mats |> List.map (fun m -> m.Value |> int) |> List.fold (*) 1) else None
        ) |> Seq.sum
    s1 |> int64, s2 |> int64
    
let example1 = [| "467..114.."
; "...*......"
; "..35..633."
; "......#..."
; "617*......"
; ".....+.58."
; "..592....."
; "......755."
; "...$.*...."
; ".664.598.."
|] 

let solution = {
    Solve = solve
    Check = myAssert (solve example1) (4361L, 467835L)
}