# war3mapMap.blp - The Minimap Image

BLP stands for "Blip" file (likely "BLIzzard Picture"). There are two types of BLP files:
- **JPG-BLPs**: Use JPG compression
- **Paletted BLPs**: Use palettes with 1 or 2 bytes per pixel

# JPG-BLP Format

## Header

| Field | Type | Description |
|-------|------|-------------|
| File ID | char[4] | "BLP1" |
| Format Type | int | 0 = JPG-BLP, 1 = Paletted |
| Alpha Flag | int | 0x00000008 = has alpha, 0x00000000 = no alpha |
| Image Width | int | Image width in pixels |
| Image Height | int | Image height in pixels |
| Alpha/Team Color Flag | int | Usually 3, 4, or 5 (see below) |
| Unknown Flag | int | Always 0x00000001 (0x00000000 makes textures messy) |
| Mipmap Offsets | int[16] | Offset from beginning of file for each mipmap |
| Mipmap Sizes | int[16] | Size of each mipmap |

## Alpha/Team Color Flag Values

| Value | Description |
|-------|-------------|
| 3, 4 | Color and alpha information (paletted files) |
| 5 | Only color information |
| ≥5 | Won't show team color on 'unit' textures |

## JPG Data Section

| Field | Type | Description |
|-------|------|-------------|
| JPEG Header Size | int | Header size "h" (usually 0x00000270) |
| JPEG Header | byte[h] | JPEG header data |
| Padding | bytes | Zero bytes until JPEG data begins |
| Mipmap Data | byte[16][Mipmap level size] | Raw JPEG data for each mipmap |

**Processing**: With the header and mipmap data, the image can be processed like ordinary JPEG files.

# Paletted BLP Format

## Data Section

| Field | Type | Description |
|-------|------|-------------|
| Color Palette | byte[4×256] | BGRA palette defining 256 colors (1 byte each) |
| Color Indices | byte[width×height] | Color indices for each pixel (top-left to bottom-right) |
| Alpha Indices | byte[width×height] | Alpha values (0=transparent, 255=opaque) |

**Note**: If the picture type flag is set to 5, the image doesn't have an alpha channel, so the alpha indices section is omitted.

## Coordinate System

- **Top-left**: (0, 0)
- **Bottom-right**: (width-1, height-1)
- **Pixel order**: Top-left to bottom-right, row by row

# Technical Notes

## Mipmap Support

BLP files support up to 16 mipmap levels for texture optimization at different distances.

## Color Format

- **BGRA Order**: Blue, Green, Red, Alpha (1 byte each)
- **Alpha Channel**: 0 = fully transparent, 255 = fully opaque
- **Palette**: 256 predefined colors for paletted mode

## Usage Context

- **Minimap**: war3mapMap.blp serves as the minimap image
- **Textures**: BLP format is widely used for game textures
- **Compression**: JPG compression reduces file size while maintaining quality

# Additional Resources

For more detailed BLP specifications, see Magos's documentation at: http://magos.thejefffiles.com/War3ModelEditor/