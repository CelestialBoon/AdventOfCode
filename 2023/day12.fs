module Day12
open Helpers

type Spot = Operational | Damaged | Unknown 

let lineRx = System.Text.RegularExpressions.Regex @"([?#\.]+) ((?:\d+,?)+)"

let char2State c = match c with | '.' -> Operational | '#' -> Damaged | '?' -> Unknown | _ -> failwith "parsing failure" 

let isPossible (uncertainMap:Spot[]) (certainMap:Spot[]) = 
    if certainMap.Length > uncertainMap.Length then 
        failwith "oops" 
    else
        Array.indexed certainMap |> Array.forall (fun (i, s) -> uncertainMap.[i] = Unknown || uncertainMap.[i] = s)

//Array.zip uncertainMap certainMap |> Array.forall (fun (s1, s2) -> s1 = Unknown || s1 = s2)

let repeat (list:'T[]) n inbetween = 
    let res = System.Collections.Generic.List<'T> ()
    res.AddRange list
    for _ in 1..n-1 do
        match inbetween with | None -> () | Some e -> res.Add e
        res.AddRange list
    res |> Seq.toArray
    

let gaps len uncertainMap (damagedList:int[]) =
    let n = damagedList.Length
    let rec nextBatch (res:Spot[]) i prevGaps plusi =
        if i = n then
            [res]
        else
            let startIndex = res.Length
            [plusi..len-prevGaps+i-n+1] |> List.collect (fun gap ->
                let newRes = Array.create (startIndex + gap + damagedList.[i]) Unknown 
                Array.blit res 0 newRes 0 startIndex
                [0..gap-1] |> List.iter (fun j -> newRes.[startIndex+j] <- Operational)
                [0..damagedList.[i]-1] |> List.iter(fun j -> newRes.[startIndex+gap+j] <- Damaged)
                if isPossible uncertainMap newRes then nextBatch newRes (i+1) (prevGaps+gap) 1 else []
                //Some newRes |> Option.filter isItPossible |> Option.map (fun sp -> nextBatch [sp] 1 1)
            )
    nextBatch [||] 0 0 0

let solve (lines:string[]) =
    let res = lines |> Array.map (fun s -> 
        let m = lineRx.Match s
        let uncertainSpringsMap = m.Groups.[1].Value.ToCharArray() |> Array.map char2State
        let damagedList = m.Groups.[2].Value.Split(',') |> Array.map int

        let uncertainSpringsMapRepeated = repeat uncertainSpringsMap 5 (Some Unknown)
        let damagedListRepeated = repeat damagedList 5 None

        let solvei groupList (map:Spot[]) = 
            let rowLength = map.Length
            let groupslen = Array.sum groupList

            let gapslist = gaps (rowLength - groupslen) map groupList
            gapslist.Length
            //let s = 
            //    gapslist |> List.map (fun gaps ->
            //        let possibleConfig = Array.create rowLength Operational
            //        let mutable spot = 0
            //        Array.zip gaps groupList |> Array.iter (fun (gap,dam) -> 
            //            for i in spot+gap..spot+gap+dam-1 do possibleConfig.[i] <- Damaged
            //            spot <- spot+gap+dam
            //        )
            //        possibleConfig
            //    ) |> List.filter (isPossible map) |> List.length
            //s
        printfn "Doing line %s" s
        solvei damagedList uncertainSpringsMap |> int64, solvei damagedListRepeated uncertainSpringsMapRepeated |> int64
    ) 
    let s1, s2 = res |> Array.unzip
    s1 |> Array.sum, s2 |> Array.sum




let example1 = [| 
  "???.### 1,1,3" 
; ".??..??...?##. 1,1,3" 
; "?#?#?#?#?#?#?#? 1,3,1,6" 
; "????.#...#... 4,1,1" 
; "????.######..#####. 1,6,5" 
; "?###???????? 3,2,1" 
|]

let solution = {
    Solve = solve
    Check = myAssert (solve example1) (21L,525152L)
}
