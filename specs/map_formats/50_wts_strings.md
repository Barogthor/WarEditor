# war3map.wts - The Trigger String Data File

This is a text file that can be opened with notepad. It defines trigger string replacements for the map.

## How It Works

When Warcraft III encounters a `TRIGSTR_***` (where `***` is a number), it looks in the trigger string table to find the corresponding string and replaces the trigger string with that value.

## Important Rules

- **First Definition Wins**: If you have multiple definitions for the same trigger ID, only the first one counts
- **Positive Numbers Only**: The number following `STRING` must be positive; negative numbers are ignored
- **Default to Zero**: If text follows `STRING` without a valid number, it's considered trigger string 0
- **Reference Format**: For more than 999 strings, the reference simply becomes one character longer

## String Definition Format

```
STRING <ID>
{
<string content>
}
```

Where:
- `STRING` keyword starts the definition
- `<ID>` is the trigger string ID number (must be unique)
- `{` indicates the beginning of the string value
- String content can contain multiple lines
- `}` indicates the end of the trigger string definition

## Example

**In .wts file:**
```
STRING 0
{
Blah blah blah...
}
```

**Usage in other files:**
When Warcraft III finds `TRIGSTR_000` in .j, .w3i, or object editor files, it replaces it with "Blah blah blah...".

---

