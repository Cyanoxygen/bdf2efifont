# psf2efifont - a simple bitmap font converter

This tool converts a BDF bitmap font to a usable EFI GraphicsConsole font data.

> [!NOTE]
> This project is currently is a work in progress, only the basic function is working.
>
> Future plan includes a good CLI interface, snippet generation and allow combining multiple source fonts.

# Building

This project is written in Rust. You can setup a Rust toolchain by visiting https://rustup.rs.

> [!WARNING]
> If you decide to use the Rust toolchain shipped with your Linux distro, make sure the Rust version are higher than 1.85.

Build this project with `Cargo`:

```
cargo build --release
```

# Usage

Specify the input, and the output will be in the same directory as the input:

```sh
./target/release/bdf2efifont /path/to/font.bdf
```

The output will be `/path/to/font_narrow.c` (narrow glyphs) and `/path/to/font_wide.c` (wide glyphs).

# Ways to include the outputs

For now, if you just want parts of the output, you can directly open, copy and paste the part you want to your existing font data.

If you want to replace the output:

1. Copy the output file(s) to your font package directory.
2. In your INF file, edit the sources in the `[Sources]` section.
3. Remove the filename containing existing font data.
4. Add the output to the source file list.
5. In the source of the font package itself, update the references.

## Generated symbols

You must update the source code of the font package to reference the correct data:

- `gUsStdNarrowGlyphData`: array containing narrow glyph data.
- `gUsStdWideGlyphData`: array containing Wide glyph data.
- `gUsStdNarrowGlyphSize`: the size of the narrow glyph data array.
- `gUsStdWideGlyphSize`: the size of the wide glyph data array.

The symbol prefix will be customizable in the future.

# Generating a BDF font from a vector font

You can use FontForge to generate a BDF bitmap data from a vector font (`.ttf` and `.otf`).

> [!WARNING]
> Always use a monospace font. Glyphs in monospace fonts have fixed width.

To create the BDF bitmap file, you need to first rasterize the vector font:

1. Launch FontForge.
2. Navigate and open your desired font file in the dialog.
3. In the main window, click `Element` on the menu bar and then `Bitmap Strikes Available...`.
4. In the dialog, type `16` in the `Pixel Sizes` input box only (the rest two are automatically calculated).
5. Make sure `Use FreeType` and `Create Rasterized Strikes (Not empty ones)` are checked.
6. Click OK.

You can then view and edit (fine tune) the bitmap data:

1. Click View on the menu bar and on the bottom of the menu, click `16 pixel bitmap`.
2. The main window now displays the bitmap font.
3. Now you can edit the font by double-clicking the individual glyphs:
    - Click or drag to set/clear.
    - If you messed up the glyph, recover by clicking `Recalculate Bitmaps`.
    - Close the window once you are done. The changes are saved on real time.

> [!WARNING]
> Make sure the glyphs are less than 8 pixels wide (the space between the two vertical bold lines).
>
> If the fonts are too wide, you can regenerate the bitmap data with a smaller size by repeating the procedure above.

Once you are satisfied, you can then export the bitmap font as BDF:

1. Click `File` on the menu bar, then `Generate Fonts...`.
2. Under the filename input box, there are two dropdown boxes.
3. Select `No Outline Font` in the left dropdown.
4. Select `BDF` in the right dropdown.
5. Click `Generate`.

# License

[![](https://www.gnu.org/graphics/agplv3-155x51.png)](https://www.gnu.org/licenses/agpl-3.0.txt)

Copyright (C) 2026 Xinhui Yang

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

This project includes glyph data from GNU Unifont, licensed under [SIL Open Font License version 1.1](OFL-1.1.txt) and/or GPL v2.
