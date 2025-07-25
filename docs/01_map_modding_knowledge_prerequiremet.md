# Things You Need to Know First to Mod a Map

## 1.1 Your Environment

You'll need Warcraft III Retail installed, for example in `C:\Program Files\Warcraft III\`.

This documentation covers Warcraft III Retail (initial version) files. File formats may vary depending on your version - you may need to figure out differences in the file formats. The new formats from The Frozen Throne expansion are also described where the file format changed.

### Standard Installation Files

If your Warcraft III installation is clean, you should have these files:

**In `C:\Program Files\Warcraft III\`:**
- `war3.mpq`

**In `C:\Program Files\Warcraft III\Maps\`:**
- `(4)Lost Temple.w3m`
- (and many other .w3m files...)

### The Frozen Throne Expansion Files

If you have The Frozen Throne expansion installed, you'll also have:

**Additional MPQ archives:**
- `war3x.mpq`
- `war3xlocal.mpq`

**In `C:\Program Files\Warcraft III\Maps\FrozenThrone\`:**
- `(2)Circumvention.w3x` (as an example)

**In `C:\Program Files\Warcraft III\Campaigns\`:**
- `DemoCampaign.w3n`

**Patch files:**
When installing updates, a new archive is added containing the most up-to-date files:
- `war3patch.mpq`

### Required Tools

#### MPQ Editors

W3M, W3X map files and W3N campaign files can be opened with any MPQ editor that supports Warcraft III. You'll need one of these:

**WinMPQ (ShadowFlare)** - Recommended
- Download: http://shadowflare.samods.org/dwnload.html
- Read the documentation and installation notes

**Ladik's MPQ Editor**
- Download: http://www.zezula.net/en/mpq/download.html
- Simply place in a folder and run

#### Listfiles

If you only see unknown files when opening maps or archives, you need up-to-date listfiles:
- Download latest listfiles: http://www.wc3campaigns.net/tools/weu/stuff.html
- All MPQ editors have settings to specify external listfiles for missing filenames
- Add all listfiles to identify most unknown files

#### Hex Editors

For advanced map file editing:
- **HexWorkshop** (Commercial): http://www.bpsoft.com/
- **XVI32** (Freeware): http://www.chmaas.handshake.de/ (lacks some advanced features)

## 1.2 Warcraft III Files

### 1.2.1 About MPQ Files

MPQ files are archive formats similar to ZIP or RAR files - they contain a directory structure with compressed files.

For detailed MPQ format information, see: http://www.zezula.net/

**For programming with MPQ archives:**
- Use SFmpqapi for your programming language
- Available at: http://shadowflare.samods.org/dwnload.html
- Supports Delphi, C++, Visual Basic, and others
- WinMPQ source code is available to learn SFmpqapi usage

### 1.2.2 Warcraft III File Loading Structure

Warcraft III uses a priority-based file loading system:

#### File Loading Priority

1. **Local directories** (Windows Explorer folders) - *Only if registry key is set*
2. **Map file** (.w3m)
3. **Patch MPQ** (War3Patch.mpq)
4. **Main MPQ** (War3x.mpq or War3xlocal.mpq if expansion installed)
5. **Base MPQ** (War3.mpq)

#### Registry Key for Local Files

To enable local file loading:
- **Path:** `HKEY_CURRENT_USER\Software\Blizzard Entertainment\Warcraft III\`
- **Key name:** `Allow Local Files`
- **Key type:** DWORD
- **Key value:** 1

#### File Modification Guidelines

**✅ DO:**
- Use the same directory/file structure in your Warcraft III installation
- Add files to maps (.w3m) when possible

**❌ DON'T:**
- Modify official MPQs (especially War3.mpq!)

**Note:** WorldEditor reads local files from real directories even if "Allow Local Files" is not enabled in the registry.

#### File Compatibility

**Works well in maps (.w3m):**
- `Units\unitUI.slk`
- `Units\AbilityData.slk`
- `UI\MIDISounds.slk`
- `Units\HumanUnitFunc.txt`
- `Units\HumanUnitStrings.txt`
- `Units\HumanAbilityFunc.txt`
- `Units\HumanAbilityStrings.txt`
- `Units\HumanUpgradeFunc.txt`
- `Units\HumanUpgradeStrings.txt`

**Works poorly in maps:**
- `Units\UnitMetaData.slk`
- `Scripts\Blizzard.j`

**Doesn't work in maps at all:**
- `TerrainArt\CliffTypes.slk`
- `Units\MiscData.txt`

#### Using MPQDraft for System Files

For files that must be loaded outside/before a map loads, create an executable patch with embedded MPQ archive using MPQDraft.

**Example: Adding Custom Cliff Types**

1. Create new MPQ archive
2. Import modified `TerrainArt\CliffTypes.slk` with correct path
3. Ensure listfile entry is added
4. Use MPQDraft to create executable patch:
   - Select archive as source
   - Select Warcraft III as target application
5. Run the created executable to start Warcraft with modified files

#### Important Warnings

⚠️ **Multiplayer Compatibility**
- All players need the same modified files to avoid "netsync error"

⚠️ **File Format Integrity**
- Some files have special formats that can be "falsified" by modification
- Warcraft III will fall back to standard MPQ files if format is invalid
- Test thoroughly to ensure modifications work

⚠️ **File Loading Exceptions**
- Some files outside War3.mpq and War3Patch.mpq won't be used by Warcraft III

### 1.2.3 Map Files (W3M/W3X Files)

To edit a map:

1. **Extract** the .w3m files to a directory
2. **Modify** the extracted files
3. **Repack** them into a new .w3m file

**Important:** Since Warcraft III Retail, W3M files differ from simple MPQ files - they have a header and footer structure.

*See "W3M Files Format" section for detailed format specifications.*

## Further Documentation

### Warcraft III Data Formats
For detailed information about Warcraft III data formats, see:
- **[Warcraft III Data Format](03_warcraft3_data_formats.md)** - Complete specification of game data structures and formats

### W3M/W3X File Format Specifications
For detailed technical specifications of map file formats, see:
- **[W3M/W3X Files Format](02_w3m_w3x_format_overview.md)** - Complete map file format documentation

#### Key Map File Components

**Core Map Files:**
- **[W3I File](20_w3i_map_info.md)** - Map information and metadata
- **[W3E File](21_w3e_environment.md)** - Environment/terrain and tileset data
- **[W3R File](22_w3r_regions.md)** - Regions definition
- **[W3C File](23_w3c_cameras.md)** - Camera bounds and settings
- **[Pathmap File](24_pathmap.md)** - Pathfinding map data
- **[W3U File](32_w3u_custom_objects.md)** - Custom objects data (units, items, abilities, etc.)

**Object Placement:**
- **[DOO File](30_doo_doodads.md)** - Descrutables/Doodad (decoration) placement
- **[Units.doo File](31_units_items_placement.md)** - Unit and item placement

**Scripts and Logic:**
- **[WTG File](40_wtg_triggers.md)** - Trigger data (GUI triggers)
- **[WCT File](41_wct_custom_triggers.md)** - Custom text triggers
- **[JASS File](42_jass_scripts.md)** - JASS2 script code
- **[WAI File](43_wai_ai.md)** - AI data

**Assets and Media:**
- **[WTS File](50_wts_strings.md)** - String table for localization
- **[W3S File](51_w3s_sounds.md)** - Sound definitions
- **[IMP File](52_imp_imported_files.md)** - Imported files list
- **[BLP File](53_blp_minimap.md)** - Minimap image (BLP format)
- **[MMP File](54_mmp_menu_minimap.md)** - Menu minimap settings

**Advanced Files:**
- **[Shadow Files](60_shadow_files.md)** - Shadow map data (Shadow pre-calculation on terrain)
- **[Misc File](61_misc_settings.md)** - Skin and global settings

**Campaign Files (W3N):**
- **[W3N File](10_w3n_campaign_format.md)** - Campaign archive format
- **[W3F File](11_w3f_campaign_info.md)** - Campaign information

