# How this project is organized

## Frontends

Frontends are contained within the src/bin/ directory, currently plans are to support a cli and a QML based frontend. These frontends are expected to be a thin layer of code over decker-lib

## Application Logic

The core application logic that is general to decker rather than particular to any game resides within the src/lib/decker-lib library.

## Game Specific Logic

All game specific logic resides within src/games/{gamename} directories and will be interfaced with using trait objects.
