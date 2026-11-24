Consolidate keeps " Alpha", "Alpha " and a label of spaces as categories of their own, as Excel does, instead of merging them into "Alpha" or dropping their values.

Fixes #3928.

**What the user saw.** Consolidating by left-column labels, a second label `" Alpha"` or `"Alpha "` was added into `Alpha` (30 instead of 10 and 20), and a label of three spaces or one no-break space disappeared together with its value. Excel keeps two rows in the first case and the spaced label with its 20 in the second.

**Cause.** The label reader in `crates/engine/src/cmd/whatif/consolidate.rs` called `.trim()` on the cell's text before building the category key, and then skipped labels that had become empty.

**What changed.** The `.trim()` is gone. A cell that shows no text still has no label and is skipped, as before and as in Excel. Matching still ignores case (`ALPHA` joins `Alpha`). Top-row labels use the same reader, so they get the same rule.

Overlap: open PR #4173 (for #3927) rewrites this label reader around cell values and keeps the trim on purpose (`label_value`). Whichever merges second should take the other's rule; with this one merged, #4173's `label_value` would drop its `trim()`.

**Test.** `cmd::whatif::tests::consolidate_matches_labels_with_their_spaces` runs the issue's six second labels (`" Alpha"`, `"Alpha "`, three spaces, a no-break space, `"ALPHA"`, blank) and checks the output rows and totals against the issue's Excel column. On `main` it fails at `" Alpha"` (30 instead of 10).

**Verification.**
- `cargo test -p gridcraft-engine`: 1027 passed, 1 ignored (as on `main`); integration tests 133 passed.
- `cargo clippy -p gridcraft-engine --all-targets -- -D warnings`: clean.
- End to end with the issue's script and `gridcraft-cli run`: `sheet.read D1:E2` gives `[["Alpha",10],["   ",20]]`.
- Not run: `cargo xtask` checks (no layer, asset, string or command changes) and the UI crate's tests.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
