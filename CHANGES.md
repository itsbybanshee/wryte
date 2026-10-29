# Changes from WordCraft

Wryte is a modified version of [WordCraft](https://github.com/storytold/wordcraft), used under the
MIT License or the Apache License 2.0. This file lists what Wryte changes, as the Apache License
asks of modified versions.

## 1.0.0

**Interface**
- A single toolbar replaces the ribbon: document actions, the document's name, insert tools, zoom
  and view, with the window controls drawn in the same bar.
- A Format sidebar (Style, Layout and More) and a Document sidebar replace most ribbon tabs. The
  classic ribbon stays available under View › Classic Ribbon.
- New colour palette with a warm yellow accent, light and dark.
- Rounder, consistent corners across buttons, fields, menus, cards and rulers; no drop shadows.
- The File page uses a plain sidebar with rounded entries; template previews are rounded cards.
- New documents and opened files stay in one window.

**Identity**
- New name, icon, settings folder (`%APPDATA%\Wryter` on Windows) and application ID.
- Account and community buttons removed; links point to the Wryte repository.

**Behaviour**
- The update checker and self-updater are removed. Wryte never contacts an update server.
- A window saved off screen is moved back onto the screen when it opens.

**Packaging**
- A new Windows installer (Inno Setup) with a choice of file types; the previous MSI packaging,
  CI workflows and release tooling were removed.
