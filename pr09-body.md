A note shown at all times, or the active cell's data-validation input message, no longer covers the cell's context menu: the menu is drawn on top, so all its items can be read and clicked.

Fixes #3692.

**What the user saw.** With a note set to always show, right-clicking its cell opened the menu under the yellow note box, which hid the items between "Insert…" and "Filter by Selected Cell's Value". A cell with a validation input message does the same: right-clicking makes it the active cell, and its message box opens over the menu.

**Cause.** `paint_overlays` in `crates/ui-egui/src/grid/show.rs` shows notes and the input message in `egui::Area`s at `Order::Tooltip`. The cell context menu (and the header, fill, filter and drop-down list popups) use `Order::Foreground`, which egui paints below `Tooltip`.

**What changed.** Both overlays use `Order::Middle`: still above the sheet and the panels (`Background`), now below every `Foreground` popup. Nothing else about when they are shown changes. Dialog windows also live at `Middle`; before this change these overlays were always above them, now whichever appeared last is on top, so no case gets worse.

**Test.** In `context_menu_stays_open`, a shared check `under_the_menu` asserts that an area and the open menu overlap and that the area's layer comes before the menu's in egui's back-to-front layer order. `a_shown_note_stays_below_the_context_menu` uses it for an always-shown note on B2, `a_validation_input_message_stays_below_the_context_menu` for B2's validation input message. Each fails at "drawn below the menu" with its overlay back at `Order::Tooltip`.

**Accessibility.** Both areas stay non-interactive and their content and names are unchanged; the menu was already reachable through the accessibility tree, and now it is also visible on screen. No focus, keyboard or announcement changes. Not checked with a screen reader.

**Verification.**
- `cargo test -p gridcraft-ui-egui` with `CRAFT_FONTS_DIR` set to storytold/craft-fonts at `b076417b` (the commit CI pins): every test binary passes, 1,682 tests in all.
- `cargo clippy -p gridcraft-ui-egui --all-targets -- -D warnings`: clean.
- Not run: the desktop app (no screenshot), `cargo xtask` checks.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01PnyMxBZxfXbcJReWpWsqNV
