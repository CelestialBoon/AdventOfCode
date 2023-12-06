module Day06
open Helpers
open System.Text.RegularExpressions

let numberRx = Regex(@"\d+")

let beat (time:int64, distance:int64) = seq {for i in 1L..(time-1L) -> i * (time-i)} |> Seq.filter (fun d -> d > distance) |> Seq.length |> int64

let solve (lines:string[]) =
    let times = seq { for m in numberRx.Matches(lines.[0]) -> m.Value |> int64 } |> Seq.toList
    let distances = seq { for m in numberRx.Matches(lines.[1]) -> m.Value |> int64 } |> Seq.toList
    let races = List.zip times distances
    let s1 =  races |> List.map beat |> List.reduce (*)

    let time = numberRx.Match(lines.[0].Replace(" ", "")).Value |> int64
    let distance = numberRx.Match(lines.[1].Replace(" ", "")).Value |> int64
    let s2 = beat (time, distance)

    s1 , s2

let example1 = [| "Time:      7  15   30" 
    ; "Distance:  9  40  200" 
|]

let solution = {
    Solve = solve
    Check = myAssert (solve example1) (288L,71503L)
}
