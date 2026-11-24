NORM.S.INV, NORM.INV and LOGNORM.INV give Excel's results to full precision near probability 1.

Fixes #4022.

**What the user saw.** `=NORM.S.INV(0.999999999999999)` returned 7.941444479789615 (Excel 7.941444487415977) and `=LOGNORM.INV(0.999999999999999,0,1)` 2811.418614037247 (Excel 2811.4186354781427). Lower-tail and central inputs already matched.

**Cause.** `special::norm_inv` refines Acklam's approximation with Halley steps on `norm_cdf(x) - p`. For p close to 1 both terms are close to 1, so the difference keeps only a few significant digits and the refinement cannot correct the starting value.

**What changed.** For p > 0.5, `norm_inv(p)` returns `-norm_inv(1 - p)`. `1 - p` is exact in floating point for p ≥ 0.5, and in the lower tail `norm_cdf` (through `erfc`) keeps its relative precision, so the refinement works. Everything else in the function is untouched. Against Python's `statistics.NormalDist().inv_cdf` (Wichura's AS241) on 411 probabilities (200 uniform, 200 between 1 − 1e-1 and 1 − 1e-15, and the issue's points), the worst relative error drops from 1.1e-9 to 1.8e-11; the remaining worst case is at p = 0.5000001, where the absolute error is below 1e-17. CONFIDENCE.NORM and the FORECAST.ETS confidence interval call the same function and get the same precision.

**Test.** `stat::tests::normal_inverses_keep_precision_near_one`: NORM.S.INV, NORM.INV and LOGNORM.INV at the issue's probability match the AS241 values within 1e-13/1e-12, and NORM.S.INV(p) is exactly −NORM.S.INV(1−p) for five upper probabilities. On `main` it fails with `got 7.941444479789615, expected 7.941444487415977`.

**Verification.**
- `cargo test -p gridcraft-functions` (185 passed), `cargo test -p gridcraft-calc` (133 passed, 3 ignored as on `main`).
- `cargo clippy -p gridcraft-functions --all-targets -- -D warnings`: clean.
- `gridcraft-cli eval` on the issue's seven formulas: 7.941444487415979, 7.941444487415979, 2811.41863547815 for the three failing rows (within the issue's 1e-10 relative tolerance of Excel), and the controls unchanged (0, 2.326347874040841, −7.941345326170997, 1).
- Not run: `cargo xtask` checks; no comparison with Excel itself beyond the issue's figures.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
