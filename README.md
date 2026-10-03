# InstaMessage

Unofficial desktop client for Instagram DMs, built with Rust and [Tauri v2](https://tauri.app).

> Not affiliated with, endorsed by, or connected to Meta or Instagram.

## Features

- Focused on DMs: the inbox, search, and profiles only. No feed, Explore, or Reels.
- Stays logged in across restarts; only logging out ends the session
- Closing the window hides it to the tray, so reopening is instant with no reload
- Starts hidden in the tray at login
- Single instance, and Dock reopen on macOS
- External links open in your default browser
- Windows, macOS, and Linux

## Install

Download the installer for your OS from [Releases](https://github.com/hasnain607/instamessage/releases).

- **Windows:** run the `.exe` setup. SmartScreen may warn because the app is unsigned: click "More info", then "Run anyway".
- **macOS:** open the `.dmg` and drag the app to Applications. The app is unsigned, so on first launch right-click it and choose Open, or run `xattr -cr /Applications/instamessage.app`.
- **Linux:** `sudo apt install ./instamessage_*.deb`, or `chmod +x *.AppImage && ./*.AppImage`.

## Tray icon

- **Windows and macOS:** left click shows or hides the window, and right click opens the menu.
- **Linux:** any click opens the menu, because the AppIndicator system does not send click events to apps. Use **Show / Hide** from the menu. On GNOME, install the AppIndicator extension to see the tray icon.

## Development

Install the [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your OS, then:

```bash
npm install
npm run tauri dev
```

Project layout:

- `src-tauri/src/main.rs`: window, tray, autostart, and navigation filtering
- `src-tauri/src/inject.js`: route blocking and hidden UI elements
- `src-tauri/tauri.conf.json`: app and bundle configuration

Instagram changes its markup often, so hidden elements may reappear after an update. See [CONTRIBUTING.md](CONTRIBUTING.md) for how to fix them.

## License

MIT
