# war3mapUnits.doo - The Unit and Item File

This file contains the definitions and positions of all placed units and items on the map.

## Original Format (Version 7)

### Header

| Field | Type | Value | Description |
|-------|------|--------|-------------|
| File ID | char[4] | "W3do" | |
| File Version | int | 7 | |
| Subversion | int | Often `[09 00 00 00]h` | |
| Number of Units/Items | int | Count of unit and item definitions | |

### Unit/Item Data Structure (Variable length)

| Field | Type | Description |
|-------|------|-------------|
| Type ID | char[4] | Unit/item type ID (`iDNR` = random item, `uDNR` = random unit) |
| Variation | int | Variation number |
| X Coordinate | float | X position |
| Y Coordinate | float | Y position |
| Z Coordinate | float | Z position |
| Rotation Angle | float | Rotation angle |
| X Scale | float | X axis scaling |
| Y Scale | float | Y axis scaling |
| Z Scale | float | Z axis scaling |
| Flags | byte | Unit flags (similar to doodad flags) |
| Player Number | int | Owner (player1 = 0, 16 = neutral passive) |
| Unknown | byte | Usually 0 |
| Unknown | byte | Usually 0 |
| Hit Points | int | HP (-1 = use default) |
| Mana Points | int | MP (-1 = use default, 0 = no mana) |
| Dropped Item Sets Count | int | Number of item drop sets |
| **Dropped Item Sets** | Item drop | Item drop definitions |
| Gold Amount | int | Gold value (default = 12500) |
| Target Acquisition | float | Acquisition range (-1 = normal, -2 = camp) |
| Hero Level | int | Hero level (1 for non-heroes) |
| Inventory Items Count | int | Number of inventory items |
| **Inventory Items** | Inventory Item | Inventory  |
| Modified Abilities Count | int | Number of ability modifications |
| **Modified Abilities** | Ability modification |  |
| Random Flag | int | Random unit/item configuration |
| Custom Color | int | Unit color (-1 = none, 0 = red, 1 = blue, ...) |
| Waygate Destination | int | Target rect ID (-1 = deactivated) |
| Creation Number | int | Unique creation ID |

### Random Unit/Item Values

| Value | Description | Additional Data |
|-------|-------------|-----------------|
| 0 | Any neutral passive building/item | byte[3]: level (-1 = any), byte: item class |
| 1 | Random unit from global group | int: group number, int: position number |
| 2 | Random unit from custom table | int: unit count, then unit structures |

### Sub-Structures

#### Dropped Item Set Format

| Field | Type | Description |
|-------|------|-------------|
| Item Count | int | Number of droppable items |

For each item:
| Field | Type | Description |
|-------|------|-------------|
| Item ID | char[4] | Item type ID (`[00 00 00 00]h` = none) |
| Drop Chance | int | Percentage chance to drop |

#### Inventory Item Format

| Field | Type | Description |
|-------|------|-------------|
| Slot | int | Inventory slot (actual slot - 1, so slot 1 = 0) |
| Item ID | char[4] | Item type ID (`0x00000000` = none) |

#### Ability Modification Format

| Field | Type | Description |
|-------|------|-------------|
| Ability ID | char[4] | Ability type ID (from AbilityData.slk) |
| Active | int | Autocast status (0 = no, 1 = active) |
| Level | int | Ability level (for hero abilities) |

#### Random Unit Format

| Field | Type | Description |
|-------|------|-------------|
| Unit ID | char[4] | Unit type ID (from UnitUI.slk) |
| Choice Chance | int | Percentage chance of selection |

---

## Frozen Throne Format (Version 8)

### Header

| Field | Type | Value | Description |
|-------|------|--------|-------------|
| File ID | char[4] | "W3do" | |
| File Version | int | 8 | |
| Subversion | int | Often `[0B 00 00 00]h` | |
| Number of Units/Items | int | Count of unit and item definitions | |

### Enhanced Unit/Item Data Structure

The structure is similar to version 7, with these additions:

| Field | Type | Description |
|-------|------|-------------|
| ... | ... | (Same as version 7 up to Mana Points) |
| **Map Item Table Pointer** | int | Item table reference (-1 = none, ≥0 = table number) |
| Dropped Item Sets Count | int | Number of item sets (only if table pointer = -1) |
| ... | ... | (Same as version 7 up to Hero Level) |
| **Hero Strength** | int | Strength attribute (0 = use default) |
| **Hero Agility** | int | Agility attribute (0 = use default) |
| **Hero Intelligence** | int | Intelligence attribute (0 = use default) |
| ... | ... | (Rest same as version 7) |

## Random ID Systems

### Random Item IDs

Format: `char[4]` where:
- **1st letter**: "Y"
- **2nd letter**: Item type filter
  - "Y" = any type
  - "i" to "o" = specific item type (ordered by dropdown: "i" = charged)
- **3rd letter**: "I"
- **4th letter**: Level filter
  - "/" = any level (ASCII 47)
  - "0"+ = specific level (ASCII 48 + level, so level 10 = ":", level 15 = "?")

### Random Unit IDs

Format: `char[4]` where:
- **First 3 letters**: "YYU"
- **4th letter**: Level filter
  - "/" = any level (ASCII 47)
  - "0"+ = specific level (ASCII 48 + level)

## Reference Files

- **Unit data**: UnitUI.slk, UnitData.slk
- **Item data**: ItemData.slk
- **Ability data**: AbilityData.slk