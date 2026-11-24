CHISQ.INV and GAMMA.INV give Excel's results near probability 1 instead of values about 0.1% off.

Fixes #4018.

**What the user saw.** `=CHISQ.INV(0.999999999999999,2)` returned 69.0 (Excel 69.07915198468176, which is −2·ln(1−p)) and `=GAMMA.INV(0.999999999999999,2,3)` returned 114.75 (Excel 114.62540626645446). Ordinary probabilities matched.

**Cause.** `chi_inv` and `gamma_inv` in `crates/functions/src/stat/distributions.rs` bisect on `reg_gamma_p(…) - p`. Close to 1 the lower CDF rounds to the same double over a wide range of x, so bisection stops at an arbitrary point of that range. The error is already about 2e-12 relative at p = 0.999999.

**What changed.** For p > 0.5 both functions invert the upper tail, `reg_gamma_q(…) = 1 - p`, with the same `invert` bracket; `1 - p` is exact for p ≥ 0.5 and `reg_gamma_q` keeps relative precision in the tail. This is what `chi_inv_rt` (CHISQ.INV.RT) already does, and `chi_inv` now calls it. The lower half is unchanged. CHISQ.INV is the only caller of `chi_inv`; CHIINV is the right-tail form and is not touched.

**Test.** `stat::tests::chi_square_and_gamma_inverses_keep_precision_near_one` checks CHISQ.INV with 2 degrees of freedom against −2·ln(1−p) and GAMMA.INV with shape 1 against −β·ln(1−p) for six probabilities from 0.6 to 0.999999999999999 (relative 1e-13), and the issue's Gamma(2, 3) case against Excel's value. On `main` it fails at p = 0.999999 (`got 27.63102111592889, expected 27.631021115871036`).

**Verification.**
- `cargo test -p gridcraft-functions` (185 passed), `cargo test -p gridcraft-calc` (133 passed, 3 ignored as on `main`).
- `cargo clippy -p gridcraft-functions --all-targets -- -D warnings`: clean.
- `gridcraft-cli eval` on the issue's formulas: 69.07915198468177 and 114.62540626645445 for the failing rows; controls 9.210340371976184 (Excel 9.210340371976182; exact −2·ln 0.01 is 9.210340371976184) and 19.915056203981436 (Excel 19.915056203981422, 7e-16 relative), both within the issue's 1e-10 tolerance.
- Not run: `cargo xtask` checks.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
