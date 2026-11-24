A two-variable Data Table whose row and column input are the same cell is refused, and the table keeps its results, as in Excel.

Fixes #3917.

**What the user saw.** `data.dataTable {"range":"C3:E5","rowInput":"A1","columnInput":"A1"}` reported success and overwrote D4:E5 with `[[203,203],[204,204]]`: every column got the same values, because the row values were ignored. Excel refuses it with an invalid input cell reference error and leaves the body as it was.

**Cause.** `data_table` in `crates/engine/src/cmd/whatif.rs` refused an input inside the table but never compared the two inputs. With both on one cell, the column value is substituted after the row value and wins.

**What changed.** When both inputs resolve to the same sheet and cell, the command returns the existing "Input cell reference is not valid." error before anything is calculated or written. The inputs are compared after `cell_arg` has resolved them, so `B2` and `$B$2`, or `Sheet1!B1` and `b1`, count as the same cell. Different inputs, one-input tables and the existing inside-the-table check are unchanged.

**Test.** `cmd::whatif::tests::data_tables` gains three same-cell pairs (`B1`/`B1`, `B2`/`$B$2`, `Sheet1!B1`/`b1`) on the existing two-variable table: each is refused and the table's results stay. On `main` the first pair succeeds and the test fails.

**Verification.**
- `cargo test -p gridcraft-engine`: 1026 passed, 1 ignored (as on `main`); integration tests 133 passed.
- `cargo clippy -p gridcraft-engine --all-targets -- -D warnings`: clean.
- End to end with the issue's script and `gridcraft-cli run`: A1/A1 and B1/B1 are refused with "Input cell reference is not valid."; A1/B1 gives `[[31,32],[41,42]]` and B1/A1 `[[13,23],[14,24]]`, as in the issue's Excel column.
- Not run: `cargo xtask` checks (no layer, asset, string or command changes) and the UI crate's tests. The Data Table dialog calls the same command, so it gets the same refusal; I did not open it.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
