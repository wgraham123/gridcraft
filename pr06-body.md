Subtotal refuses a group or total column outside its range instead of writing summaries into unrelated columns.

Fixes #3948.

**What the user saw.** `data.subtotal {"range":"A1:B4","groupBy":"A","columns":["C"]}` inserted subtotal rows and wrote `SUBTOTAL` formulas into column C, which is outside the range; with `"groupBy":"C"` it grouped the unrelated values in C. Excel refuses both and leaves the sheet as it was.

**Cause.** `subtotal` in `crates/engine/src/cmd/data/grouping.rs` parsed `groupBy` and `columns` to absolute column numbers and never checked them against the range.

**What changed.** After parsing, the command refuses with "column C is outside A1:B4" when the group column or any total column lies outside the range's columns. The check runs before the outline setting, the row inserts and the writes, so a refused call changes nothing and adds no undo step. Columns given as letters or as offsets from the range are both checked. Unparseable column parameters keep their current handling; that is a separate question from this issue.

**Test.** `tests::subtotal_refuses_columns_outside_its_range` runs the issue's two cases and an offset form (`"columns":[2]` on a two-column range): each is refused, A1:C8 keeps its values and no row is grouped. The valid `groupBy: 0, columns: [1]` still gives a grand total of 60. On `main` the first case succeeds and the test fails.

**Verification.**
- `cargo test -p gridcraft-engine`: 1027 passed, 1 ignored (as on `main`); integration tests 133 passed.
- `cargo clippy -p gridcraft-engine --all-targets -- -D warnings`: clean.
- End to end with the issue's script and `gridcraft-cli run`: both invalid cases are refused (`invalid parameters for data.subtotal: column C is outside A1:B4`); the valid control gives A Total 30, B Total 30, Grand Total 60.
- Not run: `cargo xtask` checks (no layer, asset, string or command changes) and the UI crate's tests. The Subtotal dialog takes the column as a typed letter and calls the same command, so a letter outside the range is now refused there too; I did not open the dialog.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
