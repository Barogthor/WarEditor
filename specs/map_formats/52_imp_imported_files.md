# war3map.imp - The Imported File List

This file contains a list of all custom files imported into the map archive, ensuring they persist through World Editor saves.

## File Format

### Header

| Field | Type | Description |
|-------|------|-------------|
| File Format Version | int | Format version number |
| Imported File Count | int | Number of imported files (n) |

### Import Entry Structure

For each imported file:

| Field | Type | Description |
|-------|------|-------------|
| Path Type | byte | Path format indicator |
| File Path | string | Path inside the MPQ archive |

## Path Type Values

| Value | Path Type | Description |
|-------|-----------|-------------|
| 5, 8 | Standard Path | Uses default "war3mapImported\" prefix |
| 10, 13 | Custom Path | Complete custom path specified |

### Path Examples

#### Standard Path (Type 5 or 8)
- **Storage**: Path stored without prefix
- **Example**: "mysound.wav"
- **Full Path**: "war3mapImported\mysound.wav"

#### Custom Path (Type 10 or 13)
- **Storage**: Complete path as specified
- **Example**: "war3mapImported\sounds\ambient\mysound.wav"
- **Full Path**: Same as stored path

## File Persistence

### World Editor Protection
- **Purpose**: Prevents automatic cleanup of imported files
- **Behavior**: Files listed in .imp survive World Editor save operations
- **Without .imp**: Custom files may be removed during editor saves

### Import Process
1. **File Addition**: Custom files are imported into MPQ archive
2. **Path Registration**: File paths are added to .imp list
3. **Persistence**: Files remain through subsequent editor operations

## Campaign Files

### war3campaign.imp
- **Location**: Campaign archives (.w3n files)
- **Format**: Identical structure to map format
- **Path Difference**: Uses "war3campaignImported\" instead of "war3mapImported\"

### Path Comparison

| File Type | Standard Prefix |
|-----------|-----------------|
| Maps (.w3m/.w3x) | war3mapImported\ |
| Campaigns (.w3n) | war3campaignImported\ |

## Usage Scenarios

### Custom Assets
- **Textures**: Custom BLP/TGA image files
- **Models**: Custom MDX/MDL 3D models
- **Sounds**: Custom WAV/MP3 audio files

### File Organization
```
war3mapImported\
├── textures\
│   ├── custom_ui.blp
│   └── terrain_blend.tga
├── models\
│   ├── custom_hero.mdx
│   └── special_effect.mdl
└── sounds\
    ├── ambient\
    │   └── forest_loop.wav
    └── effects\
        └── spell_cast.wav
```
