module Day11
open Helpers

let solve (lines:string[]) =
    let height, width = lines.Length |> int64, lines.[0].Length |> int64
    let galaxies = lines |> Array.indexed |> Array.collect (fun (y, row) -> row.ToCharArray() |> Array.indexed |> Array.choose (fun (x, c) -> if c = '#' then Some (int64 x, int64 y) else None))
    let missingRows = [0L..height-1L] |> List.filter (fun my -> galaxies |> Array.forall (fun (_,y) -> y <> my))
    let missingColumns = [0L..width-1L] |> List.filter (fun mx -> galaxies |> Array.forall (fun (x,_) -> x <> mx))
    
    let solvei extraDistance =
        let galaxiesC = Array.copy galaxies
        missingRows |> List.rev |> List.iter (fun my -> for (i, (x,y)) in galaxiesC |> Array.indexed |> Array.filter (fun (_, (_, y)) -> y > my) do galaxiesC.[i] <- (x, y+extraDistance))
        missingColumns |> List.rev |> List.iter (fun mx -> for (i, (x,y)) in galaxiesC |> Array.indexed |> Array.filter (fun (_, (x, _)) -> x > mx) do galaxiesC.[i] <- (x+extraDistance, y))
        let allDistinctPairs arr =
            let iarr = Array.indexed arr
            iarr |> Array.allPairs iarr |> Array.choose (fun ((i1, a), (i2,b)) -> if i1 < i2 then Some (a,b) else None )
        galaxiesC |> allDistinctPairs |> Array.map (fun ((x1, y1),(x2,y2)) -> abs(x2-x1) + abs(y2-y1)) |> Array.sum

    solvei 1L, solvei 999999L

let example1 = [| 
  "...#......" 
; ".......#.." 
; "#........." 
; ".........." 
; "......#..." 
; ".#........" 
; ".........#" 
; ".........." 
; ".......#.." 
; "#...#....." 
|]

let solution = {
    Solve = solve
    Check = myAssert (solve example1) (374L,0L)
}
