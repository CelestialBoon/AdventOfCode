module Day15
open Helpers
open System.Text.RegularExpressions

let solve (lines:string[]) =
 let pieces = lines.[0].Split(',');
 let toHash (s:string) = s.ToCharArray() |> Array.fold (fun n c -> ((n+ int c) * 17) % 256 ) 0
 let s1 = pieces |> Array.map toHash |> Array.sum |> int64
 let rx = Regex @"^(.*)(-|=(\d))$"
 let (boxes:ResizeArray<(string*int)>[]) = Array.zeroCreate 256
 for i in [0..255] do boxes.[i] <- new ResizeArray<string*int>()
 let lenses = pieces |> Array.iter (fun s -> 
  let m = rx.Match s
  let name = m.Groups.[1].Value
  let box = boxes.[toHash name]
  match m.Groups.[2].Value with
   | "-" -> Seq.tryFindIndex (fun (theName, str) -> theName = name) box |> Option.iter (fun i -> box.RemoveAt i)
   | _ -> 
    let strength = m.Groups.[3].Value |> int
    match Seq.tryFindIndex (fun (theName, str) -> theName = name) box with
    | Some i -> box.[i] <- (name, strength)
    | None -> box.Add((name, strength))
 )
 let s2 = boxes |> Array.indexed |> Array.map (fun (i, box) -> (i+1) * (Seq.indexed box |> Seq.map (fun (i, (s,str)) -> (i+1)*str) |> Seq.sum)) |> Array.sum |> int64

 s1, s2

let example1 = [| "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"|]

let solution = {
 Solve = solve
 Check = myAssert (solve example1) (1320L,145L)
}
