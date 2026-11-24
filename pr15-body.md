Saving a workbook whose only visible tab is a chart sheet keeps its hidden worksheets hidden.

Fixes #4673.

**What the user saw.** With two data worksheets hidden (or very hidden) and a chart sheet as the only visible tab, Save and `gridcraft-cli convert` made the first data worksheet visible and active. The chart sheet itself survived.

**Cause.** `write_sheets` in `crates/xlsx/src/write.rs` keeps a workbook valid (at least one visible tab, and a visible active tab) by writing the active worksheet without a `state` when no worksheet is visible. It counted only worksheets; the chart sheets kept through `passthrough` were not considered.

**What changed.**
- `WorkbookKept::first_visible` gives the file position of the first visible kept chart sheet. A chart sheet recorded after the last worksheet is written after it even when worksheets were deleted since the file was read (as `sheet_entries` already does), so its recorded position is capped at the current worksheet count.
- When no worksheet is visible and such a chart sheet exists, `write_sheets` leaves every worksheet's state as it is, writes no worksheet as `tabSelected`, and returns that chart sheet's position for `<workbookView activeTab>`. Otherwise everything is as before: a visible active worksheet is the tab, and with no visible tab at all the active worksheet is still shown.
- Only `xl/workbook.xml` and the `tabSelected` flags change, as the issue's analysis expects. The XLSB-to-XLSX path mentioned in the issue skips chart sheets entirely, so it still shows a worksheet; that is a different case.

Not in this PR: #4672 (a chart sheet that was the active tab while worksheets were also visible). Keeping it needs the reader to remember the chart sheet's active state and a rule for when the user has since activated a worksheet, which is a separate decision.

**Test.** `tests::passthrough_tests::a_visible_chart_sheet_is_the_active_tab_when_every_worksheet_is_hidden` takes the existing chart-sheet fixture with Data very hidden, More and Spare hidden and Chart1 visible and active: after a round trip the states are unchanged, `activeTab` is 1 (Chart1) and no worksheet is `tabSelected`; the unmodified fixture still has `activeTab` 2. On `main` it fails with Data written as visible. `the_active_chart_sheet_tab_counts_deleted_worksheets` makes the last chart sheet the only visible tab and deletes a worksheet before saving: `activeTab` is 3, the last of four sheets (without the cap it was 4, outside the list).

**Verification.**
- `cargo test -p gridcraft-xlsx`: 415 passed. `cargo test -p gridcraft-engine`: 1026 passed, 1 ignored (as on `main`); integration 133 passed.
- `cargo clippy -p gridcraft-xlsx --all-targets -- -D warnings`: clean.
- Not run: opening the output in Excel (the expected result is the issue's), `cargo xtask` checks.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
