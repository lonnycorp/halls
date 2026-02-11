Halls
=====

<p align="center">
  <img src="asset/build/logo.svg" alt="Halls logo" width="220" />
</p>

Halls is a free, open-source first-person exploration game. Players can traverse 3D spaces hosted anywhere on the internet, linked by portals that are addressed by URL. There is no objective - just portals to step through and places to see!

## Usage

Download a prebuilt binary from [Releases](https://github.com/tlonny/halls/releases). Alternatively, build from source (requires Rust stable):

```
cargo run --release
```

## Level Creation

A level is a collection of assets linked together by a `manifest.json`. The manifest schema:

```json
{
    "_version": "coco",
    "meta": {
        "name": "My Level",
        "author": "Author Name",
        "track": "Song Title - Artist"
    },
    "level": {
        "model": "mesh.glb",
        "collider": "collider.glb",
        "lightmap": "lightmap.png",
        "track": "music.ogg",
        "material": {
            "MyMaterial": { "image": "texture.png" }
        }
    },
    "portal": {
        "my_portal": {
            "spawn": true,
            "model": "portal.glb",
            "link": "../other_level/manifest.json#their_portal"
        }
    }
}
```

`meta.name`, `level.model`, and exactly one portal with `"spawn": true` are required. Everything else is optional.

### Materials

Materials referenced by the level model can have entries in `level.material`. A material entry can be a static texture:

```json
{ "image": "texture.png" }
```

Or an animated texture that cycles through a series of frames:

```json
{ "images": ["frame1.png", "frame2.png", "frame3.png"], "animation_speed": 0.1 }
```

### Requirements

If a level material is missing from `level.material`, the engine uses the glTF material base color and creates a fallback 64x64 texture automatically. If `level.lightmap` is missing, the engine uses a fallback 64x64 white lightmap. Texture dimensions must be one of the following sizes, each with a maximum number of textures per level:

| Size      | Max |
|-----------|-----|
| 2048x2048 | 1   |
| 1024x1024 | 4   |
| 512x512   | 8   |
| 256x256   | 32  |
| 128x128   | 64  |
| 64x64     | 256 |

### Tips

- Keep vertex counts low — every vertex is processed per frame.
- Avoid geometric seams — vertices that should meet must share the exact same position. Small gaps or overlaps cause collision detection issues.
- Use a separate collider mesh for complex scenes. This lets you include non-collidable geometry (e.g. grass, decorations) in your model without affecting physics.

### Portals

- Portal geometry must be a flat rectangular quad (4 unique coplanar vertices).
- UVs must use the standard `(0,0)→(1,0)→(1,1)→(0,1)` corner layout.
- Linked portals must have matching dimensions.
- **Wall portals** have a vertical surface with arbitrary yaw. The UV `(0,0)→(1,0)` edge must be horizontal. Wall portals can only link to other wall portals.
- **Floor/ceiling portals** have a horizontal surface. Floor portals link to ceiling portals and vice versa. The UV `(0,0)→(1,0)` edge determines the portal's forward direction — the player's facing is rotated to match the destination portal's orientation.
- The `link` field is a relative URL where the fragment (`#name`) identifies the destination portal name.

### Publishing

Levels are entirely static, so any static file host works for distribution. GitHub Pages is a simple, free option. For better load times, consider putting your levels behind a CDN.

## Thanks

- [JDWasabi](https://jdwasabi.itch.io/8-bit-16-bit-sound-effects-pack) — Sound effects
- [timmycakes](https://gamebanana.com/sounds/19212) — Walking sound effects
- [Jayvee Enaguas](https://www.dafont.com/pixel-operator.font) — Font
- Ji-Hoon Myung - SVG logo
