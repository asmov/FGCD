FGCD Roadmap
===============================================================================


todo!()
-------------------------------------------------------------------------------

### Spin-off crates
- Spin-off EnumTrait crate
- Spin-off SheetEnum crate


### User-friendly spreadsheets
- Add field notes to spreadsheet definitions
  + Write field notes to spreadsheet on New
- Add sheet notes to spreadsheet definitions
  + Write sheet notes on New
- Rename model properties and spreadsheet fields to be more user-friendly


### Neutral
- Support a non-input Symbol: Neutral.

#### Examples:
- Tekken uses Neutral, denoted with a star.

### Move: During
- Support context: `{during: "Move Name"}`
  + Support parameterized contexts
- Support model "during" field for moves.
  + Support spreadsheet "during" field for moves.
- Support context elision for "during" field -> "during" context.
  + While at it, support context elision for move Category -> context as well.

All games have moves that require another move to be in progress or active.


Initial game support
-------------------------------------------------------------------------------

For now, focus on the following games:
- Mortal Kombat 1 (MK12)
- Street Fighter 6 (SF6)
- Tekken 8 (TKN8) (use Tekken 7 as an example until released)

### Differences

#### Provides: Frame data

- MK12

#### Uses QCF type motions

- SF6

#### Heavy use of charging moves

- SF6

#### Categorizes moves

- MK12
- SF6


Support Research
---------------------------------------------------------------

### [EVO 2023 Lineup](https://www.evo.gg/lineup)
- Street Fighter 6 (SF6)
- The King of Fighters XV (KOFXV)
- Melty Blood: Type Lumina ()
- Guilty Gear Strive (GGS)
- ~~Mortal Kombat 11 (MK11)~~ Mortal Kombat 1 (MK12)
- Dragon Ball Fighter Z (DBZ)
- ~~Tekken 7 (TKN7)~~ Tekken 8 (TKN8)
- Ultimate Marvel VS Capcom 3 (MVC3)

### SteamDB Popularity Stats (2023-10-05)
[SteamDB: Comparison of Lineup](https://steamdb.info/charts/?compare=357190,389730,678950,1364780,1372280,1384160,1498570,1971870)

- Street Fighter 6 (SF6)
  - 24-Hour Peak: 22,004
  - All-time Peak: 70,573
- Mortal Kombat 1 (MK12)
  - 24-Hour Peak: 11,524
  - All-time Peak: 38,129
- Tekken 7 (TKN7)
  - 24-Hour Peak: 4,404
  - All-time Peak: 18,966
- Guilty Gear Strive (GGS)
  - 24-Hour Peak: 2,036
  - All-time Peak: 31,156
- Dragon Ball Fighter Z (DBZ)
  - 24-Hour Peak: 1,231
  - All-time Peak: 44,303
- The King of Fighters XV (KOFXV)
  - 24-Hour Peak: 618
  - All-time Peak: 8,226
- Melty Blood: Type Lumina ()
  - 24-Hour Peak: 192
  - All-time Peak: 13,182
- Ultimate Marvel VS Capcom 3 (MVC3)
  - 24-Hour Peak: 175
  - All-time Peak: 2,487

