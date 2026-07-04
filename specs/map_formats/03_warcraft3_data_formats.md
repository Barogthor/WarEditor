# Warcraft 3 Data Format

Blizzard uses several standardized data types in its files. This section describes the common data types and their binary representation.

## Data Types

### Integers

Integers are stored using 4 bytes in **Little Endian** order, meaning the first byte read is the lowest byte.

- **Type**: Similar to C++ `int` (signed)
- **Size**: 4 bytes
- **Range**: Standard 32-bit signed integer range
- **Example**: 1234 decimal = `[00 00 04 D2]h` stored as `[D2 04 00 00]h`

### Short Integers

Short integers are stored using 2 bytes in **Little Endian** order.

- **Type**: Similar to C++ `signed short` but with modified range
- **Size**: 2 bytes  
- **Range**: -16,384 to 16,383 (2 highest bits are available for flags)

### Floats

Floats use the standard IEEE 32-bit float format stored in **Little Endian** order.

- **Type**: Equivalent to C++ `float`
- **Size**: 4 bytes
- **Example**: 7654.32 decimal becomes 7654.319824 (closest representable value) = `[45 EF 32 8F]h` stored as `[8F 32 EF 45]h`

### Characters and Character Arrays

- **Characters**: Standard 1 byte per character
- **Character Arrays**: Usually 4 bytes, no null termination required
- **Size**: 1 byte (char), typically 4 bytes (array)

### Strings and Trigger Strings

Strings are null-terminated character arrays (`'\0'`). Blizzard uses special control codes for color formatting.

#### Color Codes
- Format: `|c00BBGGRR`
- BB, GG, RR are hexadecimal values (2 digits each) for blue, green, and red
- Example: `"blah |c000080FFblah"` displays "blah blah" with the second "blah" in orange

#### Trigger Strings
Strings starting with `"TRIGSTR_"` (case sensitive) are trigger strings that reference entries in the trigger string table.

**Rules:**
- `TRIGSTR_7`, `TRIGSTR_07`, `TRIGSTR_007`, `TRIGSTR_7abc` all refer to trigger string #7
- `TRIGSTR_ab7`, `TRIGSTR_abc`, `TRIGSTR_` refer to trigger string #0
- `TRIGSTR_-7` (negative) refers to an empty string
- Convention: `TRIGSTR_` followed by 3 digits plus null terminator

**Sizes:**
- Regular string: variable length + 1 (null terminator)
- Trigger string: 12 bytes

#### UTF-8 Encoding

Warcraft uses UTF-8 for internationalization support.

**UTF-8 to UCS Conversion:**
```
If FirstByte <= 191: return FirstByte
If 192 <= FirstByte <= 223: return (FirstByte - 192) * 64 + (SecondByte - 128)
If 224 <= FirstByte <= 239: return (FirstByte - 224) * 4096 + (SecondByte - 128) * 64 + (ThirdByte - 128)
If 240 <= FirstByte <= 247: return (FirstByte - 240) * 262144 + (SecondByte - 128) * 4096 + (ThirdByte - 128) * 64 + (FourthByte - 128)
If 248 <= FirstByte <= 251: return (FirstByte - 248) * 16777216 + (SecondByte - 128) * 262144 + (ThirdByte - 128) * 4096 + (FourthByte - 128) * 64 + (FifthByte - 128)
If 252 <= FirstByte: return (FirstByte - 252) * 1073741824 + (SecondByte - 128) * 16777216 + (ThirdByte - 128) * 262144 + (FourthByte - 128) * 4096 + (FifthByte - 128) * 64 + (SixthByte - 128)
```

**UCS to UTF-8 Conversion:**
```
If ASCII <= 127: FirstByte = ASCII
If 128 <= ASCII <= 2047: FirstByte = 192 + (ASCII \ 64), SecondByte = 128 + (ASCII Mod 64)
If 2048 <= ASCII <= 65535: FirstByte = 224 + (ASCII \ 4096), SecondByte = 128 + ((ASCII \ 64) Mod 64), ThirdByte = 128 + (ASCII Mod 64)
If 65536 <= ASCII <= 2097151: FirstByte = 240 + (ASCII \ 262144), SecondByte = 128 + ((ASCII \ 4096) Mod 64), ThirdByte = 128 + ((ASCII \ 64) Mod 64), FourthByte = 128 + (ASCII Mod 64)
If 2097152 <= ASCII <= 67108863: FirstByte = 248 + (ASCII \ 16777216), SecondByte = 128 + ((ASCII \ 262144) Mod 64), ThirdByte = 128 + ((ASCII \ 4096) Mod 64), FourthByte = 128 + ((ASCII \ 64) Mod 64), FifthByte = 128 + (ASCII Mod 64)
If 67108864 <= ASCII <= 2147483647: FirstByte = 252 + (ASCII \ 1073741824), SecondByte = 128 + ((ASCII \ 16777216) Mod 64), ThirdByte = 128 + ((ASCII \ 262144) Mod 64), FourthByte = 128 + ((ASCII \ 4096) Mod 64), FifthByte = 128 + ((ASCII \ 64) Mod 64), SixthByte = 128 + (ASCII Mod 64)
```

> **Note**: UTF-8 conversion is only needed for text display or user input. For all other purposes, UTF-8 strings can be treated as ordinary strings.

### Flags

Flags are boolean values stored as bits within integers.

- **Storage**: 4 bytes (32 bits = 32 flags)
- **Type**: Each bit represents a boolean value (true/false, 1/0)
- **Implementation**: Blizzard uses integers to store flag collections

### Custom Types

Sometimes integers and flags share the same bytes for space efficiency.

**Example**: In W3E file format, water level and 2 flags share 4 bytes:
- 2 highest bits: flags
- Remaining 30 bits: water level (reduced value range)

A single byte can also contain multiple different data fields.

### Structures

Warcraft 3 uses structured data types of various sizes for complex data organization.