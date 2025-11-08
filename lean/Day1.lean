import Std.Data.HashMap
open Std (HashMap)


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

def parseLine (s : String): Option (Nat × Nat) :=
  let isNonZero (str : String) := str.length > 0
  match List.mapM String.toNat? (List.filter isNonZero s.splitOn) with
    | some [x, y] => some (x, y)
    | _ => none

def unzip (ps: List (α × β)): List α × List β :=
  match ps with
  | List.nil => ([], [])
  | (x, y)::qs =>
    let (xs, ys) := unzip qs
    (x::xs, y::ys)

def keepSome: List (Option α) → List α
  | [] => []
  | (some x)::xs => x::keepSome xs
  | none::xs => keepSome xs

def abs (x: Nat) (y: Nat): Nat := if x > y then x - y else y - x

def part1 (xs: List Nat) (ys: List Nat): Nat :=
  let rec sumDists : Nat → List Nat → List Nat → Nat
    | acc, u::us, v::vs => sumDists (acc + abs u v) us vs
    | acc, [], [] => acc
    | acc, [], v::vs => acc
    | acc, u::us, [] => acc
  sumDists 0 xs.mergeSort ys.mergeSort

def createCounts (xs : List Nat) : HashMap Nat Nat :=
  let incr: Option Nat → Option Nat
    | none => some 1
    | some n => some (n + 1)
  xs.foldl (fun counts x => counts.alter x incr) HashMap.emptyWithCapacity

def part2 (xs : List Nat) (ys : List Nat): Nat :=
  let counts := createCounts ys
  xs.foldl (fun total x => total + x * counts.getD x 0) 0

def main (args : List String) : IO Unit := do
  match args with
  | [] => pure ()
  | path::_ => do
    match (← openFile path) with
      | none => pure ()
      | some stream => do
        let tuples ← processLines stream parseLine
        let (xs, ys) := unzip (keepSome tuples)
        let stdout ← IO.getStdout
        stdout.putStrLn s!"Part 1: {part1 xs ys}"
        stdout.putStrLn s!"Part 2: {part2 xs ys}"
