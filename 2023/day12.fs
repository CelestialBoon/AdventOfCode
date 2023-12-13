module Day12
open Helpers
open System.Text.RegularExpressions

let solve (lines:string[]) =
    let lineRx = Regex @"([?#\.]+) ((?:\d+,?)+)"
    let dict = [(([1;2],"s"),3L)] |> dict |> System.Collections.Generic.Dictionary
    dict.Clear()

    let noDamageRx = Regex "^[^#]*$"

    let rec placeDamage (map:string) (ds:int list)=
        if dict.ContainsKey ((ds, map)) then dict.[(ds,map)] else
        match ds with
        |[] -> if noDamageRx.IsMatch map then 1L else 0L
        | d :: drest -> 
            let sol = 
                [0..map.Length-d] |> List.map( fun index -> 
                    let rx = sprintf @"^[^#]{%i}[^.]{%i}(?:[^#](.*))?$" index d |> Regex
                    if rx.IsMatch map then 
                        let m = rx.Match map
                        let mapRest = m.Groups.[1].Value.Trim('.')
                        placeDamage mapRest drest
                    else 0L
                ) |> List.sum
            dict.Add((ds,map), sol)
            sol

    let res = lines |> Array.map (fun s -> 
        let m = lineRx.Match s
        let map = m.Groups.[1].Value
        let damageList = m.Groups.[2].Value.Split(',') |> Array.map int |> Array.toList

        let fiveMap = String.concat "?" [map;map;map;map;map]
        let fiveDamage = [0..4] |> List.collect (fun _ -> damageList)

        placeDamage (map.Trim('.')) damageList, placeDamage (fiveMap.Trim('.')) fiveDamage
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
