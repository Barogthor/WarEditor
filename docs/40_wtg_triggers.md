# WTG File Format Specification

## Overview
The WTG file format is used to store trigger data in Warcraft III maps. This document describes the binary structure of WTG files.

## Root Structure

| Field | Type | Description |
|-------|------|-------------|
| File ID | `char[4]` | File identifier: "WTG!" |
| File Format Version | `int32` | Version number:<br>• 4 = Reign of Chaos<br>• 7 = Frozen Throne |
| Number of Categories | `int32` | Count "x" of trigger categories |
| Trigger Categories | `TriggerCategory[x]` | Array of trigger categories |
| Unknown | `int32` | Always 0 |
| Number of Variables | `int32` | Count "y" of variables |
| Variables | `Variable[y]` | Array of variables |
| Number of Triggers | `int32` | Count "z" of triggers |
| Triggers | `Trigger[z]` | Array of triggers |

## TriggerCategory Structure

| Field | Type | Description |
|-------|------|-------------|
| ID | `int32` | Category identifier |
| Name | `string` | Category name |
| Is Comment | `int32` | **Version 7 only**<br>• 1 = yes<br>• 0 = no |

## Variable Structure

| Field | Type | Description |
|-------|------|-------------|
| Name | `string` | Variable name |
| Type | `string` | Variable type |
| Unknown | `int32` | Always 1 |
| Is Array | `int32` | • 1 = yes<br>• 0 = no |
| Array Size | `int32` | **Version 7 only** |
| Is Initialized | `int32` | • 1 = yes<br>• 0 = no |
| Initial Value | `string` | Initial value |

## Trigger Structure

| Field | Type | Description |
|-------|------|-------------|
| Name | `string` | Trigger name |
| Description | `string` | Trigger description |
| Is Comment | `int32` | **Version 7 only**<br>• 0 = no<br>• any other = yes |
| Is Enabled | `int32` | • 1 = yes<br>• 0 = no |
| Is Custom | `int32` | • 1 = yes<br>• 0 = no |
| Is Initially Off | `int32` | • 1 = yes<br>• 0 = no |
| Run on Initialization | `int32` | • 1 = yes<br>• 0 = no |
| Trigger Category ID | `int32` | References TriggerCategory ID |
| Total ECA Count | `int32` | Count "x" of events/conditions/actions |
| ECAs | `ECA[x]` | Array of events, conditions, and actions |

## ECA Structure

| Field | Type | Description |
|-------|------|-------------|
| Type | `int32` | • 0 = event<br>• 1 = condition<br>• 2 = action |
| Group | `int32` | **Child ECA only**<br>• 0 = condition<br>• 1 = then action<br>• 2 = else action |
| Name | `string` | Name "x" |
| Is Enabled | `int32` | • 1 = yes<br>• 0 = no |
| Parameters | `Parameter[y]` | Parameters (count from lookup table) |
| Child ECA Count | `int32` | **Version 7 only**<br>Count "z" of child ECAs |
| Child ECAs | `ECA[z]` | **Version 7 only**<br>Array of child ECAs |

## Parameter Structure

The parameter structure varies between versions due to slight differences.

### Version 4 (Reign of Chaos)

| Field | Type | Description |
|-------|------|-------------|
| Type | `int32` | • 0 = PRESET<br>• 1 = VARIABLE<br>• 2 = FUNCTION<br>• 3 = STRING<br>• -1 = INVALID |
| Value | `string` | Parameter value |
| Has Sub Parameters | `int32` | • 1 = yes<br>• 0 = no |
| Sub Parameters | `SubParameters` | **Only if has sub parameters** |
| Unknown | `int32` | **Only if type = FUNCTION**<br>Always 0 |
| Is Array | `int32` | **Only if type ≠ 2**<br>• 1 = yes<br>• 0 = no |
| Array Index | `Parameter` | **Only if is array** |

### Version 7 (Frozen Throne)

| Field | Type | Description |
|-------|------|-------------|
| Type | `int32` | • 0 = PRESET<br>• 1 = VARIABLE<br>• 2 = FUNCTION<br>• 3 = STRING<br>• -1 = INVALID |
| Value | `string` | Parameter value |
| Has Sub Parameters | `int32` | • 1 = yes<br>• 0 = no |
| Sub Parameters | `SubParameters` | **Only if has sub parameters** |
| Unknown | `int32` | **Only if has sub parameters**<br>Always 0 |
| Is Array | `int32` | • 1 = yes<br>• 0 = no |
| Array Index | `Parameter` | **Only if is array** |

## SubParameters Structure

| Field | Type | Description |
|-------|------|-------------|
| Type | `int32` | Type identifier |
| Name | `string` | Name "x" |
| Begin Parameters | `int32` | • 0 = no<br>• any other = yes |
| Parameters | `Parameter[z]` | Parameters (count from lookup table) |

## Version Differences

The main differences between Version 4 (RoC) and Version 7 (TFT):

1. **TriggerCategory**: Version 7 adds `Is Comment` field
2. **Variable**: Version 7 adds `Array Size` field
3. **Trigger**: Version 7 adds `Is Comment` field
4. **ECA**: Version 7 adds child ECA support
5. **Parameter**: Version 7 has different conditional field logic