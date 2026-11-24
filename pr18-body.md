BETA.DIST, F.DIST, CHISQ.DIST, GAMMA.DIST, POISSON.DIST and T.DIST (and the functions built on them) keep full precision when one parameter is large, instead of losing up to six digits.

Found while reviewing `crates/functions/src/special.rs`; there is no issue for it yet (search found none, and the closed #3000 covered only the t density).

**What the user sees on `main`** (references: 40-digit incomplete beta and gamma from mpmath):

| Formula | `main` | Correct | Relative error |
|---|---|---|---|
| `=BETA.DIST(1E-8,2,1E8,TRUE)` | 0.26424118955337 | 0.26424112133590979 | 2.6e-7 |
| `=F.DIST(1.01,5,1E8,TRUE)` | 0.590191255202581 | 0.59019132322850955 | 1.2e-7 |
| `=CHISQ.DIST(1E6,1E6,TRUE)` | 0.5001880630669054 | 0.5001880631966055 | 2.6e-10 |
| `=T.DIST(0.5,1E10,TRUE)` | 0.6914609273048098 | 0.69146246126851208 | 2.2e-6 |
| `=T.INV.2T(0.6,1E10)` | 0.5244049696604616 | 0.5244005127… (= NORM.S.INV(0.7)) | 8.5e-6 |
| `=CHISQ.DIST(1E6,1E6,FALSE)` | 0.0002820947446848882 | 0.00028209474475808343 | 2.6e-10 |
| `=GAMMA.DIST(1E7,1E7,1,FALSE)` | 0.0001261566252339481 | 0.00012615662504970279 | 1.5e-9 |
| `=BETA.DIST(1E-8,2,1E8,FALSE)` | 36787954.16627463 | 36787944.668963397 | 2.6e-7 |
| `=F.DIST(1.01,5,1E8,FALSE)` | 0.6040908460888614 | 0.60409091581542746 | 1.2e-7 |

**Cause.** Three places subtract huge, nearly equal numbers:
- `ln_beta(a, b) = lnΓ(a) + lnΓ(b) − lnΓ(a+b)`: with b = 5e9 the two large log-gammas are about 1.1e11 each, so the difference keeps only about five significant digits.
- The incomplete-gamma prefactor `−x + a·ln x − lnΓ(a)` cancels the same way for large a.
- `reg_inc_beta` took `b·ln(1 − x)` from a rounded `1 − x`, and `t_upper` evaluated the beta function at v/(v+t²), which is within a few ulps of 1 for large v.

**What changed** (`crates/functions/src/special.rs`, one function in `stat/distributions.rs`):
- `ln_gamma_ratio(a, b)` = lnΓ(b) − lnΓ(a+b) for b ≥ 10, from Stirling's series with an eight-term remainder (`stirling_remainder`, error below 1e-17 from 10 up): −a·ln(a+b) − (b−½)·ln_1p(a/b) + a + δ(b) − δ(a+b), with nothing large cancelling. `ln_beta` uses it when the larger parameter is ≥ 10.
- `front_of(a, x)` (the e^−x·x^a/Γ(a) prefactor) for a ≥ 10 is a·(ln(1+d) − d) + ½·ln(a/2π) − δ(a), with d = (x − a)/a and `ln_1p_minus` summing the series where the terms cancel. `reg_gamma_q` now shares `front_of` instead of its own copy of the old formula.
- The densities: CHISQ.DIST and GAMMA.DIST use `front_of` (their density is exactly `front_of(shape, x/scale)/x`), and the F and beta densities take `ln_1p` instead of the log of a rounded `1 + y` or `1 − z`. NEGBINOM.DIST and HYPGEOM.DIST have the same kind of loss through log-gamma differences, but open upstream PRs are changing those functions (#4036, #4281, #3004), so they are left for a separate change.
- `reg_inc_beta` uses `ln_1p(−x)`. A new `reg_inc_beta_complement` gives 1 − I_x(a, b) directly, and `t_upper` uses it on the side where the beta argument t²/(v+t²) is small.

Below the thresholds nothing changes. Above them the formulas are algebraically identical, so ordinary parameters agree to rounding.

**Still not exact.** Far into the t tail at very large df (T.DIST.RT at t = 2…5 with df ≥ 1e8), the beta continued fraction itself converges near its switch point with about 1e-9…1e-7 relative error. That error is also on `main`, where it is 70 to 100 times larger. Fixing it needs a different incomplete-beta algorithm for that region (an asymptotic expansion), which is a larger change for its own PR.

**Test.** `stat::tests::distributions_keep_precision_with_large_parameters` checks eleven cases against the mpmath values: the table's rows except T.INV.2T, plus `CHISQ.DIST(1001000,1E6,TRUE)`, `T.DIST(0.5,1E8,TRUE)` and `T.DIST.2T(0.5,1E8)`, within 1e-13 *relative* error. On `main` it fails at the first one (`got 0.26424118955337`).

**Verification.**
- Precision sweep against mpmath, `main` → this branch, worst relative error of T.DIST.RT over t ∈ {0.1…5}: df 1e5 1.8e-10 → 2.1e-12; 1e7 7.3e-9 → 3.8e-10; 1e8 2.0e-7 → 2.9e-9; 1e10 1.7e-5 → 6.7e-7; df 30 and 1000 unchanged.
- Regression sweep at ordinary parameters: 300 random BETA.DIST, CHISQ.DIST, CHISQ.DIST.RT, POISSON.DIST and F.DIST cases with parameters from 0.5 to 500, straddling the new thresholds. On `main` 9 were above 1e-13 relative error (worst 3.4e-13); on this branch none (worst 9.8e-14).
- Density regression sweep: 200 random CHISQ/GAMMA/BETA/F densities with parameters from 0.3 to 400, straddling the thresholds: 17 above 1e-13 on `main` (worst 5.3e-13), none on this branch (worst 8.6e-14).
- `cargo test -p gridcraft-functions` (185 passed), `cargo test -p gridcraft-calc` (133 passed, 3 ignored as on `main`).
- `cargo clippy -p gridcraft-functions --all-targets -- -D warnings`: clean. `cargo xtask wasm`: ok.
- Not run: a comparison with Excel itself (the references are exact values, not Excel's output).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
