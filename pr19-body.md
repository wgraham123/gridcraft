T.DIST.RT and F.TEST return small right-tail probabilities to full precision, instead of losing digits or returning 0.

Found while reviewing `crates/functions/src/stat/distributions.rs`; no issue or PR covers it (searched T.DIST.RT and F.TEST).

**What the user sees on `main`** (references: 40-digit incomplete beta, mpmath):

| Formula | `main` | Correct |
|---|---|---|
| `=T.DIST.RT(1E6,3)` | 0 | 1.1026577908396145e-18 |
| `=T.DIST.RT(100,5)` | 9.48000677958305e-10 | 9.4800071123118137e-10 (3.5e-8 relative error) |
| `=T.DIST.RT(10,30)` | 2.2876256444703813e-11 | 2.2876257041148066e-11 (2.6e-8) |
| `=F.TEST({1,2,3,4,5},{0,1E-6,2E-6,3E-6,4E-6})` | 0 | 5.999999999984e-24 |

T.DIST.2T and F.DIST.RT were already right; T.DIST.RT disagreed with half of T.DIST.2T.

**Cause.** `t_dist_rt` returned `1.0 - t_cdf(x, v)`, and for x ≥ 0 `t_cdf` is itself `1 − t_upper(x, v)`, so the tail went through two subtractions from 1. `f_test` took the smaller of `c` and `1.0 - c` with `c = F.DIST`.

**What changed.** T.DIST.RT returns `t_upper(x, v)` for x ≥ 0 (as T.DIST.2T and TDIST already do) and `1 − t_upper(−x, v)` below 0. F.TEST takes the smaller of `f_cdf` and `f_upper` (the function F.DIST.RT uses). Nothing else changes. Z.TEST's `1 − Φ(z)` is left as it is: Excel documents Z.TEST as exactly that expression, so its result may match Excel's.

**Test.** `stat::tests::right_tails_keep_their_digits`: T.DIST.RT at (1e6, 3) and (10, 30) within 1e-13 relative of the references; for three more points T.DIST.RT(−x) + T.DIST.RT(x) = 1 and T.DIST.RT(x) is exactly T.DIST.2T(x)/2; F.TEST of the two arrays above, in both orders, within 1e-12 relative of 6e-24. On `main` it fails at the first assertion.

**Verification.**
- `cargo test -p gridcraft-functions` (185 passed).
- `cargo clippy -p gridcraft-functions --all-targets -- -D warnings`: clean.
- Not run: `cargo xtask` checks; a comparison with Excel itself (the references are exact values).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
