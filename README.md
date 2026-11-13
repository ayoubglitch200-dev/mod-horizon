# XElement Value Editor

Version: **0.2.0**

Native Horizon mod prototype for Nightly source revision fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca.

## Controls

- `INSERT`: open/close the editor.
- `1`-`4`: select a category on the main page.
- `ENTER`: open the selected category.
- `UP` / `DOWN` or `W` / `S`: select a row.
- `LEFT` / `RIGHT` or `A` / `D`: decrease/increase a supported value.
- `ESC` or `BACKSPACE`: return to the category list; on the main page Escape closes the editor.

## Implemented values

- PLAYER: Health, Food and Saturation.
- MOVEMENT: current local player movement speed.
- WORLD and ENTITIES & EFFECTS: submenu placeholders; editing is not implemented yet.

The editor is restricted to a local integrated single-player world. Mouse selection is not implemented in this Horizon revision.

## Target pins

- Horizon: fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca
- horizon-mixins: d9c83d7096b7ea5b83678da2efdfd6161f74997e
- Game: 26.3

## Build and test

Build this repository with the exact pinned Horizon Nightly revision. The Rust source changes have been reviewed against the pinned Horizon source, but have not been compiled in this environment. Test in a disposable single-player world first.
