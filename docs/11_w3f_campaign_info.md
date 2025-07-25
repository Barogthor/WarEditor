# war3campaign.w3f - The Campaign Info File

This file contains campaign metadata and configuration for Warcraft III campaigns (.w3n files).

## File Format

### Header

| Field | Type | Description |
|-------|------|-------------|
| File Format Version | int | Currently 1 |
| Campaign Version | int | Save count (how many times campaign has been saved) |
| Editor Version | int | Version of World Editor used |
| Campaign Name | string | Campaign display name |
| Campaign Difficulty | string | Difficulty description |
| Author Name | string | Campaign creator |
| Campaign Description | string | Campaign description text |

### Configuration Flags

| Field | Type | Description |
|-------|------|-------------|
| Difficulty + Expansion Flag | int | Combined difficulty and map type flags |

#### Difficulty + Expansion Flag Values

| Value | Description |
|-------|-------------|
| 0 | Fixed Difficulty, Only w3m maps |
| 1 | Variable Difficulty, Only w3m maps |
| 2 | Fixed Difficulty, Contains w3x maps |
| 3 | Variable Difficulty, Contains w3x maps |

### Visual Settings

| Field | Type | Description |
|-------|------|-------------|
| Background Screen Index | int | Preset background (-1 = none/custom) |
| Custom Background Path | string | Path to custom background (empty if preset/none) |
| Minimap Picture Path | string | Path to minimap image (empty = none) |

### Audio Settings

| Field | Type | Description |
|-------|------|-------------|
| Ambient Sound Index | int | Sound selection (-1=imported, 0=none, >0=preset) |
| Custom Ambient Sound Path | string | Path to custom MP3 file |

### Environmental Settings

| Field | Type | Description |
|-------|------|-------------|
| Uses Terrain Fog | int | Fog usage (0=none, >0=fog style index) |
| Fog Start Z Height | float | Fog start altitude |
| Fog End Z Height | float | Fog end altitude |
| Fog Density | float | Fog thickness |
| Fog Red | byte | Red color component (0-255) |
| Fog Green | byte | Green color component (0-255) |
| Fog Blue | byte | Blue color component (0-255) |
| Fog Alpha | byte | Alpha transparency (0-255) |

### Interface Settings

| Field | Type | Description |
|-------|------|-------------|
| Cursor and UI Race Index | int | UI theme (0=Human, 1=Orc, 2=Undead, 3=Night Elf) |

### Map Configuration

| Field | Type | Description |
|-------|------|-------------|
| Map Count | int | Number of maps in campaign (n) |
| **Map Titles** | MapTitle[n] | Map title information |
| Flow Chart Map Count | int | Number of maps in flow chart (m, usually = n) |
| **Map Order** | MapOrder[m] | Map flow sequence |

## Sub-Structures

### Map Title Structure

| Field | Type | Description |
|-------|------|-------------|
| Is Visible | int | 1=visible from start, 0=initially hidden |
| Chapter Title | string | Chapter/section name |
| Map Title | string | Individual map name |
| Map Path | string | Path within campaign archive |

### Map Order Structure

| Field | Type | Description |
|-------|------|-------------|
| Unknown | string | Always empty (possibly reserved) |
| Map Path | string | Path within campaign archive |

---

## File Structure Storage Order

The following section provides the complete sequential order of data storage in the W3F file:

### Main File Structure

| Field | Type | Description |
|-------|------|-------------|
| File Format Version | int | Currently 1 |
| Campaign Version | int | Save count |
| Editor Version | int | World Editor version used |
| Campaign Name | string | Campaign display name |
| Campaign Difficulty | string | Difficulty description |
| Author Name | string | Campaign creator name |
| Campaign Description | string | Campaign description text |
| Difficulty + Expansion Flag | int | Combined flags (0-3) |
| Background Screen Index | int | Preset background index (-1=custom/none) |
| Custom Background Path | string | Path to custom background (empty if preset) |
| Minimap Picture Path | string | Path to minimap image (empty=none) |
| Ambient Sound Index | int | Sound selection (-1=imported, 0=none, >0=preset) |
| Custom Ambient Sound Path | string | Path to custom MP3 file |
| Uses Terrain Fog | int | Fog usage (0=none, >0=style index) |
| Fog Start Z Height | float | Fog start altitude |
| Fog End Z Height | float | Fog end altitude |
| Fog Density | float | Fog thickness |
| Fog Red | byte | Red color component (0-255) |
| Fog Green | byte | Green color component (0-255) |
| Fog Blue | byte | Blue color component (0-255) |
| Fog Alpha | byte | Alpha transparency (0-255) |
| Cursor and UI Race Index | int | UI theme (0=Human, 1=Orc, 2=Undead, 3=Night Elf) |
| Map Count | int | Number of maps in campaign (n) |
| **Map Titles** | MapTitle[n] | Array of Map Title structures |
| Flow Chart Map Count | int | Number of maps in flow chart (m) |
| **Map Orders** | MapOrder[m] | Array of Map Order structures |
