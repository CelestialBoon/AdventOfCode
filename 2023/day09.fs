module Day09
open Helpers
let numberRx = System.Text.RegularExpressions.Regex @"-?\d+"

let solve (lines:string[]) =
    let nums = lines |> Array.map (fun s -> seq { for m in numberRx.Matches s -> m.Value |> int64} |> Seq.toList)

    let rec sideNums (ns:int64 list) = 
        let first, last = List.item 0 ns, List.last ns
        let diffs = ns |> List.pairwise |> List.map (fun (n1, n2) -> n2 - n1)
        if diffs |> List.forall ((=) 0L) then (first, last) else 
            let before, after = sideNums diffs
            (first - before, last + after)

    let sides = nums |> Array.map sideNums
    sides |> Array.map snd |> Array.sum, sides |> Array.map fst |> Array.sum
    
let example1 = [| "0 3 6 9 12 15" 
; "1 3 6 10 15 21" 
; "10 13 16 21 30 45" 
|]

let solution = {
    Solve = solve
    Check = myAssert (solve example1) (114L,2L)
}
