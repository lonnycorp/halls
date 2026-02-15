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
        "name": "My Level"
    },
    "level": {
        "mesh": "mesh.glb",
        "surface": {
            "MyMaterial": {
                "collider": "Wall",
                "type": "TextureSingle",
                "frame": "texture.png",
                "color": [255, 255, 255, 255],
                "unlit": false
            }
        }
    },
    "portal": {}
}
```

Required fields:

- `_version` (must be `"coco"`)
- `meta.name`
- `level.mesh`
- `level.surface`
- `portal`

Optional fields:

- `meta.author`
- `meta.track`
- `level.spawn`
- `level.lightmap`
- `level.track`

Limits:

- `portal` may be empty (`{}`), but cannot contain more than 4 entries.

### Manifest Fields

- `meta.name`: level name shown in UI.
- `meta.author`: optional author credit shown in UI.
- `meta.track`: optional track credit shown in UI.
- `level.mesh`: level mesh (`.glb`), used for both rendering and collision.
- `level.spawn`: optional player spawn position `[x, y, z]` (defaults to origin).
- `level.track`: optional background music file.
- `level.lightmap`: optional lightmap texture.
- `level.surface`: required surface map keyed by glTF surface name.
- `portal`: required portal map (can be empty), max 4 entries.
- `portal.<name>.mesh`: portal mesh (`.glb`).
- `portal.<name>.link`: relative URL to destination manifest with `#portal_name` fragment.

### Surface Types

Each `level.surface.<surface_name>` entry is one of the following:

Single texture:

```json
{
    "collider": "Wall",
    "type": "TextureSingle",
    "frame": "texture.png",
    "color": [255, 255, 255, 255],
    "unlit": false
}
```

Animated texture:

```json
{
    "collider": "Null",
    "type": "TextureMulti",
    "frames": ["frame1.png", "frame2.png", "frame3.png"],
    "animation_speed": 0.1,
    "color": [255, 255, 255, 255],
    "unlit": false
}
```

Flat color:

```json
{
    "collider": "Ladder",
    "type": "Untextured",
    "color": [64, 200, 255, 255],
    "unlit": false
}
```

Invisible:

```json
{
    "collider": "Wall",
    "type": "Invisible"
}
```

Per-type field rules:

- `TextureSingle`: required `frame`; optional `collider`, `color`, `unlit`.
- `TextureMulti`: required `frames` (must be non-empty) and `animation_speed`; optional `collider`, `color`, `unlit`.
- `Untextured`: required `color`; optional `collider`, `unlit`.
- `Invisible`: optional `collider` only.

Defaults and behavior:

- `collider` defaults to `Wall` when omitted.
- `color` defaults to white for `TextureSingle`/`TextureMulti`.
- `unlit` defaults to `false` when omitted.
- If `unlit` is `true`, the surface is not multiplied by the level lightmap.
- If `unlit` is `false`, final color is multiplied by the lightmap when a lightmap is present.

### Material Mapping

- `level.surface` keys should match material names in the level `.glb`.
- If a mesh material has no matching surface entry, that geometry is skipped for rendering and level collision.

### Texture Constraints

Texture dimensions must be one of the following sizes, each with a maximum number of textures per level:

| Size      | Max |
|-----------|-----|
| 2048x2048 | 1   |
| 1024x1024 | 4   |
| 512x512   | 8   |
| 256x256   | 32  |
| 128x128   | 64  |
| 64x64     | 256 |

### Portals

- Portal geometry can be any coplanar polygon (not just a rectangle).
- Portals must be either **wall-aligned** (vertical surface) or **floor/ceiling-aligned** (horizontal surface).
- Portal orientation is defined by a single vertex colored `MAGENTA`.
- The `link` field is a relative URL where the fragment (`#name`) identifies the destination portal name.

#### Linking Criteria

- **Wall** portals can only link to **wall** portals.
- **Floor** portals link to **ceiling** portals, and **ceiling** portals link to **floor** portals.
- Linked portals must have the same polygon shape. Shape compatibility is validated using a fingerprint.

### Tips

- Keep vertex counts low — every vertex is processed per frame.
- Avoid geometric seams — vertices that should meet must share the exact same position. Small gaps or overlaps cause collision detection issues.
- Keep portal polygons convex. Concave portal layouts surrounded by convex level geometry can cause players to snag on seams.
- Keep open space on both sides of each portal. The teleport only triggers after the player has already crossed the portal plane, so blocking geometry too close to either face can prevent crossing.
- Use a separate collider mesh for complex scenes. This also lets you include non-collidable geometry (e.g. grass, decorations) in your model without affecting physics.

## Thanks

- [JDWasabi](https://jdwasabi.itch.io/8-bit-16-bit-sound-effects-pack) — Sound effects
- [timmycakes](https://gamebanana.com/sounds/19212) — Walking sound effects
- [Jayvee Enaguas](https://www.dafont.com/pixel-operator.font) — Font
- [Ji-Hoon Myung](https://github.com/edwardmyung) - SVG logo
