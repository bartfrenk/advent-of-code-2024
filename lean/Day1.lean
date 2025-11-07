

def openFile (path : System.FilePath) : IO (Option IO.FS.Stream) := do
  if !(← path.pathExists) then
    (← IO.getStderr).putStrLn s!"File not found {path}"
    pure none
  else
    let handle ← IO.FS.Handle.mk path IO.FS.Mode.read
    pure (some (IO.FS.Stream.ofHandle handle))


partial def processLines [ToString α] (stream : IO.FS.Stream) (f : String -> α): IO (List α)
  := do
    let line ← stream.getLine
    if line.length > 0 then
      let tail ← processLines stream f
      let parsed := f line.trim
      let stdout ← IO.getStdout
      stdout.putStrLn line
      stdout.putStrLn (toString parsed)
      pure (parsed :: tail)
    else
      pure []

def isNonZero (str : String): Bool := str.length > 0

def parseLine (s : String): Option (Nat × Nat) :=
  let isNonZero (str : String) := str.length > 0
  match List.mapM String.toNat? (List.filter isNonZero (s.dropRight 1).splitOn) with
    | some [x, y] => some (x, y)
    | _ => none

def unzip (ps: List (α × α)): List α × List α :=
  match ps with
  | List.nil => ([], [])
  | (x, y)::qs =>
    let (xs, ys) := unzip qs
    (x::xs, y::ys)

def keepSome: List (Option α) → List α
  | [] => []
  | (some x)::xs => x::keepSome xs
  | none::xs => keepSome xs

def main (args : List String) : IO Unit := do
  match args with
  | [] => pure ()
  | path::_ => do
    match (← openFile path) with
      | none => pure ()
      | some stream => do
        let tuples ← processLines stream parseLine
        let (xs, ys) := unzip (keepSome tuples)
        (← IO.getStdout).putStrLn xs.toString
        (← IO.getStdout).putStrLn ys.toString
        pure ()


