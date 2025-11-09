open IO.FS (Stream Handle)

abbrev Report := List Nat

abbrev ResultT := ExceptT String

def openFile (path : System.FilePath) : ResultT IO Stream := do
  if !(← path.pathExists) then
    throw s!"File not found {path}"
  else do
    let handle ← Handle.mk path IO.FS.Mode.read
    pure (Stream.ofHandle handle)

partial def processLines (parse : String -> Option α) (s : Stream): ResultT IO (List α) := do
  let line ← flip String.dropRight 1 <$> s.getLine
  if line.length == 0 then pure []
  else match parse line with
    | none => throw s!"Failed to parse line {line}"
    | some x => do
      let xs ← processLines parse s
      pure (x::xs)

def parseLine (s : String): Option Report :=
  let isNotEmpty := not ∘ String.isEmpty
  List.mapM String.toNat? (List.filter isNotEmpty s.splitOn)

def main (args: List String): IO UInt32 :=
  match args with
    | [path] => do
      let result ← ExceptT.run (openFile path >>= processLines parseLine)
      let stdout ← IO.getStdout
      match result with
      | Except.error s => do
        stdout.putStrLn s
        pure 2
      | Except.ok reports => do
        reports.forM (stdout.putStrLn ∘ toString)
        pure 0
    | _ => pure 1

