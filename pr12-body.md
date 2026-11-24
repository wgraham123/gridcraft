BETA.INV gives Excel's result near probability 1.

Fixes #4031.

**What the user saw.** `=BETA.INV(0.999999999999999,2,2)` returned 0.9999999813735485; Excel returns 0.9999999817498791. The lower tail and ordinary probabilities matched.

**Cause.** `beta_inv` in `crates/functions/src/stat/distributions.rs` bisects on `reg_inc_beta(x, α, β) - p`. Close to 1 the CDF rounds to the same double over a wide range of x, so the bisection ends at an arbitrary point in it.

**What changed.** For p > 0.5 the quantile comes from the reflection I_x(α, β) = 1 − I_(1−x)(β, α): `1 - invert(|y| reg_inc_beta(y, β, α), 1 - p, …)`. `1 - p` is exact for p ≥ 0.5 and the reflected CDF is in its accurate lower tail. p ≤ 0.5, the endpoints and the A/B scaling are unchanged. This is the same approach as the separate fixes for #4022 (NORM.S.INV) and #4018 (CHISQ.INV, GAMMA.INV); the code paths are separate, so this PR does not depend on them.

**Test.** `stat::tests::beta_inverse_keeps_precision_near_one` checks Beta(1, 3) against its closed form 1 − (1 − p)^(1/3) for six probabilities from 0.6 to 0.999999999999999 (within 1e-15), and the issue's Beta(2, 2) case against Excel. On `main` it fails at p = 0.999999 (`got 0.9899999999997817, expected 0.9899999999999042`).

**Verification.**
- `cargo test -p gridcraft-functions` (185 passed), `cargo test -p gridcraft-calc` (133 passed, 3 ignored as on `main`).
- `cargo clippy -p gridcraft-functions --all-targets -- -D warnings`: clean.
- `gridcraft-cli eval`: `=BETA.INV(0.999999999999999,2,2)` gives 0.9999999817498791, Excel's value exactly; the issue's controls are unchanged (0.9410968642218047, 0.4999999999999998, 1.8257418694616704e-8); `=BETA.INV(0.7,3,0.5,2,4)` still scales into [2, 4].
- Not run: `cargo xtask` checks.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
