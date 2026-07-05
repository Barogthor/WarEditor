# war3mapMisc.txt, war3mapSkin.txt, war3mapExtra.txt - The Global Settings Files

**Note**: These files are exclusive to the Frozen Throne expansion pack.

These files contain global map configuration settings stored in standard INI file format.

## File Format

### INI Structure
All three files use the standard INI format:
- **Sections**: Defined by `[sectionname]`
- **Values**: Set by `valuename=value` statements
- **Text Format**: Plain text files readable with any text editor

### Character Encoding
- **Format**: Plain text (ASCII/UTF-8)
- **Line Endings**: Windows-style (CRLF) or Unix-style (LF)
- **Comments**: Not typically used in these files

## File Purposes

### war3mapMisc.txt - Gameplay Constants
Contains data from the **Gameplay Constants** screen in the World Editor.

**Purpose**: Modifies core game mechanics and balance values
**Examples**:
- Unit movement speeds
- Resource gathering rates
- Combat damage multipliers
- Building construction times
- Hero experience requirements

### war3mapSkin.txt - Game Interface
Contains changes from the **Game Interface** screen in the World Editor.

**Purpose**: Customizes the user interface appearance and behavior
**Examples**:
- Custom UI textures
- Button layouts
- Interface colors
- Menu modifications
- HUD customizations

### war3mapExtra.txt - External Data Sources
Contains settings from the last tab in the **Map Properties** screen.

**Purpose**: References external data sources and environmental settings
**Examples**:
- External object data files (.w3o)
- Custom sky models
- Import file references
- Third-party asset integration

## INI Format Structure

### Section Format
```ini
[SectionName]
key1=value1
key2=value2
key3=value3
```

### Value Types
- **Strings**: `name=Custom Map Name`
- **Numbers**: `speed=1.5`
- **Booleans**: `enabled=1` (1=true, 0=false)
- **Paths**: `file=war3mapImported\custom.mdx`

## Usage in World Editor

### Map Properties Integration
- **Automatic Generation**: World Editor creates these files when settings are modified
- **Loading**: Files are read when map is opened in editor
- **Validation**: Editor validates syntax and values

### Setting Persistence
- **Save Behavior**: Files are updated when map is saved
- **Default Values**: Missing keys use game defaults
- **Override Logic**: Values override built-in game constants

