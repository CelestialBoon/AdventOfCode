module Day02
open Helpers

open System.Text.RegularExpressions

let gameRx = Regex(@"Game (\d+):")
let matchRx = Regex(@"((\d+) (red|green|blue),?)+")

type Draw = {Red: int; Blue:int; Green:int}
let baseDraw = {Red = 0; Blue = 0; Green = 0}

let cubes (d:Draw) (m:Match) = 
    let num = m.Groups.[2].Value |> int
    match m.Groups.[3].Value with
        | "red" -> {d with Red = num}
        | "blue" -> {d with Blue = num}
        | "green" -> {d with Green = num}
        | s -> failwith $"{s} is not a color"

let maxDraw (a:Draw) (b:Draw) =
    let mutable result:Draw = a
    if(b.Red > a.Red) then result <- {result with Red = b.Red}
    if(b.Blue > a.Blue) then result <- {result with Blue = b.Blue}
    if(b.Green > a.Green) then result <- {result with Green = b.Green}
    result

let maxDraws (lines:string[]) = lines |> Seq.map(fun s -> 
    let id = gameRx.Match(s).Groups.[1].Value |> int
    let draws = s.Split([|';'|]) |> Seq.map (fun draw -> seq {for m in matchRx.Matches(draw) -> m } |> Seq.fold cubes baseDraw )
    let maxDraw = draws |> Seq.fold maxDraw baseDraw
    id, maxDraw
)

let doa (lines:string[]) = 
    let limitDraw = {Red = 12; Green = 13; Blue = 14}
    maxDraws lines |> Seq.filter(fun (id, draw) -> draw.Red <= limitDraw.Red && draw.Blue <= limitDraw.Blue && draw.Green <= limitDraw.Green) |> Seq.map fst |> Seq.sum


let dob (lines:string[]) =
    maxDraws lines |> Seq.map(fun (id, draw) -> draw.Red * draw.Blue * draw.Green) |> Seq.sum

let example1 = [| "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
; "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
; "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
; "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
; "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
|] 

let solution = {
    Solve = fun lines -> (doa lines |> int64, dob lines |> int64)
    Check = myAssert ((doa example1), (dob example1)) (8, 2286)
}