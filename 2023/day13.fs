module Day13
open Helpers

let triangular n = (n * (n-1)) /2

let fequals (arr:'a[]) i1 i2 = i1 < 0 || i1 >= arr.Length || i2<0 || i2>=arr.Length || arr.[i1]=arr.[i2]

let solve (lines:string[]) =
    let splits = lines |> splitwhere (fun s -> s.Length = 0) 

    let s1 = 
        let input = splits |> Array.map (fun ss -> (ss, transposestrings ss))
        let findReflections arr = arr |> Array.pairwise |> Array.indexed |> Array.filter (fun (i, (r1, r2)) -> r1 = r2) |> Array.map fst |> Array.filter (fun i -> [0..i] |> List.forall (fun j -> fequals arr j (2*i-j+1))) |> Array.toList
        input |> Array.map (fun (rows, cols) ->
            match findReflections rows with
            | i :: _ -> 100 * (i+1)
            | [] -> match findReflections cols with 
                | i :: _ -> (i+1)
                | [] -> failwith "no reflections found"
        ) |> Array.sum

    let s2 = 
        let findDivergence (cs1:char[]) (cs2:char[]) = Array.zip cs1 cs2 |> Array.filter (fun (c1, c2) -> c1 <> c2) |> Array.length
        let fDivergence (arr:char[][]) i1 i2 = if i1 < 0 || i1 >= arr.Length || i2<0 || i2>=arr.Length then 0 else findDivergence arr.[i1] arr.[i2]
        let findSmudgedReflections arr = arr |> Array.pairwise |> Array.indexed |> Array.choose (fun (i, (cs1, cs2)) -> 
            if findDivergence cs1 cs2 <=1 
                && [0..i] |> List.map (fun j -> fDivergence arr j (2*i-j+1)) |> List.sum = 1 then Some i else None) |> Array.toList
        splits |> Array.map (fun ss -> 
            let rows = ss |> Array.map (fun s -> s.ToCharArray())
            let cols = rows |> Array.transpose
            match findSmudgedReflections rows with
            | i :: _ -> 100 * (i+1)
            | [] -> match findSmudgedReflections cols with 
                    | i :: _ -> (i+1)
                    | [] -> failwith "no reflections found"
        ) |> Array.sum        

    int64 s1, int64 s2


let example1 = [| 
  "#.##..##." 
; "..#.##.#." 
; "##......#" 
; "##......#" 
; "..#.##.#." 
; "..##..##." 
; "#.#.##.#." 
; "" 
; "#...##..#" 
; "#....#..#" 
; "..##..###" 
; "#####.##." 
; "#####.##." 
; "..##..###" 
; "#....#..#" 
|]

let solution = {
    Solve = solve
    Check = myAssert (solve example1) (405L,400L)
}
