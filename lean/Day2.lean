
def main : IO Unit := do
  (← IO.getStdout).putStrLn "Hello Day2"

def openFile (path : System.FilePath) : IO (Option IO.FS.Stream) := do
  if !(← path.pathExists) then
    (← IO.getStderr).putStrLn s!"File not found {path}"
    pure none
  else
    let handle ← IO.FS.Handle.mk path IO.FS.Mode.read
    pure (some (IO.FS.Stream.ofHandle handle))

partial def processLines (stream : IO.FS.Stream) (f : String -> α): IO (List α)
  := do
    let line ← stream.getLine
    if line.length > 0 then
      let tail ← processLines stream f
      pure (f (line.dropRight 1) :: tail) -- Drop the final linebreak
    else
      pure []

def parseLine (s : String): Option (List Nat) :=
  let isNonZero (str: String) := str.length > 0
  List.mapM String.toNat? (List.filter isNonZero s.splitOn)

def readFile (path : System.FilePath): IO (Option (List Nat)) := do
  let stream' ← openFile path
  match stream' with
  | none => pure none
  | some stream => processLines stream parseLine
