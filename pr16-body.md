Formulas opened from an `.xlsb` keep Excel's implicit intersection: `=A1:A3` and `=ABS(A1:A3)` return one value as in Excel instead of `#SPILL!` or a spill.

Fixes #4665.

**What the user saw.** With 11, −22, 33 in A1:A3, an XLSB with `=A1:A3` in B2, `=ABS(A1:A3)` in D2 and `=@A1:A3` in F2 converted to `#SPILL!` in B2 and spills from 11 in D2 and F2. Excel, and GridCraft reading the same workbook saved as XLSX, give −22, 22 and −22.

**Cause.** XLSB, like XLSX, stores a dynamic-array formula as an array formula with metadata; a plain or shared formula is a legacy one that Excel evaluates with implicit intersection. The XLSX reader (formulas without `cm`) and the XLS reader (every formula) pass such cells to `array_formula::apply_legacy`, which adds the `@` where a dynamic-array reading would differ. `read_xlsb` never did.

**What changed.**
- `read_xlsb` collects the cells whose formula is neither an array formula nor dynamic and passes them to `apply_legacy`, the same call `read_xls_with` makes. Array formulas, which is how dynamic formulas arrive, are untouched.
- `read_xlsb_with(bytes, FunctionInfo)` is added next to `read_xlsx_with` and `read_xls_with`; `read_xlsb` uses the default `FunctionInfo`, and the engine's import passes `XLSX_FUNCTIONS` as it does for XLSX and XLS, so the function library decides which arguments take a single value.

**Test.** `tests::xlsb_tests::xlsb_legacy_formulas_keep_implicit_intersection` builds BIFF12 records for `=$A$1:$A$3`, `=ABS($A$1:$A$3)` and `=SUM($A$1:$A$3)` and checks they are read as `@$A$1:$A$3`, `ABS(@$A$1:$A$3)` and an unchanged `SUM($A$1:$A$3)`. On `main` it fails with `$A$1:$A$3`.

**Verification.**
- `cargo test -p gridcraft-xlsx` (414 passed), `cargo test -p gridcraft-engine` (1026 passed, 1 ignored as on `main`; integration 133 passed).
- `cargo clippy -p gridcraft-xlsx -p gridcraft-engine --all-targets -- -D warnings`: clean.
- End to end: the same records with 11, −22, 33 in A1:A3, written to a file and opened with `gridcraft-cli cat`: B2 −22, D2 22, F2 (SUM) 22, no spill; `gridcraft-cli run` setting A2 to −40 gives −40, 40 and 4.
- Not run: a real Excel-written XLSB (none in this repository's fixtures), `cargo xtask` checks.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
