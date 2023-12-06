open System

let solutionsList = [Day01.solution; Day02.solution; Day03.solution; Day04.solution; Day05.solution; Day06.solution; Day07.solution; Day08.solution; Day09.solution; Day10.solution; Day11.solution; Day12.solution; Day13.solution; Day14.solution; Day15.solution; Day16.solution; Day17.solution; Day18.solution; Day19.solution; Day20.solution; Day21.solution; Day22.solution; Day23.solution; Day24.solution; Day25.solution ]

[<EntryPoint>]
let main argv =
    //let day = argv.[0]
    let day = "05"
    
    let run day = 
        let daySolution = solutionsList.[(int day) - 1]
        daySolution.Check()
        let inputPath = __SOURCE_DIRECTORY__+ $"/inputs/i{day}.txt"
        let lines = System.IO.File.ReadAllLines(inputPath)
        daySolution.Solve lines
    
    let result = run day
    Console.WriteLine(fst result)
    Console.WriteLine(snd result)

    0