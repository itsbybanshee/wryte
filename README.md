<p align="center">
  <img src="assets/app-icon/wryte-256.png" width="112" height="112" alt="Wryte icon: a black W on a yellow tile">
</p>

<h1 align="center">Wryte</h1>

<p align="center">
  <b>A lightweight word processor that gets out of your way.</b>
</p>

---

Wryte is a fast, sleek writing app for Windows. Open it and you're writing almost at once: one quiet
toolbar along the top, your page in the middle, and a format panel on the side for when you want it.
No ribbon to dig through and no account to sign in to.

It's built in Rust and runs natively, so it starts quickly, stays light on memory, and keeps up
with long documents.

## Why Wryte

- **Clean by design.** One toolbar for the things you reach for most: insert a table, a chart, a
  shape or a photo, add a comment, zoom, change the view. Everything else sits a click away.
- **Format without hunting.** The sidebar puts paragraph styles, fonts, sizes, alignment, spacing
  and lists in one place, and updates as you move through your text.
- **Lightweight.** A single small app with no background services and no account.
- **Works with your files.** Opens and saves Word documents (.docx, .doc), Rich Text, OpenDocument,
  Markdown and plain text, and exports to PDF.
- **Light and dark.** A warm, easy-on-the-eyes palette in both, following your system setting or
  your choice.
- **Your files stay yours.** Wryte doesn't check for updates in the background. When a new version
  is out, you'll find it on the [Releases](https://github.com/itsbybanshee/wryte/releases) page.

Under the toolbar there's a full word processor: styles, tables, headers and footers, footnotes,
tables of contents, citations, track changes and comments, find and replace, spelling, and mail
merge. Prefer a ribbon? Turn it on under **View › Classic Ribbon**.

## Install

1. Download **Wryte-v1.0.0-Windows-x64.zip** from the
   [latest release](https://github.com/itsbybanshee/wryte/releases/latest).
2. Unzip it and run **Wryte-Setup-1.0.0.exe**.
3. Pick which file types Wryte should open, and whether you'd like a desktop shortcut.

Wryte installs for your account only, so it doesn't need admin rights. You can uninstall it any
time from **Settings › Apps**.

> The installer isn't code-signed yet, so Windows SmartScreen may show a warning the first time.
> Choose **More info › Run anyway** to continue.

If you've already picked another app for a file type, Windows keeps that choice. To switch, tick
**Open Default apps** on the last page of setup, or right-click a document and choose
**Open with › Wryte › Always**.

## Build from source

You'll need [Rust](https://rustup.rs) 1.95 or newer.

```bash
cargo build --release -p wryte
```

The app is `target/release/wryte.exe`. The installer script is
[`packaging/windows/wryte.iss`](packaging/windows/wryte.iss) and builds with Inno Setup 6.

## Feedback

Found a bug or have an idea? [Open an issue](https://github.com/itsbybanshee/wryte/issues).

## License and credits

Wryte is free and open source under the [MIT License](LICENSE-MIT) or the
[Apache License 2.0](LICENSE-APACHE), at your option.

Wryte is built on [WordCraft](https://github.com/storytold/wordcraft) by the ArtCraft team and
the WordCraft contributors, whose work makes up most of its engine. Their copyright and licence
notices are kept in [NOTICE](NOTICE), and [CHANGES.md](CHANGES.md) lists what Wryte changes.
Wryte isn't affiliated with or endorsed by them. Fonts and other bundled assets are credited in
[ATTRIBUTION.md](ATTRIBUTION.md).
