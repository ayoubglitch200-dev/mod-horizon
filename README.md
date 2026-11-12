# XElement Value Editor

Native Horizon mod prototype for Nightly source revision fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca (short: fdc87ae).

## Current prototype

- Press Insert in a local single-player world to show or hide the overlay.
- The overlay is drawn into the game's HUD.
- The solo check uses Horizon's integrated server field. A single-player client also has a remote connection to its integrated server, so checking remote.is_some() incorrectly disables the mod.
- The overlay currently lists planned editor categories; it does not yet write gameplay values.

## Target pins

- Horizon: fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca
- horizon-mixins: d9c83d7096b7ea5b83678da2efdfd6161f74997e
- Game: 26.3

## Important compatibility note

The launcher may leave this mod out if the native build is being made for a different Horizon commit (for example, another enabled native mod pins a newer commit). The build log then says it was left out because it was made for another Horizon version. Temporarily disable/remove other native mods and rebuild the instance on Nightly fdc87ae to isolate this pin.

## Build and test

Install this repository from the Horizon Mods page using its exact commit. Then run the native build for the Nightly above. Check the launcher build log for any left-out diagnostic.

This is native code compiled into a custom Horizon client, not a Java/Fabric mod. Native mods run with the user's account permissions. Review the source before trusting/building it. This prototype intentionally does not alter saves or send commands to servers.
