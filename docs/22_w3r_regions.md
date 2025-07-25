# war3map.w3r - The Trigger Regions File

This file defines rectangular regions on the map that can be used in triggers for events and conditions.

## File Format

### Header

| Field | Type | Description |
|-------|------|-------------|
| Version | int | Always 5 |
| Region Count | int | Number of region definition structures (n) |

### Region Definition Structure

| Field | Type | Description |
|-------|------|-------------|
| Left | float | Left boundary (JASS coordinates) |
| Right | float | Right boundary (JASS coordinates) |
| Bottom | float | Bottom boundary (JASS coordinates) |
| Top | float | Top boundary (JASS coordinates) |
| Region Name | string | Region identifier name |
| Region Index | int | Creation number (unique identifier) |
| Weather Effect ID | char[4] | Weather effect identifier |
| Ambient Sound | string | Sound ID name (references w3s file) |
| Region Color | byte[3] | Display color in World Editor (BGR format) |
| End Marker | byte | Structure terminator |

## Coordinate System

### JASS Coordinates
- **Origin**: Bottom-left corner of the map
- **Units**: Map coordinate system units
- **Rectangle Definition**: Left < Right, Bottom < Top

### Boundary Definition
- **Left/Right**: X-axis boundaries
- **Bottom/Top**: Y-axis boundaries
- **Area**: Rectangular region defined by these four coordinates

## Weather Effects

### Weather Effect ID Format
- **Type**: 4-character identifier
- **Example**: "RLlr" = "Rain Lordaeron Light Rain"
- **Disabled**: All characters set to 0 (null)

### Common Weather Effects
Weather IDs reference effects defined in the game's weather system.

