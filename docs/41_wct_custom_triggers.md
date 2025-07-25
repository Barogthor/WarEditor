# war3map.wct - The Custom Text Trigger File

This file stores custom JASS code for triggers that have been converted to custom text format, complementing the GUI trigger definitions in the WTG file.

## Original Format (Version 0)

### Header

| Field | Type | Description |
|-------|------|-------------|
| File Version | int | Always 0 |
| Trigger Count | int | Number of triggers (n) |

### Custom Text Trigger Structure

| Field | Type | Description |
|-------|------|-------------|
| Text Size | int | Size of text including null terminator (s) |
| Custom Text | string | JASS code string (s characters with null terminator) |

### Important Notes

- **Order Correspondence**: Custom text triggers follow the same order as triggers in the WTG file
- **All Triggers Included**: Every trigger must have an entry, even GUI triggers
- **Empty Entries**: Non-custom triggers have size = 0 (only the 4-byte size integer)
- **Null Termination**: Text size includes the null terminating character

---

## Frozen Throne Format (Version 1)

### Enhanced Header

| Field | Type | Description |
|-------|------|-------------|
| File Version | int | Always 1 |
| Custom Script Comment | string | Global custom script code comment |
| Global Custom Text | CustomTextStructure | Global custom script text |
| Trigger Count | int | Number of triggers (n) |

### Custom Text Trigger Structure

Same structure as Version 0:

| Field | Type | Description |
|-------|------|-------------|
| Text Size | int | Size of text including null terminator (s) |
| Custom Text | string | JASS code string (s characters with null terminator) |

### Enhanced Features

#### Global Custom Script
- **Purpose**: Code that runs globally, not tied to specific triggers
- **Location**: Appears before individual trigger custom text entries
- **Usage**: Global variables, functions, and initialization code

#### Custom Script Comment
- **Purpose**: Documentation for the global custom script section
- **Format**: Standard string field
- **Usage**: Helps identify the purpose of global custom code
