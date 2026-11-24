A one-cell `=LARGE(SEQUENCE(1000000),SEQUENCE(1000000))` (or TRIMMEAN, RANK.EQ, FORECAST, …) no longer hangs GridCraft, including while the file opens: it is `#NUM!` at once, as the lifted lookups and criteria aggregates already are.

Fixes #4572.

**What the user saw.** A lifted statistical formula did O(N²) work with no limit: `=ROWS(LARGE(SEQUENCE(64000),SEQUENCE(64000)))` took about 9 s, TRIMMEAN 10.5 s, and at a million elements a few-KB workbook hung on open for an estimated tens of minutes or more.

**Cause.** `call_lifted_within` in `crates/functions/src/lift.rs` charges each lifted call `scan_len(spec, args)`, which is non-zero only for the `SCANS` lookups and `CRITERIA` aggregates. Every other function that re-reads a large non-lifted array on each call was charged 1 per element, so `LIFTED_SCAN_BUDGET` never applied.

**What changed.** A `READS_ALL` list names the functions that read every cell of their array arguments on each call, and `scan_len` charges them the total cells of those arrays, as it does `CRITERIA`. Past the existing 5e8 budget the call is `#NUM!` before any element runs. The list was taken from the registry: every function with both a liftable scalar argument and an array argument, minus INDEX (reads one entry per call) and the lookups and criteria aggregates (already metered). Besides the issue's statistics functions it covers their siblings (T.TEST/TTEST, Z.TEST/ZTEST, FORECAST.ETS*), and the same pattern elsewhere: SERIESSUM, NPV, XNPV, IRR, XIRR, MIRR, FVSCHEDULE, NETWORKDAYS(.INTL) and WORKDAY(.INTL) with holidays, TEXTJOIN, ARRAYTOTEXT, TEXTBEFORE, TEXTAFTER and TEXTSPLIT with delimiter lists.

Cost per cell. The budget's unit is about one scanned cell (~8 ns). Measured at the edge of what the budget admits, the numeric functions take 1.5 to 4.3 s and TEXTJOIN 6.4 s (`LARGE(SEQUENCE(1000000),SEQUENCE(499))` 2.1 s, `XNPV` over two 500,000-cell arrays 4.3 s). TEXTBEFORE, TEXTAFTER and TEXTSPLIT are much dearer: they turn every delimiter into text and sort and index the list on each call, about 600 ns a cell, so `TEXTAFTER(x,SEQUENCE(10000),1,SEQUENCE(10000))` still took 57 s at one unit a cell. Their delimiter cells count 64 units each (`DELIMITED`, `DELIMITER_COST`); that case is now `#NUM!` at once, and the worst one admitted takes about 4 s.

Trade-off, the same one the lookups already make: a lifted call over more than 5e8 element × cell pairs is refused rather than computed slowly, e.g. `=RANK.EQ(A1:A30000,A1:A30000)` (9e8) is now `#NUM!`, while 20,000 rows (4e8) still compute. Raising the shared budget, or a cheaper per-call algorithm, would be a separate decision.

**Test.** `lift::tests::lifted_functions_reading_whole_arrays_stop_past_their_work_budget`: LARGE, SMALL, TRIMMEAN, RANK.EQ and PERCENTILE.INC with 501 elements over 1,000,000 cells, FORECAST.LINEAR and XNPV over two 500,000-cell arrays, and TEXTJOIN are `#NUM!`; TEXTAFTER over 10,000 delimiters with 1,000 elements and TEXTSPLIT over 10,000 row delimiters with 1,000 texts are `#NUM!` (each passes at one unit a delimiter cell); small lifted LARGE and RANK.EQ still give their results; every `READS_ALL` name is registered and has a liftable argument. With the new condition disabled it fails at the first LARGE case.

**Verification.**
- `cargo test -p gridcraft-functions` (185 passed), `cargo test -p gridcraft-calc` (133 passed, 3 ignored as on `main`), `cargo test -p gridcraft-engine` (1026 passed, 1 ignored as on `main`; integration 133 passed).
- `cargo clippy -p gridcraft-functions --all-targets -- -D warnings`: clean.
- `gridcraft-cli eval`, wall time: `=ROWS(LARGE(SEQUENCE(64000),SEQUENCE(64000)))` and the TRIMMEAN equivalent give `#NUM!` in 0.01 s; `=ROWS(LARGE(SEQUENCE(16000),SEQUENCE(16000)))` still gives 16000 (0.7 s, debug build).
- Not run: `cargo xtask` checks; opening an actual workbook holding the formula (the open path calls the same evaluator).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
