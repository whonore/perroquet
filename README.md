# perroquet

`perroquet`
- repeats your Coq code back to you with prettier indentation.
  Its goal is to provide a standalone implementation of the IDE-specific
  indentation scripts in [Proof General] and [Coqtail] and tries to mostly copy
  their behaviors except where they fail to produce reasonable results.
- is intended only to be a line-by-line indenter, not a full formatter.
  In particular, it will not split or join lines, or change anything except
  line-leading whitespace.
- tries to make uncontroversial decisions by default, but it does provide some
  [configuration](#configuration) options for cases where there are multiple or
  no generally agreed-upon styles.

## Installation

TODO

## Usage

### Indenting whole files

By default `perroquet` indents the entire file it is given and writes the
changes in place.
Input files can be specified individually or collectively in a directory.
`perroquet` can also read from `stdin` if `-` is the only argument or if no
arguments are given.
In this case the indented output is printed to `stdout`.

```sh
# Format files in place.
perroquet PeqNP.v DecideHalt.v

# Recursively search directories.
perroquet calculus/theories/ amazing-haskell-compiler/

# Exclude files or directories.
perroquet my-huge-project --exclude 'third-party-libs' --exclude 'MyWeirdDSL.v'

# Read from stdin, print to stdout.
perroquet < linear-time-sorting.v
echo -e 'Ltac margin_too_small :=\nadmit.' | perroquet > FermatTactic.v
```

### Indenting specific lines

`perroquet` can also indent only a specified range of lines with the `--from`
and `--upto` options, which represent the lines to start from, and stop at (inclusive).
Either one can be omitted and `perroquet` will default to the beginning or end
of the file.

```sh
# Only indent lines 30-40.
perroquet Goldbach.v --from=30 --upto=40

# Indent everything up to and including line 20.
perroquet Riemann.v --upto=20

# Indent everything on and after line 10.
perroquet Hilbert10.v --from=10
```

### Disabling indentation

Coq has a very complex syntax and especially with user-defined notations it is
likely that `perroquet` will at some point produce undesirable output.
In these situations `perroquet` can be told to skip all lines following a
comment containing the directive `perroquet: off` until it reaches another
comment with `perroquet: on`.
For example, given `TicTacToe.v`

```coq
(* TicTacToe.v *)
Require Import List.
Definition tictactoe :=
list (list nat).
Notation "[t b00 | b01 | b02 --------- b10 | b11 | b12 --------- b20 | b21 | b22 t]" :=
((b00::b01::b02::nil)::(b10::b11::b12::nil)::(b20::b21::b22::nil)::nil).

(* perroquet: off *)
Definition board := [t
  0 | 1 | 1
  ---------
  2 | 1 | 0
  ---------
  0 | 0 | 2
t].
(* perroquet: on *)

Goal length board = 3.
Proof.
reflexivity.
Qed.
```

`perroquet TicTacToe.v` will produce

```coq
(* TicTacToe.v *)
Require Import List.
Definition tictactoe :=
  list (list nat).
Notation "[t b00 | b01 | b02 --------- b10 | b11 | b12 --------- b20 | b21 | b22 t]" :=
  ((b00::b01::b02::nil)::(b10::b11::b12::nil)::(b20::b21::b22::nil)::nil).

(* perroquet: off *)
Definition board := [t
  0 | 1 | 1
  ---------
  2 | 1 | 0
  ---------
  0 | 0 | 2
t].
(* perroquet: on *)

Goal length board = 3.
Proof.
  reflexivity.
Qed.
```

### Checking for correct indentation

`perroquet` can be run in "check" mode, which leaves the inputs unchanged but
indicates whether any would be indented differently.
If all inputs are correctly indented `perroquet` exits with status code 0, and
otherwise exits with 1 and prints the names of the files it would change.

```sh
# Check indentation without modifying.
perroquet --check TwinPrimes.v
```

## IDE Integration

Vim
```
TODO
```

Emacs
```
TODO
```

Visual Studio Code
```
TODO
```

## Configuration

TODO

## Contributing

Pull requests, feature requests, and bug reports are welcome.
Examples of code that `perroquet` handles badly or differently from existing
IDEs (e.g., [Proof General], [Coqtail], etc) are especially helpful.

## Licence

[MIT](https://choosealicense.com/licenses/mit/)

[Proof General]: https://github.com/ProofGeneral/PG
[Coqtail]: https://github.com/whonore/Coqtail
