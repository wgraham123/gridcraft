On a cell that has a note, the cell menu offers **Edit Note**, and the dialog it opens is titled Edit Note and holds the note's text.

Fixes #3693.

**What the user saw.** Right-clicking a cell with a note offered only "New Note", so there seemed to be no way to edit an existing note. (The dialog already opened with the note's text and its OK already replaced the text, through `review.newNote`, which edits a note that exists; only the wording said otherwise.)

**Cause.** The cell context menu in `crates/ui-egui/src/grid/menus.rs` always listed "New Note", and the note dialog in `crates/ui-egui/src/dialogs.rs` was always titled "New Note".

**What changed.** When the active cell holds a note (not a threaded comment), the menu item reads "Edit Note" and the dialog's title is "Edit Note", the way a linked cell already gets "Edit Hyperlink…" and an "Edit Hyperlink" dialog. The item runs the same `ui:newNote` action and the dialog the same `review.newNote` command, so behaviour and automation are unchanged. The dialog now reads the cell's comment once and uses it for both the text and the title. Comments, the ribbon's Notes menu and Delete Note are left as they are; this issue is about editing a note.

**Test.** `context_menu_stays_open::a_cell_with_a_note_offers_to_edit_it` (kittest, accessibility check on): a cell without a note offers New Note and not Edit Note; after `review.newNote` it offers Edit Note and not New Note; clicking Edit Note opens a dialog titled "Edit Note" holding the note's text. On `main` it fails at "a cell with a note".

**Accessibility.** The item is the same menu button with a new label, found by its accessible name in the test; the dialog is the existing form with a new title. Both strings go through the translation system ("Edit Note" is a new English key; other languages show English until storytold/craft-i18n adds it). No keyboard path, focus handling or announcement changes, so no keyboard test was added. Not checked with a screen reader.

**Verification.**
- `cargo test -p gridcraft-ui-egui` with `CRAFT_FONTS_DIR` set to storytold/craft-fonts at `b076417b` (the commit CI pins): every test binary passes, 1,681 tests in all.
- `cargo clippy -p gridcraft-ui-egui --all-targets -- -D warnings`: clean.
- Not run: the desktop app itself (no screenshot), `cargo xtask` checks.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
