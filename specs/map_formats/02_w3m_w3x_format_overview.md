# W3M/W3X Files Format

A W3M or W3X file is a Warcraft III Map file (also known as Warcraft III Scenario in the World Editor). It's essentially an MPQ archive using a "new" compression format with a 512-byte header. Official W3M files sometimes include an optional 260-byte footer for authentication purposes.

## File Structure

### Header Format (512 bytes, fixed size)

| Field | Type | Description |
|-------|------|-------------|
| File ID | char[4] | Should be "HM3W" |
| Unknown | int | Purpose unknown |
| Map Name | string | Name of the map |
| Map Flags | int | Same flags as in W3I file (see below) |
| Max Players | int | Maximum number of players |
| Padding | bytes | Zero bytes to fill remaining header space |

### Map Flags

| Flag | Value | Description |
|------|-------|-------------|
| Hide Minimap | 0x0001 | Hide minimap in preview screens |
| Modify Ally Priorities | 0x0002 | Modify ally priorities |
| Melee Map | 0x0004 | Melee map |
| Large Map Size | 0x0008 | Playable map size was large and never reduced to medium |
| Masked Areas Visible | 0x0010 | Masked areas are partially visible |
| Fixed Player Settings | 0x0020 | Fixed player setting for custom forces |
| Use Custom Forces | 0x0040 | Use custom forces |
| Use Custom Techtree | 0x0080 | Use custom techtree |
| Use Custom Abilities | 0x0100 | Use custom abilities |
| Use Custom Upgrades | 0x0200 | Use custom upgrades |
| Properties Menu Opened | 0x0400 | Map properties menu opened at least once since creation |
| Water Waves on Cliffs | 0x0800 | Show water waves on cliff shores |
| Water Waves on Rolling | 0x1000 | Show water waves on rolling shores |

### Footer Format (Optional, 260 bytes)

| Field | Type | Description |
|-------|------|-------------|
| Footer Sign ID | char[4] | Should be "NGIS" ('sign' reversed) |
| Authentication Data | byte[256] | 256 bytes for authentication (usage unknown) |

## MPQ Archive Contents

The MPQ portion can contain the following files:

### System Files
- `(listfile)` - MPQ file listing
- `(signature)` - Digital signature
- `(attributes)` - File attributes

### Core Map Files
- `war3map.w3e` - **[Environment/terrain data](21_w3e_environment.md)**
- `war3map.w3i` - **[Map information](20_w3i_map_info.md)**
- `war3map.w3r` - **[Regions](22_w3r_regions.md)**
- `war3map.w3c` - **[Cameras](23_w3c_cameras.md)**
- `war3map.wpm` - **[Pathing map](24_pathmap.md)**
- `war3mapPath.tga` - **[Pathing map (old format)](24_pathmap.md)**

### Scripts and Logic
- `war3map.wtg` - **[Triggers](40_wtg_triggers.md)**
- `war3map.wct` - **[Custom text triggers](41_wct_custom_triggers.md)**
- `war3map.j` - **[JASS script](42_jass_scripts.md)**
- `war3map.wai` - **[AI data](43_wai_ai.md)**

### Object Placement
- `war3map.doo` - **[Doodads/decorations](30_doo_doodads.md)**
- `war3mapUnits.doo` - **[Units and items](31_units_items_placement.md)**

### Custom Object Data
- `war3map.w3u` - **[Custom units](32_w3u_custom_objects.md)**
- `war3map.w3t` - **[Custom items](32_w3u_custom_objects.md)**
- `war3map.w3a` - **[Custom abilities](32_w3u_custom_objects.md)**
- `war3map.w3b` - **[Custom buffs/effects](32_w3u_custom_objects.md)**
- `war3map.w3d` - **[Custom doodads](32_w3u_custom_objects.md)**
- `war3map.w3q` - **[Custom upgrades](32_w3u_custom_objects.md)**

### Assets and Media
- `war3map.wts` - **[Trigger strings](50_wts_strings.md)**
- `war3map.w3s` - **[Sounds](51_w3s_sounds.md)**
- `war3map.imp` - **[Import definitions](52_imp_imported_files.md)**
- `war3mapImported\*.*` - **[Imported custom files](52_imp_imported_files.md)**
- `war3mapMap.blp` - **[Minimap image (BLP)](53_blp_minimap.md)**
- `war3mapMap.b00` - Minimap (alternative format)
- `war3mapMap.tga` - Minimap (TGA format)
- `war3mapPreview.tga` - Preview image
- `war3map.mmp` - **[Menu minimap](54_mmp_menu_minimap.md)**

### Advanced Files
- `war3map.shd` - **[Shadow map](60_shadow_files.md)**
- `war3mapMisc.txt` - **[Miscellaneous data](61_misc_settings.md)**
- `war3mapSkin.txt` - **[UI skin data](61_misc_settings.md)**
- `war3mapExtra.txt` - **[Extra data](61_misc_settings.md)**