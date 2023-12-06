module Day05
open Helpers
open System.Text.RegularExpressions

let numberRx = Regex(@"\d+")

type Rule = {Min:int64; Max:int64; Effect:int64; DestMin:int64; DestMax:int64}

let solve (lines:string[]) =
    let takeNumbers (s:string) = seq {for m in numberRx.Matches(s) -> m.Value |> int64 } |> Seq.toList
    let seeds = takeNumbers lines.[0]
    let seedsRanges = seeds |> List.chunkBySize 2 |> Seq.map (fun is -> (is.[0], is.[1] + is.[0] - 1L)) |> Seq.toList
    let findLine line = lines |> Seq.findIndex(fun s -> s = line)
    let indexes = 
        lines |> Array.indexed |> Array.filter (fun (i, s) -> s.Contains("map:")) |> Array.map fst |> Array.append [|lines.Length + 1|] |> Array.sort |> Array.toList
    let makeMap index = seq {for i in indexes.[index]+1 .. indexes.[index+1] - 2 -> takeNumbers lines.[i]} |> Seq.toList  
    let maps = 
        seq {for i in 0..indexes.Length-2 -> makeMap i } |> Seq.toArray

    let toRule (is:int64 list) = {Min = is.[1] ; Max = is.[1] + is.[2] - 1L; Effect = (int64 is.[0]) - (int64 is.[1]); DestMin=is.[0]; DestMax=is.[0] + is.[2] - 1L}
    let findRule (rules:Rule list) (n:int64)     = rules |> List.tryFind (fun r -> r.Min <= n && n <= r.Max)
    let findDestRule (rules:Rule list) (n:int64) = rules |> List.tryFind (fun r -> r.DestMin <= n && n <= r.DestMax)
    let getEffect (rule:Rule option) = rule |> Option.map (fun r -> r.Effect) |> Option.defaultValue 0L
    let findDest (rule:Rule option) n = n + getEffect rule

    let composeRules (r1s:Rule list) (r2s:Rule list) = 
        let newList = System.Collections.Generic.List<Rule>()
        let jumpPoints = List.append (List.collect (fun rule -> [rule.DestMin; rule.DestMax + 1L]) r1s) (List.collect (fun rule -> [rule.Min; rule.Max + 1L]) r2s) |> List.append [0L] |> List.sort
        for (min, max) in List.pairwise jumpPoints |> List.filter (fun (min, max) -> min < max) do
            let rule1 = findDestRule r1s min
            let effect1 = getEffect rule1 
            let currMin = min - effect1
            let currMax = max - 1L - effect1
            let rule2 = findRule r2s min
            let totalEffect = effect1 + getEffect rule2
            if totalEffect <> 0L then 
                newList.Add({Min=currMin; Max=currMax; Effect=totalEffect; DestMin=currMin+totalEffect; DestMax=currMax+totalEffect})
        newList |> Seq.toList
        
    let fullRules = maps |> Array.map(fun map -> map |> List.map toRule) |> Array.reduce composeRules

    let applicableRules = fullRules |> Seq.filter (fun rule -> seedsRanges |> Seq.exists (fun (min, max) -> rule.Min <= max && rule.Max >= min)) |> Seq.toList
    let isInRanges n = seedsRanges |> List.exists (fun (min, max) -> min <= n && n <= max)
    let mins2test = seedsRanges |> List.map fst |> List.append (applicableRules |> List.map (fun r -> r.Min) |> List.filter isInRanges) |> List.sort

    let resolve (rules:Rule list) nums = nums |> List.map (fun n -> findDest (findRule rules n) n) |> List.min
    let s1 = seeds |> resolve fullRules
    let s2 = mins2test |> resolve fullRules 

    s1, s2


let example1 = [| "seeds: 79 14 55 13"
; ""
; "seed-to-soil map:"
; "50 98 2"
; "52 50 48"
; ""
; "soil-to-fertilizer map:"
; "0 15 37"
; "37 52 2"
; "39 0 15"
; ""
; "fertilizer-to-water map:"
; "49 53 8"
; "0 11 42"
; "42 0 7"
; "57 7 4"
; ""
; "water-to-light map:"
; "88 18 7"
; "18 25 70"
; ""
; "light-to-temperature map:"
; "45 77 23"
; "81 45 19"
; "68 64 13"
; ""
; "temperature-to-humidity map:"
; "0 69 1"
; "1 0 69"
; ""
; "humidity-to-location map:"
; "60 56 37"
; "56 93 4"
|] 


let solution = {
    Solve = solve
    Check = myAssert (solve example1) (35L, 46L)
}