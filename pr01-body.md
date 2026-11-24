PERCENTRANK, PERCENTRANK.INC and PERCENTRANK.EXC give Excel's results when the data set comes from cells holding text, booleans or nothing.

Fixes #4041.

**What the user saw.** With `'2` in Z2, `=PERCENTRANK(Z2,2)` returned 1 (Excel: `#N/A`). A cell holding `=""` or `'x` gave `#VALUE!`, a blank cell or a range with no numbers (`Z2:Z4`, `Z3:Z6`) gave `#NUM!`; Excel returns `#N/A` for all of them. The same held for `.INC` and `.EXC`.

**Cause.** `percentrank_impl` read the data set with `array_numbers`, which coerces a scalar argument whether it was typed or came from a single-cell reference, and it returned `#NUM!` for an empty data set.

**What changed.** The data set is read with `util::numbers`, which already applies Excel's rule (inside a reference only numbers count; a typed scalar is converted), and a data set without numbers is `#N/A`. A significance below 1 is still checked first and stays `#NUM!`. Only `percentrank_impl` changes; `arg_nums`, which other functions share, is untouched, so nothing else moves.

**Test.** `stat::tests::percentrank_takes_only_numbers_from_a_reference` checks, for all three functions: a referenced `"2"`, `""`, `"x"`, blank or TRUE is `#N/A`; a referenced range without numbers is `#N/A`; a range mixing numbers with text and blanks ranks as the numbers alone; an error in the data still propagates; a typed `"2"` still converts. It fails on `main` (`left: Number(1.0)`, `right: Error(NA)`).

**Verification.**
- `cargo test -p gridcraft-functions`: 185 passed.
- `cargo clippy -p gridcraft-functions --all-targets -- -D warnings`: clean.
- `cargo xtask layers`, `assets`, `wasm`, `i18n-report`: ok. `cargo xtask parity`: `docs/parity-checklist.md` unchanged.
- End to end with `gridcraft-cli run` on the issue's setup (Z1:Z12): every failing row of the issue's table now gives `#N/A`; the controls stay `0.5`, `0.75` and `0.625`; `=PERCENTRANK("2",2)` stays 1.
- Not run: the full workspace test suite, and no comparison against Excel itself (the expected values come from the issue's Excel run).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
