# war3map.wai - The Artificial Intelligence File

**Note**: This file format is exclusive to the Frozen Throne expansion pack.

This file defines AI behavior, strategies, and configuration for computer-controlled players.

## File Format

### Header

| Field | Type | Description |
|-------|------|-------------|
| File Format Version | int | Always 2 |
| AI Name | string | AI script identifier |
| Race | int | AI race preference |
| Options | int | AI behavior flags |

### Race Values

| Value | Race |
|-------|------|
| 0 | Custom |
| 1 | Human |
| 2 | Orc |
| 3 | Undead |
| 4 | Night Elf |

### AI Options (Bitmask)

Most options are in the HiWord, except where noted:

| Option | Value | Description |
|--------|-------|-------------|
| SetPlayerName | 0x2000 | Set custom player name |
| Melee | 0x0001 | Melee AI behavior |
| DefendUsers | 0x8000 | Defend user players |
| RandomPaths | 0x4000 | Use random pathfinding |
| TargetHeroes | 0x0002 | Prioritize hero targets |
| RepairStructures | 0x0004 | Repair damaged buildings |
| HeroesFlee | 0x0008 | Heroes flee when low on health |
| UnitsFlee | 0x0010 | Units flee when damaged |
| GroupsFlee | 0x0020 | Groups flee together |
| HaveNoMercy | 0x0040 | Aggressive behavior |
| IgnoreInjured | 0x0080 | Ignore injured units |
| RemoveInjuries | 0x1000 | Heal injured units |
| TakeItems | 0x0100 | Pick up items |
| BuyItems | 0x0001 (LoWord) | Purchase items |
| SlowHarvesting | 0x0200 | Reduced harvesting speed |
| AllowHomeChanges | 0x0400 | Allow base relocation |
| SmartArtillery | 0x0800 | Intelligent siege unit usage |

## Worker and Building Configuration

| Field | Type | Description |
|-------|------|-------------|
| Worker/Building Count | int | Should be 4 |
| Gold Worker | char[4] | Gold harvesting unit ID |
| Wood Worker | char[4] | Lumber harvesting unit ID |
| Base Building | char[4] | Main base structure ID |
| Mine Building | char[4] | Resource building ID (or same as base) |

## Conditions System

| Field | Type | Description |
|-------|------|-------------|
| Condition Count | int | Number of condition definitions (a) |
| Unknown | int | Always 7 |
| **Conditions** | Condition[a] | Condition definitions |

### Condition Definition Structure

| Field | Type | Description |
|-------|------|-------------|
| Condition Index | int | Unique identifier |
| Condition Name | string | Human-readable name |
| Has Condition | int | 0=empty, 1=has condition |

**If Has Condition = 1:**
| Field | Type | Description |
|-------|------|-------------|
| Operator Function | string | Function name |
| Begin Function | int | Always 1 |
| **Parameters** | Parameter[x] | Function parameters (count hardcoded) |
| End Function | int | Always 0 |

### Parameter Structure

| Field | Type | Description |
|-------|------|-------------|
| Type | int | 0=preset, 1=operator, 2=function, 3=string |
| Value | string | Parameter value |
| Begin Function | int | 1=function/operator, 0=other |

**Nested Function Parameters:**
- **If function with non-empty value**: Nested parameter structure
- **If operator function**: Additional parameter structures

## Hero Configuration

### Hero Selection

| Field | Type | Description |
|-------|------|-------------|
| First Hero | char[4] | Primary hero unit ID (null if none) |
| Second Hero | char[4] | Secondary hero unit ID (null if none) |
| Third Hero | char[4] | Tertiary hero unit ID (null if none) |

### Training Order Probabilities

Six integers representing percentage chances for different hero training orders:
1. First → Second → Third
2. First → Third → Second  
3. Second → First → Third
4. Second → Third → First
5. Third → First → Second
6. Third → Second → First

### Skill Selection Order

For each hero in each position (9 total combinations):
| Field | Type | Description |
|-------|------|-------------|
| Skill IDs | char[4×10] | Ten skill IDs for ability learning order |

## Priority Systems

### Build Priorities

| Field | Type | Description |
|-------|------|-------------|
| Priority Type | int | Always 0 |
| Build Type | int | 0=unit, 1=upgrade, 2=expansion |
| Unit/Upgrade ID | char[4] | Target ID ("XEIA" for expansion) |
| Town | int | Target location (see town values) |
| Condition Index | int | Condition reference |
| **Condition** | ConditionDef | Inline condition (without index) |

### Harvest Priorities

| Field | Type | Description |
|-------|------|-------------|
| Priority Type | int | Always 1 |
| Harvest Type | int | 0=gold, 1=lumber |
| Town | int | Target location |
| Workers | int | Worker allocation (see worker values) |
| Condition Index | int | Condition reference |
| **Condition** | ConditionDef | Inline condition |

### Target Priorities

| Field | Type | Description |
|-------|------|-------------|
| Priority Type | int | Always 2 |
| Target Type | int | Target selection (see target types) |
| Creep Min Strength | int | Minimum creep strength (0xFFFFFFFF if not creeps) |

**If Target Type = 5 (creep camp):**
| Field | Type | Description |
|-------|------|-------------|
| Creep Max Strength | int | Maximum creep strength |
| Allow Flyers | int | 0=no, 1=yes |

| Field | Type | Description |
|-------|------|-------------|
| Condition Index | int | Condition reference |
| **Condition** | ConditionDef | Inline condition |

## Town Values

| Value | Description |
|-------|-------------|
| 0 | Main base |
| 1-9 | Expansion #1-9 |
| 0xFFFFFFFD-0xFFFFFFF5 | Current mine #1-9 |
| 0xFFFFFFFF | Any location |

## Worker Values

| Value | Description |
|-------|-------------|
| 0-90 | Fixed worker count |
| 0xFFFFFFFF | All workers |
| 0xFFFFFFFE | All non-attacking workers |

## Target Types

| Value | Description |
|-------|-------------|
| 0 | Common alliance target |
| 1 | New expansion location |
| 2 | Enemy major assault |
| 3 | Enemy expansion |
| 4 | Enemy any town |
| 5 | Creep camp |
| 6 | Purchase goblin zeppelin |

## Attack System

### Attack Configuration

| Field | Type | Description |
|-------|------|-------------|
| Repeats Waves | int | Wave repetition setting |
| Minimum Forces | int | Attack group index (or "HAIA" for first hero) |
| Initial Delay | int | Delay before first attack |
| Attack Group Count | int | Number of attack groups (f) |
| **Attack Groups** | AttackGroup[f] | Group definitions |
| Attack Wave Count | int | Number of attack waves (g) |
| **Attack Waves** | AttackWave[g] | Wave definitions |

### Attack Group Structure

| Field | Type | Description |
|-------|------|-------------|
| Group Index | int | Unique identifier |
| Group Name | string | Human-readable name |
| Unit Count | int | Number of unit types (g) |
| **Units** | GroupUnit[g] | Unit specifications |

### Group Unit Structure

| Field | Type | Description |
|-------|------|-------------|
| Unit ID | char[4] | Unit type ("1HIA"=first hero, "2HIA"=second, "3HIA"=third) |
| Quantity | int | Desired quantity (0xFFFFFFFF=all) |
| Max Quantity | int | Maximum quantity |
| Condition Index | int | Condition reference |
| **Condition** | ConditionDef | Inline condition |

### Attack Wave Structure

| Field | Type | Description |
|-------|------|-------------|
| Attack Group Index | int | References attack group |
| Delay | int | Wave timing delay |

## Game Configuration

| Field | Type | Description |
|-------|------|-------------|
| Unknown | int | Always 1 |
| Game Options | int | Game setting flags |
| Game Speed | int | Regular game speed |
| Map Path | string | Path to map file |
| Player Count | int | Number of players (0-2) (h) |
| **Players** | Player[h] | Player definitions |
| Unknown | int | Unknown purpose |

### Game Options

| Option | Value | Description |
|--------|-------|-------------|
| Disable Fog of War | 0x0001 | Remove fog of war |
| Disable Victory/Defeat | 0x0002 | Disable win/lose conditions |

### Player Structure

| Field | Type | Description |
|-------|------|-------------|
| Player Index | int | Player slot number |
| Team Number | int | Team assignment |
| Race | int | Player race (see race values) |
| Color | int | Player color (see color values) |
| Handicap | int | Handicap percentage (0-100) |
| AI Type | int | AI controller type |
| AI Difficulty | int | AI skill level |
| Custom AI Path | string | Path to custom AI script |

### Race Values (Player)

| Value | Race |
|-------|------|
| 1 | Human |
| 2 | Orc |
| 4 | Night Elf |
| 8 | Undead |
| 20 | Random |

### Color Values

| Value | Color |
|-------|-------|
| 0 | Red |
| 1 | Blue |
| 2 | Teal |
| 3 | Purple |
| 4 | Yellow |
| 5 | Orange |
| 6 | Green |
| 7 | Pink |
| 8 | Gray |
| 9 | Light Blue |
| 10 | Dark Green |
| 11 | Brown |

### AI Types

| Value | Type |
|-------|------|
| 0 | Standard |
| 1 | User |
| 4 | Custom |
| 12 | Current |

### AI Difficulty

| Value | Difficulty |
|-------|-----------|
| 0 | Easy |
| 1 | Normal |
| 2 | Insane |

---

## File Structure Storage Order

The following section provides a complete overview of how data is stored sequentially in the WAI file:

### Main File Structure

| Field | Type | Description |
|-------|------|-------------|
| File Format Version | int | Always 2 |
| AI Name | string | AI script identifier |
| Race | int | AI race preference |
| Options | int | AI behavior flags |
| Worker/Building Count | int | Should be 4 |
| Gold Worker ID | char[4] | Gold harvesting unit ID |
| Wood Worker ID | char[4] | Lumber harvesting unit ID |
| Base Building ID | char[4] | Main base structure ID |
| Mine Building ID | char[4] | Resource building ID |
| Condition Count | int | Number of condition definitions (a) |
| Unknown | int | Always 7 |
| **Conditions** | Condition[a] | Array of Condition Definition structures |
| First Hero ID | char[4] | Primary hero unit ID (null if none) |
| Second Hero ID | char[4] | Secondary hero unit ID (null if none) |
| Third Hero ID | char[4] | Tertiary hero unit ID (null if none) |
| Training Order Probabilities | int[6] | Hero training order percentages |
| Skill Selection Orders | char[4×10][9] | Skill learning orders for each hero position |
| Build Priorities Count | int | Number of build priorities (c) |
| **Build Priorities** | BuildPriority[c] | Array of Build Priority structures |
| Harvest Priorities Count | int | Number of harvest priorities (d) |
| **Harvest Priorities** | HarvestPriority[d] | Array of Harvest Priority structures |
| Target Priorities Count | int | Number of target priorities (e) |
| **Target Priorities** | TargetPriority[e] | Array of Target Priority structures |
| Repeats Waves | int | Wave repetition setting |
| Minimum Forces | int | Attack group index (or "HAIA" for first hero) |
| Initial Delay | int | Delay before first attack |
| Attack Groups Count | int | Number of attack groups (f) |
| **Attack Groups** | AttackGroup[f] | Array of Attack Group structures |
| Attack Waves Count | int | Number of attack waves (g) |
| **Attack Waves** | AttackWave[g] | Array of Attack Wave structures |
| Unknown | int | Always 1 |
| Game Options | int | Game setting flags |
| Game Speed | int | Regular game speed |
| Map Path | string | Path to map file |
| Player Count | int | Number of players (h) (0-2) |
| **Players** | Player[h] | Array of Player structures |
| Unknown | int | Unknown purpose |
