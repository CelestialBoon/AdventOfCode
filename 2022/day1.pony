use "files"

actor Main
  new create(env: Env) =>
    let file_name = "input/i01.txt"
    let path = FilePath(FileAuth(env.root), file_name)
    match OpenFile(path)
      | let file: File =>
        while file.errno() is FileOK do
          for line in file.lines() do
            env.out.print(consume line)
          end
        end
      else
        env.err.print("Error opening file '" + file_name + "'")
      end
//read file
//parse sets
//parse numbers
//sum numbers
//find max