A hidden or very hidden worksheet can be deleted while one other worksheet is visible.

Fixes #4514.

**What the user saw.** With `Sheet1` visible and `Hidden` hidden (or very hidden), `home.deleteSheet {"sheet":"Hidden"}` failed with "A workbook must contain at least one visible worksheet.", although `Sheet1` would stay visible.

**Cause.** `delete_sheet` in `crates/engine/src/cmd/sheet/tabs.rs` refused when the workbook had at most one visible sheet, without looking at which sheet was being deleted.

**What changed.** The guard now asks whether a visible sheet other than the target remains. Deleting the last visible sheet is still refused with the same message, and a structure-protected workbook is still refused first. The active sheet already shifts correctly when an earlier sheet is removed, so nothing else needed to change. `sheet.hide` has a similar count, but there the target is always one of the visible sheets being counted, so it is left alone.

**Test.** `tests::deleting_a_hidden_sheet_beside_the_last_visible_one`, for a Hidden and a VeryHidden sheet: deleting the visible `Sheet1` is refused, deleting the hidden sheet succeeds and leaves `Sheet1` as the only and active sheet, and deleting the only sheet is then refused. On `main` it fails at the hidden sheet's deletion with the error above.

**Verification.**
- `cargo test -p gridcraft-engine`: 1027 passed, 1 ignored (as on `main`); integration tests 133 passed.
- `cargo clippy -p gridcraft-engine --all-targets -- -D warnings`: clean.
- End to end with `gridcraft-cli run`: `home.insertSheet Hidden`, `sheet.hide`, `home.deleteSheet Hidden` succeed; a following `home.deleteSheet Sheet1` is refused with the same message.
- `cargo xtask layers`, `wasm`: ok.
- Not run: the other `cargo xtask` checks (no asset, string or command changes) and the UI crate's tests.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
