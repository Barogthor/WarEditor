# war3map.w3c - The Camera File

This file contains pre-defined camera positions and settings for cinematic sequences.

## File Format

### Header

| Field | Type | Description |
|-------|------|-------------|
| File Version | int | Always 0 |
| Camera Count | int | Number of camera definition structures (n) |

### Camera Definition Structure

| Field | Type | Description |
|-------|------|-------------|
| Target X | float | Camera target X coordinate |
| Target Y | float | Camera target Y coordinate |
| Z Offset | float | Height offset from target |
| Rotation | float | Camera rotation angle (degrees) |
| Angle of Attack (AoA) | float | Camera pitch angle (degrees) |
| Distance | float | Distance from target |
| Roll | float | Camera roll rotation |
| Field of View (FoV) | float | Camera field of view (degrees) |
| Far Clipping (FarZ) | float | Far clipping plane distance |
| Unknown | float | Unknown purpose (usually set to 100) |
| Cinematic Name | string | Name identifier for the camera |

## Camera Parameters

### Position and Target
- **Target X/Y**: World coordinates where the camera is looking
- **Z Offset**: Vertical offset from the target point
- **Distance**: How far the camera is from the target

### Orientation
- **Rotation**: Horizontal rotation around the target (0° = north)
- **Angle of Attack**: Vertical angle (0° = horizontal, positive = looking down)
- **Roll**: Camera rotation around its viewing axis

### Viewing Properties
- **Field of View**: Camera's viewing angle (typical range: 45°-90°)
- **Far Clipping**: Maximum rendering distance

### Usage
- **Cinematic Name**: Used to reference this camera in triggers and scripts
- **Pre-defined Positions**: Cameras can be set up in the World Editor for cinematic sequences

## Coordinate System

- **World Coordinates**: Uses the same coordinate system as the map
- **Angles**: Measured in degrees
- **Distance Units**: Same units as map coordinates

## Integration with Triggers

Cameras defined in this file can be referenced by name in trigger actions to create smooth cinematic sequences and cutscenes.