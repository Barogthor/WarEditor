# war3map.w3s - The Sounds Definition File

This file defines custom sounds that can be used in triggers, regions, and other map elements.

## File Format

### Header

| Field | Type | Description |
|-------|------|-------------|
| File Format Version | int | Always 1 |
| Sound Count | int | Number of sound definitions (n) |

### Sound Definition Structure

| Field | Type | Description |
|-------|------|-------------|
| Sound ID Name | string | Unique identifier (e.g., "gg_snd_HumanGlueScreenLoop1") |
| Sound File | string | File path (e.g., "Sound\Ambient\HumanGlueScreenLoop1.wav") |
| EAX Effects | string | Audio effect preset name |
| Sound Flags | int | Behavior flags (see below) |
| Fade In Rate | int | Fade in speed |
| Fade Out Rate | int | Fade out speed |
| Volume | int | Volume level (-1 = use default) |
| Pitch | float | Pitch modification |
| Unknown 1 | float | Unknown purpose |
| Unknown 2 | int | Unknown purpose (-1 or 8) |
| Channel | int | Audio channel assignment |
| Min Distance | float | 3D audio minimum distance |
| Max Distance | float | 3D audio maximum distance |
| Distance Cutoff | float | 3D audio cutoff distance |
| Unknown 3 | float | Unknown purpose |
| Unknown 4 | float | Unknown purpose |
| Unknown 5 | int | Unknown purpose (-1 or 127) |
| Unknown 6 | float | Unknown purpose |
| Unknown 7 | float | Unknown purpose |
| Unknown 8 | float | Unknown purpose |

## Sound Flags

| Flag | Value | Description |
|------|-------|-------------|
| Looping | 0x00000001 | Sound repeats continuously |
| 3D Sound | 0x00000002 | Positional audio with distance falloff |
| Stop When Out of Range | 0x00000004 | Stop playing when listener moves away |
| Music | 0x00000008 | Treated as music track |
| Unknown | 0x00000010 | Unknown purpose |

## EAX Effects

Environmental Audio Extensions (EAX) presets for different sound types:

| EAX String | Description |
|------------|-------------|
| DefaultEAXON | Default environmental effect |
| CombatSoundsEAX | Combat sound effects |
| KotoDrumsEAX | Drum sound effects |
| SpellsEAX | Spell and magic effects |
| MissilesEAX | Projectile sound effects |
| HeroAcksEAX | Hero speech and acknowledgments |
| DoodadsEAX | Environmental object sounds |

## Audio Channels

| Channel | Value | Description |
|---------|-------|-------------|
| General | 0 | General game sounds |
| Unit Selection | 1 | Unit selection sounds |
| Unit Acknowledgement | 2 | Unit response sounds |
| Unit Movement | 3 | Unit movement sounds |
| Unit Ready | 4 | Unit ready notifications |
| Combat | 5 | Combat and battle sounds |
| Error | 6 | Error and warning sounds |
| Music | 7 | Background music |
| User Interface | 8 | UI interaction sounds |
| Looping Movement | 9 | Continuous movement sounds |
| Looping Ambient | 10 | Ambient environment sounds |
| Animations | 11 | Animation-triggered sounds |
| Constructions | 12 | Building construction sounds |
| Birth | 13 | Unit creation sounds |
| Fire | 14 | Fire and flame sounds |

## 3D Audio Properties

### Distance Settings
- **Min Distance**: Range where sound is at full volume
- **Max Distance**: Range where sound reaches minimum volume
- **Distance Cutoff**: Maximum range where sound is audible

### Spatial Audio
When **3D Sound** flag is enabled:
- Sound volume decreases with distance
- Sound can be positioned in 3D space
- Left/right stereo positioning based on listener location

## Float Value Handling

### Default Values
- **Unset Float**: When a float parameter is not specified
- **Default Marker**: `[4F800000]h` = `4.2949673e+009`
- **Usage**: Game uses internal defaults when this marker is detected

