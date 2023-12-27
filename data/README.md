FGCD: Data Collection Project
=============================

This directory maintains all of the official game, moves, and platform data for the FGCD project.

All data is manually entered into spreadsheets by community volunteers. The spreadsheets provide a common human-friendly format for gathering officially published data as presented in in-game menus, manuals, and websites.

## Directory structure

### Data directory structure

The README file in each directory will explain further.

```
games/       Fighting Games     (Street Fighter, Mortal Kombat, Tekken, etc.)
platforms/   Gaming Platforms   (Playstation, Xbox, etc.)
devices/     Input devices      (Gamepads, Game sticks, etc.) 
```

### Project directory structure
```
bin/      Tools for processing the data
target/   Where processed data is saved to 
```

## Spreadsheets

Data is saved in the flat-file [OpenDocument](https://en.wikipedia.org/wiki/OpenDocument) spreadsheet format (`.fods`). Using the flat-file format rather than the compressed format (`.ods`) allows us to use the [Git](https://en.wikipedia.org/wiki/Git) source-code manager to properly track historical changes to the files.

Due to this, we ask that all contributions use [LibreOffice](https://www.libreoffice.org) to edit spreadsheet files, so as to avoid superfluous file changes between different editors. LibreOffice, formally OpenOffice, is a popular open-source office suite that is freely available on all desktop platforms. LibreOffice's spreadsheet app, **Calc**, is similar to Microsoft's Excel.

### Spreadsheet columns

The template that the spreadsheets use is similar to our data model, however the spreadsheets are designed to be human-friendly first. Therefore it is rarely a 1:1 comparison between the two schemas, which is where the [parse](../parse) project comes into play.

## License (GPL 3)
```
FGCD: Strategy guide & data collection for fighting games
Copyright (C) 2023 Asmov LLC

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
