T.INV.2T gives Excel's tiny quantiles for probabilities near 1 instead of stopping at 1.49e-8; T.INV, TINV and CONFIDENCE.T gain the same precision near their centre.

Fixes #4023.

**What the user saw.** `=T.INV.2T(0.999999999999999,2)` returned 1.4901161193847663e-08; Excel returns 1.4130832128154019e-15, which is the closed form √2·(1−p)/√(p(2−p)). `=T.INV.2T(0.999999999999,2)` returned the same 1.49e-8.

**Cause.** `t_inv_upper(q, v)` in `crates/functions/src/stat/distributions.rs` bisects on `t_upper(t) - q`. For t near 0, `t_upper(t)` is 0.5 minus a quantity far below 0.5's ulp, so every t under about √ε ≈ 1.49e-8 gives the same value and the bisection stops there.

**What changed.** For q > 0.25 (t below the quartile), `t_inv_upper` solves the central probability instead: P(|T| < t) = I_(t²/(v+t²))(½, v/2) = 1 − 2q. For q ≥ 0.25, 1 − 2q is exact, and the incomplete beta is in its accurate lower tail. q ≤ 0.25 keeps the existing tail form. `t_inv_upper` serves T.INV, T.INV.2T, TINV and CONFIDENCE.T, so all four get the precision; their outer handling is unchanged.

**Test.** `stat::tests::t_inverses_keep_precision_near_the_centre` checks T.INV.2T for 1 and 2 degrees of freedom against tan(π(1−p)/2) and √2·(1−p)/√(p(2−p)) for seven probabilities from 0.51 to 0.999999999999999, within 1e-13 *relative* error (quantiles go down to 1.4e-15, so an absolute tolerance would check nothing there). It also checks T.INV at the matching one-tailed probability q, against the closed form at 2·(1−q), the probability the rounded q stands for. On `main` it fails at p = 0.99 (1.4e-13 relative error) and, further on, at p = 1 − 1e-9 (`1.4901161193847663e-8` for `1.414213522376404e-9`).

**Verification.**
- `cargo test -p gridcraft-functions` (185 passed), `cargo test -p gridcraft-calc` (133 passed, 3 ignored as on `main`).
- `cargo clippy -p gridcraft-functions --all-targets -- -D warnings`: clean.
- `gridcraft-cli eval` on the issue's rows: 1.4130832128153967e-15 and 1.4141822775364743e-12 for the failing ones (Excel 1.4130832128154019e-15 and 1.4141822775364765e-12, within the issue's 1e-10 tolerance); controls 0.014142842783549568 (Excel …549572; before this change …551553), 0.8164965809277251 (unchanged; p = 0.5 is outside the new branch), 0.
- Not run: `cargo xtask` checks.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
