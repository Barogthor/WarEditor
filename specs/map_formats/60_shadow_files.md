# war3map.shd - The Shadow Map File

This file contains shadow information for the map and has no header, only raw data.

## File Structure

- **File size**: `16 × map_width × map_height` bytes
- **Data format**: Raw binary data

## Shadow Data

Each byte can have 2 values:
- `00h` = No shadow
- `FFh` = Shadow

**Shadow Resolution**: Each byte controls the shadow status of 1/16 of a tileset, meaning each tileset is divided into 16 parts (4×4 grid).

