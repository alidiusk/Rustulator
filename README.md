# Rustulator
A basic scientific calculator implemented in Rust.

[![asciicast](https://asciinema.org/a/Q6DRLAReFxHDhoFD7mEQQ6CoG.svg)](https://asciinema.org/a/Q6DRLAReFxHDhoFD7mEQQ6CoG)

Supports:
  - Arithmetic
  - Sin, Cos, Tan, Arcsin, Arccos, Arctan
  - Log, Ln
  - Floor, Abs
  - Constants: pi, e
  - Variable assignment (ex: `x = 2`)
  - Implicit multiplication (ex: `6(3 - 2)`)
  - Implicit closing parantheses (ex: `9sin(pi/2`)

Supports a calculator repl and a web interface. The web interface frontend is powered by Elm.

To run the calculator repl, run `cargo run -- repl`.
To run the calculator web interface, run `cargo run -- web`.

To build the web interface, `cd` into `elm` and run `elm make src/main.elm --output=../static/index.html`.
To build the rust executable, run `cargo build`.

The actual calculator logic is stored in the `calculator` crate. It is implemented with a custom lexer, parser, and evaluator, and exports a `Calculator` object that maintains an internal state of all previous calculations and the currently defined constants/variables.

### NOTE: The project currently only works on Rust nightly since it uses Rocket and uses language features that have not yet been stabilized.
