# Contributing

1. Fork the repo and create a branch: `git checkout -b feature/my-change`
2. Install the [Tauri prerequisites](https://tauri.app/start/prerequisites/) and run `npm install`
3. Run the app with `npm run tauri dev`
4. Before opening a PR, run:

   ```bash
   cargo fmt --manifest-path src-tauri/Cargo.toml
   cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
   ```

5. Open a pull request describing what changed and which OS you tested on.

## Where things live

- `src-tauri/src/main.rs`: window, tray, autostart, and navigation filtering
- `src-tauri/src/inject.js`: blocked routes and hidden UI elements
- `src-tauri/tauri.conf.json`: app and bundle configuration

## Fixing a UI element that reappeared

Instagram updates its markup often, so hidden elements can come back.

1. Run `npm run tauri dev`, right-click the element, and choose Inspect Element.
2. Add its visible text or icon `aria-label` (lowercase) to the `LABELS` set in `inject.js`, or add a selector to `CSS`.
3. To block a page, add a pattern to `BLOCKED_PATHS`.

## Notes

- Linux tray icons open the menu on any click. This is a limitation of AppIndicator, not a bug.
- Instagram labels are matched in English only. Translations are welcome.
