module Day14
open Helpers

let solve (theLines:string[]) =
 let lines = theLines |> Array.map (fun s -> s.ToCharArray())
 let get x y = lines.[y].[x]
 let set x y v = lines.[y].[x] <- v
 
 //let printgrid () = 
 // lines |> Array.iter (fun cs -> printfn "%s" (new string(cs)))
 // printfn "------------------"

 let height = lines.Length
 let width = lines.[0].Length

 let q = System.Collections.Generic.Queue<int>()
 let roll getter setter length times () = //here we sort west
  for y in [0..times-1] do 
   q.Clear()
   for x in [0..length-1] do
   match getter x y with
   | '.' -> q.Enqueue x
   | 'O' -> 
    if q.Count > 0 then
     let dotx = q.Dequeue()
     setter dotx y 'O'; setter x y '.'
     q.Enqueue x
   | _ -> q.Clear()

 let rollWest = roll get set width height
 let rollEast = roll (fun x y -> get (width - x - 1) y) (fun x y c -> set (width - x - 1) y c) width height
 let rollNorth = roll (fun x y -> get y x) (fun x y c -> set y x c) height width
 let rollSouth = roll (fun x y -> get y (width - x - 1)) (fun x y c -> set y (width - x - 1) c) height width

 let calculateLoad () = lines |> Array.indexed |> Array.map (fun (i,cs) -> (height - i) * (cs |> Array.filter ((=) 'O') |> Array.length)) |> Array.sum

 let copyLines () = lines |> Array.map (fun cs -> Array.copy cs)
 rollNorth ()
 let s1 = calculateLoad ()
 rollWest ()
 rollSouth ()
 rollEast ()
 let prevList = ResizeArray<_> [| copyLines() |]
 let mutable i = 1L
 let cycles = 1000000000L
 while i < cycles do
  i <- 1L + i
  rollNorth ()
  rollWest ()
  rollSouth ()
  rollEast ()
  prevList |> Seq.tryFindIndex (fun a -> a = lines) |> Option.iter (fun pi -> 
   let period = i - (int64 (pi+1))
   i <- i + ((cycles-i)/period)*period
   prevList.Clear()
  )
  prevList.Add(copyLines ())

 int64 s1, int64 <| calculateLoad ()

let example1 = [| 
 "O....#...." 
; "O.OO#....#" 
; ".....##..." 
; "OO.#O....O" 
; ".O.....O#." 
; "O.#..O.#.#" 
; "..O..#O..O" 
; ".......O.." 
; "#....###.." 
; "#OO..#...." 
|]

let solution = {
    Solve = solve
    Check = myAssert (solve example1) (136L,64L)
}
