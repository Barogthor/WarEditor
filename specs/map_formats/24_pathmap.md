# war3mapPath.tga/war3map.wpm - Pathmap Files

Only one of these two files is used for pathing. The choice depends on the Warcraft III version:
- **Old War3 beta versions (≤1.21)**: Use `war3mapPath.tga`
- **Since beta 1.30**: Use `war3map.wpm`

## 6.1) war3mapPath.tga - The Image Path File

A standard 32-bit RGB TGA file with no compression and a black alpha channel.

### Critical Requirements

- **Format**: Standard 32-bit RGB TGA
- **Compression**: None
- **Alpha channel**: Required (black)
- **Importance**: If Warcraft III doesn't recognize the format, it will cause tileset issues (like blight everywhere)

### File Dimensions

- **TGA width**: `map_width × 4`
- **TGA height**: `map_height × 4`
- **Resolution**: Each tile divided into 16 pixels (4×4 grid, same as shadow file)
- **Coordinate system**: Top-left corner of image = upper-left corner of map

### TGA Header Format (18 bytes)

| Field | Size | Value | Description |
|-------|------|--------|-------------|
| ID Length | 1 byte | 0 | |
| Color Map Type | 1 byte | 0 | |
| Image Type | 1 byte | 2 | Uncompressed RGB |
| **Color Map Specification** | **5 bytes** | | |
| First Entry Index | 2 bytes | 0 | |
| Color Map Length | 2 bytes | 0 | |
| Color Map Entry Size | 1 byte | 0 | |
| **Image Specification** | **10 bytes** | | |
| X Origin | 2 bytes | 0 | |
| Y Origin | 2 bytes | 0 | |
| Image Width | 2 bytes | (little endian) | |
| Image Height | 2 bytes | (little endian) | |
| Pixel Depth | 1 byte | 32 (0x20) | |
| Image Descriptor | 1 byte | 0x28 | 0x20=top-left start, 0x08=8-bit alpha |

**Header Example**: `00 00 02 00 00 00 00 00 00 00 00 00 XX XX YY YY 20 28`
(where XX XX = width, YY YY = height)

### Pixel Data Format

Each pixel = 4 bytes: `BB GG RR AA`
- **BB**: Blue value (0 or 255)
- **GG**: Green value (0 or 255)  
- **RR**: Red value (0 or 255)
- **AA**: Alpha channel (set to 0)

There are 4×4 pixels for 1 tileset.

### Path Color Codes

| Color | Build | Walk | Fly |
|-------|--------|------|-----|
| **White** | No build | No walk | No fly |
| **Red** | Build OK | No walk | Fly OK |
| **Yellow** | Build OK | No walk | No fly |
| **Green** | Build OK | Walk OK | No fly |
| **Cyan** | No build | Walk OK | No fly |
| **Blue** | No build | Walk OK | Fly OK |
| **Magenta** | No build | No walk | Fly OK |
| **Black** | Build OK | Walk OK | Fly OK |

### Color Logic Summary

- **Red set** = "No walk"
- **Green set** = "No fly"  
- **Blue set** = "No build"
- **Alpha channel**: Black = normal, White = blight

## 6.2) war3map.wpm - The Path Map File

The newer binary format for pathing information.

### Header

| Field | Type | Value | Description |
|-------|------|--------|-------------|
| File ID | char[4] | 'MP3W' | |
| File Version | int | 0 | |
| Path Map Width | int | `map_width × 4` | |
| Path Map Height | int | `map_height × 4` | |

### Data Section

- **Size**: `(map_height × 4) × (map_width × 4)` bytes
- **Format**: Each byte represents a part of a tileset (same as TGA format)

### Flag Bits

| Bit | Flag | Description |
|-----|------|-------------|
| 0x01 | (unused) | 0 |
| 0x02 | Walk | 1=no walk, 0=walk OK |
| 0x04 | Fly | 1=no fly, 0=fly OK |
| 0x08 | Build | 1=no build, 0=build OK |
| 0x10 | (unused) | 0 |
| 0x20 | Blight | 1=blight, 0=normal |
| 0x40 | Water | 1=no water, 0=water |
| 0x80 | Unknown | 1=unknown, 0=normal |

### Common Values

| Value | Usage |
|-------|--------|
| 0x00 | Bridge doodad |
| 0x08 | Shallow water |
| 0x0A | Deep water |
| 0x40 | Normal ground |
| 0x48 | Water ramp, unbuildable grounds, unbuildable doodad parts |
| 0xCA | Cliff edges, solid doodad parts (no build and no walk) |
| 0xCE | Map boundaries |