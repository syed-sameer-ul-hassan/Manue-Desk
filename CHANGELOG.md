# Changelog

All notable changes to **Manue Desk** (`manue-desk`) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0] - 2026-09-11

### Added
- **UI Navigation**: Multi-tab interface featuring Launcher Creator, Manage Shortcuts, Settings, and Info Page.
- **Info & Developer Page**: Added dedicated Info tab featuring developer photo (retrieved from GitHub `syed-sameer-ul-hassan`), website (`manuedesk.orildo.sbs`), email (`manuedesk@orildo.sbs`), and Apache 2.0 open-source licensing.
- **Clickable Links**: Interactivity using `open-url` callback in Slint connected to `xdg-open` backend for browser navigation.
- **Squarish Profile Avatar**: Customized squarish avatar card with rounded border design.
- **Shortcuts Management**: Real-time listing, search filtering, inline editing, and deletion of `.desktop` files in `~/.local/share/applications/`.
- **Custom Frameless Window Controls**: Integrated custom minimize (`─`) and close (`✕`) buttons matching the dark-teal glassmorphism theme.
- **Native File Pickers**: Integrated `rfd` dialogs for selecting executable paths and application icons.
- **Open Source Licensing**: Released officially under Apache License 2.0.
