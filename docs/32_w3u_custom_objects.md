# war3map.w3u - The Custom Units File & Object Data Files

These files store modifications made in the Object Editor. All object data files share a common format structure.

## Original Format (Version 1)

### Header

| Field | Size | Description |
|-------|------|-------------|
| W3U Version | int | 1 |
| Original Units Table | Variable | Modifications to Blizzard's original units |
| User-Created Units Table | Variable | Custom units created by map designer |

### Table Definition

| Field | Type | Description |
|-------|------|-------------|
| Object Count | int | Number of objects in this table (n) |

> **Note**: Even if no changes are made to the original table, this value must be present. If 0, skip the default object table.

Followed by `n` object definition structures.

### Object Definition Structure

| Field | Size | Description |
|-------|------|-------------|
| Original Object ID | char[4] | Base object ID (from game data files) |
| New Object ID | char[4] | Custom object ID (0 if modifying original) |
| Modification Count | int | Number of modifications (m) |

Followed by `m` modification structures.

### Modification Structure

| Field | Size | Description |
|-------|------|-------------|
| Modification ID | char[4] | Property ID (from metadata files) |
| Variable Type | int | Data type (see variable types below) |
| Value | Variable | Modification value (size depends on variable type) |
| End Marker | int | Usually 0 |

### Variable Types (Original Format)

| Value | Type | Description |
|-------|------|-------------|
| 0 | int | Integer value |
| 1 | real | Float value |
| 2 | unreal | Float value (0 ≤ val ≤ 1) |
| 3 | string | Null-terminated string |
| 4 | bool | Boolean value |
| 5 | char | Single character |
| 6 | unitList | List of unit IDs |
| 7 | itemList | List of item IDs |
| 8 | regenType | Regeneration type |
| 9 | attackType | Attack type |
| 10 | weaponType | Weapon type |
| 11 | targetType | Target type |
| 12 | moveType | Movement type |
| 13 | defenseType | Defense type |
| 14 | pathingTexture | Pathing texture |
| 15 | upgradeList | List of upgrades |
| 16 | stringList | List of strings |
| 17 | abilityList | List of abilities |
| 18 | heroAbilityList | List of hero abilities |
| 19 | missileArt | Missile art |
| 20 | attributeType | Attribute type |
| 21 | attackBits | Attack bits |

---

## Frozen Throne Format (Enhanced)

### Common Structure for All Object Files

All object data files (w3u, w3t, w3b, w3d, w3a, w3h, w3q) share this format:

#### Header

| Field | Type | Description |
|-------|------|-------------|
| File Version | int | Usually 1 |
| Original Objects Table | Variable | Standard Blizzard objects |
| Custom Objects Table | Variable | User-created objects |

> **Note**: Same structure for Table definition and Object definition.

#### Enhanced Modification Structure

| Field | Type | Description |
|-------|------|-------------|
| Modification ID | char[4] | Property ID |
| Variable Type | int | Data type |
| **Level/Variation** | int | Optional: Level or variation (file-dependent) |
| **Data Pointer** | int | Optional: Data column pointer (file-dependent) |
| Value | Variable | Modification value |
| End Marker | int | Validation marker |

### Variable Types (Frozen Throne)

| Value | Type | Format | Description |
|-------|------|--------|-------------|
| 0 | Integer | int | Whole numbers |
| 1 | Real | float | Single precision floating point |
| 2 | Unreal | float | Normalized float (0.0 to 1.0) |
| 3 | String | string | Null-terminated string |

### Object Data File Types

| Extension | Object Type | Object IDs Source | Modification IDs Source | Uses Optional Ints |
|-----------|-------------|-------------------|-------------------------|-------------------|
| **w3u** | Units | Units\UnitData.slk | Units\UnitMetaData.slk | No |
| **w3t** | Items | Units\ItemData.slk | Units\UnitMetaData.slk (useItem=1) | No |
| **w3b** | Destructables | Units\DestructableData.slk | Units\DestructableMetaData.slk | No |
| **w3d** | Doodads | Doodads\Doodads.slk | Doodads\DoodadMetaData.slk | Yes |
| **w3a** | Abilities | Units\AbilityData.slk | Units\AbilityMetaData.slk | Yes |
| **w3h** | Buffs | Units\AbilityBuffData.slk | Units\AbilityBuffMetaData.slk | No |
| **w3q** | Upgrades | Units\UpgradeData.slk | Units\UpgradeMetaData.slk | Yes |

### Optional Integers Usage

**Level/Variation:**
- **Abilities & Upgrades**: Ability/upgrade level
- **Doodads**: Variation number
- **Others**: Set to 0 if not applicable

**Data Pointer (Abilities only):**
- Maps to AbilityData.slk columns:
  - 0 = DataA, 1 = DataB, 2 = DataC, 3 = DataD
  - 4 = DataF, 5 = DataG, 6 = DataH
- Example: DataA3 → level=3, data pointer=0

### End Marker Validation

The end marker can be:
- 0 (most common)
- Original object ID
- New object ID

**Usage**: For validation when reading, use new object ID when writing.

## Campaign Files

These files also exist in campaign archives with identical format:
- `war3campaign.w3u` / `w3t` / `w3b` / `w3d` / `w3a` / `w3h` / `w3q`

## W3O Compilation Format

The `.w3o` file combines all object data files into a single file for external data source usage.

### W3O Structure

| Field | Type | Description |
|-------|------|-------------|
| File Version | int | Currently 1 |
| Contains Unit Data | int | 1=yes, 0=no |
| **[Unit Data]** | w3u file | Complete w3u file (if present) |
| Contains Item Data | int | 1=yes, 0=no |
| **[Item Data]** | w3t file | Complete w3t file (if present) |
| Contains Destructable Data | int | 1=yes, 0=no |
| **[Destructable Data]** | w3b file | Complete w3b file (if present) |
| Contains Doodad Data | int | 1=yes, 0=no |
| **[Doodad Data]** | w3d file | Complete w3d file (if present) |
| Contains Ability Data | int | 1=yes, 0=no |
| **[Ability Data]** | w3a file | Complete w3a file (if present) |
| Contains Buff Data | int | 1=yes, 0=no |
| **[Buff Data]** | w3h file | Complete w3h file (if present) |
| Contains Upgrade Data | int | 1=yes, 0=no |
| **[Upgrade Data]** | w3q file | Complete w3q file (if present) |

### W3O Usage

- **Export**: Generated when exporting all object data from Object Editor
- **External Source**: Can be selected as external data source in map properties
- **Location**: Must be in same folder as the map using it

## Reference Files

All object IDs and modification IDs are found in the corresponding `.slk` files within `war3.mpq`:

- **Unit Data**: `Units\UnitData.slk`, `Units\UnitMetaData.slk`
- **Item Data**: `Units\ItemData.slk`
- **Destructable Data**: `Units\DestructableData.slk`, `Units\DestructableMetaData.slk`
- **Doodad Data**: `Doodads\Doodads.slk`, `Doodads\DoodadMetaData.slk`
- **Ability Data**: `Units\AbilityData.slk`, `Units\AbilityMetaData.slk`
- **Buff Data**: `Units\AbilityBuffData.slk`, `Units\AbilityBuffMetaData.slk`
- **Upgrade Data**: `Units\UpgradeData.slk`, `Units\UpgradeMetaData.slk`