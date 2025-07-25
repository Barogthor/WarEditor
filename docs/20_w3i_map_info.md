# war3map.w3i - The Info File

This file contains map information displayed when starting a game.

## Original Format (Version 18)

### Header

| Field | Type | Description |
|-------|------|-------------|
| File Format Version | int | 18 |
| Number of Saves | int | Map version |
| Editor Version | int | Editor version |
| Map Name | string | Map name |
| Map Author | string | Map author |
| Map Description | string | Map description |
| Players Recommended | string | Recommended player count |

### Map Bounds

| Field | Type | Description |
|-------|------|-------------|
| Camera Bounds | float[8] | Camera bounds as defined in JASS |
| Camera Bounds Complements | int[4] | Ints A, B, C, D (see map size formula) |
| Map Playable Width | int | E value (see map size formula) |
| Map Playable Height | int | F value (see map size formula) |

**Map Size Formula:**
- Map width = A + E + B
- Map height = C + F + D

### Map Flags

| Field | Type | Description |
|-------|------|-------------|
| Flags | int | Map behavior flags |

| Flag | Value | Description |
|------|--------|-------------|
| Hide Minimap | 0x0001 | Hide minimap in preview screens |
| Modify Ally Priorities | 0x0002 | Modify ally priorities |
| Melee Map | 0x0004 | Melee map |
| Large Map Size | 0x0008 | Playable map size was large and never reduced |
| Masked Areas Visible | 0x0010 | Masked areas are partially visible |
| Fixed Player Settings | 0x0020 | Fixed player setting for custom forces |
| Use Custom Forces | 0x0040 | Use custom forces |
| Use Custom Techtree | 0x0080 | Use custom techtree |
| Use Custom Abilities | 0x0100 | Use custom abilities |
| Use Custom Upgrades | 0x0200 | Use custom upgrades |
| Properties Menu Opened | 0x0400 | Map properties menu opened since creation |
| Water Waves on Cliffs | 0x0800 | Show water waves on cliff shores |
| Water Waves on Rolling | 0x1000 | Show water waves on rolling shores |

### Additional Map Data

| Field | Type | Description |
|-------|------|-------------|
| Main Ground Type | char | Tileset ID ('A' = Ashenvale, 'X' = City Dalaran) |
| Campaign Background | int | Campaign background number (-1 = none) |
| Loading Screen Text | string | Loading screen text |
| Loading Screen Title | string | Loading screen title |
| Loading Screen Subtitle | string | Loading screen subtitle |
| Loading Screen Number | int | Loading screen number (-1 = none) |
| Prologue Text | string | Prologue screen text |
| Prologue Title | string | Prologue screen title |
| Prologue Subtitle | string | Prologue screen subtitle |

### Dynamic Arrays

| Field | Type | Description |
|-------|------|-------------|
| Max Players | int | Number of player entries (MAXPL) |
| **Player Data** | Player[MAXPL] | Player definitions |
| Max Forces | int | Number of force entries (MAXFC) |
| **Force Data** | Force[MAXFC] | Force definitions |
| Upgrade Changes Count | int | Number of upgrade changes (UCOUNT) |
| **Upgrade Changes** | UpgradeChange[UCOUNT] | Upgrade availability changes |
| Tech Changes Count | int | Number of tech changes (TCOUNT) |
| **Tech Changes** | TechChange[TCOUNT] | Tech availability changes |
| Random Unit Tables Count | int | Number of unit tables (UTCOUNT) |
| **Random Unit Tables** | UnitTable[UTCOUNT] | Random unit table definitions |

## Sub-Structures

### Player Data Format

| Field | Type | Description |
|-------|------|-------------|
| Internal Player Number | int | Internal player ID |
| Player Type | int | 1=Human, 2=Computer, 3=Neutral, 4=Rescuable |
| Player Race | int | 1=Human, 2=Orc, 3=Undead, 4=Night Elf |
| Fixed Start Position | int | 0x00000001 = fixed start position |
| Player Name | string | Player name |
| Starting X | float | Starting coordinate X |
| Starting Y | float | Starting coordinate Y |
| Ally Low Priorities | int | Ally low priority flags (bit x = player x) |
| Ally High Priorities | int | Ally high priority flags (bit x = player x) |

### Force Data Format

| Field | Type | Description |
|-------|------|-------------|
| Force Flags | int | Force behavior flags |
| Player Mask | int | Player membership (bit x = player x in force) |
| Force Name | string | Force name |

#### Force Flags

| Flag | Value | Description |
|------|--------|-------------|
| Allied | 0x00000001 | Allied force |
| Allied Victory | 0x00000002 | Allied victory |
| Share Vision | 0x00000004 | Share vision |
| Share Unit Control | 0x00000010 | Share unit control |
| Share Advanced Control | 0x00000020 | Share advanced unit control |

### Upgrade Availability Change Format

| Field | Type | Description |
|-------|------|-------------|
| Player Flags | int | Affected players (bit x = player x) |
| Upgrade ID | char[4] | Upgrade ID (from UpgradeData.slk) |
| Level | int | Upgrade level (actual level - 1) |
| Availability | int | 0=unavailable, 1=available, 2=researched |

### Tech Availability Change Format

| Field | Type | Description |
|-------|------|-------------|
| Player Flags | int | Affected players (bit x = player x) |
| Tech ID | char[4] | Technology ID (item, unit, or ability) |

> **Note**: If a tech ID is in this list, it's unavailable (no availability value needed).

### Random Unit Table Format

| Field | Type | Description |
|-------|------|-------------|
| Group Count | int | Number of random groups (n) |

For each group:
| Field | Type | Description |
|-------|------|-------------|
| Group Number | int | Group identifier |
| Group Name | string | Group name |
| Position Count | int | Number of positions/columns (m) |
| Position Types | int[m] | Position types (0=unit, 1=building, 2=item) |
| Unit Count | int | Number of table rows (i) |

For each row:
| Field | Type | Description |
|-------|------|-------------|
| Chance | int | Spawn chance percentage |
| Unit IDs | char[m × 4] | Unit/item IDs for each position |

> **Note**: Unit/item ID of `0x00000000` indicates no unit/item created.

---

## Frozen Throne Format (Version 25)

### Enhanced Header

Same as version 18, but with **File Format Version = 25** and additional fields:

| Field | Type | Description |
|-------|------|-------------|
| ... | ... | (Same as version 18 up to flags) |
| **Enhanced Flags** | int | Includes additional unknown flags |
| Main Ground Type | char | Same as version 18 |
| **Loading Screen Background** | int | Background index (-1 = none/custom) |
| **Custom Loading Screen Path** | string | Custom loading screen model path |
| Loading Screen Text | string | Same as version 18 |
| Loading Screen Title | string | Same as version 18 |
| Loading Screen Subtitle | string | Same as version 18 |
| **Game Data Set** | int | Used game data set (0 = standard) |
| **Prologue Path** | string | Prologue screen path (usually empty) |
| **Prologue Text** | string | Prologue screen text (usually empty) |
| **Prologue Title** | string | Prologue screen title (usually empty) |
| **Prologue Subtitle** | string | Prologue screen subtitle (usually empty) |

#### Enhanced Flags (Additional)

| Flag | Value | Description |
|------|--------|-------------|
| Unknown 1 | 0x2000 | Unknown purpose |
| Unknown 2 | 0x4000 | Unknown purpose |
| Unknown 3 | 0x8000 | Unknown purpose |

### Environment Settings

| Field | Type | Description |
|-------|------|-------------|
| Terrain Fog | int | Fog usage (0 = none, >0 = fog style index) |
| Fog Start Z | float | Fog start height |
| Fog End Z | float | Fog end height |
| Fog Density | float | Fog density |
| Fog Red | byte | Fog red component |
| Fog Green | byte | Fog green component |
| Fog Blue | byte | Fog blue component |
| Fog Alpha | byte | Fog alpha component |
| Global Weather ID | int | Weather ID (0 = none, else 4-letter ID from Weather.slk) |
| Sound Environment | string | Custom sound environment label |
| Light Environment | char | Custom light environment tileset ID |
| Water Tint Red | byte | Water tinting red component |
| Water Tint Green | byte | Water tinting green component |
| Water Tint Blue | byte | Water tinting blue component |
| Water Tint Alpha | byte | Water tinting alpha component |

### Enhanced Arrays

Same structure as version 18, but with additional:

| Field | Type | Description |
|-------|------|-------------|
| Random Item Tables Count | int | Number of item tables (ITCOUNT) |
| **Random Item Tables** | ItemTable[ITCOUNT] | Random item table definitions |

### Random Item Table Format

| Field | Type | Description |
|-------|------|-------------|
| Table Count | int | Number of random item tables (n) |

For each table:
| Field | Type | Description |
|-------|------|-------------|
| Table Number | int | Table identifier |
| Table Name | string | Table name |
| Item Set Count | int | Number of item sets (m) |

For each item set:
| Field | Type | Description |
|-------|------|-------------|
| Item Count | int | Number of items in set (i) |

For each item:
| Field | Type | Description |
|-------|------|-------------|
| Chance | int | Drop chance percentage |
| Item ID | char[4] | Item ID (from ItemData.slk) |

## Reference Files

- **Upgrade data**: UpgradeData.slk
- **Item data**: ItemData.slk
- **Weather data**: TerrainArt\Weather.slk