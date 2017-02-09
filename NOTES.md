# MISTER

So I decided to throw out any old plans, and to write a sprite/pixel editor? Why?
For profit and fun, of course!

## But the one you use...

Who says I'm gonna stop using it? I love it. I just wanna experiment.

## How is it different?

This is gonna be in Rust. I'm going to support some features that I think should be supported from the start, such as:
  - flexible import/export
  - palettes w/ crediting (they recently added support for this)
  - tile map testing support

## If you have flexible import/export...

Yes, there is a native format, it's just that this format can be exported into a varying amount of formats.

## Features?

All the things above (if featurable) and:
- Layers (hideable)
- Folders/Groups
- Palettes
- Multiple views
- Full multi-color system color support (RGBA, HSL, HSV, etc.)
- Playing with channels (a la Krita, but hopefully not as complicated)

Planned features:
- Filters
- Preview (ASAP)
- Scripting (prob. using [gluon](https://github.com/gluon-lang/gluon)) (gluon may be used for other features, too.)
- Selections (full [move, rotate, scale, substitute, apply filters, etc.])

## Format?

Glad you asked.
How we'll solve the age old problem of timemaps versus sprites by combining the two into 2 types of images:
  - Default mode: Sprite-dependant (Sprite images) [ Main target for version 1.0 ]
    - By default, the view is that of a sprite.
    - However, at anytime, the user can switch to a tilemap mode, where they specify the size of a tile, and the system will automatically split the sprite into tile-sized segments, which the user can freely arrange.
  - Second mode: Map-dependant (Tilemap w/ backing sprite) (Very close to PyxelEdit behavior)
    - By default, a tilemap of unmapped tiles is created.
    - Can modify an tile freely, but can map tiles into the tileset.
    - Can switch into sprite mode to draw more tiles, or modify the existing ones
  - *This is all conjecture, so we'll partially see w/ implementation.*

REAL IMPLEMENTATION:

- 2 storage formats:
  - stores color components as bytes (4 bytes for RGBA image) (1 byte per channel)
    - Pros:
      - optimized for space
      - easier to share (smaller files sizes)
      - matches other file formats much more closely
    - Cons:
      - slightly inaccurate (we use floating point internally)
  - stores color components as [IEEE 32-bit](https://en.wikipedia.org/wiki/Single-precision_floating-point_format) (4 byte) (16 bytes for RGBA image) (4 bytes per channel)
    - Pros:
      - accurate internally
    - Cons:
      - files are quite a bit larger
      - doesn't match other image storing formats

# Notes?

- I'll try to do TDD this time.
