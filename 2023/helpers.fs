module Helpers

type Solution = {Solve: string[] -> (int64 * int64); Check:((unit->unit))}

let myAssert a b = fun () ->
    if a = b then printfn "Checks are good" else System.Console.WriteLine($"assert failed! {a} is not {b}")

let ipow b e = 
    let mutable res = 1
    if e = 0 then 1 else 
        for _ in 1..e do res <- res * b
        res