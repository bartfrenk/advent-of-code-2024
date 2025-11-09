open IO.FS (Stream Handle Mode)

abbrev Report := List Nat

abbrev ResultT := ExceptT String

def openFile (path : System.FilePath) : ResultT IO Stream := do
  if !(← path.pathExists) then
    throw s!"File not found {path}"
  else do
    let handle ← Handle.mk path Mode.read
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

def isSafe (report : Report) : Bool :=
  let rec boundedIncreasing: Report → Bool
    | [] => true
    | [_] => true
    | x1::x2::xs => x1 < x2 && x2 <= x1 + 3 && boundedIncreasing (x2::xs)
  boundedIncreasing report || boundedIncreasing (List.reverse report)

def part1 (reports : List Report): Nat :=
  List.length (List.filter isSafe reports)

def subReports (report : Report): List Report :=
  skip report <$> List.range (report.length)
  where
    skip (report : Report) (i : Nat): List Nat :=
      List.append (report.take i) (report.drop (i + 1))

def part2 (reports : List Report): Nat :=
  List.length (List.filter (flip List.any isSafe ∘ subReports) reports)

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
        stdout.putStrLn s!"Part1: {part1 reports}"
        stdout.putStrLn s!"Part2: {part2 reports}"
        pure 0
    | _ => pure 1

