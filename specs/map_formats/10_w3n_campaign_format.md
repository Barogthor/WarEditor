# W3N Campaign File Format

*Thanks to PitzerMike for the original documentation*

## Overview

A W3N file is a Warcraft III Campaign file. These campaign files are only available in The Frozen Throne expansion pack.

## File Structure

### Header and Footer
W3N files use the same structure as map files:
- **512-byte header** - Same format as W3M files
- **MPQ archive** - Contains campaign data files
- **260-byte footer** - Authentication purposes

> **Note:** See the [W3M specification](W3M_W3X_Files_Format.md) for detailed information about the header and footer structure.

## Campaign File Contents

The MPQ archive within a W3N file can contain the following files:

### Standard MPQ Files
- `(listfile)` - File listing for the archive
- `(signature)` - Digital signature data
- `(attributes)` - File attributes

### Campaign-Specific Files

#### Custom Object Data
- **`war3campaign.w3u`** - Custom units data
- **`war3campaign.w3t`** - Custom items data  
- **`war3campaign.w3a`** - Custom abilities data
- **`war3campaign.w3b`** - Custom destructables data
- **`war3campaign.w3d`** - Custom doodads data
- **`war3campaign.w3q`** - Custom upgrades data

#### Campaign Information
- **`war3campaign.w3f`** - Campaign information file (detailed below)

#### Imported Assets
- **`war3campaign.imp`** - Imported files list
- **`war3campaignImported\*.*`** - Imported asset files

## Campaign Information File (war3campaign.w3f)

The `war3campaign.w3f` file contains campaign-specific metadata and configuration.

> **Note:** For detailed specifications of the other custom object files (w3u, w3t, w3a, etc.), refer to the [W3M specification](W3M_W3X_Files_Format.md) as they use the same format as their map file counterparts.

## File Format Details

*The specific binary format of the war3campaign.w3f file would be documented here based on the original specification.*

## Usage Notes

- W3N files are campaign archives that can contain multiple maps
- Custom object data in campaign files affects all maps within the campaign
- The campaign information file defines the campaign structure and map progression
- Imported files are shared across all maps in the campaign

## Related Documentation

- **[W3M/W3X Files Format](02_w3m_w3x_format_overview.md)** - Map file format (shares header/footer structure)
- **[Custom Objects Data](32_w3u_custom_objects.md)** - Format for custom object files
- **[Imported Files](52_imp_imported_files.md)** - Imported assets format