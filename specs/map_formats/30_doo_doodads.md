# war3map.doo - The Doodad File for Trees

This file contains tree definitions and positions on the map.

## Original Format (Version 7)

### Header

| Field | Type | Value | Description |
|-------|------|--------|-------------|
| File ID | char[4] | "W3do" | |
| File Version | int | 7 | |
| Subversion | int | Usually `[09 00 00 00]h`, rarely `[07 00 00 00]h` | |
| Number of Trees | int | Count of tree definitions | |

### Tree Data Structure (42 bytes each)

| Field | Size | Description |
|-------|------|-------------|
| Tree ID | 4 bytes | Tree type ID (found in `Units\DestructableData.slk`) |
| Variation | 4 bytes | Variation number (little endian) |
| X Coordinate | 4 bytes | Float: Tree X position on map |
| Y Coordinate | 4 bytes | Float: Tree Y position on map |
| Z Coordinate | 4 bytes | Float: Tree Z position on map |
| Angle | 4 bytes | Float: Rotation angle in radians (degrees = radians × 180 ÷ π) |
| X Scale | 4 bytes | Float: X axis scaling |
| Y Scale | 4 bytes | Float: Y axis scaling |
| Z Scale | 4 bytes | Float: Z axis scaling |
| Flags | 1 byte | Tree state flags |
| Life | 1 byte | Tree life percentage (100% = 0x64, 170% = 0xAA) |
| Editor ID | 4 bytes | Unique tree ID number in World Editor (little endian) |

### Tree Flags

| Value | Description |
|-------|-------------|
| 0 | Invisible and non-solid tree |
| 1 | Visible but non-solid tree |
| 2 | Normal tree (visible and solid) |

### Data Layout Summary

```
tt tt tt tt vv vv vv vv xx xx xx xx yy yy yy yy zz zz zz zz aa aa aa aa xs xs xs xs ys ys ys ys zs zs zs zs ff ll dd dd dd dd
```

Where:
- `tt`: Tree type
- `vv`: Variation
- `xx`: X coordinate
- `yy`: Y coordinate
- `zz`: Z coordinate
- `aa`: Rotation angle
- `xs`: X scale
- `ys`: Y scale
- `zs`: Z scale
- `ff`: Flags
- `ll`: Life
- `dd`: Editor ID number

### Example Tree Data

```
Raw: 4C 54 6C 74 08 00 00 00 00 00 74 45 00 00 70 44 00 10 24 44 E5 CB 96 40 98 85 98 3F 98 85 98 3F 98 85 98 3F 02 64 8D 01 00 00

Parsed:
4C 54 6C 74 → "LTlt" (tree type)
08 00 00 00 → 8 (variation #8)
00 00 74 45 → 3904.0 (X coordinate)
00 00 70 44 → 960.0 (Y coordinate)  
00 10 24 44 → 656.25 (Z coordinate)
E5 CB 96 40 → 4.7123895 radians (270°)
98 85 98 3F → 1.191577 (X scale)
98 85 98 3F → 1.191577 (Y scale)
98 85 98 3F → 1.191577 (Z scale)
02 → Solid and selectable tree
64 → 100% life
8D 01 00 00 → 397 (tree #397 in editor)
```

### Special Doodads Section

After all tree definitions:

| Field | Type | Description |
|-------|------|-------------|
| Format Version | int | Set to 0 |
| Number of Special Doodads | int | Count of special doodads (cliffs, etc.) |

Each special doodad (16 bytes):
| Field | Type | Description |
|-------|------|-------------|
| Doodad ID | char[4] | Doodad type ID |
| Z Value | int | Usually 0 |
| X Coordinate | int | W3E coordinate system |
| Y Coordinate | int | W3E coordinate system |

---

## Frozen Throne Format (Version 8)

### Header

| Field | Type | Value | Description |
|-------|------|--------|-------------|
| File ID | char[4] | "W3do" | |
| File Version | int | 8 | |
| Subversion | int | `[0B 00 00 00]h` | |
| Number of Trees | int | Count of tree definitions | |

### Tree Data Structure (Variable length, usually 50 bytes)

The basic structure is the same as version 7, but with additional fields for item drops:

| Field | Size | Description |
|-------|------|-------------|
| Tree ID | 4 bytes | Tree type ID |
| Variation | 4 bytes | Variation number |
| X, Y, Z Coordinates | 12 bytes | Position (3 floats) |
| Angle | 4 bytes | Rotation angle (float, radians) |
| X, Y, Z Scale | 12 bytes | Scaling (3 floats) |
| Flags | 1 byte | Tree state flags |
| Life | 1 byte | Life percentage (0x64 = 100%, 170% = 0xAA) |
| **Item Table Pointer** | 4 bytes | Random item table reference |
| **Item Set Count** | 4 bytes | Number of custom item sets |
| **Item Sets** | Variable | Item drop definitions (if any) |
| Editor ID | 4 bytes | Unique editor ID |

### Item Table Pointer

- **-1**: No item table
- **≥ 0**: Uses item table with this number (defined in W3I file)

### Item Set Format

If Item Table Pointer is ≥ 0 and Item Set Count > 0:

For each item set:
| Field | Type | Description |
|-------|------|-------------|
| Item Count | int | Number of items in this set |

For each item in the set:
| Field | Type | Description |
|-------|------|-------------|
| Item ID | char[4] | Item ID (from ItemData.slk) or random item ID |
| Drop Chance | int | Percentage chance of dropping |

### Data Layout Summary (Version 8)

```
tt tt tt tt vv vv vv vv xx xx xx xx yy yy yy yy zz zz zz zz aa aa aa aa xs xs xs xs ys ys ys ys zs zs zs zs ff ll bb bb bb bb cc cc cc cc dd dd dd dd
```

Where:
- `tt-zs`: Same as version 7
- `ff`: Flags
- `ll`: Life
- `bb`: Item table pointer
- `cc`: Item set count
- `dd`: Editor ID number

### Special Doodads Section

Same format as version 7.