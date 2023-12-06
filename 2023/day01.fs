module Day01
open Helpers

let doa (lines:string[]) = 
    let firstNum (s:string) =  s |> Seq.find(System.Char.IsDigit)
    let lastNum (s:string) = s |> Seq.findBack(System.Char.IsDigit)

    lines 
    |> Seq.map (fun l -> $"{firstNum l}{lastNum l}" |> int)
    |> Seq.sum

open System.Text.RegularExpressions
let dob (lines:string[]) =
    let rx = Regex("([0-9]|one|two|three|four|five|six|seven|eight|nine)")
    let toNum (s:string) = 
        match s with
        | "one" -> 1
        | "two" -> 2
        | "three" -> 3
        | "four" -> 4
        | "five" -> 5
        | "six" -> 6
        | "seven" -> 7
        | "eight" -> 8
        | "nine" -> 9
        | _ -> int s
    let firstNum (s:string) = rx.Match(s).Value |> toNum
    let lastNum (s:string) = seq { 1.. s.Length } |> Seq.pick (fun i -> 
        let m = rx.Match <| s.Substring(s.Length - i)
        if m.Success then m.Value |> toNum |> Some else None
    )
    lines |> Seq.map (fun s -> $"{firstNum s}{lastNum s}" |> int) |> Seq.sum

let example1 = List.toArray [ "1abc2"
; "pqr3stu8vwx"
; "a1b2c3d4e5f"
; "treb7uchet"
] 
 
let example2 = List.toArray [ "two1nine"
; "eightwothree"
; "abcone2threexyz"
; "xtwone3four"
; "4nineeightseven2"
; "zoneight234"
; "7pqrstsixteen"
]

let solution = {
    Solve = (fun lines -> (doa lines |> int64, dob lines |> int64)) 
    Check = myAssert ((doa example1), (dob example2)) (142, 281)
    }