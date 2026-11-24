Thirty financial and date functions return `#VALUE!` for TRUE or FALSE, as Excel does, instead of quietly reading them as 1 or 0.

Fixes #4067.

**What the user saw.** `=EFFECT(0.05,TRUE)` returned 0.05, `=PRICE(…,100,TRUE,0)` a price, `=ACCRINTM(TRUE,…)` 6275, and the same with TRUE or FALSE in a referenced cell. Excel returns `#VALUE!` for all thirty functions in the issue's table: ACCRINT, ACCRINTM, AMORDEGRC, AMORLINC, the six COUP* functions, DISC, DOLLARDE, DOLLARFR, DURATION, EFFECT, INTRATE, MDURATION, NOMINAL, ODDFPRICE, ODDFYIELD, ODDLPRICE, ODDLYIELD, PRICE, PRICEDISC, PRICEMAT, RECEIVED, YEARFRAC, YIELD, YIELDDISC and YIELDMAT.

**Cause.** These functions read every argument with `num`, which converts a boolean to 1 or 0 like most worksheet functions. These thirty came to Excel from the Analysis ToolPak and take only numbers and dates.

**What changed.** A new `util::no_logicals(args, flags)` returns `#VALUE!` for a boolean argument, unless its position is a logical flag. The thirty registrations call it before their implementation; ACCRINT passes its `calc_method` (position 7) as a flag, so `ACCRINT(…, FALSE)` works as before. An error to the left of the first boolean is still returned by the function, so error precedence does not change. The check covers every argument of these functions, not only the positions the issue lists, because the issue's wider matrix swept every position and found no exception besides `calc_method`. Functions outside the issue's list (the TVM functions and their `type`, CUMIPMT/CUMPRINC in #4065, XIRR in #4045, the TBILL functions) are untouched.

**Test.** `financial::tests::analysis_toolpak_functions_reject_logicals` takes each of the thirty formulas from the issue and checks that a typed or referenced TRUE or FALSE at that position is `#VALUE!`, that 1 at the same position is not, and that an earlier `#N/A` argument still wins. It also checks that ACCRINT's `calc_method` still takes TRUE and FALSE. On `main` it fails on the first case (`left: Number(54.166…)`, `right: Error(Value)`).

**Verification.**
- `cargo test -p gridcraft-functions` (185 passed), `cargo test -p gridcraft-calc` (133 passed, 3 ignored as on `main`).
- `cargo clippy -p gridcraft-functions --all-targets -- -D warnings`: clean.
- `cargo xtask layers`, `wasm`: ok. `cargo xtask parity`: `docs/parity-checklist.md` unchanged.
- End to end with `gridcraft-cli run`: the issue's ACCRINT, ACCRINTM, COUPNUM, DOLLARDE, EFFECT, YEARFRAC and PRICE rows and its referenced-Boolean rows (Z1=TRUE, Z2=FALSE) give `#VALUE!`; the controls give 54.16666667, 6275 and 0 as in the issue; `=EFFECT({0.05;0.06},{TRUE;4})` spills `#VALUE!` and 0.0614.
- Not run: the full workspace test suite, and no check against Excel itself (the expected values come from the issue's Excel run).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
