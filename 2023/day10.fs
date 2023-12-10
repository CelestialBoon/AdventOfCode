module Day10
open Helpers

let solve (lines:string[]) =
    let startPosition = lines |> Array.indexed |> Array.choose (fun (i,s) -> if s.Contains('S') then Some (s.IndexOf('S'), i) else None) |> Array.item 0

    let width, height = lines.[0].Length, lines.Length

    let isValid (x,y) = x>=0 && x < width && y>=0 && y<height
    let pointAt (x,y) = if isValid (x,y) then Some lines.[y].[x] else None
    let neighbors (x,y) = [(x-1,y); (x+1,y); (x,y-1); (x,y+1)] |> List.filter isValid

    let getFollowingPipe (pprev:int*int) (p:int*int) = 
        let x, y = p
        pointAt p |> Option.bind(fun c ->   
            match c with
            | 'L' -> Some ((x+1,y),(x,y-1))
            | 'F' -> Some ((x+1,y),(x,y+1))
            | 'J' -> Some ((x-1,y),(x,y-1))
            | '7' -> Some ((x-1,y),(x,y+1))
            | '|' -> Some ((x,y-1),(x,y+1))
            | '-' -> Some ((x-1,y),(x+1,y))
            | _ -> None) |> Option.bind (fun (d1, d2) -> if pprev = d1 then Some d2 elif pprev = d2 then Some d1 else None )
        
    let shoelace p1 p2 =
        let x1, y1 = p1
        let x2, y2 = p2
        x2 * y1 - x1 * y2

    let rec walk pprev p steps area = 
        match getFollowingPipe pprev p with 
            | Some pnext ->  walk p pnext (steps+1) (area + shoelace p pnext)
            | None -> steps, area // we back at the start
    let firstPipe = neighbors startPosition |> Seq.choose (fun pipe -> 
        getFollowingPipe startPosition pipe |> Option.map (fun _ -> pipe)) |> Seq.item 0
    let loopSteps, areaDoubled = walk startPosition firstPipe 1 (shoelace startPosition firstPipe)

    let s1 = loopSteps / 2 
    let s2 = (areaDoubled - loopSteps) / 2 + 1

    int64 s1, int64 s2

let example1 = [| 
 "FF7FSF7F7F7F7F7F---7" 
; "L|LJ||||||||||||F--J" 
; "FL-7LJLJ||||||LJL-77" 
; "F--JF--7||LJLJ7F7FJ-" 
; "L---JF-JLJ.||-FJLJJ7" 
; "|F|F-JF---7F7-L7L|7|" 
; "|FFJF7L7F-JF7|JL---7" 
; "7-L-JL7||F7|L7F-7F7|" 
; "L.L7LFJ|||||FJL7||LJ" 
; "L7JLJL-JLJLJL--JLJ.L"  
|]

let solution = {
    Solve = solve
    Check = myAssert (solve example1) (80L,10L)
}
