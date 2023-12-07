module Day07
open Helpers
open System.Text.RegularExpressions

let betRx = Regex(@"([AKQJT2-9]{5}) (\d+)")

type Hand = HighCard =0 | OnePair =1 | TwoPair =2 | ThreeKind =3 | FullHouse =4 | FourKind =5 | FiveKind =6

let pairings1 (cs:char[]) = cs |> Seq.groupBy id |> Seq.map (fun (c, seq) -> Seq.length seq) |> Seq.sortDescending |> Seq.toArray
let pairings2 (cs:char[]) =
    let pairings = cs |> Seq.groupBy id |> Seq.map (fun (c, seq) -> c, Seq.length seq) |> Seq.toArray
    let dict = pairings |> dict
    let jokers = if dict.ContainsKey('J') then dict.['J'] else 0
    let res = pairings |> Array.where (fun (c, seq) -> c <> 'J') |> Array.map snd |> Array.sortDescending
    if Array.isEmpty res then [|5|] else res.[0] <- res.[0] + jokers; res
    
let judgeHand (ps:int[]) =
    match ps.[0] with
        | 5 -> Hand.FiveKind
        | 4 -> Hand.FourKind
        | 3 -> if ps.[1] = 2 then Hand.FullHouse else Hand.ThreeKind
        | 2 -> if ps.[1] = 2 then Hand.TwoPair else Hand.OnePair
        | 1 -> Hand.HighCard
        | _ -> failwith "Something has gone deeply wrong"

let cardValue1 (c:char) = match c with 
    | 'A' -> 12
    | 'K' -> 11
    | 'Q' -> 10
    | 'J' -> 9
    | 'T' -> 8
    | _ -> int c - int '2'

let cardValue2 (c:char) = match c with 
    | 'A' -> 12
    | 'K' -> 11
    | 'Q' -> 10
    | 'J' -> 0
    | 'T' -> 9
    | _ -> int c - int '1'

let cardValues (valuer:char->int) (cs:char[]) = cs |> Array.map valuer

let powers = seq {for i in 0..5 -> ipow 13 i} |> Seq.toArray

let project (h:Hand, cs:int[]) = [|int h|] |> Array.append (Array.rev cs) |> Array.zip powers |> Array.map (fun (a, b) -> a*b) |> Array.sum

let solve (lines:string[]) =
    let bets =
        lines |> Array.map (fun s -> 
            let m = betRx.Match(s)
            let cs = m.Groups.[1].Value.ToCharArray()
            cs, m.Groups.[2].Value |> int
        )
    let solver pairings cardValuer = 
        bets 
        |> Array.map (fun (cs, bet) -> project (judgeHand (pairings cs), cardValues cardValuer cs), bet)
        |> Seq.sortBy fst |> Seq.indexed |> Seq.map (fun (i, (_, bet)) -> bet * (i+1)) |> Seq.sum
    let s1 = solver pairings1 cardValue1
    let s2 = solver pairings2 cardValue2
        
    s1 |> int64, s2 |> int64
    

let example1 = [| "32T3K 765" 
; "T55J5 684" 
; "KK677 28" 
; "KTJJT 220" 
; "QQQJA 483" 
|]

let solution = {
    Solve = solve
    Check = myAssert (solve example1) (6440L,5905L)
}
