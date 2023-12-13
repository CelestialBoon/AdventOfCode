module Helpers

type Solution = {Solve: string[] -> (int64 * int64); Check:((unit->unit))}

let myAssert a b = fun () ->
    if a = b then printfn "Checks are good" else System.Console.WriteLine($"assert failed! {a} is not {b}")

let ipow b e = 
    let mutable res = 1
    if e = 0 then 1 else 
        for _ in 1..e do res <- res * b
        res

let splitwhere f arr = 
    let mid = arr |> Array.indexed |> Array.filter (fun (i,e) -> f e) |> Array.map fst
    [|arr.Length|] |> Array.append mid |> Array.append [|-1|] |> Array.pairwise |> Array.map (fun (i1, i2) -> arr |> Array.skip (i1+1) |> Array.take (i2-i1-1))

let transposestrings (ss:string[]) =
    Array.transpose (ss |> Array.map (fun s -> s.ToCharArray())) |> Array.map (fun cs -> new string(cs))