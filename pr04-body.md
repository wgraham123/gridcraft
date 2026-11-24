BESSELJ, BESSELY, BESSELI and BESSELK return `#VALUE!` for a TRUE or FALSE argument, as Excel does.

Fixes #4040.

**What the user saw.** `=BESSELJ(TRUE,2)` returned 0.1149 and `=BESSELJ(1,Z4)` with FALSE in Z4 returned 0.7652; the same for BESSELY, BESSELI and BESSELK, in either argument, typed or from a cell. Excel returns `#VALUE!`.

**Cause.** `bessel_args` read x and n with `num`, which converts a boolean to 1 or 0.

**What changed.** `bessel_args` reads both arguments through a small `bessel_num` that returns `#VALUE!` for a boolean and otherwise calls `num` as before. Text, blanks, numbers and errors are read as they were, and x is still read before n, so an error in x still comes first. Only the four Bessel functions use `bessel_args`.

**Test.** `engineering::tests::bessel` gains, for each of the four functions: typed and referenced TRUE and FALSE in x and in n give `#VALUE!`; `#N/A` in x with TRUE in n gives `#N/A`; referenced numbers still give a number. On `main` it fails at the first case (`left: Number(0.1149…)`).

**Verification.**
- `cargo test -p gridcraft-functions`: 184 passed.
- `cargo clippy -p gridcraft-functions --all-targets -- -D warnings`: clean.
- End to end with `gridcraft-cli run` on the issue's setup (Z3=TRUE, Z4=FALSE): all 24 rows of the issue's table give Excel's result (16 × `#VALUE!` for the failing rows, the `ISNUMBER` and `"x"` controls unchanged).
- Not run: `cargo xtask` checks (no layer, asset, string or command changes) and the full workspace suite.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
