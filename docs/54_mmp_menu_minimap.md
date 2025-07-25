# war3map.mmp - The Menu Minimap

This file contains minimap icon data displayed in the game's map selection screen.

## File Format

### Header

| Field | Type | Description |
|-------|------|-------------|
| Unknown | int | Usually 0 (possibly file format version) |
| Dataset Count | int | Number of icon datasets |

### Icon Dataset Structure (16 bytes each)

| Field | Type | Description |
|-------|------|-------------|
| Icon Type | int | Type of icon to display |
| X Coordinate | int | Icon X position on map |
| Y Coordinate | int | Icon Y position on map |
| Player Color | byte[4] | Icon color (BGRA format) |

## Icon Types

| Value | Icon Type | Description |
|-------|-----------|-------------|
| 0 | Gold Mine | Resource location indicator |
| 1 | House | Building/settlement indicator |
| 2 | Player Start | Starting position (cross symbol) |

## Map Coordinate System

The minimap uses a hexadecimal coordinate system:

| Position | X Coordinate | Y Coordinate |
|----------|--------------|--------------|
| Top Left | 0x10 | 0x10 |
| Center | 0x80 | 0x80 |
| Bottom Right | 0xF0 | 0xF0 |

**Coordinate Range**: 0x10 to 0xF0 (16 to 240 decimal)

## Player Colors

Colors are stored in **BGRA format** (Blue, Green, Red, Alpha):

| Color | BGRA Value | Hex Code |
|-------|------------|----------|
| Red | 03 03 FF FF | `#FF0303` |
| Blue | FF 42 00 FF | `#0042FF` |
| Cyan | B9 E6 1C FF | `#1CE6B9` |
| Purple | 81 00 54 FF | `#540081` |
| Yellow | 00 FC FF FF | `#FFFC00` |
| Orange | 0E 8A FE FF | `#FE8A0E` |
| Green | 00 C0 20 FF | `#20C000` |
| Pink | B0 5B E5 FF | `#E55BB0` |
| Light Gray | 97 96 95 FF | `#959697` |
| Light Blue | F1 BF 7E FF | `#7EBFF1` |
| Aqua | 46 62 10 FF | `#106246` |
| Brown | 04 2A 49 FF | `#492A04` |
| None | FF FF FF FF | `#FFFFFF` |

## Usage Notes

- **Purpose**: Provides visual indicators on the minimap for important locations
- **Display Context**: Shows in game lobby during map selection
- **Icon Positioning**: Icons are positioned relative to the minimap's coordinate system
- **Color Association**: Player colors match standard Warcraft III player color scheme
- **Alpha Channel**: All colors use full alpha (0xFF) for opaque display