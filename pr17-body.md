A legacy array formula opened from an `.xlsb` calculates across its range instead of showing `#SPILL!`.

Fixes #4669.

**What the user saw.** With 2 and 3 in A1:A2 and `{=A1:A2*2}` entered over B1:B2, the XLSB opened with `#SPILL!` in B1 and the old 6 left in B2; converted to XLSX it saved `#VALUE!` as B1's cached value. The same workbook saved as XLSX opened with 4 and 6.

**Cause.** In an XLSB, an array formula's other cells are formula records that only point at the first cell (`PtgExp`), each with its saved result. craft-xlsx gives those cells no formula, as in `.xlsx`, but keeps the saved value, and `read_xlsb` stored it. The array then found its own result cells occupied. The XLSX reader (`sheet_read.rs`) leaves the cells of an array range other than the anchor empty, so the formula fills them again; the XLSB import didn't.

**What changed.** `read_xlsb` records each array formula's range as it reads the anchor (the top-left cell, which comes first) and skips the saved values of formula-less cells inside a recorded range. A value outside every range, or any cell with its own formula, is kept. The ranges are bounded by the XLSX reader's own caps (`MAX_DYNAMIC_RANGES`, `MAX_DYNAMIC_CELLS`, `MAX_DYNAMIC_CHECKS`, now `pub(crate)`), so a hostile file can't make every cell scan a long list; past them cells keep their saved values, as in the XLSX reader. Dynamic-array formulas also arrive as array formulas from XLSB (#4670), so their cached spill values now give way to the formula in the same way.

**Test.** `tests::xlsb_tests::xlsb_array_formula_results_give_way_to_the_formula` builds the issue's sheet in BIFF12 records (both array cells as `PtgExp` formula records, then `BrtArrFmla`, plus a value just below the range): the anchor holds the array formula over B1:B2, B2 is empty, the value below is kept. On `main` it fails at "the saved result in B2 gives way to the formula".

**Verification.**
- `cargo test -p gridcraft-xlsx` (414 passed), `cargo test -p gridcraft-engine` (1026 passed, 1 ignored as on `main`; integration 133 passed).
- `cargo clippy -p gridcraft-xlsx --all-targets -- -D warnings`: clean.
- End to end: the same records written to a file; `gridcraft-cli cat` shows B1 4, B2 6, B3 7; `gridcraft-cli convert` to XLSX writes `<f t="array" ref="B1:B2">` with cached 4 and a numeric 6 in B2.
- Not run: a real Excel-written XLSB, `cargo xtask` checks.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
