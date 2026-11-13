# XElement Value Editor

Version: **0.3.0**

Native Horizon mod prototype for Horizon Nightly revision `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca`.

## Controls

- `INSERT`: open/close the editor.
- `1`-`4`: select a category on the main page.
- `ENTER`: open the selected category.
- `UP` / `DOWN` or `W` / `S`: select a row.
- `LEFT` / `RIGHT` or `A` / `D`: decrease/increase or toggle a supported value.
- `ESC` or `BACKSPACE`: return to the category list; on the main page Escape closes the editor.

## Current implementation

- PLAYER: health, food, saturation, max health, attack damage, attack speed, armor, armor toughness, knockback resistance.
- MOVEMENT: movement speed.
- WORLD: day-time counter.
- ENTITIES & EFFECTS: five potion effects on the local player (Speed, Strength, Jump Boost, Regeneration, Invisibility).

## Known limitations

- The effects page currently edits the local player's effects; selecting arbitrary mobs/entities is not implemented yet.
- Weather, difficulty, gamerules and arbitrary world settings are not implemented yet.
- Some game state is held by Horizon's integrated server. The current prototype has not yet been verified for authoritative server synchronization or persistence.
- Mouse selection is not implemented in this Horizon revision.

## Target pins

- Horizon: `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca`
- horizon-mixins: `d9c83d7096b7ea5b83678da2efdfd6161f74997e`
- Game: 26.3

## Build and test

Build with the exact pinned Horizon Nightly revision. Source was checked against the pinned API, but this repository change has **not** been compiled in this environment. Test in a disposable single-player world first.
