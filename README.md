# XElement Value Editor

Native Horizon mod prototype for the Nightly source revision `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca` (short: `fdc87ae`).

## Current prototype

- Press **Insert** in a local single-player world to show or hide the overlay.
- The overlay is drawn into the game's HUD.
- The toggle and overlay are disabled when the game has a remote server connection.
- The panel currently lists planned editor categories; it does **not yet write gameplay values**.

## Target pins

- Horizon: `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca`
- horizon-mixins: `d9c83d7096b7ea5b83678da2efdfd6161f74997e`
- Game: `26.3`

## Safety and scope

This is native code compiled into a custom Horizon client, not a Java/Fabric mod. Native mods run with the user's account permissions. Review the source before trusting/building it. This prototype intentionally does not alter saves or send commands to servers.

## Build

Use the Horizon launcher’s native build for the exact pinned Nightly. A full compile/runtime test still needs the matching Horizon and horizon-mixins source checkouts and the launcher's build environment. Do not treat the presence of source files in this repository as proof that a binary has been built.
