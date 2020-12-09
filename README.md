These are my solutions to [Advent of Code
2020](https://adventofcode.com/2020/), using Rust.

`ci.sh` compiles and runs all solutions, validating that the results (still)
match the confirmed correct answers for my inputs, which are in `inputs`.

In some of the earlier days I had some additional self-imposed goals, for practice:

* When feasible, process input a line/chunk at a time rather than reading the
  entire input into memory. This is overkill for the input sizes given, but is
  useful for handling large inputs, setting up shell or distributed pipelines,
  etc.

* Trying to avoid panic-ing in the core logic, and instead bubbling up `Result`
  types to `main`, where I finally do just unwrap them. This adds a bit of
extra complexity, but means that in principle the library code could be called
from environments where we *wouldn't* want to panic, without having to be
rewritten. Again this is overkill for advent-of-code itself, but I'm using
advent-of-code to practice the relevant techniques.

In later days (after 6 or so) I abandoned both for expediency and succinctness :).
